use indexmap::IndexMap;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::attribute_schema;
use crate::attribute_schema::AttributeSchemaMap;
use crate::availability_step::AvailabilityStep;
use crate::canonical_names::CanonicalNames;
use crate::compile_step::CompileStep;
use crate::consume_step::ConsumeStep;
use crate::diagnostics::Error;
use crate::diagnostics::ErrorKind;
use crate::experimental_flags::ExperimentalFlag;
use crate::experimental_flags::ExperimentalFlags;
use crate::flat_ast::*;
use crate::name::NamingContext;
use crate::names::{OwnedLibraryName, OwnedQualifiedName};
use crate::raw_ast;
use crate::raw_ast::LibraryDeclaration;
pub use crate::raw_ast::RawDecl;
use crate::reporter::Reporter;
use crate::resolve_step::ResolveStep;
use crate::source_file::{SourceFile, VirtualSourceFile};
use crate::source_span::SourceSpan;
use crate::step::Step;
use crate::token::TokenSubkind;
use crate::versioning_types::Availability;
use crate::versioning_types::VersionSelection;
pub use protocols::compute_method_ordinal;

pub(crate) mod aliases;
pub(crate) mod attributes;
pub(crate) mod bits;
pub(crate) mod constants;
pub(crate) mod dependencies;
pub(crate) mod enums;
pub(crate) mod protocols;
pub(crate) mod resources;
pub(crate) mod services;
pub(crate) mod structs;
pub(crate) mod tables;
pub(crate) mod unions;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberKind {
    EnumValue,
    Member,
    TableField,
    UnionMember,
    StructMember,
    ResourceProperty,
    ServiceMember,
    Method,
}

impl std::fmt::Display for MemberKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::EnumValue => "member",
            Self::Member => "member",
            Self::TableField => "table field",
            Self::UnionMember => "union member",
            Self::StructMember => "struct member",
            Self::ResourceProperty => "resource property",
            Self::ServiceMember => "service member",
            Self::Method => "method",
        };
        write!(f, "{}", s)
    }
}

pub fn to_camel_case(s: &str) -> String {
    let mut camel = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            camel.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            camel.push(c);
        }
    }
    camel
}

pub struct Compiler<'node, 'src> {
    // Compiled shapes for types
    pub shapes: HashMap<OwnedQualifiedName, TypeShape>,
    pub source_files: Vec<&'src SourceFile>,
    pub reporter: &'src Reporter<'src>,

    // State
    pub library_name: OwnedLibraryName,
    pub library_decl: Option<LibraryDeclaration<'src>>,
    pub raw_decls: HashMap<OwnedQualifiedName, RawDecl<'node, 'src>>,
    /// Internal mapping from a declaration's qualified name to its kind.
    /// This is used heavily during compilation (resolution, type checking,
    /// layout calculation) for fast unordered lookups.
    pub decl_kinds: HashMap<OwnedQualifiedName, DeclarationKind>,
    pub sorted_names: Vec<OwnedQualifiedName>,

    // Outputs
    pub declarations: Declarations,

    pub declaration_order: Vec<String>,
    pub decl_availability: HashMap<OwnedQualifiedName, Availability>,
    pub member_availability: HashMap<usize, Availability>,
    pub version_selection: VersionSelection,
    pub compiling_shapes: HashSet<OwnedQualifiedName>,
    /// A mapping of imported library dependencies to their compiled declarations.
    ///
    /// The outer map is keyed by `OwnedLibraryName` (the name of the dependency).
    /// The inner `IndexMap` stores the declarations belonging to that specific library:
    /// - Key (`String`): The fully qualified name of the declaration (e.g., `"fuchsia.some.lib/MyStruct"`).
    /// - Value (`DependencyDeclaration`): A structured representation of the minimal IR schema for
    ///   the declaration.
    ///
    /// This representation provides necessary metadata—such as memory layout
    /// shapes, padding flags, and `max_handles`—for dependent types in the compiling library
    /// to correctly compute their own shapes and behaviors across boundaries.
    pub dependency_declarations:
        BTreeMap<OwnedLibraryName, IndexMap<String, DependencyDeclaration>>,
    pub inline_names: HashMap<usize, String>,
    pub compiled_decls: HashSet<OwnedQualifiedName>,
    pub generated_source_file: VirtualSourceFile,
    pub skip_eager_compile: bool,
    pub anonymous_structs: HashSet<OwnedQualifiedName>,
    pub experimental_flags: ExperimentalFlags,
    pub attribute_schemas: AttributeSchemaMap,
    pub library_imports: HashMap<OwnedLibraryName, raw_ast::UsingDeclaration<'src>>,
    pub used_imports: std::cell::RefCell<HashSet<OwnedLibraryName>>,
    pub allow_unused_imports: bool,
}

impl<'node, 'src> Compiler<'node, 'src> {
    pub fn new(reporter: &'src Reporter<'src>) -> Self {
        Self {
            shapes: HashMap::new(),
            source_files: Vec::new(),
            reporter,
            library_name: OwnedLibraryName::new("unknown".to_string()),
            library_decl: None,
            raw_decls: HashMap::new(),
            decl_kinds: HashMap::new(),
            sorted_names: Vec::new(),
            declarations: Declarations::new(),

            declaration_order: Vec::new(),
            decl_availability: HashMap::new(),
            member_availability: HashMap::new(),
            version_selection: VersionSelection::new(),
            compiling_shapes: HashSet::new(),
            dependency_declarations: BTreeMap::new(),
            inline_names: HashMap::new(),
            compiled_decls: HashSet::new(),
            generated_source_file: VirtualSourceFile::new("generated".to_string()),
            skip_eager_compile: false,
            anonymous_structs: HashSet::new(),
            experimental_flags: ExperimentalFlags::new(),
            attribute_schemas: AttributeSchemaMap::new(),
            library_imports: HashMap::new(),
            used_imports: std::cell::RefCell::new(HashSet::new()),
            allow_unused_imports: false,
        }
    }

    pub fn generated_location(&self, text: &str) -> Location {
        let span = self.generated_source_file.add_line(text);
        let pos = span.position();
        Location {
            filename: span.source_file.filename().to_string(),
            line: pos.line,
            column: pos.column,
            length: span.data.len(),
        }
    }

    pub fn resolve_constant_decl<'a>(&'a self, name: &'a str) -> Option<(String, Option<String>)> {
        // returns (full_decl_name, maybe_member_name)
        let mut full_name = name.to_string();
        if !full_name.contains('/') {
            full_name = format!("{}/{}", self.library_name, name);
        }
        if self.raw_decls.contains_key::<str>(full_name.as_ref()) {
            return Some((full_name, None));
        }

        if let Some((type_name, member_name)) = name.rsplit_once('.') {
            let mut type_full_name = type_name.to_string();
            if !type_full_name.contains('/') {
                let local_fqn = self.library_name.with_declaration(type_name);
                if self.raw_decls.contains_key(&local_fqn) {
                    type_full_name = local_fqn.as_string();
                } else if let Some((lib_prefix, rest)) = type_name.split_once('.') {
                    let mut actual_lib = lib_prefix.to_string();
                    if let Some(import) = self.library_imports.get(lib_prefix) {
                        self.used_imports
                            .borrow_mut()
                            .insert(OwnedLibraryName::new(lib_prefix.to_string()));
                        actual_lib = import.using_path.to_string();
                    }
                    let dep_fqn = OwnedLibraryName::new(actual_lib).with_declaration(rest);
                    if self.raw_decls.contains_key(&dep_fqn) {
                        type_full_name = dep_fqn.as_string();
                    }
                } else if let Some(import) = self.library_imports.get(type_name) {
                    self.used_imports
                        .borrow_mut()
                        .insert(OwnedLibraryName::new(type_name.to_string()));
                    let dep_fqn = OwnedLibraryName::new(import.using_path.to_string())
                        .with_declaration(member_name);
                    if self.raw_decls.contains_key(&dep_fqn) {
                        return Some((dep_fqn.as_string(), None));
                    }
                }
            }
            if self.raw_decls.contains_key::<str>(type_full_name.as_ref()) {
                return Some((type_full_name, Some(member_name.to_string())));
            }

            let imported_name =
                OwnedLibraryName::new(type_name.to_string()).with_declaration(member_name);
            if self.raw_decls.contains_key(&imported_name) {
                return Some((imported_name.as_string(), None));
            }
        }
        None
    }

    pub fn verify_used_imports(&self) {
        if self.allow_unused_imports {
            return;
        }
        let has_errors = self
            .reporter
            .diagnostics()
            .iter()
            .any(|d| d.def.kind() == ErrorKind::Error);
        if has_errors {
            return;
        }
        let used = self.used_imports.borrow();
        for (local_name, decl) in &self.library_imports {
            if !used.contains(local_name) {
                let span = unsafe {
                    std::mem::transmute::<SourceSpan<'_>, SourceSpan<'_>>(
                        decl.using_path.element.span(),
                    )
                };
                self.reporter.fail(
                    Error::ErrUnusedImport,
                    span,
                    &[&self.library_name, &decl.using_path.to_string()],
                );
            }
        }
    }

    pub fn compile(
        &mut self,
        main_files: &'node [raw_ast::File<'src>],
        dependency_files: &'node [raw_ast::File<'src>],
        source_files: &[&'src SourceFile],
    ) -> Result<Root, String> {
        self.source_files = source_files.to_vec();

        // 1. Consume
        let mut consume = ConsumeStep {
            main_files,
            dependency_files,
        };
        consume.run(self);
        self.verify_attributes();

        // 2. Resolve
        // 1.5. Availability
        let mut avail = AvailabilityStep;
        avail.run(self);

        let mut resolve = ResolveStep;
        resolve.run(self);

        // 3. Compile
        let mut compile = CompileStep;
        compile.run(self);

        self.verify_used_imports();
        // Fixup max_handles for resources in cycles
        for decl in self.declarations.structs_mut() {
            if decl.resource && decl.type_shape.depth == u32::MAX {
                decl.type_shape.max_handles = u32::MAX;
            }
        }
        for decl in self.declarations.tables_mut() {
            if decl.resource && decl.type_shape.depth == u32::MAX {
                decl.type_shape.max_handles = u32::MAX;
            }
        }
        for decl in self.declarations.unions_mut() {
            if decl.resource && decl.type_shape.depth == u32::MAX {
                decl.type_shape.max_handles = u32::MAX;
            }
        }

        self.patch_member_shapes();
        self.recompute_declaration_order();

        // Sort declarations by name to match fidlc output order (alphabetical)
        self.declarations.sort_by(|a, b| a.name.cmp(&b.name));

        let platform = if self.is_versioned_library() {
            self.library_name.versioning_platform().to_string()
        } else {
            "unversioned".to_string()
        };

        let mut used_deps = HashSet::new();

        fn extract_deps_from_type(
            ty: &Type,
            deps: &mut HashSet<OwnedLibraryName>,
            compiler: &Compiler,
        ) {
            let identifier = ty.identifier();
            let mut target_id = identifier.as_deref();
            if let Some(alias) = &ty.experimental_maybe_from_alias {
                target_id = Some(alias.name.as_ref());
            }

            if let Some(id) = target_id
                && let Some(pos) = id.find('/')
            {
                let d = id[..pos].to_string();
                if compiler.library_name.to_string() != *d {
                    deps.insert(OwnedLibraryName::new(d.clone()));
                }

                if compiler.anonymous_structs.contains(id)
                    && let Some(s) = compiler.declarations.structs().find(|s| s.name == id)
                {
                    for m in &s.members {
                        extract_deps_from_type(&m.type_, deps, compiler);
                    }
                }
            }
            if let Some(inner) = ty.element_type()
                && ty.experimental_maybe_from_alias.is_none()
            {
                extract_deps_from_type(inner, deps, compiler);
            }
            if let Some(proto) = ty.protocol()
                && let Some(pos) = proto.find('/')
            {
                let d = proto[..pos].to_string();
                if compiler.library_name.to_string() != *d {
                    deps.insert(OwnedLibraryName::new(d));
                }
            }
            if let Some(res) = ty.resource_identifier()
                && let Some(pos) = res.find('/')
            {
                let d = res[..pos].to_string();
                if compiler.library_name.to_string() != *d {
                    deps.insert(OwnedLibraryName::new(d));
                }
            }
            if let Some(c_name) = &ty.maybe_size_constant_name {
                if let Some(pos) = c_name.find('/') {
                    let d = c_name[..pos].to_string();
                    if compiler.library_name.to_string() != *d {
                        deps.insert(OwnedLibraryName::new(d));
                    }
                } else if c_name.contains('.') {
                    let d = c_name.split('.').next().unwrap().to_string();
                    if compiler.library_name.to_string() != *d {
                        deps.insert(OwnedLibraryName::new(d));
                    }
                }
            }
        }

        for u in self.declarations.unions() {
            if u.name
                .as_string()
                .starts_with(&format!("{}/", self.library_name))
            {
                for m in &u.members {
                    if let Some(t) = &m.type_ {
                        extract_deps_from_type(t, &mut used_deps, self);
                    }
                }
            }
        }
        for a in self.declarations.aliases() {
            if a.name
                .as_string()
                .starts_with(&format!("{}/", self.library_name))
            {
                let id = &a.partial_type_ctor.name;
                if let Some(pos) = id.find('/') {
                    let d = id[..pos].to_string();
                    if d != self.library_name.to_string() {
                        used_deps.insert(OwnedLibraryName::new(d));
                    }
                }
            }
        }
        for s in self.declarations.structs() {
            if s.name
                .as_string()
                .starts_with(&format!("{}/", self.library_name))
            {
                for m in &s.members {
                    extract_deps_from_type(&m.type_, &mut used_deps, self);
                }
            }
        }
        let mut visited_protocols = HashSet::new();
        fn extract_deps_from_protocol(
            p_name: &str,
            deps: &mut HashSet<OwnedLibraryName>,
            compiler: &Compiler,
            visited: &mut HashSet<String>,
        ) {
            if !visited.insert(p_name.to_string()) {
                return;
            }

            // Find protocol in main lib or external
            let mut methods = vec![];
            let mut composed = vec![];

            if let Some(p) = compiler.declarations.protocols().find(|p| p.name == p_name) {
                methods.extend(p.methods.iter().cloned());
                composed.extend(p.composed_protocols.iter().cloned());
            }

            for m in methods {
                if let Some(req) = &m.maybe_request_payload {
                    extract_deps_from_type(req, deps, compiler);
                }
                if let Some(res) = &m.maybe_response_payload {
                    extract_deps_from_type(res, deps, compiler);
                }
                if let Some(suc) = &m.maybe_response_success_type {
                    extract_deps_from_type(suc, deps, compiler);
                }
                if let Some(err) = &m.maybe_response_err_type {
                    extract_deps_from_type(err, deps, compiler);
                }
            }

            for c in composed {
                extract_deps_from_protocol(&c.name.as_string(), deps, compiler, visited);
            }
        }

        for p in self.declarations.protocols() {
            extract_deps_from_protocol(
                &p.name.as_string(),
                &mut used_deps,
                self,
                &mut visited_protocols,
            );
        }

        let mut library_dependencies = vec![];
        for (name, declarations) in &self.dependency_declarations {
            let using_stmt = format!("using {};", name);
            if used_deps.contains(name)
                || main_files.iter().any(|f| {
                    f.library_decl.as_ref().map(|l| l.path.to_string())
                        == Some(self.library_name.to_string())
                        && f.element
                            .start_token
                            .span
                            .source_file
                            .data()
                            .contains(&using_stmt)
                })
            {
                let mut sorted_declarations = IndexMap::new();
                let mut all_dep_decls: Vec<_> = declarations.iter().collect();
                all_dep_decls.sort_by(|a, b| a.0.cmp(b.0));

                let order = [
                    DeclarationKind::Bits,
                    DeclarationKind::Const,
                    DeclarationKind::Enum,
                    DeclarationKind::ExperimentalResource,
                    DeclarationKind::Protocol,
                    DeclarationKind::Service,
                    DeclarationKind::Struct,
                    DeclarationKind::Table,
                    DeclarationKind::Union,
                    DeclarationKind::Overlay,
                    DeclarationKind::Alias,
                    DeclarationKind::NewType,
                ];

                for kind_group in &order {
                    for (decl_name, decl_obj) in &all_dep_decls {
                        if &decl_obj.kind == kind_group {
                            sorted_declarations.insert((*decl_name).clone(), (*decl_obj).clone());
                        }
                    }
                }

                library_dependencies.push(LibraryDependency {
                    name: name.to_string(),
                    declarations: sorted_declarations,
                });
            }
        }
        library_dependencies.sort_by(|a, b| a.name.cmp(&b.name));
        let json_root = Root {
            name: self.library_name.to_string(),
            platform,
            available: Some(self.version_selection.as_available_map()),
            maybe_attributes: main_files
                .iter()
                .filter(|f| {
                    f.library_decl.as_ref().map(|l| l.path.to_string())
                        == Some(self.library_name.to_string())
                })
                .find_map(|f| f.library_decl.as_ref())
                .map_or(vec![], |decl| self.compile_attribute_list(&decl.attributes)),
            experiments: {
                let mut exps = vec![];
                exps.extend(self.experimental_flags.clone().into_vec());
                exps
            },
            library_dependencies,
            bits_declarations: self.declarations.bits().cloned().collect(),
            const_declarations: self.declarations.consts().cloned().collect(),
            enum_declarations: self
                .declarations
                .enums()
                .filter(|d| {
                    d.name
                        .as_string()
                        .starts_with(&format!("{}/", self.library_name))
                })
                .cloned()
                .collect(),
            experimental_resource_declarations: self
                .declarations
                .experimental_resources()
                .cloned()
                .collect(),
            protocol_declarations: self
                .declarations
                .protocols()
                .filter(|d| {
                    d.name
                        .as_string()
                        .starts_with(&format!("{}/", self.library_name))
                })
                .cloned()
                .collect(),
            service_declarations: self.declarations.services().cloned().collect(),
            struct_declarations: self
                .declarations
                .structs()
                .filter(|d| {
                    d.name
                        .as_string()
                        .starts_with(&format!("{}/", self.library_name))
                })
                .cloned()
                .collect(),
            external_struct_declarations: self
                .declarations
                .structs()
                .filter(|d| {
                    !d.name
                        .as_string()
                        .starts_with(&format!("{}/", self.library_name))
                })
                .cloned()
                .collect(),
            table_declarations: self.declarations.tables().cloned().collect(),
            union_declarations: self.declarations.unions().cloned().collect(),
            overlay_declarations: if self
                .experimental_flags
                .is_enabled(ExperimentalFlag::ZxCTypes)
            {
                Some(self.declarations.overlays().cloned().collect())
            } else {
                None
            },
            alias_declarations: self.declarations.aliases().cloned().collect(),
            new_type_declarations: self.declarations.new_types().cloned().collect(),
            declaration_order: self.declaration_order.clone(),
        };

        let has_errors = self
            .reporter
            .diagnostics()
            .iter()
            .any(|d| d.def.kind() == ErrorKind::Error);
        let has_warnings = self
            .reporter
            .diagnostics()
            .iter()
            .any(|d| d.def.kind() == ErrorKind::Warning);

        if has_errors || (self.reporter.warnings_as_errors && has_warnings) {
            Err("Compilation failed".to_string())
        } else {
            Ok(json_root)
        }
    }

    fn patch_member_shapes(&mut self) {
        let shapes = self.shapes.clone();
        let mut struct_names = HashSet::new();

        for (name, decl) in &self.raw_decls {
            match decl {
                RawDecl::Struct(_) => {
                    struct_names.insert(name.to_string());
                }
                RawDecl::Type(t) => {
                    if let raw_ast::Layout::Struct(_) = t.layout {
                        struct_names.insert(name.to_string());
                    }
                }
                _ => {}
            }
        }
        for decls in self.dependency_declarations.values() {
            for (name, val) in decls {
                if val.kind == DeclarationKind::Struct {
                    struct_names.insert(name.to_string());
                }
            }
        }
        let envelope = |shape: &TypeShape| -> TypeShape {
            let inlined = shape.inline_size <= 4;
            let align = if inlined { 4 } else { 8 };
            let padding = (align - (shape.inline_size % align)) % align;
            let added_ool = if inlined {
                0
            } else {
                shape.inline_size.saturating_add(padding)
            };
            TypeShape {
                inline_size: 8,
                alignment: 8,
                depth: shape.depth.saturating_add(1),
                max_handles: shape.max_handles,
                max_out_of_line: shape.max_out_of_line.saturating_add(added_ool),
                has_padding: shape.has_padding || padding > 0,
                has_flexible_envelope: shape.has_flexible_envelope,
            }
        };

        for decl in self.declarations.structs_mut() {
            let mut offset = 0u32;
            let mut alignment = 1u32;
            let mut sum_ool = 0u32;
            let mut sum_handles = 0u32;
            let mut max_depth = 0u32;
            let mut has_padding = false;
            let mut has_flex = false;
            let mut overflowed = false;

            for member in &mut decl.members {
                Self::update_type_shape(&mut member.type_, &shapes, &struct_names);
                let type_shape = &member.type_.type_shape;

                let align = type_shape.alignment;
                let size = type_shape.inline_size;

                if align > alignment {
                    alignment = align;
                }

                sum_handles = sum_handles.saturating_add(type_shape.max_handles);
                sum_ool = sum_ool.saturating_add(type_shape.max_out_of_line);

                if type_shape.depth == u32::MAX {
                    max_depth = u32::MAX;
                } else if max_depth != u32::MAX && type_shape.depth > max_depth {
                    max_depth = type_shape.depth;
                }

                let padding_before = (align - (offset % align)) % align;

                let maybe_offset = offset.checked_add(padding_before);
                if maybe_offset.is_none() && !overflowed {
                    overflowed = true;
                    if let Some(raw_decl) = self.raw_decls.get::<str>(decl.base.name.as_ref()) {
                        let span = raw_decl.element().span();
                        self.reporter.fail(
                            Error::ErrTypeShapeIntegerOverflow,
                            span,
                            &[&offset, &"+", &padding_before],
                        );
                    }
                }
                offset = maybe_offset.unwrap_or(u32::MAX);

                member.field_shape.offset = offset;

                let maybe_offset2 = offset.checked_add(size);
                if maybe_offset2.is_none() && !overflowed {
                    overflowed = true;
                    if let Some(raw_decl) = self.raw_decls.get::<str>(decl.base.name.as_ref()) {
                        let span = raw_decl.element().span();
                        self.reporter.fail(
                            Error::ErrTypeShapeIntegerOverflow,
                            span,
                            &[&offset, &"+", &size],
                        );
                    }
                }
                offset = maybe_offset2.unwrap_or(u32::MAX);

                has_flex |= type_shape.has_flexible_envelope;
            }

            let final_padding = (alignment - (offset % alignment)) % alignment;
            let total_size = if offset == 0 && final_padding == 0 {
                1
            } else {
                let maybe_total = offset.checked_add(final_padding);
                if maybe_total.is_none() && !overflowed {
                    overflowed = true;
                    if let Some(raw_decl) = self.raw_decls.get::<str>(decl.base.name.as_ref()) {
                        let span = raw_decl.element().span();
                        self.reporter.fail(
                            Error::ErrTypeShapeIntegerOverflow,
                            span,
                            &[&offset, &"+", &final_padding],
                        );
                    }
                }
                maybe_total.unwrap_or(u32::MAX)
            };

            for i in 0..decl.members.len() {
                let next_offset = if i + 1 < decl.members.len() {
                    decl.members[i + 1].field_shape.offset
                } else {
                    total_size
                };
                let current_end = decl.members[i]
                    .field_shape
                    .offset
                    .saturating_add(decl.members[i].type_.type_shape.inline_size);
                decl.members[i].field_shape.padding = next_offset.saturating_sub(current_end);
                has_padding |= decl.members[i].field_shape.padding > 0
                    || decl.members[i].type_.type_shape.has_padding;
            }

            decl.type_shape.inline_size = total_size;
            decl.type_shape.alignment = alignment;
            decl.type_shape.depth = max_depth;
            decl.type_shape.max_out_of_line = sum_ool;
            decl.type_shape.max_handles = sum_handles;
            decl.type_shape.has_padding = has_padding || final_padding > 0;
            decl.type_shape.has_flexible_envelope = has_flex;

            if !overflowed
                && total_size > 65535
                && let Some(raw_decl) = self.raw_decls.get::<str>(decl.base.name.as_ref())
            {
                let span = raw_decl.element().span();
                let decl_fqn = decl.name.clone();
                let display_name = decl_fqn.declaration();
                self.reporter.fail(
                    Error::ErrInlineSizeExceedsLimit,
                    span,
                    &[&display_name, &total_size, &65535u32],
                );
            }
        }
        for decl in self.declarations.unions_mut() {
            let mut max_ool = 0u32;
            let mut max_handles = 0u32;
            let mut has_padding = false;
            let mut has_flex = false;
            for member in &mut decl.members {
                if let Some(ty) = &mut member.type_ {
                    Self::update_type_shape(ty, &shapes, &struct_names);
                    let env = envelope(&ty.type_shape);
                    max_ool = std::cmp::max(max_ool, env.max_out_of_line);
                    max_handles = std::cmp::max(max_handles, env.max_handles);
                    has_padding |= env.has_padding;
                    has_flex |= env.has_flexible_envelope;
                }
            }
            if decl.type_shape.depth != u32::MAX {
                decl.type_shape.max_out_of_line = max_ool;
                decl.type_shape.max_handles = max_handles;
                decl.type_shape.has_padding = has_padding;
                let is_flexible = decl
                    .base
                    .maybe_attributes
                    .iter()
                    .any(|a| a.name == "flexible")
                    || !decl.strict;
                // Also check if any member has the flexible trait
                decl.type_shape.has_flexible_envelope = has_flex || is_flexible;
            }
        }
        for decl in self.declarations.tables_mut() {
            let mut max_ool = 0u32;
            let mut max_handles = 0u32;
            let mut has_padding = false;
            let mut max_ordinal = 0u32;
            for member in &mut decl.members {
                max_ordinal = std::cmp::max(max_ordinal, member.ordinal);
                if let Some(ty) = &mut member.type_ {
                    Self::update_type_shape(ty, &shapes, &struct_names);
                    let env = envelope(&ty.type_shape);
                    max_ool = max_ool.saturating_add(env.max_out_of_line);
                    max_handles = max_handles.saturating_add(env.max_handles);
                    has_padding |= env.has_padding;
                }
            }
            if decl.type_shape.depth != u32::MAX {
                decl.type_shape.max_out_of_line =
                    max_ool.saturating_add(max_ordinal.saturating_mul(8));
                decl.type_shape.max_handles = max_handles;
                decl.type_shape.has_padding = has_padding;
                decl.type_shape.has_flexible_envelope = true;
            }
        }
        for decl in self.declarations.aliases_mut() {
            Self::update_type_shape(&mut decl.type_, &shapes, &struct_names);
        }
        for decl in self.declarations.new_types_mut() {
            Self::update_type_shape(&mut decl.type_, &shapes, &struct_names);
        }
        for decl in self.declarations.consts_mut() {
            Self::update_type_shape(&mut decl.type_, &shapes, &struct_names);
        }
        for decl in self.declarations.protocols_mut() {
            for method in &mut decl.methods {
                if let Some(ty) = &mut method.maybe_request_payload {
                    Self::update_type_shape(ty, &shapes, &struct_names);
                }
                if let Some(ty) = &mut method.maybe_response_payload {
                    Self::update_type_shape(ty, &shapes, &struct_names);
                }
                if let Some(ty) = &mut method.maybe_response_success_type {
                    Self::update_type_shape(ty, &shapes, &struct_names);
                }
                if let Some(ty) = &mut method.maybe_response_err_type {
                    Self::update_type_shape(ty, &shapes, &struct_names);
                }
            }
        }
    }

    fn update_type_shape(
        ty: &mut Type,
        shapes: &HashMap<OwnedQualifiedName, TypeShape>,
        struct_names: &HashSet<String>,
    ) {
        if ty.kind() == TypeKind::Identifier
            && let Some(ref id) = ty.identifier()
            && let Some(shape) = shapes.get::<str>(id.as_str())
        {
            if ty.nullable() && struct_names.contains(id.as_str()) {
                let inner_inline = shape.inline_size;
                let padding = (8 - (inner_inline % 8)) % 8;
                let max_out_of_line = shape
                    .max_out_of_line
                    .saturating_add(inner_inline.saturating_add(padding));

                ty.type_shape = TypeShape {
                    inline_size: 8,
                    alignment: 8,
                    depth: shape.depth.saturating_add(1),
                    max_handles: shape.max_handles,
                    max_out_of_line,
                    has_padding: shape.has_padding || padding > 0,
                    has_flexible_envelope: shape.has_flexible_envelope,
                };
            } else {
                ty.type_shape = shape.clone();
            }
        }
        let mut inner_shape_opt = None;
        if let Some(inner) = ty.element_type_mut() {
            Self::update_type_shape(inner, shapes, struct_names);
            inner_shape_opt = Some(inner.type_shape.clone());
        }

        if let Some(inner_shape) = inner_shape_opt {
            if ty.kind() == TypeKind::Vector {
                let count = ty.maybe_element_count().unwrap_or(u32::MAX);

                let new_depth = inner_shape.depth.saturating_add(1);
                let elem_size = inner_shape.inline_size;
                let elem_ool = inner_shape.max_out_of_line;

                let content_size = count.saturating_mul(elem_size.saturating_add(elem_ool));
                // OOL padding logic: vector body is contiguous. If content size not 8-aligned?
                // Actually FIDL vector padding is 8-byte alignment of the body? Yes.
                let max_ool = if content_size % 8 == 0 {
                    content_size
                } else {
                    content_size.saturating_add(8 - (content_size % 8))
                };

                let max_handles = count.saturating_mul(inner_shape.max_handles);
                let has_padding = inner_shape.has_padding || !elem_size.is_multiple_of(8);

                ty.type_shape = TypeShape {
                    inline_size: 16,
                    alignment: 8,
                    depth: new_depth,
                    max_handles,
                    max_out_of_line: max_ool,
                    has_padding,
                    has_flexible_envelope: inner_shape.has_flexible_envelope,
                };
            } else if ty.kind() == TypeKind::Array {
                let count = ty.element_count().unwrap_or(0);

                let elem_size = inner_shape.inline_size;
                let elem_ool = inner_shape.max_out_of_line;
                let depth = inner_shape.depth;

                let max_handles = count.saturating_mul(inner_shape.max_handles);
                let max_out_of_line = count.saturating_mul(elem_ool);

                ty.type_shape = TypeShape {
                    inline_size: count.saturating_mul(elem_size),
                    alignment: inner_shape.alignment,
                    depth,
                    max_handles,
                    max_out_of_line,
                    has_padding: inner_shape.has_padding,
                    has_flexible_envelope: inner_shape.has_flexible_envelope,
                };
            }
        }
    }

    pub fn compile_partial_type_ctor(
        &mut self,
        type_ctor: &raw_ast::TypeConstructor<'src>,
        library_name: &str,
    ) -> PartialTypeCtor {
        let name = if let raw_ast::LayoutParameter::Identifier(id) = &type_ctor.layout {
            let mut n = id.to_string();
            if n == "client_end" {
                if let Some(param) = type_ctor.parameters.first() {
                    if let raw_ast::LayoutParameter::Identifier(id2) = &param.layout {
                        n = id2.to_string();
                    }
                } else if let Some(constraint) = type_ctor.constraints.first()
                    && let raw_ast::Constant::Identifier(id2) = constraint
                {
                    n = id2.identifier.to_string();
                }
                if n.contains('.') {
                    n = n.replace('.', "/");
                } else if !n.contains('/') {
                    n = format!("{}/{}", library_name, n);
                }
            } else if n == "server_end" {
                n = "request".to_string();
            } else if !["array", "vector", "string", "bytes", "box"].contains(&n.as_str()) {
                if n.contains('.') {
                    n = n.replace('.', "/");
                } else if !n.contains('/') {
                    let full = OwnedLibraryName::new(library_name.to_string()).with_declaration(&n);
                    if self.decl_kinds.contains_key(&full)
                        || self.shapes.contains_key::<str>(n.as_str())
                    {
                        n = full.as_string();
                    }
                }

                // Resolving aliases recursively to base primitive if applicable!
                // Only primitive types are substituted back in PartialTypeCtor output
                if let Some(RawDecl::Alias(a)) = self
                    .raw_decls
                    .get::<str>(n.as_ref())
                    .or_else(|| self.get_underlying_decl(&n))
                {
                    let resolved = self.resolve_type(&a.type_ctor, library_name, None);
                    if let Type::Primitive(p) = &resolved {
                        n = p.subtype.to_string();
                    }
                }
            }
            n
        } else {
            "".to_string()
        };

        let mut args = Vec::new();
        if let raw_ast::LayoutParameter::Identifier(id) = &type_ctor.layout {
            let n = id.to_string();
            if n == "server_end" {
                let mut p_name = "".to_string();
                if let Some(param) = type_ctor.parameters.first() {
                    if let raw_ast::LayoutParameter::Identifier(id2) = &param.layout {
                        p_name = id2.to_string();
                    }
                } else if let Some(constraint) = type_ctor.constraints.first()
                    && let raw_ast::Constant::Identifier(id2) = constraint
                {
                    p_name = id2.identifier.to_string();
                }
                if !p_name.is_empty() {
                    let full = if p_name.contains('.') {
                        p_name.replace('.', "/")
                    } else if p_name.contains('/') {
                        p_name
                    } else {
                        format!("{}/{}", library_name, p_name)
                    };
                    args.push(PartialTypeCtor {
                        name: full,
                        args: vec![],
                        nullable: false,
                        maybe_size: None,
                        handle_rights: None,
                    });
                }
            } else if n == "box" || n == "array" {
                if let Some(param) = type_ctor.parameters.first() {
                    args.push(self.compile_partial_type_ctor(param, library_name));
                }
            } else if n != "client_end" {
                for param in &type_ctor.parameters {
                    args.push(self.compile_partial_type_ctor(param, library_name));
                }
            }
        }

        let mut maybe_size = None;
        if let raw_ast::LayoutParameter::Identifier(id) = &type_ctor.layout {
            let n = id.to_string();
            if n == "array" {
                if type_ctor.parameters.len() > 1 {
                    if let raw_ast::LayoutParameter::Literal(lit) = &type_ctor.parameters[1].layout
                    {
                        let c = raw_ast::Constant::Literal(lit.clone());
                        maybe_size = Some(self.compile_constant(&c));
                    } else if let raw_ast::LayoutParameter::Identifier(id) =
                        &type_ctor.parameters[1].layout
                    {
                        let c = raw_ast::Constant::Identifier(raw_ast::IdentifierConstant {
                            identifier: id.clone(),
                            element: type_ctor.parameters[1].element.clone(),
                        });
                        maybe_size = Some(self.compile_constant(&c));
                    }
                }
            } else if (n == "vector" || n == "string")
                && let Some(c) = type_ctor.constraints.first()
                && !matches!(c, raw_ast::Constant::Identifier(id) if id.identifier.to_string() == "optional")
            {
                maybe_size = Some(self.compile_constant(c));
            }
        }

        let mut handle_rights = None;
        if name == "zx/Handle" || name == "handle" {
            let filtered_constraints: Vec<_> = type_ctor
                .constraints
                .iter()
                .filter(|c| !matches!(c, raw_ast::Constant::Identifier(id) if id.identifier.to_string() == "optional"))
                .collect();
            if filtered_constraints.len() > 1 {
                handle_rights = Some(self.compile_constant(filtered_constraints[1]));
            }
        }

        PartialTypeCtor {
            nullable: type_ctor.nullable || type_ctor.constraints.iter().any(|c| matches!(c, raw_ast::Constant::Identifier(id) if id.identifier.to_string() == "optional")),
            name,
            args,
            maybe_size,
            handle_rights,
        }
    }
    pub fn compile_decl_by_name(&mut self, name: &str) {
        if self.compiled_decls.contains::<str>(name) || self.compiling_shapes.contains::<str>(name)
        {
            return;
        }

        if name == "zx/Handle" {
            let obj = DependencyDeclaration {
                kind: DeclarationKind::ExperimentalResource,
                type_shape: None,
                resource: None,
            };
            self.dependency_declarations
                .entry(OwnedLibraryName::new("zx".to_string()))
                .or_default()
                .insert("zx/Handle".to_string(), obj);
            return;
        }

        let decl = if let Some(d) = self.raw_decls.get::<str>(name.as_ref()) {
            d.clone()
        } else {
            return;
        };

        self.compiling_shapes
            .insert(OwnedQualifiedName::from(name.to_string()));

        let mut parts = name.splitn(2, '/');
        let library_name = parts.next().unwrap_or("unknown").to_string();
        let short_name_from_key = parts.next().unwrap_or("unknown");
        let is_main_library = library_name == self.library_name.to_string();

        match decl {
            RawDecl::Type(t) => {
                if let raw_ast::Layout::Struct(ref s) = t.layout {
                    let compiled = self.compile_struct(
                        t.name.data(),
                        s,
                        &library_name,
                        Some(&t.name.element),
                        None,
                        t.attributes.as_deref(),
                    );
                    self.declarations.push(Decl::Struct(compiled));
                } else if let raw_ast::Layout::Enum(ref e) = t.layout {
                    let compiled = self.compile_enum(
                        t.name.data(),
                        e,
                        &library_name,
                        Some(&t.name.element),
                        t.attributes.as_deref(),
                        None,
                    );
                    self.declarations.push(Decl::Enum(compiled));
                } else if let raw_ast::Layout::Bits(ref b) = t.layout {
                    let compiled = self.compile_bits(
                        t.name.data(),
                        b,
                        &library_name,
                        Some(&t.name.element),
                        t.attributes.as_deref(),
                        None,
                    );
                    if is_main_library {
                        self.declarations.push(Decl::Bits(compiled));
                    }
                } else if let raw_ast::Layout::Table(ref ta) = t.layout {
                    let compiled = self.compile_table(
                        t.name.data(),
                        ta,
                        &library_name,
                        Some(&t.name.element),
                        t.attributes.as_deref(),
                        None,
                    );
                    if is_main_library {
                        self.declarations.push(Decl::Table(compiled));
                    }
                } else if let raw_ast::Layout::Union(ref u) = t.layout {
                    let compiled = self.compile_union(
                        t.name.data(),
                        u,
                        &library_name,
                        Some(&t.name.element),
                        t.attributes.as_deref(),
                        None,
                    );
                    if u.is_overlay {
                        self.declarations.push(Decl::Overlay(compiled));
                    } else {
                        self.declarations.push(Decl::Union(compiled));
                    }
                } else if let raw_ast::Layout::TypeConstructor(ref tc) = t.layout {
                    let mut existing_type_name = "unknown".to_string();
                    if let raw_ast::LayoutParameter::Identifier(id) = &tc.layout {
                        existing_type_name = id.to_string();
                    }
                    if !self
                        .experimental_flags
                        .is_enabled(ExperimentalFlag::AllowNewTypes)
                    {
                        self.reporter.fail(
                            Error::ErrNewTypesNotAllowed,
                            t.name.element.span(),
                            &[&t.name.data(), &existing_type_name],
                        );
                    }
                    let mut typ = self.resolve_type(tc, &library_name, None);
                    let alias = typ
                        .outer_alias
                        .take()
                        .or_else(|| typ.experimental_maybe_from_alias.take());
                    let compiled = NewTypeDeclaration::new(
                        format!("{}/{}", library_name, t.name.data()).into(),
                        self.get_location(&t.name.element),
                        self.is_deprecated(t.attributes.as_deref()),
                        self.compile_attribute_list(&t.attributes),
                        typ,
                        alias,
                    );
                    if is_main_library {
                        self.declarations.push(Decl::NewType(compiled));
                    }
                }
            }
            RawDecl::Struct(s) => {
                if short_name_from_key != "anonymous" {
                    let compiled = self.compile_struct(
                        short_name_from_key,
                        s,
                        &library_name,
                        None,
                        None,
                        s.attributes.as_deref(),
                    );
                    self.declarations.push(Decl::Struct(compiled));
                }
            }
            RawDecl::Enum(e) => {
                if short_name_from_key != "anonymous" {
                    let compiled = self.compile_enum(
                        short_name_from_key,
                        e,
                        &library_name,
                        None,
                        e.attributes.as_deref(),
                        None,
                    );
                    self.declarations.push(Decl::Enum(compiled));
                }
            }
            RawDecl::Bits(b) => {
                if short_name_from_key != "anonymous" {
                    let compiled = self.compile_bits(
                        short_name_from_key,
                        b,
                        &library_name,
                        None,
                        b.attributes.as_deref(),
                        None,
                    );
                    if is_main_library {
                        self.declarations.push(Decl::Bits(compiled));
                    }
                }
            }
            RawDecl::Union(u) => {
                if short_name_from_key != "anonymous" {
                    let compiled = self.compile_union(
                        short_name_from_key,
                        u,
                        &library_name,
                        None,
                        u.attributes.as_deref(),
                        None,
                    );
                    if u.is_overlay {
                        self.declarations.push(Decl::Overlay(compiled));
                    } else {
                        self.declarations.push(Decl::Union(compiled));
                    }
                }
            }
            RawDecl::Table(t) => {
                if short_name_from_key != "anonymous" {
                    let compiled = self.compile_table(
                        short_name_from_key,
                        t,
                        &library_name,
                        None,
                        t.attributes.as_deref(),
                        None,
                    );
                    if is_main_library {
                        self.declarations.push(Decl::Table(compiled));
                    }
                }
            }
            RawDecl::Protocol(p) => {
                let short_name = p.name.data();
                let compiled = self.compile_protocol(short_name, p, &library_name);

                let mut extra_to_compile = vec![];
                for m in &compiled.methods {
                    if let Some(req) = &m.maybe_request_payload
                        && let Some(id) = req.identifier()
                    {
                        extra_to_compile.push(id.clone());
                    }
                    if let Some(res) = &m.maybe_response_payload
                        && let Some(id) = res.identifier()
                    {
                        extra_to_compile.push(id.clone());
                    }
                    if let Some(suc) = &m.maybe_response_success_type
                        && let Some(id) = suc.identifier()
                    {
                        extra_to_compile.push(id.clone());
                    }
                    if let Some(err) = &m.maybe_response_err_type
                        && let Some(id) = err.identifier()
                    {
                        extra_to_compile.push(id.clone());
                    }
                }

                self.declarations.push(Decl::Protocol(compiled));

                for id in extra_to_compile {
                    if id.is_empty() {
                        continue;
                    }
                    self.compile_decl_by_name(&id);
                }
            }
            RawDecl::Service(s) => {
                let short_name = s.name.data();
                let compiled = self.compile_service(short_name, s, &library_name);
                if is_main_library {
                    self.declarations.push(Decl::Service(compiled));
                }
            }
            RawDecl::Resource(r) => {
                let short_name = r.name.data();
                let compiled = self.compile_resource(short_name, r, &library_name);
                if is_main_library {
                    self.declarations.push(Decl::ExperimentalResource(compiled));
                }
            }
            RawDecl::Const(c) => {
                let compiled = self.compile_const(c, &library_name);
                if is_main_library {
                    self.declarations.push(Decl::Const(compiled));
                }
            }
            RawDecl::Alias(a) => {
                let compiled = self.compile_alias(a, &library_name);
                if is_main_library {
                    self.declarations.push(Decl::Alias(compiled));
                }
            }
        }

        if !is_main_library {
            let kind = self
                .decl_kinds
                .get::<str>(name.as_ref())
                .copied()
                .unwrap_or(DeclarationKind::Struct);
            let decl_obj = if name == "zx/Handle" {
                DependencyDeclaration {
                    kind: DeclarationKind::ExperimentalResource,
                    type_shape: None,
                    resource: None,
                }
            } else {
                let type_shape = match kind {
                    DeclarationKind::Const
                    | DeclarationKind::Alias
                    | DeclarationKind::Protocol
                    | DeclarationKind::Service => None,
                    _ => self.shapes.get::<str>(name).cloned(),
                };
                
                let resource = self.declarations.decls().find_map(|d| {
                    match d {
                        Decl::Struct(s) if s.name.as_string() == *name => Some(s.resource),
                        Decl::Table(t) if t.name.as_string() == *name => Some(t.resource),
                        Decl::Union(u) if u.name.as_string() == *name => Some(u.resource),
                        Decl::Overlay(o) if o.name.as_string() == *name => Some(o.resource),
                        _ => None,
                    }
                });

                DependencyDeclaration { kind, type_shape, resource }
            };

            self.dependency_declarations
                .entry(OwnedLibraryName::new(library_name.clone()))
                .or_default()
                .insert(name.to_string(), decl_obj);
        }

        self.compiling_shapes.remove::<str>(name.as_ref());
        self.compiled_decls
            .insert(OwnedQualifiedName::from(name.to_string()));

        if is_main_library {}
    }

    fn check_canonical_insert(
        &mut self,
        names: &mut CanonicalNames<'src>,
        raw_name: String,
        kind: MemberKind,
        span: SourceSpan<'src>,
        is_versioned: bool,
    ) {
        if let Err((is_exact, prev_raw, prev_kind, prev_site)) =
            names.insert(raw_name.clone(), kind, span, is_versioned)
        {
            let kind_str = kind.to_string();
            let prev_kind_str = prev_kind.to_string();
            let prev_site_str = prev_site.position_str();

            if is_exact {
                self.reporter.fail(
                    Error::ErrNameCollision,
                    span,
                    &[&kind_str, &raw_name, &prev_kind_str, &prev_site_str],
                );
            } else {
                let canon = attribute_schema::canonicalize(&raw_name);
                self.reporter.fail(
                    Error::ErrNameCollisionCanonical,
                    span,
                    &[
                        &kind_str,
                        &raw_name,
                        &prev_kind_str,
                        &prev_raw,
                        &prev_site_str,
                        &canon,
                    ],
                );
            }
        }
    }

    pub fn resolve_type(
        &mut self,
        type_ctor: &'node raw_ast::TypeConstructor<'src>,
        library_name: &str,
        naming_context: Option<std::rc::Rc<NamingContext<'src>>>,
    ) -> Type {
        let name = match &type_ctor.layout {
            raw_ast::LayoutParameter::Identifier(id) => {
                if id.components.len() > 1 {
                    let mut parts = vec![];
                    for c in &id.components[..id.components.len() - 1] {
                        parts.push(c.data());
                    }
                    let mut local_lib_name = parts.join(".");
                    if library_name == self.library_name.to_string() {
                        if let Some(import) =
                            self.library_imports.get::<str>(local_lib_name.as_str())
                        {
                            self.used_imports
                                .borrow_mut()
                                .insert(OwnedLibraryName::new(local_lib_name.clone()));
                            local_lib_name = import.using_path.to_string();
                        } else if local_lib_name != self.library_name.to_string()
                            && local_lib_name != "fidl"
                        {
                            let span_safe = unsafe {
                                std::mem::transmute::<SourceSpan<'_>, SourceSpan<'_>>(
                                    id.element.span(),
                                )
                            };
                            if id.components.len() > 2 {
                                let fallback_lib = parts[..parts.len() - 1].join(".");
                                self.reporter.fail(
                                    Error::ErrUnknownDependentLibrary,
                                    span_safe,
                                    &[&local_lib_name, &fallback_lib],
                                );
                            } else {
                                self.reporter.fail(
                                    Error::ErrNameNotFound,
                                    span_safe,
                                    &[&local_lib_name, &self.library_name],
                                );
                            }
                            return Type::unknown();
                        }
                    }
                    let id_str = format!(
                        "{}/{}",
                        local_lib_name,
                        id.components.last().unwrap().data()
                    );
                    id_str
                } else {
                    id.to_string()
                }
            }
            raw_ast::LayoutParameter::Literal(_) => {
                self.reporter
                    .fail(Error::ErrExpectedType, type_ctor.element.span(), &[]);
                return Type::unknown();
            }
            raw_ast::LayoutParameter::Type(_) => {
                panic!("Type layout not supported in resolve_type yet")
            }

            raw_ast::LayoutParameter::Inline(layout) => {
                let (_, default_name, attrs) = match &**layout {
                    raw_ast::Layout::Struct(s) => (
                        s.attributes.as_ref().is_some_and(|a| {
                            a.attributes
                                .iter()
                                .any(|attr| attr.name.data() == "generated_name")
                        }),
                        "inline_struct",
                        s.attributes.as_deref(),
                    ),
                    raw_ast::Layout::Enum(e) => (
                        e.attributes.as_ref().is_some_and(|a| {
                            a.attributes
                                .iter()
                                .any(|attr| attr.name.data() == "generated_name")
                        }),
                        "inline_enum",
                        e.attributes.as_deref(),
                    ),
                    raw_ast::Layout::Bits(b) => (
                        b.attributes.as_ref().is_some_and(|a| {
                            a.attributes
                                .iter()
                                .any(|attr| attr.name.data() == "generated_name")
                        }),
                        "inline_bits",
                        b.attributes.as_deref(),
                    ),
                    raw_ast::Layout::Union(u) => (
                        u.attributes.as_ref().is_some_and(|a| {
                            a.attributes
                                .iter()
                                .any(|attr| attr.name.data() == "generated_name")
                        }),
                        "inline_union",
                        u.attributes.as_deref(),
                    ),
                    raw_ast::Layout::Table(t) => (
                        t.attributes.as_ref().is_some_and(|a| {
                            a.attributes
                                .iter()
                                .any(|attr| attr.name.data() == "generated_name")
                        }),
                        "inline_table",
                        t.attributes.as_deref(),
                    ),
                    raw_ast::Layout::TypeConstructor(_) => (false, "inline_type", None),
                };

                if let Some(a_list) = attrs {
                    let decl_kind = default_name.strip_prefix("inline_").unwrap_or(default_name);
                    self.attribute_schemas
                        .clone()
                        .validate(self, decl_kind, true, a_list);

                    // Re-fetch the generated_name value for our own use without re-emitting errors.
                    let gen_attr = a_list
                        .attributes
                        .iter()
                        .find(|a| a.name.data() == "generated_name");
                    if let Some(a) = gen_attr
                        && let Some(arg) = a.args.first()
                        && let raw_ast::Constant::Literal(l) = &arg.value
                    {
                        // But still check if it's a valid identifier here, which isn't done by AttributeSchemaMap right now
                        let val = l.literal.value.trim_matches('"').to_string();
                        let mut is_valid = true;
                        if val.is_empty() {
                            is_valid = false;
                        } else {
                            let mut chars = val.chars();
                            let first = chars.next().unwrap();
                            if !first.is_ascii_alphabetic() {
                                is_valid = false;
                            }
                            if val.ends_with('_') || val.contains("__") {
                                is_valid = false;
                            }
                            if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
                                is_valid = false;
                            }
                        }
                        if !is_valid {
                            let span_transmuted: SourceSpan =
                                unsafe { std::mem::transmute(arg.value.element().span()) };
                            self.reporter.fail(
                                Error::ErrInvalidGeneratedName,
                                span_transmuted,
                                &[&val],
                            );
                        }
                    }
                }

                let generated_name = if let Some(a_list) = attrs {
                    a_list
                        .attributes
                        .iter()
                        .find(|a| a.name.data() == "generated_name")
                        .and_then(|a| a.args.first())
                        .map(|arg| {
                            if let raw_ast::Constant::Literal(l) = &arg.value {
                                l.literal.value.trim_matches('"').to_string()
                            } else {
                                default_name.to_string()
                            }
                        })
                } else {
                    None
                };

                if !type_ctor.parameters.is_empty() {
                    self.reporter.fail(
                        Error::ErrWrongNumberOfLayoutParameters,
                        type_ctor.element.start_token.span,
                        &[
                            &generated_name.as_deref().unwrap_or(default_name),
                            &0_usize,
                            &type_ctor.parameters.len(),
                        ],
                    );
                }

                let _decl_context = naming_context
                    .as_ref()
                    .map(|ctx| ctx.context())
                    .unwrap_or_default();

                let final_short_name = generated_name.unwrap_or_else(|| {
                    naming_context
                        .as_ref()
                        .map(|ctx| ctx.flattened_name())
                        .unwrap_or_else(|| "Unknown".to_string())
                });

                let full_name = format!("{}/{}", library_name, final_short_name);

                let mut is_collision = false;
                if let Some(_prev_decl) = self.raw_decls.get::<str>(full_name.as_ref()) {
                    // We might be evaluating a method payload that was pre-inserted by ConsumeStep
                    // If spans differ, it's a real collision
                    // Wait, actually the prev_span is the decl's span, while curr_span is type_ctor's span.
                    // For method payload, the Pre-inserted RawDecl is the struct layout itself.
                    // So we can check if the struct's start string matches.
                    // Instead of full span matching, let's just say if it's NOT in `compiled_decls` and we are inserting it now,
                    // AND it's already in `raw_decls` but NOT a pre-inserted anonymous struct, then collision.
                    // But wait, what if it IS a pre-inserted anonymous struct?
                    if !self.anonymous_structs.contains::<str>(&full_name) {
                        is_collision = true;
                    }
                }

                if is_collision {
                    let prev_decl = self.raw_decls.get::<str>(full_name.as_ref()).unwrap();
                    let prev_kind = prev_decl.kind().to_string();
                    let prev_site = prev_decl.element().span().position_str();
                    let kind = default_name.strip_prefix("inline_").unwrap_or(default_name);
                    let span_transmuted: SourceSpan =
                        unsafe { std::mem::transmute(type_ctor.element.span()) };
                    if let Some(a_list) = attrs {
                        if let Some(gen_attr) = a_list
                            .attributes
                            .iter()
                            .find(|a| a.name.data() == "generated_name")
                        {
                            let gen_span_transmuted: SourceSpan =
                                unsafe { std::mem::transmute(gen_attr.element.span()) };
                            self.reporter.fail(
                                Error::ErrNameCollision,
                                gen_span_transmuted,
                                &[
                                    &kind.to_string(),
                                    &final_short_name.to_string(),
                                    &prev_kind.to_string(),
                                    &prev_site,
                                ],
                            );
                        } else {
                            self.reporter.fail(
                                Error::ErrNameCollision,
                                span_transmuted,
                                &[
                                    &kind.to_string(),
                                    &final_short_name.to_string(),
                                    &prev_kind.to_string(),
                                    &prev_site,
                                ],
                            );
                        }
                    } else {
                        self.reporter.fail(
                            Error::ErrNameCollision,
                            span_transmuted,
                            &[
                                &kind.to_string(),
                                &final_short_name.to_string(),
                                &prev_kind.to_string(),
                                &prev_site,
                            ],
                        );
                    }
                }

                if !self.compiled_decls.contains::<str>(&full_name) {
                    match &**layout {
                        raw_ast::Layout::Struct(s) => {
                            let compiled = self.compile_struct(
                                &final_short_name,
                                s,
                                library_name,
                                None,
                                naming_context.clone(),
                                None,
                            );
                            self.declarations.push(Decl::Struct(compiled));
                            self.raw_decls.insert(
                                OwnedQualifiedName::from(full_name.clone()),
                                RawDecl::Struct(s),
                            );
                        }
                        raw_ast::Layout::Enum(e) => {
                            let compiled = self.compile_enum(
                                &final_short_name,
                                e,
                                library_name,
                                None,
                                None,
                                naming_context.clone(),
                            );
                            self.declarations.push(Decl::Enum(compiled));
                            self.raw_decls.insert(
                                OwnedQualifiedName::from(full_name.clone()),
                                RawDecl::Enum(e),
                            );
                        }
                        raw_ast::Layout::Bits(b) => {
                            let compiled = self.compile_bits(
                                &final_short_name,
                                b,
                                library_name,
                                None,
                                None,
                                naming_context.clone(),
                            );
                            self.declarations.push(Decl::Bits(compiled));
                            self.raw_decls.insert(
                                OwnedQualifiedName::from(full_name.clone()),
                                RawDecl::Bits(b),
                            );
                        }
                        raw_ast::Layout::Union(u) => {
                            let compiled = self.compile_union(
                                &final_short_name,
                                u,
                                library_name,
                                None,
                                None,
                                naming_context.clone(),
                            );
                            if u.is_overlay {
                                self.declarations.push(Decl::Overlay(compiled));
                            } else {
                                self.declarations.push(Decl::Union(compiled));
                            }
                            self.raw_decls.insert(
                                OwnedQualifiedName::from(full_name.clone()),
                                RawDecl::Union(u),
                            );
                        }
                        raw_ast::Layout::Table(t) => {
                            let compiled = self.compile_table(
                                &final_short_name,
                                t,
                                library_name,
                                None,
                                None,
                                naming_context.clone(),
                            );
                            self.declarations.push(Decl::Table(compiled));
                            self.raw_decls.insert(
                                OwnedQualifiedName::from(full_name.clone()),
                                RawDecl::Table(t),
                            );
                        }
                        _ => {}
                    }
                    self.inline_names.insert(
                        type_ctor.element.start_token.span.data.as_ptr() as usize,
                        full_name.clone(),
                    );

                    if library_name == self.library_name.to_string() {
                        self.compiled_decls
                            .insert(OwnedQualifiedName::from(full_name.clone()));
                    }
                }

                full_name
            }
        };

        let mut actual_constraints = type_ctor.constraints.clone();
        let mut nullable = type_ctor.nullable;

        if let Some(c) = actual_constraints.last()
            && let raw_ast::Constant::Identifier(id) = c
            && id.identifier.to_string() == "optional"
        {
            let mut is_nullability = true;
            // If it is in the same scope, it might resolve to a constant.
            if self.eval_constant_value(c).is_some() {
                is_nullability = false;
            }
            if is_nullability {
                actual_constraints.pop();
                nullable = true;
            }
        }

        let mut resolved_name = name.clone();
        if resolved_name.starts_with("fidl/") {
            let bare = &resolved_name["fidl/".len()..];
            if matches!(
                bare,
                "bool"
                    | "int8"
                    | "int16"
                    | "int32"
                    | "int64"
                    | "uint8"
                    | "uint16"
                    | "uint32"
                    | "uint64"
                    | "float32"
                    | "float64"
                    | "uchar"
                    | "usize64"
                    | "uintptr64"
                    | "string"
                    | "vector"
                    | "bytes"
                    | "string_array"
            ) {
                resolved_name = bare.to_string();
            }
        }
        if resolved_name == "byte" {
            resolved_name = "uint8".to_string();
        }

        let is_shadowed = if name.contains('/') {
            false
        } else {
            let bare = resolved_name
                .strip_prefix("fidl/")
                .unwrap_or(&resolved_name);
            let local_name = format!("{}/{}", library_name, bare);
            self.raw_decls.contains_key::<str>(local_name.as_ref())
                || self.raw_decls.contains_key::<str>(bare.as_ref())
        };

        match resolved_name.as_str() {
            "bool" | "int8" | "int16" | "int32" | "int64" | "uint8" | "uint16" | "uint32"
            | "uint64" | "float32" | "float64" | "uchar" | "usize64" | "uintptr64"
                if !is_shadowed =>
            {
                if !type_ctor.parameters.is_empty() {
                    self.reporter.fail(
                        Error::ErrWrongNumberOfLayoutParameters,
                        type_ctor.element.start_token.span,
                        &[&resolved_name, &0_usize, &type_ctor.parameters.len()],
                    );
                }

                if nullable {
                    self.reporter.fail(
                        Error::ErrCannotBeOptional,
                        type_ctor.element.start_token.span,
                        &[&resolved_name],
                    );
                }

                if !actual_constraints.is_empty() {
                    self.reporter.fail(
                        Error::ErrTooManyConstraints,
                        type_ctor.element.start_token.span,
                        &[&resolved_name, &0_usize, &actual_constraints.len()],
                    );
                }

                if matches!(resolved_name.as_str(), "uchar" | "usize64" | "uintptr64")
                    && !self
                        .experimental_flags
                        .is_enabled(ExperimentalFlag::ZxCTypes)
                {
                    self.reporter.fail(
                        Error::ErrExperimentalZxCTypesDisallowed,
                        type_ctor.element.start_token.span,
                        &[&resolved_name],
                    );
                }

                Type::primitive(resolved_name.parse().unwrap_or(PrimitiveSubtype::Uint32))
            }
            "experimental_pointer" => {
                if !self
                    .experimental_flags
                    .is_enabled(ExperimentalFlag::ZxCTypes)
                {
                    self.reporter.fail(
                        Error::ErrExperimentalZxCTypesDisallowed,
                        type_ctor.element.start_token.span,
                        &[&resolved_name],
                    );
                }

                let inner = if type_ctor.parameters.is_empty() {
                    None
                } else {
                    Some(&type_ctor.parameters[0])
                };

                let inner_type_opt =
                    inner.map(|i| Box::new(self.resolve_type(i, library_name, naming_context)));

                Type::experimental_pointer(inner_type_opt, nullable)
            }
            "string" => {
                if actual_constraints.len() > 1 {
                    self.reporter.fail(
                        Error::ErrTooManyConstraints,
                        type_ctor.element.start_token.span,
                        &[&resolved_name, &1_usize, &actual_constraints.len()],
                    );
                }
                let mut max_len = u32::MAX;
                if let Some(c) = actual_constraints.first() {
                    if let Some(val) = self.eval_constant_usize(c) {
                        max_len = val as u32;
                    } else {
                        self.reporter.fail(
                            Error::ErrTypeCannotBeConvertedToType,
                            c.element().span(),
                            &[],
                        );
                        self.reporter.fail(
                            Error::ErrCouldNotResolveSizeBound,
                            c.element().span(),
                            &[],
                        );
                    }
                }
                Type::string(
                    if max_len == u32::MAX {
                        None
                    } else {
                        Some(max_len)
                    },
                    nullable,
                    if let Some(raw_ast::Constant::Identifier(id)) = type_ctor.constraints.first() {
                        Some(id.identifier.to_string())
                    } else {
                        None
                    },
                )
            }
            "string_array" => {
                let max_len = if !type_ctor.parameters.is_empty() {
                    let size_param = &type_ctor.parameters[0];
                    self.eval_type_constant_usize(size_param)
                        .unwrap_or(u32::MAX as usize) as u32
                } else {
                    u32::MAX
                };
                Type::string_array(if max_len == u32::MAX {
                    None
                } else {
                    Some(max_len)
                })
            }
            "vector" | "bytes" => {
                let is_bytes = resolved_name == "bytes";
                if actual_constraints.len() > 1 {
                    self.reporter.fail(
                        Error::ErrUnexpectedConstraint,
                        actual_constraints[1].element().span(),
                        &[&resolved_name],
                    );
                }
                let inner = if is_bytes || type_ctor.parameters.is_empty() {
                    None
                } else {
                    Some(&type_ctor.parameters[0])
                };

                if !is_bytes && inner.is_none() {
                    // Error handling?
                    return Type::unknown();
                }

                let inner_type = if is_bytes {
                    Type::primitive(PrimitiveSubtype::Uint8)
                } else {
                    self.resolve_type(inner.unwrap(), library_name, naming_context)
                };

                let mut max_count = u32::MAX;
                if let Some(c) = actual_constraints.first() {
                    if let Some(val) = self.eval_constant_usize(c) {
                        max_count = val as u32;
                    } else {
                        self.reporter.fail(
                            Error::ErrTypeCannotBeConvertedToType,
                            c.element().span(),
                            &[],
                        );
                        self.reporter.fail(
                            Error::ErrCouldNotResolveSizeBound,
                            c.element().span(),
                            &[],
                        );
                    }
                }

                Type::vector(
                    Box::new(inner_type.clone()),
                    if max_count == u32::MAX {
                        None
                    } else {
                        Some(max_count)
                    },
                    nullable,
                    if let Some(raw_ast::Constant::Identifier(id)) = type_ctor.constraints.first() {
                        Some(id.identifier.to_string())
                    } else {
                        None
                    },
                )
            }
            "array" => {
                if type_ctor.parameters.len() < 2 {
                    self.reporter.fail(
                        Error::ErrWrongNumberOfLayoutParameters,
                        type_ctor.element.start_token.span,
                        &[],
                    );
                    return Type::unknown();
                }
                // Array validation
                let elt_type = &type_ctor.parameters[0];
                let count_param = &type_ctor.parameters[1];

                // Check for optional array: array<T, N>:optional is invalid
                if nullable {
                    self.reporter.fail(
                        Error::ErrCannotBeOptional,
                        type_ctor.element.start_token.span,
                        &[&"array"],
                    );
                }

                // Check count
                let mut count: u32 = 0;

                if let Some(val) = self.eval_type_constant_usize(count_param) {
                    if val == 0 {
                        self.reporter.fail(
                            Error::ErrMustHaveNonZeroSize,
                            count_param.element.start_token.span,
                            &[&"array"],
                        );
                    }
                    count = val as u32;
                } else {
                    let is_type =
                        match &count_param.layout {
                            raw_ast::LayoutParameter::Identifier(id) => {
                                let inner_name = if id.components.len() > 1 {
                                    let mut parts = vec![];
                                    for c in &id.components[..id.components.len() - 1] {
                                        parts.push(c.data());
                                    }
                                    format!(
                                        "{}/{}",
                                        parts.join("."),
                                        id.components.last().unwrap().data()
                                    )
                                } else {
                                    id.to_string()
                                };
                                let bare = if let Some(stripped) = inner_name.strip_prefix("fidl/")
                                {
                                    stripped
                                } else {
                                    &inner_name
                                };
                                matches!(
                                    bare,
                                    "bool"
                                        | "int8"
                                        | "int16"
                                        | "int32"
                                        | "int64"
                                        | "uint8"
                                        | "uint16"
                                        | "uint32"
                                        | "uint64"
                                        | "float32"
                                        | "float64"
                                        | "string"
                                        | "vector"
                                        | "bytes"
                                        | "array"
                                        | "handle"
                                        | "request"
                                        | "client_end"
                                        | "server_end"
                                ) || self.raw_decls.contains_key::<str>(inner_name.as_ref())
                                    || self.raw_decls.contains_key(&OwnedQualifiedName::from(
                                        format!("{}/{}", library_name, inner_name),
                                    ))
                            }
                            _ => false,
                        };
                    if is_type {
                        let name_str = match &count_param.layout {
                            raw_ast::LayoutParameter::Identifier(id) => {
                                id.components.last().unwrap().data().to_string()
                            }
                            _ => "unknown".to_string(),
                        };
                        self.reporter.fail(
                            Error::ErrExpectedValueButGotType,
                            count_param.element.span(),
                            &[&name_str],
                        );
                    } else {
                        self.reporter.fail(
                            Error::ErrNameNotFound,
                            count_param.element.span(),
                            &[&"unknown", &library_name],
                        ); // To fix argument count
                    }
                }

                // Check constraints
                if !type_ctor.constraints.is_empty() {
                    self.reporter.fail(
                        Error::ErrTooManyConstraints,
                        type_ctor.element.start_token.span,
                        &[&"array", &0_usize, &type_ctor.constraints.len()],
                    );
                }

                let inner_type = self.resolve_type(elt_type, library_name, naming_context);

                let maybe_size_constant_name = if let Some(param) = type_ctor.parameters.get(1) {
                    if let raw_ast::LayoutParameter::Identifier(id) = &param.layout {
                        Some(id.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                };

                Type::array(
                    Box::new(inner_type.clone()),
                    count,
                    maybe_size_constant_name,
                )
            }

            "client_end" | "server_end" => {
                let role = if name == "client_end" {
                    "client"
                } else {
                    "server"
                };

                let mut protocol = "".to_string();

                if let Some(constraint) = type_ctor.constraints.first() {
                    if let raw_ast::Constant::Identifier(id) = constraint {
                        let proto_name = id.identifier.to_string();
                        if let Some((lib_prefix, rest)) = proto_name.split_once('.') {
                            let mut actual_lib = lib_prefix.to_string();
                            if let Some(import) = self.library_imports.get(lib_prefix) {
                                self.used_imports
                                    .borrow_mut()
                                    .insert(OwnedLibraryName::new(lib_prefix.to_string()));
                                actual_lib = import.using_path.to_string();
                            }
                            protocol = format!("{}/{}", actual_lib, rest);
                        } else if proto_name.contains('/') {
                            protocol = proto_name;
                        } else {
                            protocol = format!("{}/{}", library_name, proto_name);
                        }
                    }
                } else if let Some(param) = type_ctor.parameters.first() {
                    if let raw_ast::LayoutParameter::Identifier(id) = &param.layout {
                        let proto_name = id.to_string();
                        if let Some((lib_prefix, rest)) = proto_name.split_once('.') {
                            let mut actual_lib = lib_prefix.to_string();
                            if let Some(import) = self.library_imports.get(lib_prefix) {
                                self.used_imports
                                    .borrow_mut()
                                    .insert(OwnedLibraryName::new(lib_prefix.to_string()));
                                actual_lib = import.using_path.to_string();
                            }
                            protocol = format!("{}/{}", actual_lib, rest);
                        } else if proto_name.contains('/') {
                            protocol = proto_name;
                        } else {
                            protocol = format!("{}/{}", library_name, proto_name);
                        }
                    }
                } else {
                    self.reporter.fail(
                        Error::ErrWrongNumberOfLayoutParameters,
                        type_ctor.element.span(),
                        &[&"client_end/server_end", &1_usize, &0_usize],
                    );
                }

                let mut transport = None;
                if !protocol.is_empty() {
                    let mut is_protocol = false;
                    if let Some(decl) = self.raw_decls.get::<str>(protocol.as_ref()) {
                        if let RawDecl::Protocol(p) = decl {
                            is_protocol = true;
                            if let Some(attrs) = p.attributes.as_ref()
                                && let Some(attr) = attrs
                                    .attributes
                                    .iter()
                                    .find(|a| a.name.data() == "transport")
                                && let Some(arg) = attr.args.iter().find(|a| {
                                    a.name.as_ref().map_or("value", |n| n.data()) == "value"
                                })
                                && let raw_ast::Constant::Literal(lit) = &arg.value
                                && lit.literal.kind == raw_ast::LiteralKind::String
                            {
                                transport = Some(lit.literal.value.trim_matches('"').to_string());
                            }
                        }
                    } else if let Some(p) =
                        self.declarations.protocols().find(|&p| p.name == protocol)
                    {
                        is_protocol = true;
                        if let Some(attr) =
                            p.maybe_attributes.iter().find(|a| a.name == "transport")
                            && let Some(arg) = attr.arguments.iter().find(|a| a.name == "value")
                            && let Some(lit) = &arg.value.literal
                        {
                            transport = Some(lit.value.trim_matches('"').to_string());
                        }
                    } else if let Some(p) =
                        std::iter::empty::<&ProtocolDeclaration>().find(|&p| p.name == protocol)
                    {
                        is_protocol = true;
                        if let Some(attr) =
                            p.maybe_attributes.iter().find(|a| a.name == "transport")
                            && let Some(arg) = attr.arguments.iter().find(|a| a.name == "value")
                            && let Some(lit) = &arg.value.literal
                        {
                            transport = Some(lit.value.trim_matches('"').to_string());
                        }
                    } else {
                        is_protocol = true; // wait, if not found and not compiled?
                    }

                    if !is_protocol {
                        self.reporter.fail(
                            Error::ErrMustBeAProtocol,
                            type_ctor.element.span(),
                            &[&protocol],
                        );
                    }
                }

                Type::endpoint(Some(protocol), Some(role.to_string()), nullable, transport)
            }
            "box" => {
                if type_ctor.parameters.is_empty() {
                    return Type::unknown();
                }
                let inner = &type_ctor.parameters[0];
                let prev = self.skip_eager_compile;
                self.skip_eager_compile = true;
                let mut inner_type = self.resolve_type(inner, library_name, naming_context);
                self.skip_eager_compile = prev;

                if nullable {
                    self.reporter.fail(
                        Error::ErrBoxCannotBeOptional,
                        type_ctor.element.span(),
                        &[],
                    );
                }

                if inner_type.kind() != TypeKind::Struct {
                    let mut is_nor_opt = false;
                    let mut is_struct = false;
                    if let Some(decl) = self.get_underlying_decl(
                        inner_type.identifier().as_ref().unwrap_or(&"".to_string()),
                    ) {
                        match decl {
                            RawDecl::Enum(_) | RawDecl::Bits(_) | RawDecl::Table(_) => {
                                is_nor_opt = true
                            }
                            RawDecl::Struct(_) => is_struct = true,
                            RawDecl::Type(t) => match &t.layout {
                                raw_ast::Layout::Enum(_)
                                | raw_ast::Layout::Bits(_)
                                | raw_ast::Layout::Table(_) => is_nor_opt = true,
                                raw_ast::Layout::Struct(_) => is_struct = true,
                                _ => {}
                            },
                            _ => {}
                        }
                    } else {
                        is_nor_opt = inner_type.kind() == TypeKind::Array
                            || inner_type.kind() == TypeKind::Primitive;
                    }

                    if !is_struct {
                        if is_nor_opt {
                            self.reporter.fail(
                                Error::ErrCannotBeBoxedNorOptional,
                                inner.element.span(),
                                &[&inner.element.span().data],
                            );
                        } else {
                            self.reporter.fail(
                                Error::ErrCannotBeBoxedShouldBeOptional,
                                inner.element.span(),
                                &[&inner.element.span().data],
                            );
                        }
                    }
                }

                let boxed_inline = inner_type.type_shape.inline_size;
                let padding = (8 - (boxed_inline % 8)) % 8;
                let max_ool = inner_type
                    .type_shape
                    .max_out_of_line
                    .saturating_add(boxed_inline.saturating_add(padding));

                inner_type.set_nullable(true); // box always makes it nullable for JSON output

                let new_depth = inner_type.type_shape.depth.saturating_add(1);

                inner_type.type_shape = TypeShape {
                    inline_size: 8,
                    alignment: 8,
                    depth: new_depth,
                    max_handles: inner_type.type_shape.max_handles,
                    max_out_of_line: max_ool,
                    has_padding: inner_type.type_shape.has_padding || padding > 0,
                    has_flexible_envelope: inner_type.type_shape.has_flexible_envelope,
                };

                inner_type
            }
            _ => {
                // Try to resolve identifier
                // 1. Check if name exists directly
                // 2. Check if name exists with library prefix
                let full_name = if name.contains('/') || self.shapes.contains_key::<str>(&name) {
                    name.clone()
                } else {
                    format!("{}/{}", library_name, name)
                };

                if !nullable && !self.skip_eager_compile {
                    self.compile_decl_by_name(&full_name);
                }

                if let Some(RawDecl::Resource(res_decl)) = self.get_underlying_decl(&full_name) {
                    let mut handle_subtype = "handle".to_string();
                    let mut handle_obj_type = 0;
                    let mut handle_rights = 2147483648;

                    let filtered_constraints: Vec<_> = type_ctor
                        .constraints
                        .iter()
                        .filter(|c| !matches!(c, raw_ast::Constant::Identifier(id) if id.identifier.to_string() == "optional"))
                        .collect();

                    let res_library_name = if full_name.contains('/') {
                        OwnedQualifiedName::parse(&full_name).library().to_string()
                    } else {
                        library_name.to_string()
                    };

                    if filtered_constraints.len() > res_decl.properties.len() {
                        self.reporter.fail(
                            Error::ErrTooManyConstraints,
                            type_ctor.element.start_token.span,
                            &[&full_name, &0_usize, &res_decl.properties.len()],
                        );
                    } else {
                        for (i, constraint) in filtered_constraints.iter().enumerate() {
                            let prop = &res_decl.properties[i];
                            let prop_name = prop.name.data();

                            let mut prop_decl_full_name = "".to_string();
                            if let raw_ast::LayoutParameter::Identifier(id) = &prop.type_ctor.layout
                            {
                                prop_decl_full_name = if id.to_string().contains('/')
                                    || self.shapes.contains_key::<str>(&id.to_string())
                                {
                                    id.to_string()
                                } else {
                                    format!("{}/{}", res_library_name, id)
                                };
                            }

                            if prop_name == "subtype" {
                                if let raw_ast::Constant::Identifier(id) = constraint {
                                    let ident_str = id.identifier.components.last().unwrap().data();
                                    let mut found = false;

                                    let decl = self.get_underlying_decl(&prop_decl_full_name);
                                    let enum_decl = match decl {
                                        Some(RawDecl::Enum(e)) => Some(*e),
                                        Some(RawDecl::Type(t)) => {
                                            if let raw_ast::Layout::Enum(e) = &t.layout {
                                                Some(e)
                                            } else {
                                                None
                                            }
                                        }
                                        _ => None,
                                    };
                                    if let Some(e) = enum_decl {
                                        for mem in &e.members {
                                            if mem.name.data() == ident_str {
                                                found = true;
                                                handle_subtype = match ident_str {
                                                    "LOG" => "debuglog".to_string(),
                                                    "SUSPEND_TOKEN" => "suspendtoken".to_string(),
                                                    s => s.replace("_", "").to_lowercase(),
                                                };
                                                if let raw_ast::Constant::Literal(lit) = &mem.value
                                                    && let Ok(v) = lit.literal.value.parse::<u32>()
                                                {
                                                    handle_obj_type = v;
                                                }
                                                break;
                                            }
                                        }
                                    }
                                    if !found {
                                        self.reporter.fail(
                                            Error::ErrUnexpectedConstraint,
                                            type_ctor.element.start_token.span,
                                            &[&full_name],
                                        );
                                    }
                                } else {
                                    self.reporter.fail(
                                        Error::ErrExpectedType,
                                        type_ctor.element.start_token.span,
                                        &[],
                                    );
                                }
                            } else if prop_name == "rights" {
                                let mut found = false;
                                let decl = self.get_underlying_decl(&prop_decl_full_name);
                                let bits_decl = match decl {
                                    Some(RawDecl::Bits(b)) => Some(*b),
                                    Some(RawDecl::Type(t)) => {
                                        if let raw_ast::Layout::Bits(b) = &t.layout {
                                            Some(b)
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                };
                                if let Some(b) = bits_decl {
                                    fn eval_bits(
                                        c: &raw_ast::Constant<'_>,
                                        b: &raw_ast::BitsDeclaration<'_>,
                                    ) -> Option<u32> {
                                        match c {
                                            raw_ast::Constant::Identifier(id) => {
                                                let ident_str =
                                                    id.identifier.components.last().unwrap().data();
                                                for mem in &b.members {
                                                    if mem.name.data() == ident_str
                                                        && let raw_ast::Constant::Literal(lit) =
                                                            &mem.value
                                                    {
                                                        let val_str = lit.literal.value.clone();
                                                        let parsed = if val_str.starts_with("0x")
                                                            || val_str.starts_with("0X")
                                                        {
                                                            u32::from_str_radix(&val_str[2..], 16)
                                                        } else {
                                                            val_str.parse::<u32>()
                                                        };
                                                        if let Ok(v) = parsed {
                                                            return Some(v);
                                                        }
                                                    }
                                                }
                                                None
                                            }
                                            raw_ast::Constant::BinaryOperator(binop) => {
                                                let left = eval_bits(&binop.left, b)?;
                                                let right = eval_bits(&binop.right, b)?;
                                                Some(left | right)
                                            }
                                            _ => None,
                                        }
                                    }
                                    if let Some(v) = eval_bits(constraint, b) {
                                        found = true;
                                        handle_rights = v;
                                    }
                                }
                                if !found {
                                    self.reporter.fail(
                                        Error::ErrUnexpectedConstraint,
                                        type_ctor.element.start_token.span,
                                        &[&full_name],
                                    );
                                }
                            }
                        }
                    }

                    return Type::handle(
                        Some(handle_subtype),
                        Some(handle_rights),
                        Some(handle_obj_type),
                        nullable,
                        Some(full_name.clone()),
                    );
                }

                if let Some(decl) = self.raw_decls.get::<str>(full_name.as_ref()) {
                    let is_user_decl_no_params = match decl {
                        RawDecl::Bits(_)
                        | RawDecl::Enum(_)
                        | RawDecl::Struct(_)
                        | RawDecl::Table(_)
                        | RawDecl::Union(_)
                        | RawDecl::Protocol(_)
                        | RawDecl::Service(_) => true,
                        RawDecl::Type(t) => matches!(
                            t.layout,
                            raw_ast::Layout::Bits(_)
                                | raw_ast::Layout::Enum(_)
                                | raw_ast::Layout::Struct(_)
                                | raw_ast::Layout::Table(_)
                                | raw_ast::Layout::Union(_)
                        ),
                        _ => false,
                    };
                    if is_user_decl_no_params
                        && (type_ctor.nullable
                            || type_ctor.constraints.iter().any(|c| {
                                if let raw_ast::Constant::Identifier(id) = c {
                                    id.identifier.to_string() != "optional"
                                } else {
                                    true
                                }
                            })
                            || (!type_ctor.constraints.is_empty() && !nullable))
                    {
                        // This is a bit ad-hoc, but effectively checks constraints > 0
                        if !type_ctor.constraints.is_empty() {
                            let has_non_optional = type_ctor.constraints.iter().any(|c| {
                                if let raw_ast::Constant::Identifier(id) = c {
                                    id.identifier.to_string() != "optional"
                                } else {
                                    true
                                }
                            });
                            if has_non_optional {
                                self.reporter.fail(
                                    Error::ErrTooManyConstraints,
                                    type_ctor.element.start_token.span,
                                    &[&name, &0_usize, &type_ctor.constraints.len()],
                                );
                            }
                        }
                    }
                }

                if nullable && let Some(decl) = self.raw_decls.get::<str>(full_name.as_ref()) {
                    let is_struct = match decl {
                        RawDecl::Struct(_) => true,
                        RawDecl::Type(t) => matches!(t.layout, raw_ast::Layout::Struct(_)),
                        _ => false,
                    };
                    if is_struct {
                        self.reporter.fail(
                            Error::ErrStructCannotBeOptional,
                            type_ctor.element.span(),
                            &[&name],
                        );
                        nullable = false;
                    }
                    let is_table = match decl {
                        RawDecl::Table(_) => true,
                        RawDecl::Type(t) => matches!(t.layout, raw_ast::Layout::Table(_)),
                        _ => false,
                    };
                    if is_table {
                        self.reporter.fail(
                            Error::ErrCannotBeOptional,
                            type_ctor.element.span(),
                            &[&name],
                        );
                        nullable = false;
                    }
                    let is_enum_or_bits_or_service = match decl {
                        RawDecl::Enum(_) | RawDecl::Bits(_) | RawDecl::Service(_) => true,
                        RawDecl::Type(t) => matches!(
                            t.layout,
                            raw_ast::Layout::Enum(_) | raw_ast::Layout::Bits(_)
                        ),
                        _ => false,
                    };
                    if is_enum_or_bits_or_service {
                        self.reporter.fail(
                            Error::ErrCannotBeOptional,
                            type_ctor.element.span(),
                            &[&name],
                        );
                        nullable = false;
                    }
                }

                let is_resource = if let Some(decl) = self.raw_decls.get::<str>(full_name.as_ref())
                {
                    match decl {
                        RawDecl::Struct(s) => s.modifiers.iter().any(|m| {
                            m.subkind == TokenSubkind::Resource
                                && self.is_active(m.attributes.as_ref())
                        }),
                        RawDecl::Table(t) => t.modifiers.iter().any(|m| {
                            m.subkind == TokenSubkind::Resource
                                && self.is_active(m.attributes.as_ref())
                        }),
                        RawDecl::Union(u) => u.modifiers.iter().any(|m| {
                            m.subkind == TokenSubkind::Resource
                                && self.is_active(m.attributes.as_ref())
                        }),
                        RawDecl::Type(t) => match &t.layout {
                            raw_ast::Layout::Struct(s) => s.modifiers.iter().any(|m| {
                                m.subkind == TokenSubkind::Resource
                                    && self.is_active(m.attributes.as_ref())
                            }),
                            raw_ast::Layout::Table(t) => t.modifiers.iter().any(|m| {
                                m.subkind == TokenSubkind::Resource
                                    && self.is_active(m.attributes.as_ref())
                            }),
                            raw_ast::Layout::Union(u) => u.modifiers.iter().any(|m| {
                                m.subkind == TokenSubkind::Resource
                                    && self.is_active(m.attributes.as_ref())
                            }),
                            _ => false,
                        },
                        RawDecl::Protocol(_) => true,
                        _ => false,
                    }
                } else {
                    false
                };
                if let Some(shape) = self.shapes.get::<str>(&full_name) {
                    Type::identifier_type(
                        Some(full_name.clone()),
                        nullable,
                        shape.clone(),
                        is_resource,
                    )
                } else if let Some(decl) = self.raw_decls.get::<str>(full_name.as_ref()) {
                    if !type_ctor.parameters.is_empty() {
                        self.reporter.fail(
                            Error::ErrWrongNumberOfLayoutParameters,
                            type_ctor.element.start_token.span,
                            &[&name, &0_usize, &type_ctor.parameters.len()],
                        );
                    }
                    if let RawDecl::Alias(a) = decl {
                        let mut has_err = false;
                        let a_constraints: Vec<_> = a.type_ctor.constraints.iter().filter(|c| !matches!(c, raw_ast::Constant::Identifier(id) if id.identifier.to_string() == "optional")).collect();

                        if !actual_constraints.is_empty() && !a_constraints.is_empty() {
                            self.reporter.fail(
                                Error::ErrCannotConstrainTwice,
                                type_ctor.element.start_token.span,
                                &[&name],
                            );
                            has_err = true;
                        }

                        let a_nullable = a.type_ctor.constraints.iter().any(|c| matches!(c, raw_ast::Constant::Identifier(id) if id.identifier.to_string() == "optional")) || a.type_ctor.nullable;
                        if nullable && a_nullable {
                            self.reporter.fail(
                                Error::ErrCannotIndicateOptionalTwice,
                                type_ctor.element.start_token.span,
                                &[&name],
                            );
                            has_err = true;
                        }

                        let mut resolved_type =
                            self.resolve_type(&a.type_ctor, library_name, naming_context);

                        let final_nullable = nullable || a_nullable;

                        if !actual_constraints.is_empty() {
                            if resolved_type.kind() == TypeKind::Vector
                                || resolved_type.kind() == TypeKind::String
                            {
                                if let Some(c) = actual_constraints.first() {
                                    if let Some(val) = self.eval_constant_usize(c) {
                                        if val == 0 {
                                            let id_str =
                                                resolved_type.identifier().unwrap_or_default();
                                            self.reporter.fail(
                                                Error::ErrMustHaveNonZeroSize,
                                                type_ctor.element.start_token.span,
                                                &[&id_str],
                                            );
                                            has_err = true;
                                        } else if resolved_type.kind() == TypeKind::Vector {
                                            resolved_type = Type::vector(
                                                Box::new(
                                                    resolved_type.element_type().unwrap().clone(),
                                                ),
                                                Some(val as u32),
                                                final_nullable,
                                                None,
                                            );
                                        } else {
                                            resolved_type = Type::string(
                                                Some(val as u32),
                                                final_nullable,
                                                None,
                                            );
                                        }
                                    } else {
                                        self.reporter.fail(
                                            Error::ErrUnexpectedConstraint,
                                            type_ctor.element.start_token.span,
                                            &[&name],
                                        );
                                        has_err = true;
                                    }
                                }
                            } else if !has_err {
                                self.reporter.fail(
                                    Error::ErrTooManyConstraints,
                                    type_ctor.element.start_token.span,
                                    &[&name, &0_usize, &actual_constraints.len()],
                                );
                                has_err = true;
                            }
                        }

                        resolved_type.outer_alias = Some(ExperimentalMaybeFromAlias {
                            name: full_name.clone(),
                            args: vec![], // TODO handle args if any
                            nullable,
                        });

                        if final_nullable {
                            resolved_type.set_nullable(true);
                        }
                        if has_err {
                            resolved_type.resource = false;
                        }
                        return resolved_type;
                    }
                    if let RawDecl::Service(_) = decl {
                        self.reporter
                            .fail(Error::ErrExpectedType, type_ctor.element.span(), &[]);
                        return Type::unknown();
                    }
                    let is_union_or_table = match decl {
                        RawDecl::Union(_) | RawDecl::Table(_) => true,
                        RawDecl::Type(t) => matches!(
                            t.layout,
                            raw_ast::Layout::Union(_) | raw_ast::Layout::Table(_)
                        ),
                        _ => false,
                    };
                    let is_protocol = matches!(decl, RawDecl::Protocol(_));
                    let (inline, align, flex, padding) = if is_union_or_table {
                        let is_strict = match decl {
                            RawDecl::Union(u) => u.modifiers.iter().any(|m| {
                                m.subkind == TokenSubkind::Strict
                                    && self.is_active(m.attributes.as_ref())
                            }),
                            RawDecl::Type(t) => match &t.layout {
                                raw_ast::Layout::Union(u) => u.modifiers.iter().any(|m| {
                                    m.subkind == TokenSubkind::Strict
                                        && self.is_active(m.attributes.as_ref())
                                }),
                                _ => false,
                            },
                            _ => false,
                        };
                        (16, 8, !is_strict, false)
                    } else if is_protocol {
                        (4, 4, false, false)
                    } else if nullable {
                        (8, 8, false, false)
                    } else {
                        (0, 1, false, false)
                    };
                    Type::identifier_type(
                        Some(full_name.clone()),
                        nullable,
                        TypeShape {
                            inline_size: inline,
                            alignment: align,
                            depth: u32::MAX,
                            max_handles: 0,
                            max_out_of_line: u32::MAX,
                            has_padding: padding,
                            has_flexible_envelope: flex,
                        },
                        is_resource,
                    )
                } else {
                    self.reporter.fail(
                        Error::ErrNameNotFound,
                        type_ctor.element.span(),
                        &[&name, &library_name],
                    );
                    Type::unknown()
                }
            }
        }
    }

    fn generate_json_string_literal(&self, s: &str) -> String {
        let mut out = String::new();
        out.push('"');

        let s_inner = if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
            &s[1..s.len() - 1]
        } else {
            s
        };

        let mut chars = s_inner.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(next) = chars.next() {
                    match next {
                        '\\' => out.push_str("\\\\"),
                        '"' => out.push_str("\\\""),
                        'n' => out.push_str("\\n"),
                        'r' => out.push_str("\\r"),
                        't' => out.push_str("\\t"),
                        'u' => {
                            if chars.peek() == Some(&'{') {
                                chars.next();
                                let mut hex = String::new();
                                while let Some(&hc) = chars.peek() {
                                    if hc == '}' {
                                        break;
                                    }
                                    hex.push(hc);
                                    chars.next();
                                }
                                chars.next(); // consume }
                                if let Ok(code) = u32::from_str_radix(&hex, 16)
                                    && let Some(ch) = char::from_u32(code)
                                {
                                    let mut b = [0; 2];
                                    for u in ch.encode_utf16(&mut b) {
                                        out.push_str(&format!("\\u{:04x}", u));
                                    }
                                    continue;
                                }
                                out.push_str("\\u{");
                                out.push_str(&hex);
                                out.push('}');
                            } else {
                                out.push_str("\\u");
                            }
                        }
                        _ => {
                            out.push_str("\\\\");
                            if next == '"' || next == '\\' {
                                out.push_str(&format!("\\{}", next));
                            } else {
                                out.push(next);
                            }
                        }
                    }
                } else {
                    out.push_str("\\\\");
                }
            } else if c == '"' {
                out.push_str("\\\"");
            } else if c == '\n' {
                out.push_str("\\n");
            } else if c == '\r' {
                out.push_str("\\r");
            } else if c == '\t' {
                out.push_str("\\t");
            } else {
                out.push(c);
            }
        }
        out.push('"');
        out
    }
}

impl<'node, 'src> Compiler<'node, 'src> {
    fn get_underlying_decl(&self, id: &str) -> Option<&RawDecl<'node, 'src>> {
        let mut curr = id.to_string();
        for _ in 0..100 {
            if let Some(decl) = self.raw_decls.get::<str>(curr.as_ref()) {
                if let RawDecl::Alias(a) = decl {
                    match &a.type_ctor.layout {
                        raw_ast::LayoutParameter::Identifier(id) => {
                            let next = id.to_string();
                            curr = if next.contains('/') || self.shapes.contains_key::<str>(&next) {
                                next
                            } else {
                                let curr_fqn = OwnedQualifiedName::parse(&curr);
                                format!("{}/{}", curr_fqn.library(), next)
                            };
                            continue;
                        }
                        _ => return Some(decl),
                    }
                } else {
                    return Some(decl);
                }
            } else {
                return None;
            }
        }
        None
    }
}
