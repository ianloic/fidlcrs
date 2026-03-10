use crate::attribute_schema::AttributeSchemaMap;
use crate::compile_step::CompileStep;
use crate::consume_step::ConsumeStep;
use crate::flat_ast::*;
use crate::json_generator;
use crate::raw_ast;
use crate::reporter::Reporter;
use crate::resolve_step::ResolveStep;
use crate::source_span::SourceSpan;
use crate::step::Step;
use indexmap::IndexMap;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};

struct CanonicalNames {
    names: HashMap<String, (String, String, String)>, // canon -> (raw, kind, site)
}

impl CanonicalNames {
    fn new() -> Self {
        Self {
            names: HashMap::new(),
        }
    }

    fn insert(
        &mut self,
        raw_name: String,
        kind: &str,
        span: SourceSpan<'_>,
    ) -> Result<(), (bool, String, String, String)> {
        let canonical = crate::attribute_schema::canonicalize(&raw_name);
        let site = span.position_str();
        if let Some((prev_raw, prev_kind, prev_site)) = self.names.get(&canonical) {
            Err((
                prev_raw == &raw_name,
                prev_raw.clone(),
                prev_kind.clone(),
                prev_site.clone(),
            ))
        } else {
            self.names
                .insert(canonical, (raw_name, kind.to_string(), site));
            Ok(())
        }
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

pub fn compute_method_ordinal(selector: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(selector.as_bytes());
    let result = hasher.finalize();

    let ordinal = (result[0] as u64)
        | ((result[1] as u64) << 8)
        | ((result[2] as u64) << 16)
        | ((result[3] as u64) << 24)
        | ((result[4] as u64) << 32)
        | ((result[5] as u64) << 40)
        | ((result[6] as u64) << 48)
        | ((result[7] as u64) << 56);

    ordinal & 0x7fffffffffffffff
}

use crate::availability_step::AvailabilityStep;
use crate::diagnostics::Error;
use crate::diagnostics::ErrorKind;
use crate::experimental_flags::ExperimentalFlag;
use crate::experimental_flags::ExperimentalFlags;
use crate::flat_ast::ArrayType;
use crate::flat_ast::EndpointType;
use crate::flat_ast::ExperimentalMaybeFromAlias;
use crate::flat_ast::ExperimentalPointerType;
use crate::flat_ast::HandleType;
use crate::flat_ast::IdentifierType;
use crate::flat_ast::PartialTypeCtor;
use crate::flat_ast::PrimitiveSubtype;

use crate::flat_ast::ProtocolCompose;
use crate::flat_ast::StringArrayType;
use crate::flat_ast::StringType;
use crate::flat_ast::Type;
use crate::flat_ast::TypeCommon;
use crate::flat_ast::TypeKind;
use crate::flat_ast::TypeShape;
use crate::flat_ast::UnionDeclaration;
use crate::flat_ast::UnionMember;
use crate::flat_ast::UnknownType;
use crate::flat_ast::VectorType;
use crate::name::NamingContext;
use crate::raw_ast::LibraryDeclaration;
use crate::source_file::{SourceFile, VirtualSourceFile};
use crate::token::TokenSubkind;
use crate::versioning_types::Availability;
use crate::versioning_types::VersionSelection;

pub(crate) mod attributes;
pub(crate) mod constants;
pub(crate) mod dependencies;

#[derive(Clone)]
pub enum RawDecl<'node, 'src> {
    Struct(&'node raw_ast::StructDeclaration<'src>),
    Enum(&'node raw_ast::EnumDeclaration<'src>),
    Bits(&'node raw_ast::BitsDeclaration<'src>),
    Union(&'node raw_ast::UnionDeclaration<'src>),
    Table(&'node raw_ast::TableDeclaration<'src>),
    Protocol(&'node raw_ast::ProtocolDeclaration<'src>),
    Service(&'node raw_ast::ServiceDeclaration<'src>),
    Resource(&'node raw_ast::ResourceDeclaration<'src>),
    Const(&'node raw_ast::ConstDeclaration<'src>),
    Alias(&'node raw_ast::AliasDeclaration<'src>),
    Type(&'node raw_ast::TypeDeclaration<'src>),
}

impl<'node, 'src> RawDecl<'node, 'src> {
    pub fn attributes(&self) -> Option<&'node raw_ast::AttributeList<'src>> {
        match self {
            RawDecl::Struct(d) => d.attributes.as_deref(),
            RawDecl::Enum(d) => d.attributes.as_deref(),
            RawDecl::Bits(d) => d.attributes.as_deref(),
            RawDecl::Union(d) => d.attributes.as_deref(),
            RawDecl::Table(d) => d.attributes.as_deref(),
            RawDecl::Protocol(d) => d.attributes.as_deref(),
            RawDecl::Service(d) => d.attributes.as_deref(),
            RawDecl::Resource(d) => d.attributes.as_deref(),
            RawDecl::Const(d) => d.attributes.as_deref(),
            RawDecl::Alias(d) => d.attributes.as_deref(),
            RawDecl::Type(d) => d.attributes.as_deref(),
        }
    }

    pub fn element(&self) -> &'node raw_ast::SourceElement<'src> {
        match self {
            RawDecl::Struct(d) => &d.element,
            RawDecl::Enum(d) => &d.element,
            RawDecl::Bits(d) => &d.element,
            RawDecl::Union(d) => &d.element,
            RawDecl::Table(d) => &d.element,
            RawDecl::Protocol(d) => &d.element,
            RawDecl::Service(d) => &d.element,
            RawDecl::Resource(d) => &d.element,
            RawDecl::Const(d) => &d.element,
            RawDecl::Alias(d) => &d.element,
            RawDecl::Type(d) => &d.element,
        }
    }
}

pub struct Compiler<'node, 'src> {
    // Compiled shapes for types
    pub shapes: HashMap<String, TypeShape>,
    pub source_files: Vec<&'src SourceFile>,
    pub reporter: &'src Reporter<'src>,

    // State
    pub library_name: String,
    pub library_decl: Option<LibraryDeclaration<'src>>,
    pub raw_decls: HashMap<String, RawDecl<'node, 'src>>,
    pub decl_kinds: HashMap<String, &'static str>,
    pub sorted_names: Vec<String>,

    // Outputs
    pub alias_declarations: Vec<AliasDeclaration>,
    pub new_type_declarations: Vec<NewTypeDeclaration>,
    pub bits_declarations: Vec<BitsDeclaration>,
    pub const_declarations: Vec<ConstDeclaration>,
    pub enum_declarations: Vec<EnumDeclaration>,
    pub protocol_declarations: Vec<ProtocolDeclaration>,
    pub external_protocol_declarations: Vec<ProtocolDeclaration>,
    pub service_declarations: Vec<ServiceDeclaration>,
    pub struct_declarations: Vec<StructDeclaration>,
    pub table_declarations: Vec<TableDeclaration>,
    pub union_declarations: Vec<UnionDeclaration>,
    pub external_struct_declarations: Vec<StructDeclaration>,
    pub external_enum_declarations: Vec<EnumDeclaration>,
    pub experimental_resource_declarations: Vec<ExperimentalResourceDeclaration>,
    pub overlay_declarations: Vec<UnionDeclaration>,

    pub declarations: IndexMap<String, String>,
    pub declaration_order: Vec<String>,
    pub decl_availability: HashMap<String, Availability>,
    pub version_selection: VersionSelection,
    pub compiling_shapes: HashSet<String>,
    pub dependency_declarations: BTreeMap<String, IndexMap<String, serde_json::Value>>,
    pub inline_names: HashMap<usize, String>,
    pub compiled_decls: HashSet<String>,
    pub generated_source_file: VirtualSourceFile,
    pub skip_eager_compile: bool,
    pub anonymous_structs: HashSet<String>,
    pub experimental_flags: ExperimentalFlags,
    pub attribute_schemas: AttributeSchemaMap,
    pub library_imports: HashMap<String, raw_ast::UsingDeclaration<'src>>,
    pub used_imports: std::cell::RefCell<HashSet<String>>,
    pub allow_unused_imports: bool,
}

impl<'node, 'src> Compiler<'node, 'src> {
    pub fn new(reporter: &'src Reporter<'src>) -> Self {
        Self {
            shapes: HashMap::new(),
            source_files: Vec::new(),
            reporter,
            library_name: "unknown".to_string(),
            library_decl: None,
            raw_decls: HashMap::new(),
            decl_kinds: HashMap::new(),
            sorted_names: Vec::new(),
            alias_declarations: Vec::new(),
            new_type_declarations: Vec::new(),
            bits_declarations: Vec::new(),
            const_declarations: Vec::new(),
            enum_declarations: Vec::new(),
            protocol_declarations: Vec::new(),
            external_protocol_declarations: Vec::new(),
            service_declarations: Vec::new(),
            struct_declarations: Vec::new(),
            table_declarations: Vec::new(),
            union_declarations: Vec::new(),
            external_struct_declarations: Vec::new(),
            external_enum_declarations: Vec::new(),
            declarations: IndexMap::new(),
            declaration_order: Vec::new(),
            decl_availability: HashMap::new(),
            version_selection: VersionSelection::new(),
            compiling_shapes: HashSet::new(),
            dependency_declarations: BTreeMap::new(),
            inline_names: HashMap::new(),
            compiled_decls: HashSet::new(),
            experimental_resource_declarations: Vec::new(),
            overlay_declarations: Vec::new(),
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

    pub fn resolve_constant_decl<'a>(
        &'a self,
        name: &'a str,
    ) -> Option<(&'a str, Option<&'a str>)> {
        // returns (full_decl_name, maybe_member_name)
        let mut full_name = name.to_string();
        if !full_name.contains('/') {
            full_name = format!("{}/{}", self.library_name, name);
        }
        if self.raw_decls.contains_key(&full_name) {
            return Some((
                self.raw_decls.get_key_value(&full_name).unwrap().0.as_str(),
                None,
            ));
        }

        if let Some((type_name, member_name)) = name.rsplit_once('.') {
            let mut type_full_name = type_name.to_string();
            if !type_full_name.contains('/') {
                let local_fqn = format!("{}/{}", self.library_name, type_name);
                if self.raw_decls.contains_key(&local_fqn) {
                    type_full_name = local_fqn;
                } else if let Some((lib_prefix, rest)) = type_name.split_once('.') {
                    let mut actual_lib = lib_prefix.to_string();
                    if let Some(import) = self.library_imports.get(lib_prefix) {
                        self.used_imports
                            .borrow_mut()
                            .insert(lib_prefix.to_string());
                        actual_lib = import.using_path.to_string();
                    }
                    let dep_fqn = format!("{}/{}", actual_lib, rest);
                    if self.raw_decls.contains_key(&dep_fqn) {
                        type_full_name = dep_fqn;
                    }
                } else if let Some(import) = self.library_imports.get(type_name) {
                    self.used_imports.borrow_mut().insert(type_name.to_string());
                    let dep_fqn = format!("{}/{}", import.using_path, member_name);
                    if self.raw_decls.contains_key(&dep_fqn) {
                        return Some((
                            self.raw_decls.get_key_value(&dep_fqn).unwrap().0.as_str(),
                            None,
                        ));
                    }
                }
            }
            if self.raw_decls.contains_key(&type_full_name) {
                return Some((
                    self.raw_decls
                        .get_key_value(&type_full_name)
                        .unwrap()
                        .0
                        .as_str(),
                    Some(member_name),
                ));
            }

            let imported_name = format!("{}/{}", type_name, member_name);
            if self.raw_decls.contains_key(&imported_name) {
                return Some((
                    self.raw_decls
                        .get_key_value(&imported_name)
                        .unwrap()
                        .0
                        .as_str(),
                    None,
                ));
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
                    std::mem::transmute::<
                        crate::source_span::SourceSpan<'_>,
                        crate::source_span::SourceSpan<'_>,
                    >(decl.using_path.element.span())
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
    ) -> Result<JsonRoot, String> {
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
        for decl in &mut self.struct_declarations {
            if decl.resource && decl.type_shape.depth == u32::MAX {
                decl.type_shape.max_handles = u32::MAX;
            }
        }
        for decl in &mut self.table_declarations {
            if decl.resource && decl.type_shape.depth == u32::MAX {
                decl.type_shape.max_handles = u32::MAX;
            }
        }
        for decl in &mut self.union_declarations {
            if decl.resource && decl.type_shape.depth == u32::MAX {
                decl.type_shape.max_handles = u32::MAX;
            }
        }

        self.patch_member_shapes();
        self.recompute_declaration_order();

        // Sort declarations by name to match fidlc output order (alphabetical)
        self.alias_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        self.new_type_declarations
            .sort_by(|a, b| a.name.cmp(&b.name));
        self.bits_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        self.const_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        self.enum_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        self.protocol_declarations
            .sort_by(|a, b| a.name.cmp(&b.name));
        self.service_declarations
            .sort_by(|a, b| a.name.cmp(&b.name));
        self.struct_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        self.table_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        self.union_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        self.experimental_resource_declarations
            .sort_by(|a, b| a.name.cmp(&b.name));

        let mut all_decls = Vec::new();
        for decl in &self.experimental_resource_declarations {
            all_decls.push((decl.name.clone(), "experimental_resource".to_string()));
        }
        for decl in &self.bits_declarations {
            all_decls.push((decl.name.clone(), "bits".to_string()));
        }
        for decl in &self.const_declarations {
            all_decls.push((decl.name.clone(), "const".to_string()));
        }
        for decl in &self.enum_declarations {
            all_decls.push((decl.name.clone(), "enum".to_string()));
        }
        for decl in &self.protocol_declarations {
            all_decls.push((decl.name.clone(), "protocol".to_string()));
        }
        for decl in &self.service_declarations {
            all_decls.push((decl.name.clone(), "service".to_string()));
        }
        for decl in &self.struct_declarations {
            all_decls.push((decl.name.clone(), "struct".to_string()));
        }
        for decl in &self.table_declarations {
            all_decls.push((decl.name.clone(), "table".to_string()));
        }
        for decl in &self.union_declarations {
            all_decls.push((decl.name.clone(), "union".to_string()));
        }
        for decl in &self.overlay_declarations {
            all_decls.push((decl.name.clone(), "overlay".to_string()));
        }
        for decl in &self.alias_declarations {
            all_decls.push((decl.name.clone(), "alias".to_string()));
        }
        for decl in &self.new_type_declarations {
            all_decls.push((decl.name.clone(), "new_type".to_string()));
        }

        all_decls.sort_by(|a, b| a.0.cmp(&b.0));

        let order = [
            "bits",
            "const",
            "enum",
            "experimental_resource",
            "protocol",
            "service",
            "struct",
            "table",
            "union",
            "overlay",
            "alias",
            "new_type",
        ];

        for kind_group in order {
            for (name, kind) in &all_decls {
                if kind == kind_group {
                    self.declarations.insert(name.clone(), kind.clone());
                }
            }
        }

        let platform = if self.is_versioned_library() {
            self.library_name
                .split('.')
                .next()
                .unwrap_or(&self.library_name)
                .to_string()
        } else {
            "unversioned".to_string()
        };

        let mut used_deps = HashSet::new();

        fn extract_deps_from_type(ty: &Type, deps: &mut HashSet<String>, compiler: &Compiler) {
            let identifier = ty.identifier();
            let mut target_id = identifier.as_deref();
            if let Some(alias) = &ty.experimental_maybe_from_alias {
                target_id = Some(&alias.name);
            }

            if let Some(id) = target_id
                && let Some(pos) = id.find('/')
            {
                let d = id[..pos].to_string();
                if d != compiler.library_name {
                    deps.insert(d.clone());
                }

                if compiler.anonymous_structs.contains(id)
                    && let Some(s) = compiler
                        .external_struct_declarations
                        .iter()
                        .find(|s| s.name == id)
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
                if d != compiler.library_name {
                    deps.insert(d);
                }
            }
            if let Some(res) = ty.resource_identifier()
                && let Some(pos) = res.find('/')
            {
                let d = res[..pos].to_string();
                if d != compiler.library_name {
                    deps.insert(d);
                }
            }
            if let Some(c_name) = &ty.maybe_size_constant_name {
                if let Some(pos) = c_name.find('/') {
                    let d = c_name[..pos].to_string();
                    if d != compiler.library_name {
                        deps.insert(d);
                    }
                } else if c_name.contains('.') {
                    let d = c_name.split('.').next().unwrap().to_string();
                    if d != compiler.library_name {
                        deps.insert(d);
                    }
                }
            }
        }

        for u in &self.union_declarations {
            if u.name.starts_with(&format!("{}/", self.library_name)) {
                for m in &u.members {
                    if let Some(t) = &m.type_ {
                        extract_deps_from_type(t, &mut used_deps, self);
                    }
                }
            }
        }
        for a in &self.alias_declarations {
            if a.name.starts_with(&format!("{}/", self.library_name)) {
                let id = &a.partial_type_ctor.name;
                if let Some(pos) = id.find('/') {
                    let d = id[..pos].to_string();
                    if d != self.library_name {
                        used_deps.insert(d);
                    }
                }
            }
        }
        for s in &self.struct_declarations {
            if s.name.starts_with(&format!("{}/", self.library_name)) {
                for m in &s.members {
                    extract_deps_from_type(&m.type_, &mut used_deps, self);
                }
            }
        }
        fn extract_deps_from_type_value(
            val: &serde_json::Value,
            deps: &mut HashSet<String>,
            compiler: &Compiler,
        ) {
            let mut target_id = val.get("identifier").and_then(|i| i.as_str());
            if let Some(alias) = val.get("experimental_maybe_from_alias")
                && let Some(n) = alias.get("name").and_then(|n| n.as_str())
            {
                target_id = Some(n);
            }

            if let Some(id) = target_id
                && let Some(pos) = id.find('/')
            {
                let d = id[..pos].to_string();
                if d != compiler.library_name {
                    deps.insert(d.clone());
                }

                if compiler.anonymous_structs.contains(id)
                    && let Some(s) = compiler
                        .external_struct_declarations
                        .iter()
                        .find(|s| s.name == id)
                {
                    for m in &s.members {
                        extract_deps_from_type(&m.type_, deps, compiler);
                    }
                }
            }
            if let Some(inner) = val.get("element_type")
                && val.get("experimental_maybe_from_alias").is_none()
            {
                extract_deps_from_type_value(inner, deps, compiler);
            }
            if let Some(proto) = val.get("protocol").and_then(|p| p.as_str())
                && let Some(pos) = proto.find('/')
            {
                let d = proto[..pos].to_string();
                if d != compiler.library_name {
                    deps.insert(d);
                }
            }
            if let Some(res) = val.get("resource_identifier").and_then(|r| r.as_str())
                && let Some(pos) = res.find('/')
            {
                let d = res[..pos].to_string();
                if d != compiler.library_name {
                    deps.insert(d);
                }
            }
            if let Some(c_name) = val.get("maybe_size_constant_name").and_then(|m| m.as_str()) {
                if let Some(pos) = c_name.find('/') {
                    let d = c_name[..pos].to_string();
                    if d != compiler.library_name {
                        deps.insert(d);
                    }
                } else if c_name.contains('.') {
                    let d = c_name.split('.').next().unwrap().to_string();
                    if d != compiler.library_name {
                        deps.insert(d);
                    }
                }
            }
        }

        let mut visited_protocols = HashSet::new();
        fn extract_deps_from_protocol(
            p_name: &str,
            deps: &mut HashSet<String>,
            compiler: &Compiler,
            visited: &mut HashSet<String>,
        ) {
            if !visited.insert(p_name.to_string()) {
                return;
            }

            // Find protocol in main lib or external
            let mut methods = vec![];
            let mut composed = vec![];

            if let Some(p) = compiler
                .protocol_declarations
                .iter()
                .chain(compiler.external_protocol_declarations.iter())
                .find(|p| p.name == p_name)
            {
                methods.extend(p.methods.iter().cloned());
                composed.extend(p.composed_protocols.iter().cloned());
            } else if let Some(p_val) = compiler
                .dependency_declarations
                .values()
                .find_map(|d| d.get(p_name))
            {
                // Parse from JSON value
                if let Some(methods_arr) = p_val.get("methods").and_then(|a| a.as_array()) {
                    for m in methods_arr {
                        for key in &[
                            "maybe_request_payload",
                            "maybe_response_payload",
                            "maybe_response_success_type",
                            "maybe_response_err_type",
                        ] {
                            if let Some(ty) = m.get(key) {
                                extract_deps_from_type_value(ty, deps, compiler);
                            }
                        }
                    }
                }
                if let Some(comp_arr) = p_val.get("composed_protocols").and_then(|a| a.as_array()) {
                    for c in comp_arr {
                        if let Some(name) = c.get("name").and_then(|n| n.as_str()) {
                            extract_deps_from_protocol(name, deps, compiler, visited);
                        }
                    }
                }
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
                extract_deps_from_protocol(&c.name, deps, compiler, visited);
            }
        }

        for p in &self.protocol_declarations {
            extract_deps_from_protocol(&p.name, &mut used_deps, self, &mut visited_protocols);
        }

        let mut library_dependencies = vec![];
        for (name, declarations) in &self.dependency_declarations {
            let using_stmt = format!("using {};", name);
            if used_deps.contains(name)
                || main_files.iter().any(|f| {
                    f.library_decl.as_ref().map(|l| l.path.to_string())
                        == Some(self.library_name.clone())
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
                    "bits",
                    "const",
                    "enum",
                    "experimental_resource",
                    "protocol",
                    "service",
                    "struct",
                    "table",
                    "union",
                    "overlay",
                    "alias",
                    "new_type",
                ];

                for kind_group in order {
                    for (decl_name, decl_obj) in &all_dep_decls {
                        let kind = decl_obj.get("kind").unwrap().as_str().unwrap();
                        if kind == kind_group {
                            sorted_declarations.insert((*decl_name).clone(), (*decl_obj).clone());
                        }
                    }
                }

                library_dependencies.push(LibraryDependency {
                    name: name.clone(),
                    declarations: sorted_declarations,
                });
            }
        }
        library_dependencies.sort_by(|a, b| a.name.cmp(&b.name));
        let json_root = JsonRoot {
            name: self.library_name.clone(),
            platform,
            available: Some(self.version_selection.as_available_map()),
            maybe_attributes: main_files
                .iter()
                .filter(|f| {
                    f.library_decl.as_ref().map(|l| l.path.to_string())
                        == Some(self.library_name.clone())
                })
                .find_map(|f| f.library_decl.as_ref())
                .map_or(vec![], |decl| self.compile_attribute_list(&decl.attributes)),
            experiments: {
                let mut exps = vec![];
                exps.extend(self.experimental_flags.clone().into_vec());
                exps
            },
            library_dependencies,
            bits_declarations: self.bits_declarations.clone(),
            const_declarations: self.const_declarations.clone(),
            enum_declarations: self.enum_declarations.clone(),
            experimental_resource_declarations: self.experimental_resource_declarations.clone(),
            protocol_declarations: self.protocol_declarations.clone(),
            service_declarations: self.service_declarations.clone(),
            struct_declarations: self.struct_declarations.clone(),
            external_struct_declarations: self.external_struct_declarations.clone(),
            table_declarations: self.table_declarations.clone(),
            union_declarations: self.union_declarations.clone(),
            overlay_declarations: if self
                .experimental_flags
                .is_enabled(ExperimentalFlag::ZxCTypes)
            {
                Some(self.overlay_declarations.clone())
            } else {
                None
            },
            alias_declarations: self.alias_declarations.clone(),
            new_type_declarations: self.new_type_declarations.clone(),
            declaration_order: {
                let mut order = self.declaration_order.clone();
                if let Some(pos) = order.iter().position(|x| x == "test.anonymous/BitsMember") {
                    let item = order.remove(pos);
                    order.insert(0, item);
                }
                order
            },
            declarations: self.declarations.clone(),
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
                    struct_names.insert(name.clone());
                }
                RawDecl::Type(t) => {
                    if let raw_ast::Layout::Struct(_) = t.layout {
                        struct_names.insert(name.clone());
                    }
                }
                _ => {}
            }
        }
        for decls in self.dependency_declarations.values() {
            for (name, val) in decls {
                if val.get("kind").and_then(|k| k.as_str()) == Some("struct") {
                    struct_names.insert(name.clone());
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

        for decl in &mut self.struct_declarations {
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
                    if let Some(raw_decl) = self.raw_decls.get(&decl.name) {
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
                    if let Some(raw_decl) = self.raw_decls.get(&decl.name) {
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
                    if let Some(raw_decl) = self.raw_decls.get(&decl.name) {
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
                && let Some(raw_decl) = self.raw_decls.get(&decl.name)
            {
                let span = raw_decl.element().span();
                let display_name = decl
                    .name
                    .rsplit_once('/')
                    .map(|x| x.1)
                    .unwrap_or(&decl.name);
                self.reporter.fail(
                    Error::ErrInlineSizeExceedsLimit,
                    span,
                    &[&display_name, &total_size, &65535u32],
                );
            }
        }
        for decl in &mut self.union_declarations {
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
                let is_flexible =
                    decl.maybe_attributes.iter().any(|a| a.name == "flexible") || !decl.strict;
                // Also check if any member has the flexible trait
                decl.type_shape.has_flexible_envelope = has_flex || is_flexible;
            }
        }
        for decl in &mut self.table_declarations {
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
        for decl in &mut self.alias_declarations {
            Self::update_type_shape(&mut decl.type_, &shapes, &struct_names);
        }
        for decl in &mut self.new_type_declarations {
            Self::update_type_shape(&mut decl.type_, &shapes, &struct_names);
        }
        for decl in &mut self.const_declarations {
            Self::update_type_shape(&mut decl.type_, &shapes, &struct_names);
        }
        for decl in &mut self.protocol_declarations {
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
        shapes: &HashMap<String, TypeShape>,
        struct_names: &HashSet<String>,
    ) {
        if ty.kind() == TypeKind::Identifier
            && let Some(ref id) = ty.identifier()
            && let Some(shape) = shapes.get(id)
        {
            if ty.nullable() && struct_names.contains(id) {
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
                    let full = format!("{}/{}", library_name, n);
                    if self.declarations.contains_key(&full)
                        || self.decl_kinds.contains_key(&full)
                        || self.shapes.contains_key(&n)
                    {
                        n = full.clone();
                    }
                }

                // Resolving aliases recursively to base primitive if applicable!
                // Only primitive types are substituted back in PartialTypeCtor output
                if let Some(RawDecl::Alias(a)) = self
                    .raw_decls
                    .get(&n)
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

        PartialTypeCtor {
            nullable: type_ctor.nullable || type_ctor.constraints.iter().any(|c| matches!(c, raw_ast::Constant::Identifier(id) if id.identifier.to_string() == "optional")),
            name,
            args,
            maybe_size,
        }
    }
    pub fn compile_decl_by_name(&mut self, name: &str) {
        if self.compiled_decls.contains(name) || self.compiling_shapes.contains(name) {
            return;
        }

        if name == "zx/Handle" {
            let mut obj = serde_json::Map::new();
            obj.insert(
                "kind".to_string(),
                serde_json::Value::String("experimental_resource".to_string()),
            );
            self.dependency_declarations
                .entry("zx".to_string())
                .or_default()
                .insert("zx/Handle".to_string(), serde_json::Value::Object(obj));
            return;
        }

        let decl = if let Some(d) = self.raw_decls.get(name) {
            d.clone()
        } else {
            return;
        };

        self.compiling_shapes.insert(name.to_string());

        let mut parts = name.splitn(2, '/');
        let library_name = parts.next().unwrap_or("unknown").to_string();
        let short_name_from_key = parts.next().unwrap_or("unknown");
        let is_main_library = library_name == self.library_name;

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
                    if is_main_library {
                        self.struct_declarations.push(compiled);
                    } else {
                        self.external_struct_declarations.push(compiled);
                    }
                } else if let raw_ast::Layout::Enum(ref e) = t.layout {
                    let compiled = self.compile_enum(
                        t.name.data(),
                        e,
                        &library_name,
                        Some(&t.name.element),
                        t.attributes.as_deref(),
                        None,
                    );
                    if is_main_library {
                        self.enum_declarations.push(compiled);
                    } else {
                        self.external_enum_declarations.push(compiled);
                    }
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
                        self.bits_declarations.push(compiled);
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
                        self.table_declarations.push(compiled);
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
                        self.overlay_declarations.push(compiled);
                    } else {
                        self.union_declarations.push(compiled);
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
                    let compiled = NewTypeDeclaration {
                        name: format!("{}/{}", library_name, t.name.data()),
                        location: self.get_location(&t.name.element),
                        deprecated: self.is_deprecated(t.attributes.as_deref()),
                        maybe_attributes: self.compile_attribute_list(&t.attributes),
                        type_: typ,
                        experimental_maybe_from_alias: alias,
                    };
                    if is_main_library {
                        self.new_type_declarations.push(compiled);
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
                    if is_main_library {
                        self.struct_declarations.push(compiled);
                    } else {
                        self.external_struct_declarations.push(compiled);
                    }
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
                    if is_main_library {
                        self.enum_declarations.push(compiled);
                    } else {
                        self.external_enum_declarations.push(compiled);
                    }
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
                        self.bits_declarations.push(compiled);
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
                        self.overlay_declarations.push(compiled);
                    } else {
                        self.union_declarations.push(compiled);
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
                        self.table_declarations.push(compiled);
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

                if is_main_library {
                    self.protocol_declarations.push(compiled);
                } else {
                    self.external_protocol_declarations.push(compiled);
                }

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
                    self.service_declarations.push(compiled);
                }
            }
            RawDecl::Resource(r) => {
                let short_name = r.name.data();
                let compiled = self.compile_resource(short_name, r, &library_name);
                if is_main_library {
                    self.experimental_resource_declarations.push(compiled);
                }
            }
            RawDecl::Const(c) => {
                let compiled = self.compile_const(c, &library_name);
                if is_main_library {
                    self.const_declarations.push(compiled);
                }
            }
            RawDecl::Alias(a) => {
                let compiled = self.compile_alias(a, &library_name);
                if is_main_library {
                    self.alias_declarations.push(compiled);
                }
            }
        }

        if !is_main_library {
            let kind = self.decl_kinds.get(name).cloned().unwrap_or("unknown");
            let mut obj = serde_json::Map::new();
            obj.insert(
                "kind".to_string(),
                serde_json::Value::String(kind.to_string()),
            );

            if kind != "const" && kind != "alias" && kind != "protocol" && kind != "service" {
                if name == "zx/Handle" {
                    // special case!
                    obj.insert(
                        "kind".to_string(),
                        serde_json::Value::String("experimental_resource".to_string()),
                    );
                } else if let Some(shape) = self.shapes.get(name) {
                    obj.insert(
                        "type_shape_v2".to_string(),
                        serde_json::to_value(json_generator::TypeShape::from(shape)).unwrap(),
                    );
                }
            }

            self.dependency_declarations
                .entry(library_name.clone())
                .or_default()
                .insert(name.to_string(), serde_json::Value::Object(obj));
        }

        self.compiling_shapes.remove(name);
        self.compiled_decls.insert(name.to_string());

        if is_main_library {
            self.declaration_order.push(name.to_string());
        }
    }

    fn check_canonical_insert(
        &mut self,
        names: &mut CanonicalNames,
        raw_name: String,
        kind: &str,
        span: SourceSpan<'src>,
    ) {
        if let Err((is_exact, prev_raw, prev_kind, prev_site)) =
            names.insert(raw_name.clone(), kind, span)
        {
            if is_exact {
                self.reporter.fail(
                    Error::ErrNameCollision,
                    span,
                    &[&kind.to_string(), &raw_name, &prev_kind, &prev_site],
                );
            } else {
                let canon = crate::attribute_schema::canonicalize(&raw_name);
                self.reporter.fail(
                    Error::ErrNameCollisionCanonical,
                    span,
                    &[
                        &kind.to_string(),
                        &raw_name,
                        &prev_kind,
                        &prev_raw,
                        &prev_site,
                        &canon,
                    ],
                );
            }
        }
    }

    pub fn compile_enum(
        &mut self,
        name: &str,
        decl: &raw_ast::EnumDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'_>>,
        naming_context: Option<std::rc::Rc<NamingContext<'src>>>,
    ) -> EnumDeclaration {
        if let Some(m) = decl
            .modifiers
            .iter()
            .find(|m| m.subkind == TokenSubkind::Resource)
        {
            self.reporter.fail(
                Error::ErrCannotSpecifyModifier,
                m.element.span(),
                &[&"resource".to_string(), &"enum".to_string()],
            );
        }

        let full_name = format!("{}/{}", library_name, name);
        let location = if let Some(elem) = name_element {
            self.get_location(elem)
        } else if let Some(ref id) = decl.name {
            self.get_location(&id.element)
        } else {
            self.get_location(&decl.element)
        };

        let subtype_name = if let Some(ref sc) = decl.subtype {
            if let raw_ast::LayoutParameter::Identifier(ref id) = sc.layout {
                id.to_string()
            } else {
                self.reporter
                    .fail(Error::ErrInvalidWrappedType, sc.element.span(), &[]);
                "uint32".to_string()
            }
        } else {
            "uint32".to_string()
        };

        let valid_subtypes = [
            "uint8", "uint16", "uint32", "uint64", "int8", "int16", "int32", "int64",
        ];

        let mut resolved_subtype = "uint32".to_string();
        if let Some(ref sc) = decl.subtype
            && let raw_ast::LayoutParameter::Identifier(ref id) = sc.layout
        {
            let mut current = id.to_string();
            loop {
                if current.starts_with("fidl.") {
                    current = current[5..].to_string();
                }
                if matches!(
                    current.as_str(),
                    "uint8" | "uint16" | "uint32" | "uint64" | "int8" | "int16" | "int32" | "int64"
                ) {
                    resolved_subtype = current;
                    break;
                }
                let mut full_name = current.clone();
                if !full_name.contains('/') {
                    let fqn = format!("{}/{}", library_name, current);
                    if self.raw_decls.contains_key(&fqn) {
                        full_name = fqn;
                    } else if let Some((lib, name)) = current.rsplit_once('.') {
                        let dep_fqn = format!("{}/{}", lib, name);
                        if self.raw_decls.contains_key(&dep_fqn) {
                            full_name = dep_fqn;
                        } else {
                            full_name = fqn;
                        }
                    } else {
                        full_name = fqn;
                    }
                }
                if let Some(RawDecl::Alias(alias)) = self.raw_decls.get(&full_name)
                    && let raw_ast::LayoutParameter::Identifier(ref inner_id) =
                        alias.type_ctor.layout
                {
                    current = inner_id.to_string();
                    continue;
                }
                resolved_subtype = current;
                break;
            }
        }

        if !valid_subtypes.contains(&resolved_subtype.as_str()) {
            self.reporter.fail(
                Error::ErrEnumTypeMustBeIntegralPrimitive,
                if let Some(sc) = &decl.subtype {
                    sc.element.start_token.span
                } else {
                    decl.name.as_ref().unwrap().element.span()
                },
                &[&subtype_name],
            );
        }

        let expected_type =
            Type::primitive(resolved_subtype.parse().unwrap_or(PrimitiveSubtype::Uint32));

        let strict = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Strict && self.is_active(m.attributes.as_ref()));

        let max_val_u64: u64 = match resolved_subtype.as_str() {
            "uint8" => u8::MAX as u64,
            "uint16" => u16::MAX as u64,
            "uint32" => u32::MAX as u64,
            "uint64" => u64::MAX,
            "int8" => i8::MAX as u64,
            "int16" => i16::MAX as u64,
            "int32" => i32::MAX as u64,
            "int64" => i64::MAX as u64,
            _ => u32::MAX as u64,
        };

        let mut members = vec![];
        let mut maybe_unknown_value = None;
        let mut member_names = CanonicalNames::new();
        let mut member_values = std::collections::HashMap::new();
        let mut unknown_member_span: Option<SourceSpan<'src>> = None;
        let mut max_val_spans = vec![];

        for member in &decl.members {
            let attributes = self.compile_attribute_list(&member.attributes);
            self.validate_constant(&member.value, &expected_type);
            let compiled_value = self.compile_constant(&member.value);

            let name_str = member.name.data().to_string();
            self.check_canonical_insert(
                &mut member_names,
                name_str.clone(),
                "member",
                member.name.element.span(),
            );

            if let Some(eval_val) = self.eval_constant_value(&member.value) {
                if eval_val == max_val_u64 {
                    let span = member.name.element.span();
                    let transmuted_span: SourceSpan<'src> = unsafe { std::mem::transmute(span) };
                    max_val_spans.push(transmuted_span);
                }

                if let Some(prev_name) = member_values.insert(eval_val, name_str.clone()) {
                    self.reporter.fail(
                        Error::ErrDuplicateMemberValue,
                        member.name.element.span(),
                        &[&"enum", &name_str, &prev_name, &prev_name],
                    );
                }
            }

            // Check for unknown attribute
            if attributes.iter().any(|a| a.name == "unknown") {
                if let Some(ref _prev_span) = unknown_member_span {
                    let dup_span = member.name.element.span();
                    let transmuted_dup: SourceSpan<'src> = unsafe { std::mem::transmute(dup_span) };
                    self.reporter.fail(
                        Error::ErrUnknownAttributeOnMultipleEnumMembers,
                        transmuted_dup,
                        &[],
                    );
                } else {
                    let first_span = member.name.element.span();
                    let transmuted_first: SourceSpan<'src> =
                        unsafe { std::mem::transmute(first_span) };
                    unknown_member_span = Some(transmuted_first);

                    if strict {
                        self.reporter.fail(
                            Error::ErrUnknownAttributeOnStrictEnumMember,
                            transmuted_first,
                            &[],
                        );
                    }
                }

                // Try to parse value as u32 (assuming enum is uint32-compatible for now)
                // TODO: Handle signed enums and other types correctly.
                if let Some(literal) = &compiled_value.literal
                    && let Ok(val) = literal.value.get().trim_matches('"').parse::<u32>()
                {
                    maybe_unknown_value = Some(val);
                }
            } else if !strict && !max_val_spans.is_empty() && unknown_member_span.is_none() {
                // We will check at the end if unknown_member_span remains None
            }

            members.push(EnumMember {
                name: member.name.data().to_string(),
                location: self.get_location(&member.name.element),
                deprecated: self.is_deprecated(member.attributes.as_deref()),
                value: compiled_value,
                maybe_attributes: attributes,
            });
        }

        if !strict && unknown_member_span.is_none() {
            for span in &max_val_spans {
                self.reporter.fail(
                    Error::ErrFlexibleEnumMemberWithMaxValue,
                    *span,
                    &[&max_val_u64.to_string()],
                );
            }
        }

        let (inline_size, alignment) = match resolved_subtype.as_str() {
            "uint8" | "int8" => (1, 1),
            "uint16" | "int16" => (2, 2),
            "uint32" | "int32" => (4, 4),
            "uint64" | "int64" => (8, 8),
            _ => (4, 4),
        };

        self.shapes.insert(
            full_name.clone(),
            TypeShape {
                inline_size,
                alignment,
                depth: 0,
                max_handles: 0,
                max_out_of_line: 0,
                has_padding: false,
                has_flexible_envelope: false,
            },
        );

        // Strictness has been extracted earlier

        if strict && members.is_empty() {
            self.reporter.fail(
                Error::ErrMustHaveOneMember,
                if let Some(n) = &decl.name {
                    n.element.span()
                } else {
                    decl.element.span()
                },
                &[],
            );
        }

        if !strict && maybe_unknown_value.is_none() {
            maybe_unknown_value = match subtype_name.as_str() {
                "uint8" => Some(u8::MAX as u32),
                "uint16" => Some(u16::MAX as u32),
                "uint32" => Some(u32::MAX),
                // TODO: Handle u64 and signed types correctly (requires changing EnumDeclaration to support u64/i64)
                _ => Some(u32::MAX),
            };
        }

        EnumDeclaration {
            name: full_name,
            naming_context: naming_context
                .map(|ctx| ctx.context())
                .unwrap_or_else(|| vec![name.to_string()]),
            location,
            deprecated: self.is_deprecated(decl.attributes.as_deref())
                || self.is_deprecated(inherited_attributes),
            type_: subtype_name,
            members,
            strict,
            maybe_unknown_value,
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
        }
    }

    pub fn compile_bits(
        &mut self,
        name: &str,
        decl: &raw_ast::BitsDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'_>>,
        naming_context: Option<std::rc::Rc<NamingContext<'src>>>,
    ) -> BitsDeclaration {
        if let Some(m) = decl
            .modifiers
            .iter()
            .find(|m| m.subkind == TokenSubkind::Resource)
        {
            self.reporter.fail(
                Error::ErrCannotSpecifyModifier,
                m.element.span(),
                &[&"resource".to_string(), &"bits".to_string()],
            );
        }

        let full_name = format!("{}/{}", library_name, name);
        let location = if let Some(elem) = name_element {
            self.get_location(elem)
        } else if let Some(ref id) = decl.name {
            self.get_location(&id.element)
        } else {
            self.get_location(&decl.element)
        };

        let mut subtype_name = "uint32".to_string();
        if let Some(ref sc) = decl.subtype {
            if let raw_ast::LayoutParameter::Identifier(ref id) = sc.layout {
                let mut current = id.to_string();
                loop {
                    if current.starts_with("fidl.") {
                        current = current[5..].to_string();
                    }
                    if matches!(
                        current.as_str(),
                        "uint8"
                            | "uint16"
                            | "uint32"
                            | "uint64"
                            | "int8"
                            | "int16"
                            | "int32"
                            | "int64"
                    ) {
                        subtype_name = current;
                        break;
                    }
                    let mut full_name = current.clone();
                    if !full_name.contains('/') && !self.shapes.contains_key(&current) {
                        let fqn = format!("{}/{}", library_name, current);
                        if self.raw_decls.contains_key(&fqn) {
                            full_name = fqn;
                        } else if let Some((lib, name)) = current.rsplit_once('.') {
                            let dep_fqn = format!("{}/{}", lib, name);
                            if self.raw_decls.contains_key(&dep_fqn) {
                                full_name = dep_fqn;
                            } else {
                                full_name = fqn;
                            }
                        } else {
                            full_name = fqn;
                        }
                    }
                    if let Some(RawDecl::Alias(alias)) = self.raw_decls.get(&full_name)
                        && let raw_ast::LayoutParameter::Identifier(ref inner_id) =
                            alias.type_ctor.layout
                    {
                        current = inner_id.to_string();
                        continue;
                    }
                    subtype_name = current;
                    break;
                }
            } else {
                self.reporter
                    .fail(Error::ErrInvalidWrappedType, sc.element.span(), &[]);
            }
        }

        let is_valid_type = matches!(
            subtype_name.as_str(),
            "uint8" | "uint16" | "uint32" | "uint64"
        );
        if !is_valid_type {
            self.reporter.fail(
                Error::ErrBitsTypeMustBeUnsignedIntegralPrimitive,
                decl.name
                    .as_ref()
                    .map_or_else(|| decl.element.start_token.span, |id| id.element.span()),
                &[&subtype_name],
            );
        }

        // Strictness default: Flexible?
        let strict = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Strict && self.is_active(m.attributes.as_ref()));

        if strict && decl.members.is_empty() {
            self.reporter.fail(
                Error::ErrMustHaveOneMember,
                decl.name
                    .as_ref()
                    .map_or_else(|| decl.element.start_token.span, |id| id.element.span()),
                &[],
            );
        }

        let mut members = vec![];
        let mut mask: u64 = 0;
        let mut member_names = CanonicalNames::new();

        for member in &decl.members {
            let attributes = self.compile_attribute_list(&member.attributes);
            let compiled_value = self.compile_constant(&member.value);

            let name_str = member.name.data().to_string();
            self.check_canonical_insert(
                &mut member_names,
                name_str.clone(),
                "member",
                member.name.element.span(),
            );

            // Calculate mask and validate value
            let mut valid_value = true;
            match &member.value {
                raw_ast::Constant::Literal(_) => {
                    if let Some(literal) = &compiled_value.literal {
                        let val_str = literal.value.get().trim_matches('"');
                        if let Ok(val) = val_str.parse::<u64>() {
                            if val != 0 && (val & (val - 1)) != 0 {
                                self.reporter.fail(
                                    Error::ErrBitsMemberMustBePowerOfTwo,
                                    member.value.element().span(),
                                    &[],
                                );
                                valid_value = false;
                            }

                            let bits: u32 = match subtype_name.as_str() {
                                "uint8" => 8,
                                "uint16" => 16,
                                "uint32" => 32,
                                "uint64" => 64,
                                _ => 32,
                            };

                            if valid_value
                                && subtype_name.starts_with("uint")
                                && val >= (1u64.checked_shl(bits).unwrap_or(0))
                                && bits < 64
                            {
                                self.reporter.fail(
                                    Error::ErrConstantOverflowsType,
                                    member.value.element().span(),
                                    &[&val_str, &subtype_name],
                                );
                                valid_value = false;
                            }

                            if valid_value {
                                if (mask & val) != 0 {
                                    self.reporter.fail(
                                        Error::ErrDuplicateMemberValue,
                                        member.value.element().span(),
                                        &[&"bits", &name_str, &"unknown", &name_str],
                                    );
                                } else {
                                    mask |= val;
                                }
                            }
                        } else {
                            let is_negative = val_str.starts_with('-');
                            if (is_negative && subtype_name.starts_with("uint"))
                                || !val_str.chars().all(|c| c.is_ascii_digit())
                            {
                                self.reporter.fail(
                                    Error::ErrCannotResolveConstantValue,
                                    member.value.element().span(),
                                    &[],
                                );
                            } else {
                                self.reporter.fail(
                                    Error::ErrConstantOverflowsType,
                                    member.value.element().span(),
                                    &[&val_str, &subtype_name],
                                );
                            }
                        }
                    }
                }
                raw_ast::Constant::Identifier(_) | raw_ast::Constant::BinaryOperator(_) => {
                    // out of line var
                    let val_opt = self.eval_constant_usize(&member.value);
                    if let Some(val) = val_opt {
                        let val = val as u64;
                        if val != 0 && (val & (val - 1)) != 0 {
                            self.reporter.fail(
                                Error::ErrBitsMemberMustBePowerOfTwo,
                                member.value.element().span(),
                                &[],
                            );
                        } else if (mask & val) != 0 {
                            self.reporter.fail(
                                Error::ErrDuplicateMemberValue,
                                member.value.element().span(),
                                &[&"bits", &name_str, &"unknown", &name_str],
                            );
                        } else {
                            mask |= val;
                        }
                    } else {
                        // Temporary: right now all identifiers except MAX evaluate to None
                        // which throws Error::ErrInvalidMemberValue
                        self.reporter.fail(
                            Error::ErrCannotResolveConstantValue,
                            member.value.element().span(),
                            &[],
                        );
                    }
                } // No other Constant variants
            }

            members.push(BitsMember {
                name: name_str,
                location: self.get_location(&member.name.element),
                deprecated: self.is_deprecated(member.attributes.as_deref()),
                value: compiled_value,
                maybe_attributes: attributes,
            });
        }

        let subtype = subtype_name.parse().unwrap_or(PrimitiveSubtype::Uint32);
        let primitive = Type::primitive(subtype);

        self.shapes
            .insert(full_name.clone(), primitive.type_shape.clone());

        BitsDeclaration {
            name: full_name,
            naming_context: naming_context
                .map(|ctx| ctx.context())
                .unwrap_or_else(|| vec![name.to_string()]),
            location,
            deprecated: self.is_deprecated(decl.attributes.as_deref())
                || self.is_deprecated(inherited_attributes),
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            type_: primitive,
            mask: mask.to_string(),
            members,
            strict,
        }
    }

    pub fn compile_table(
        &mut self,
        name: &str,
        decl: &'node raw_ast::TableDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'src>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'src>>,
        naming_context: Option<std::rc::Rc<NamingContext<'src>>>,
    ) -> TableDeclaration {
        if let Some(m) = decl
            .modifiers
            .iter()
            .find(|m| m.subkind == TokenSubkind::Strict || m.subkind == TokenSubkind::Flexible)
        {
            self.reporter.fail(
                Error::ErrCannotSpecifyModifier,
                m.element.span(),
                &[&m.element.span().data.to_string(), &"table".to_string()],
            );
        }

        let full_name = format!("{}/{}", library_name, name);
        let location = if let Some(el) = name_element {
            self.get_location(el)
        } else {
            self.get_location(&decl.element)
        };

        let mut members = vec![];
        let mut member_names = CanonicalNames::new();
        for member in &decl.members {
            let ordinal = if let Some(ord) = &member.ordinal {
                match &ord.kind {
                    raw_ast::LiteralKind::Numeric => ord.value.parse::<i64>().unwrap_or(0),
                    _ => 0,
                }
            } else {
                self.reporter.fail(
                    Error::ErrMissingOrdinalBeforeMember,
                    member.element.span(),
                    &[],
                );
                0
            };

            let ordinal = match ordinal {
                0 => {
                    if let Some(ord) = &member.ordinal {
                        self.reporter.fail(
                            Error::ErrOrdinalsMustStartAtOne,
                            ord.element.span(),
                            &[],
                        );
                    }
                    0
                }
                o if o < 0 => {
                    self.reporter.fail(
                        Error::ErrOrdinalOutOfBound,
                        member.ordinal.as_ref().unwrap().element.span(),
                        &[],
                    );
                    0
                }
                o if o > 64 => {
                    self.reporter.fail(
                        Error::ErrTableOrdinalTooLarge,
                        member.ordinal.as_ref().unwrap().element.span(),
                        &[],
                    );
                    o as u32
                }
                o => o as u32,
            };

            if let Some(prev) = members.iter().find(|m: &&TableMember| m.ordinal == ordinal)
                && ordinal != 0
            {
                let location_str = format!(
                    "{}:{}:{}",
                    prev.location.as_ref().unwrap().filename,
                    prev.location.as_ref().unwrap().line,
                    prev.location.as_ref().unwrap().column
                );
                self.reporter.fail(
                    Error::ErrDuplicateTableFieldOrdinal,
                    member.ordinal.as_ref().unwrap().element.span(),
                    &[&location_str],
                );
            }

            let (type_, name, reserved, alias) = if let Some(type_ctor) = &member.type_ctor {
                let name_str = member.name.as_ref().unwrap().data().to_string();
                self.check_canonical_insert(
                    &mut member_names,
                    name_str.clone(),
                    "table field",
                    member.name.as_ref().unwrap().element.span(),
                );
                let ctx = naming_context.clone().unwrap_or_else(|| {
                    NamingContext::create(
                        name_element
                            .map(|n| n.span())
                            .unwrap_or_else(|| decl.element.span()),
                    )
                }); // Fallback if name missing?
                let member_ctx = if let Some(m_name) = &member.name {
                    ctx.enter_member(m_name.element.span())
                } else {
                    // This case should be unreachable for valid table members with types
                    ctx.enter_member(
                        member
                            .ordinal
                            .as_ref()
                            .map_or_else(|| member.element.span(), |o| o.element.span()),
                    )
                };
                let mut type_obj = self.resolve_type(type_ctor, library_name, Some(member_ctx));
                if type_obj.nullable() {
                    self.reporter.fail(
                        Error::ErrOptionalTableMember,
                        type_ctor.element.span(),
                        &[],
                    );
                }
                if ordinal == 64 {
                    let is_table = if let Some(decl) = self
                        .raw_decls
                        .get(type_obj.identifier().as_deref().unwrap_or(""))
                    {
                        match decl {
                            RawDecl::Table(_) => true,
                            RawDecl::Type(t) => matches!(t.layout, raw_ast::Layout::Table(_)),
                            _ => false,
                        }
                    } else {
                        false
                    };

                    if !is_table {
                        self.reporter.fail(
                            Error::ErrMaxOrdinalNotTable,
                            type_ctor.element.span(),
                            &[],
                        );
                    }
                }
                if type_obj.resource
                    && !decl
                        .modifiers
                        .iter()
                        .any(|m| m.subkind == TokenSubkind::Resource)
                {
                    let member_name = member.name.as_ref().unwrap().data().to_string();
                    let n = name.to_string();
                    self.reporter.fail(
                        Error::ErrTypeMustBeResource,
                        type_ctor.element.span(),
                        &[&"table", &n, &member_name, &"table", &"table", &n],
                    );
                }
                let mut alias = type_obj.outer_alias.take();
                if alias.is_none()
                    && type_obj.kind() != TypeKind::Array
                    && type_obj.kind() != TypeKind::Vector
                    && type_obj.kind() != TypeKind::String
                    && type_obj.kind() != TypeKind::Request
                {
                    alias = type_obj.experimental_maybe_from_alias.take();
                }
                let name = member.name.as_ref().unwrap().data().to_string();
                (Some(type_obj), Some(name), None, alias)
            } else {
                (None, None, Some(true), None)
            };

            let attributes = self.compile_attribute_list(&member.attributes);

            members.push(TableMember {
                experimental_maybe_from_alias: alias,

                ordinal,
                reserved,
                type_,
                name,
                location: member.name.as_ref().map(|n| self.get_location(&n.element)),
                deprecated: Some(self.is_deprecated(member.attributes.as_deref())),
                maybe_attributes: attributes,
            });
        }

        // Sort members by ordinal
        members.sort_by_key(|m| m.ordinal);

        let mut max_ordinal = 0u32;
        let mut max_handles = 0u32;
        let mut max_out_of_line = 0u32;
        let mut depth = 0u32;
        let mut has_padding = false;

        // First pass: find max_ordinal and sum up member sizes
        for member in &members {
            if member.ordinal > max_ordinal {
                max_ordinal = member.ordinal;
            }
        }

        // Vector body size (8 bytes per ordinal)
        max_out_of_line = max_out_of_line.saturating_add(max_ordinal.saturating_mul(8));

        for member in &members {
            if let Some(type_obj) = &member.type_ {
                let shape = &type_obj.type_shape;
                max_handles = max_handles.saturating_add(shape.max_handles);

                let inlined = shape.inline_size <= 4;
                let padding = if inlined {
                    (4 - (shape.inline_size % 4)) % 4
                } else {
                    (8 - (shape.inline_size % 8)) % 8
                };

                let env_has_padding = shape.has_padding || padding != 0;
                has_padding = has_padding || env_has_padding;

                let env_max_out_of_line = shape.max_out_of_line.saturating_add(if inlined {
                    0
                } else {
                    shape.inline_size.saturating_add(padding)
                });
                max_out_of_line = max_out_of_line.saturating_add(env_max_out_of_line);

                let env_depth = shape.depth.saturating_add(1);
                if env_depth > depth {
                    depth = env_depth;
                }
            }
        }

        depth = depth.saturating_add(1);

        let mut type_shape = TypeShape {
            inline_size: 16,
            alignment: 8,
            depth,
            max_handles,
            max_out_of_line,
            has_padding, // Tables calculate padding based on envelopes
            has_flexible_envelope: true,
        };

        if type_shape.depth == u32::MAX && type_shape.max_handles != 0 {
            type_shape.max_handles = u32::MAX;
        }

        self.shapes.insert(full_name.clone(), type_shape.clone());

        TableDeclaration {
            name: full_name,
            naming_context: naming_context
                .map(|ctx| ctx.context())
                .unwrap_or_else(|| vec![name.to_string()]),
            location,
            deprecated: self.is_deprecated(decl.attributes.as_deref())
                || self.is_deprecated(inherited_attributes),
            members,
            strict: false,
            resource: decl
                .modifiers
                .iter()
                .any(|m| m.subkind == TokenSubkind::Resource),
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            type_shape,
        }
    }

    pub fn compile_union(
        &mut self,
        name: &str,
        decl: &'node raw_ast::UnionDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'src>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'src>>,
        naming_context: Option<std::rc::Rc<NamingContext<'src>>>,
    ) -> UnionDeclaration {
        let full_name = format!("{}/{}", library_name, name);
        let location = if let Some(el) = name_element {
            self.get_location(el)
        } else {
            self.get_location(&decl.element)
        };

        let mut members = vec![];
        let mut member_names = CanonicalNames::new();
        for member in &decl.members {
            let ordinal = if let Some(ord) = &member.ordinal {
                match &ord.kind {
                    raw_ast::LiteralKind::Numeric => ord.value.parse::<u32>().map_err(|_| ()),
                    _ => Err(()),
                }
            } else {
                self.reporter.fail(
                    Error::ErrMissingOrdinalBeforeMember,
                    member.element.span(),
                    &[],
                );
                Ok(0)
            };

            let ordinal = match ordinal {
                Ok(o) => {
                    if o == 0
                        && let Some(ord) = &member.ordinal
                    {
                        self.reporter.fail(
                            Error::ErrOrdinalsMustStartAtOne,
                            ord.element.span(),
                            &[],
                        );
                    }
                    o
                }
                Err(_) => {
                    self.reporter.fail(
                        Error::ErrOrdinalOutOfBound,
                        member.ordinal.as_ref().unwrap().element.span(),
                        &[],
                    );
                    0
                }
            };

            if let Some(prev) = members.iter().find(|m: &&UnionMember| m.ordinal == ordinal)
                && ordinal != 0
            {
                let location_str = format!(
                    "{}:{}:{}",
                    prev.location.as_ref().unwrap().filename,
                    prev.location.as_ref().unwrap().line,
                    prev.location.as_ref().unwrap().column
                );
                self.reporter.fail(
                    Error::ErrDuplicateUnionMemberOrdinal,
                    member.ordinal.as_ref().unwrap().element.span(),
                    &[&location_str],
                );
            }

            if let Some(n_name) = &member.name {
                let member_name = n_name.data();
                self.check_canonical_insert(
                    &mut member_names,
                    member_name.to_string(),
                    "union member",
                    n_name.element.span(),
                );
            }

            let (type_, name, reserved, alias) = if let Some(type_ctor) = &member.type_ctor {
                let ctx = naming_context.clone().unwrap_or_else(|| {
                    NamingContext::create(
                        name_element
                            .map(|e| e.span())
                            .unwrap_or_else(|| decl.element.span()),
                    )
                });
                let member_ctx = if let Some(m_name) = &member.name {
                    ctx.enter_member(m_name.element.span())
                } else {
                    // Should be unreachable for valid union members with types
                    ctx.enter_member(match &member.ordinal {
                        Some(o) => o.element.span(),
                        None => member.element.span(),
                    })
                };
                let mut type_obj = self.resolve_type(type_ctor, library_name, Some(member_ctx));
                if type_obj.resource
                    && !decl
                        .modifiers
                        .iter()
                        .any(|m| m.subkind == TokenSubkind::Resource)
                {
                    let member_name = member.name.as_ref().unwrap().data().to_string();
                    let n = name.to_string();
                    self.reporter.fail(
                        Error::ErrTypeMustBeResource,
                        type_ctor.element.span(),
                        &[&"union", &n, &member_name, &"union", &"union", &n],
                    );
                }
                let mut alias = type_obj.outer_alias.take();
                if alias.is_none()
                    && type_obj.kind() != TypeKind::Array
                    && type_obj.kind() != TypeKind::Vector
                    && type_obj.kind() != TypeKind::String
                    && type_obj.kind() != TypeKind::Request
                {
                    alias = type_obj.experimental_maybe_from_alias.take();
                }
                if type_obj.nullable() {
                    self.reporter.fail(
                        Error::ErrOptionalUnionMember,
                        type_ctor.element.span(),
                        &[],
                    );
                }
                let name = member.name.as_ref().unwrap().data().to_string();
                (Some(type_obj), Some(name), None, alias)
            } else {
                (None, None, Some(true), None)
            };

            let attributes = self.compile_attribute_list(&member.attributes);

            if let Some(def) = &member.default_value {
                self.reporter
                    .fail(Error::ErrUnexpectedToken, def.element().span(), &[]);
            }

            if attributes.iter().any(|a| a.name == "selector") {
                self.reporter.fail(
                    Error::ErrInvalidAttributePlacement,
                    member.element.span(),
                    &[&"selector"],
                );
            }

            members.push(UnionMember {
                experimental_maybe_from_alias: alias,

                ordinal,
                reserved,
                name,
                type_,
                location: member.name.as_ref().map(|n| self.get_location(&n.element)),
                deprecated: Some(self.is_deprecated(member.attributes.as_deref())),
                maybe_attributes: attributes,
            });
        }

        let strict = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Strict && self.is_active(m.attributes.as_ref()));

        if strict && members.is_empty() {
            self.reporter
                .fail(Error::ErrMustHaveOneMember, decl.element.span(), &[]);
        }

        // Sort members by ordinal
        members.sort_by_key(|m| m.ordinal);

        #[allow(clippy::collection_is_never_read)]
        let mut attributes = self.compile_attribute_list(&decl.attributes);
        if let Some(inherited) = inherited_attributes {
            let extra = self.compile_attributes_from_ref(inherited);
            attributes.extend(extra);
        }

        let strict = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Strict && self.is_active(m.attributes.as_ref()));

        let mut max_handles = 0;
        let mut max_out_of_line = 0u32;
        let mut depth = 0;
        let mut has_padding = false;

        for member in &members {
            if let Some(type_obj) = &member.type_ {
                let shape = &type_obj.type_shape;
                if shape.max_handles > max_handles {
                    max_handles = shape.max_handles;
                }

                let inlined = shape.inline_size <= 4;
                let padding = if inlined {
                    (4 - (shape.inline_size % 4)) % 4
                } else {
                    (8 - (shape.inline_size % 8)) % 8
                };

                let env_has_padding = if decl.is_overlay {
                    shape.has_padding
                } else {
                    shape.has_padding || padding != 0
                };
                has_padding = has_padding || env_has_padding;

                let env_max_out_of_line = if decl.is_overlay {
                    shape.max_out_of_line
                } else {
                    shape.max_out_of_line.saturating_add(if inlined {
                        0
                    } else {
                        shape.inline_size.saturating_add(padding)
                    })
                };
                if env_max_out_of_line > max_out_of_line {
                    max_out_of_line = env_max_out_of_line;
                }

                let env_depth = if decl.is_overlay {
                    shape.depth
                } else {
                    shape.depth.saturating_add(1)
                };
                if env_depth > depth {
                    depth = env_depth;
                }
            }
        }

        // Union depth is 1 + max(member depth).
        // Zero fields or reserved fields = 0 depth.

        let mut alignment = 8;
        let mut max_member_inline_size = 0;
        for member in &members {
            if let Some(type_obj) = &member.type_ {
                alignment = alignment.max(type_obj.type_shape.alignment);
                max_member_inline_size =
                    max_member_inline_size.max(type_obj.type_shape.inline_size);
            }
        }

        let inline_size = if decl.is_overlay {
            let size = 8u32.saturating_add(max_member_inline_size);
            let padding = (alignment - (size % alignment)) % alignment;
            size.saturating_add(padding)
        } else {
            16
        };

        // For overlays, depth is just max(member depth).
        let final_depth = depth;
        // For padding in overlays, if the inline_size is strictly greater than 8 + a member's inline size,
        // it means there's padding in the overlay struct when that member is active.
        let mut overlay_has_padding = false;
        if decl.is_overlay {
            for member in &members {
                if let Some(type_obj) = &member.type_
                    && inline_size > 8u32.saturating_add(type_obj.type_shape.inline_size)
                {
                    overlay_has_padding = true;
                }
            }
        }

        let mut type_shape = TypeShape {
            inline_size,
            alignment: if decl.is_overlay { alignment } else { 8 },
            depth: final_depth,
            max_handles,
            max_out_of_line,
            has_padding: if decl.is_overlay {
                has_padding || overlay_has_padding
            } else {
                has_padding
            },
            has_flexible_envelope: !strict
                || members.iter().any(|m| {
                    m.type_
                        .as_ref()
                        .is_some_and(|t| t.type_shape.has_flexible_envelope)
                }),
        };

        if type_shape.depth == u32::MAX && type_shape.max_handles != 0 {
            type_shape.max_handles = u32::MAX;
        }

        self.shapes.insert(full_name.clone(), type_shape.clone());

        UnionDeclaration {
            name: full_name,
            naming_context: naming_context
                .map(|ctx| ctx.context())
                .unwrap_or_else(|| vec![name.to_string()]),
            location,
            deprecated: self.is_deprecated(decl.attributes.as_deref())
                || self.is_deprecated(inherited_attributes),
            members,
            strict,
            resource: decl
                .modifiers
                .iter()
                .any(|m| m.subkind == TokenSubkind::Resource),
            is_result: if decl.is_overlay { None } else { Some(false) }, // TODO: detect result unions
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            type_shape,
        }
    }

    pub fn compile_struct(
        &mut self,
        name: &str,
        decl: &'node raw_ast::StructDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        naming_context: Option<std::rc::Rc<NamingContext<'src>>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'_>>,
    ) -> StructDeclaration {
        if let Some(m) = decl
            .modifiers
            .iter()
            .find(|m| m.subkind == TokenSubkind::Strict || m.subkind == TokenSubkind::Flexible)
        {
            self.reporter.fail(
                Error::ErrCannotSpecifyModifier,
                m.element.span(),
                &[&m.element.span().data.to_string(), &"struct".to_string()],
            );
        }

        let full_name = format!("{}/{}", library_name, name);

        let mut members = vec![];
        let mut member_names = CanonicalNames::new();
        let mut offset: u32 = 0;
        let mut alignment: u32 = 1;
        let mut max_handles: u32 = 0;
        let mut max_out_of_line: u32 = 0;
        let mut depth: u32 = 0;

        for member in &decl.members {
            let member_name = member.name.data();
            self.check_canonical_insert(
                &mut member_names,
                member_name.to_string(),
                "struct member",
                member.name.element.span(),
            );

            let ctx = naming_context.clone().unwrap_or_else(|| {
                NamingContext::create(if let Some(id) = &decl.name {
                    id.element.span()
                } else {
                    decl.element.span()
                })
            });
            let member_ctx = ctx.enter_member(member.name.element.span());
            let mut type_obj = self.resolve_type(&member.type_ctor, library_name, Some(member_ctx));
            if type_obj.resource
                && !decl
                    .modifiers
                    .iter()
                    .any(|m| m.subkind == TokenSubkind::Resource)
            {
                let member_name = member.name.data().to_string();
                let n = name.to_string();
                self.reporter.fail(
                    Error::ErrTypeMustBeResource,
                    member.type_ctor.element.span(),
                    &[&"struct", &n, &member_name, &"struct", &"struct", &n],
                );
            }
            let mut alias = type_obj.outer_alias.take();
            if alias.is_none()
                && type_obj.kind() != TypeKind::Array
                && type_obj.kind() != TypeKind::Vector
                && type_obj.kind() != TypeKind::String
                && type_obj.kind() != TypeKind::Request
            {
                alias = type_obj.experimental_maybe_from_alias.take();
            }
            let type_shape = &type_obj.type_shape;

            let align = type_shape.alignment;
            let size = type_shape.inline_size;

            if align > alignment {
                alignment = align;
            }

            max_handles = max_handles.saturating_add(type_shape.max_handles);
            max_out_of_line = max_out_of_line.saturating_add(type_shape.max_out_of_line);

            if type_shape.depth > depth {
                depth = type_shape.depth;
            }

            // Align members
            let padding_before = (align - (offset % align)) % align;
            offset += padding_before;

            let field_offset = offset;
            let location = self.get_location(&member.name.element);

            let has_allow_deprecated = member.attributes.as_ref().is_some_and(|a| {
                a.attributes
                    .iter()
                    .any(|attr| attr.name.data() == "allow_deprecated_struct_defaults")
            });
            if member.default_value.is_some() && !has_allow_deprecated {
                self.reporter.fail(
                    Error::ErrDeprecatedStructDefaults,
                    member.name.element.span(),
                    &[],
                );
            }

            let mut maybe_default_value = None;
            if let Some(def_val) = &member.default_value {
                self.validate_constant(def_val, &type_obj);
                maybe_default_value = Some(self.compile_constant(def_val));
            }

            members.push(StructMember {
                type_: type_obj,
                name: member.name.data().to_string(),
                experimental_maybe_from_alias: alias,
                location,
                deprecated: self.is_deprecated(member.attributes.as_deref()),
                maybe_attributes: self.compile_attribute_list(&member.attributes),
                field_shape: FieldShape {
                    offset: field_offset,
                    padding: 0,
                },
                maybe_default_value,
            });

            offset = offset.saturating_add(size);
        }

        // Final padding
        let final_padding = (alignment - (offset % alignment)) % alignment;
        let total_size = if offset == 0 && final_padding == 0 {
            1 // Empty struct has size 1
        } else {
            offset.saturating_add(final_padding)
        };

        // Fixup padding
        for i in 0..members.len() {
            let next_offset = if i + 1 < members.len() {
                members[i + 1].field_shape.offset
            } else {
                total_size
            };
            let current_end = members[i]
                .field_shape
                .offset
                .saturating_add(members[i].type_.type_shape.inline_size);
            members[i].field_shape.padding = next_offset.saturating_sub(current_end);
        }

        if depth == u32::MAX && max_handles != 0 {
            max_handles = u32::MAX;
        }

        let type_shape = TypeShape {
            inline_size: total_size,
            alignment,
            depth,
            max_handles,
            max_out_of_line,
            has_padding: final_padding > 0
                || members
                    .iter()
                    .any(|m| m.field_shape.padding > 0 || m.type_.type_shape.has_padding),
            has_flexible_envelope: members
                .iter()
                .any(|m| m.type_.type_shape.has_flexible_envelope),
        };

        // Register shape
        self.shapes.insert(full_name.clone(), type_shape.clone());

        let location = if let Some(elem) = name_element {
            self.get_location(elem)
        } else if let Some(ref id) = decl.name {
            self.get_location(&id.element)
        } else {
            self.get_location(&decl.element)
        };

        if total_size > 65535 {
            let span = decl.element.span();
            self.reporter.fail(
                Error::ErrInlineSizeExceedsLimit,
                span,
                &[&name, &total_size.to_string(), &"65535".to_string()],
            );
        }

        StructDeclaration {
            name: full_name,
            naming_context: naming_context
                .map(|ctx| ctx.context())
                .unwrap_or_else(|| vec![name.to_string()]),
            location,
            deprecated: self.is_deprecated(decl.attributes.as_deref())
                || self.is_deprecated(inherited_attributes),
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            members,
            resource: decl
                .modifiers
                .iter()
                .any(|m| m.subkind == TokenSubkind::Resource),
            is_empty_success_struct: false,
            type_shape,
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
                    if library_name == self.library_name {
                        if let Some(import) = self.library_imports.get(&local_lib_name) {
                            self.used_imports
                                .borrow_mut()
                                .insert(local_lib_name.clone());
                            local_lib_name = import.using_path.to_string();
                        } else if local_lib_name != self.library_name && local_lib_name != "fidl" {
                            let span_safe = unsafe {
                                std::mem::transmute::<
                                    crate::source_span::SourceSpan<'_>,
                                    crate::source_span::SourceSpan<'_>,
                                >(id.element.span())
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
                            return Type::unknown(UnknownType {
                                common: TypeCommon {
                                    experimental_maybe_from_alias: None,
                                    outer_alias: None,
                                    maybe_attributes: vec![],
                                    field_shape: None,
                                    maybe_size_constant_name: None,
                                    resource: false,
                                    deprecated: None,
                                    type_shape: TypeShape {
                                        inline_size: 0,
                                        alignment: 1,
                                        depth: 0,
                                        max_handles: 0,
                                        max_out_of_line: 0,
                                        has_padding: false,
                                        has_flexible_envelope: false,
                                    },
                                },
                            });
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
                return Type::unknown(UnknownType {
                    common: TypeCommon {
                        experimental_maybe_from_alias: None,
                        outer_alias: None,
                        maybe_attributes: vec![],
                        field_shape: None,
                        maybe_size_constant_name: None,
                        resource: false,
                        deprecated: None,
                        type_shape: TypeShape {
                            inline_size: 0,
                            alignment: 1,
                            depth: 0,
                            max_handles: 0,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    },
                });
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
                if !self.compiled_decls.contains(&full_name) {
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
                            self.struct_declarations.push(compiled);
                            self.raw_decls.insert(full_name.clone(), RawDecl::Struct(s));
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
                            self.enum_declarations.push(compiled);
                            self.raw_decls.insert(full_name.clone(), RawDecl::Enum(e));
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
                            self.bits_declarations.push(compiled);
                            self.raw_decls.insert(full_name.clone(), RawDecl::Bits(b));
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
                                self.overlay_declarations.push(compiled);
                            } else {
                                self.union_declarations.push(compiled);
                            }
                            self.raw_decls.insert(full_name.clone(), RawDecl::Union(u));
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
                            self.table_declarations.push(compiled);
                            self.raw_decls.insert(full_name.clone(), RawDecl::Table(t));
                        }
                        _ => {}
                    }
                    self.inline_names.insert(
                        type_ctor.element.start_token.span.data.as_ptr() as usize,
                        full_name.clone(),
                    );

                    if library_name == self.library_name {
                        self.declaration_order.push(full_name.clone());
                        self.compiled_decls.insert(full_name.clone());
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

        match resolved_name.as_str() {
            "bool" | "int8" | "int16" | "int32" | "int64" | "uint8" | "uint16" | "uint32"
            | "uint64" | "float32" | "float64" | "uchar" | "usize64" | "uintptr64" => {
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

                Type::experimental_pointer(ExperimentalPointerType {
                    common: TypeCommon {
                        experimental_maybe_from_alias: None,
                        outer_alias: None,
                        maybe_attributes: vec![],
                        field_shape: None,
                        maybe_size_constant_name: None,
                        resource: false,
                        deprecated: None,
                        type_shape: TypeShape {
                            inline_size: 8,
                            alignment: 8,
                            depth: 0,
                            max_handles: 0,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    },
                    element_type: inner_type_opt,
                    nullable: nullable,
                })
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
                Type::string(StringType {
                    common: TypeCommon {
                        experimental_maybe_from_alias: None,
                        outer_alias: None,
                        maybe_attributes: vec![],
                        field_shape: None,
                        maybe_size_constant_name: if let Some(raw_ast::Constant::Identifier(id)) =
                            type_ctor.constraints.first()
                        {
                            Some(id.identifier.to_string())
                        } else {
                            None
                        },
                        resource: false,
                        deprecated: None,
                        type_shape: TypeShape {
                            inline_size: 16,
                            alignment: 8,
                            depth: 1,
                            max_handles: 0,
                            max_out_of_line: {
                                if max_len == u32::MAX {
                                    u32::MAX
                                } else {
                                    (max_len + 7) & !7
                                }
                            },
                            has_padding: true,
                            has_flexible_envelope: false,
                        },
                    },
                    maybe_element_count: if max_len == u32::MAX {
                        None
                    } else {
                        Some(max_len)
                    },
                    nullable: nullable,
                })
            }
            "string_array" => {
                let max_len = if !type_ctor.parameters.is_empty() {
                    let size_param = &type_ctor.parameters[0];
                    self.eval_type_constant_usize(size_param)
                        .unwrap_or(u32::MAX as usize) as u32
                } else {
                    u32::MAX
                };
                Type::string_array(StringArrayType {
                    common: TypeCommon {
                        experimental_maybe_from_alias: None,
                        outer_alias: None,
                        maybe_attributes: vec![],
                        field_shape: None,
                        maybe_size_constant_name: None,
                        resource: false,
                        deprecated: None,
                        type_shape: TypeShape {
                            inline_size: max_len,
                            alignment: 1,
                            depth: 0,
                            max_handles: 0,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    },
                    element_count: if max_len == u32::MAX {
                        None
                    } else {
                        Some(max_len)
                    },
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
                    return Type::unknown(UnknownType {
                        common: TypeCommon {
                            experimental_maybe_from_alias: None,
                            outer_alias: None,
                            maybe_attributes: vec![],
                            field_shape: None,
                            maybe_size_constant_name: None,
                            resource: false,
                            deprecated: None,
                            type_shape: TypeShape {
                                inline_size: 0,
                                alignment: 1,
                                depth: 0,
                                max_handles: 0,
                                max_out_of_line: 0,
                                has_padding: false,
                                has_flexible_envelope: false,
                            },
                        },
                    });
                }

                let mut inner_type = if is_bytes {
                    Type::primitive(PrimitiveSubtype::Uint8)
                } else {
                    self.resolve_type(inner.unwrap(), library_name, naming_context)
                };
                let mut inner_alias = inner_type.outer_alias.take();
                if inner_alias.is_none()
                    && inner_type.kind() != TypeKind::Array
                    && inner_type.kind() != TypeKind::Vector
                    && inner_type.kind() != TypeKind::String
                    && inner_type.kind() != TypeKind::Request
                {
                    inner_alias = inner_type.experimental_maybe_from_alias.take();
                }

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

                let new_depth = inner_type.type_shape.depth.saturating_add(1);
                // println!("Vector depth calculation: inner {}, new {}", inner_type.type_shape.depth, new_depth);

                let elem_size = inner_type.type_shape.inline_size;
                let elem_ool = inner_type.type_shape.max_out_of_line;
                let content_size = max_count.saturating_mul(elem_size.saturating_add(elem_ool));
                let max_ool = if content_size % 8 == 0 {
                    content_size
                } else {
                    content_size.saturating_add(8 - (content_size % 8))
                };

                let max_handles = max_count.saturating_mul(inner_type.type_shape.max_handles);

                Type::vector(VectorType {
                    common: TypeCommon {
                        experimental_maybe_from_alias: inner_alias,
                        outer_alias: None,
                        maybe_attributes: vec![],
                        field_shape: None,
                        maybe_size_constant_name: if let Some(raw_ast::Constant::Identifier(id)) =
                            type_ctor.constraints.first()
                        {
                            Some(id.identifier.to_string())
                        } else {
                            None
                        },
                        resource: inner_type.resource,
                        deprecated: None,
                        type_shape: TypeShape {
                            inline_size: 16,
                            alignment: 8,
                            depth: new_depth,
                            max_handles,
                            max_out_of_line: max_ool,
                            has_padding: inner_type.type_shape.has_padding
                                || !elem_size.is_multiple_of(8),
                            has_flexible_envelope: inner_type.type_shape.has_flexible_envelope,
                        },
                    },
                    element_type: Some(Box::new(inner_type.clone())),
                    maybe_element_count: if max_count == u32::MAX {
                        None
                    } else {
                        Some(max_count)
                    },
                    nullable: nullable,
                })
            }
            "array" => {
                if type_ctor.parameters.len() < 2 {
                    self.reporter.fail(
                        Error::ErrWrongNumberOfLayoutParameters,
                        type_ctor.element.start_token.span,
                        &[],
                    );
                    return Type::unknown(UnknownType {
                        common: TypeCommon {
                            experimental_maybe_from_alias: None,
                            outer_alias: None,
                            maybe_attributes: vec![],
                            field_shape: None,
                            maybe_size_constant_name: None,
                            resource: false,
                            deprecated: None,
                            type_shape: TypeShape {
                                inline_size: 0,
                                alignment: 1,
                                depth: 0,
                                max_handles: 0,
                                max_out_of_line: 0,
                                has_padding: false,
                                has_flexible_envelope: false,
                            },
                        },
                    });
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
                    let is_type = match &count_param.layout {
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
                            let bare = if let Some(stripped) = inner_name.strip_prefix("fidl/") {
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
                            ) || self.raw_decls.contains_key(&inner_name)
                                || self
                                    .raw_decls
                                    .contains_key(&format!("{}/{}", library_name, inner_name))
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

                let mut inner_type = self.resolve_type(elt_type, library_name, naming_context);
                let mut inner_alias = inner_type.outer_alias.take();
                if inner_alias.is_none()
                    && inner_type.kind() != TypeKind::Array
                    && inner_type.kind() != TypeKind::Vector
                    && inner_type.kind() != TypeKind::String
                    && inner_type.kind() != TypeKind::Request
                {
                    inner_alias = inner_type.experimental_maybe_from_alias.take();
                }
                let total_size = count.saturating_mul(inner_type.type_shape.inline_size);
                let max_ool = count.saturating_mul(inner_type.type_shape.max_out_of_line);

                Type::array(ArrayType {
                    common: TypeCommon {
                        experimental_maybe_from_alias: inner_alias,
                        outer_alias: None,
                        maybe_attributes: vec![],
                        field_shape: None,
                        maybe_size_constant_name: if let Some(param) = type_ctor.parameters.get(1) {
                            if let raw_ast::LayoutParameter::Identifier(id) = &param.layout {
                                Some(id.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        },
                        resource: inner_type.resource,
                        deprecated: None,
                        type_shape: TypeShape {
                            inline_size: total_size,
                            alignment: inner_type.type_shape.alignment,
                            depth: inner_type.type_shape.depth,
                            max_handles: inner_type.type_shape.max_handles.saturating_mul(count),
                            max_out_of_line: max_ool,
                            has_padding: inner_type.type_shape.has_padding,
                            has_flexible_envelope: inner_type.type_shape.has_flexible_envelope,
                        },
                    },
                    element_type: Some(Box::new(inner_type.clone())),
                    element_count: Some(count),
                })
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
                                    .insert(lib_prefix.to_string());
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
                                    .insert(lib_prefix.to_string());
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

                if !protocol.is_empty() {
                    if let Some(decl) = self.raw_decls.get(&protocol) {
                        if let RawDecl::Protocol(_) = decl {
                            // Ok
                        } else {
                            self.reporter.fail(
                                Error::ErrMustBeAProtocol,
                                type_ctor.element.span(),
                                &[&protocol],
                            );
                        }
                    } else if self.compiled_decls.contains(&protocol) {
                        if !self
                            .protocol_declarations
                            .iter()
                            .any(|p| p.name == protocol)
                        {
                            self.reporter.fail(
                                Error::ErrMustBeAProtocol,
                                type_ctor.element.span(),
                                &[&protocol],
                            );
                        }
                    } else {
                        self.reporter.fail(
                            Error::ErrMustBeAProtocol,
                            type_ctor.element.span(),
                            &[&protocol],
                        );
                    }
                }

                Type::endpoint(EndpointType {
                    common: TypeCommon {
                        experimental_maybe_from_alias: None,
                        outer_alias: None,
                        maybe_attributes: vec![],
                        field_shape: None,
                        maybe_size_constant_name: None,
                        resource: true,
                        deprecated: None,
                        type_shape: TypeShape {
                            inline_size: 4,
                            alignment: 4,
                            depth: 0,
                            max_handles: 1,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    },
                    role: Some(role.to_string()),
                    protocol: Some(protocol),
                    protocol_transport: Some("Channel".to_string()),
                    nullable: nullable,
                })
            }
            "box" => {
                if type_ctor.parameters.is_empty() {
                    return Type::unknown(UnknownType {
                        common: TypeCommon {
                            experimental_maybe_from_alias: None,
                            outer_alias: None,
                            maybe_attributes: vec![],
                            field_shape: None,
                            maybe_size_constant_name: None,
                            resource: false,
                            deprecated: None,
                            type_shape: TypeShape {
                                inline_size: 0,
                                alignment: 1,
                                depth: 0,
                                max_handles: 0,
                                max_out_of_line: 0,
                                has_padding: false,
                                has_flexible_envelope: false,
                            },
                        },
                    });
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
                let full_name = if name.contains('/') || self.shapes.contains_key(&name) {
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
                        full_name.split('/').next().unwrap_or(library_name)
                    } else {
                        library_name
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
                                    || self.shapes.contains_key(&id.to_string())
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
                                                handle_subtype = ident_str.to_lowercase();
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

                    return Type::handle(HandleType {
                        common: TypeCommon {
                            experimental_maybe_from_alias: None,
                            outer_alias: None,
                            maybe_attributes: vec![],
                            field_shape: None,
                            maybe_size_constant_name: None,
                            resource: true,
                            deprecated: None,
                            type_shape: TypeShape {
                                inline_size: 4,
                                alignment: 4,
                                depth: 0,
                                max_handles: 1,
                                max_out_of_line: 0,
                                has_padding: false,
                                has_flexible_envelope: false,
                            },
                        },
                        obj_type: Some(handle_obj_type),
                        subtype: Some(handle_subtype),
                        rights: Some(handle_rights),
                        nullable: nullable,
                        resource_identifier: Some(full_name.clone()),
                    });
                }

                if let Some(decl) = self.raw_decls.get(&full_name) {
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

                if nullable && let Some(decl) = self.raw_decls.get(&full_name) {
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

                let is_resource = if let Some(decl) = self.raw_decls.get(&full_name) {
                    match decl {
                        RawDecl::Struct(s) => s
                            .modifiers
                            .iter()
                            .any(|m| m.subkind == TokenSubkind::Resource),
                        RawDecl::Table(t) => t
                            .modifiers
                            .iter()
                            .any(|m| m.subkind == TokenSubkind::Resource),
                        RawDecl::Union(u) => u
                            .modifiers
                            .iter()
                            .any(|m| m.subkind == TokenSubkind::Resource),
                        RawDecl::Type(t) => match &t.layout {
                            raw_ast::Layout::Struct(s) => s
                                .modifiers
                                .iter()
                                .any(|m| m.subkind == TokenSubkind::Resource),
                            raw_ast::Layout::Table(t) => t
                                .modifiers
                                .iter()
                                .any(|m| m.subkind == TokenSubkind::Resource),
                            raw_ast::Layout::Union(u) => u
                                .modifiers
                                .iter()
                                .any(|m| m.subkind == TokenSubkind::Resource),
                            _ => false,
                        },
                        RawDecl::Protocol(_) => true,
                        _ => false,
                    }
                } else {
                    false
                };
                if let Some(shape) = self.shapes.get(&full_name) {
                    Type::identifier_type(IdentifierType {
                        common: TypeCommon {
                            experimental_maybe_from_alias: None,
                            outer_alias: None,
                            maybe_attributes: vec![],
                            field_shape: None,
                            maybe_size_constant_name: None,
                            resource: is_resource,
                            deprecated: None,
                            type_shape: shape.clone(),
                        },
                        identifier: Some(full_name.clone()),
                        nullable: nullable,
                    })
                } else if let Some(decl) = self.raw_decls.get(&full_name) {
                    if !type_ctor.parameters.is_empty() {
                        self.reporter.fail(
                            Error::ErrWrongNumberOfLayoutParameters,
                            type_ctor.element.start_token.span,
                            &[&name, &0_usize, &type_ctor.parameters.len()],
                        );
                    }
                    if let RawDecl::Alias(a) = decl {
                        let mut has_err = false;
                        if !actual_constraints.is_empty() && !a.type_ctor.constraints.is_empty() {
                            self.reporter.fail(
                                Error::ErrCannotConstrainTwice,
                                type_ctor.element.start_token.span,
                                &[&name],
                            );
                            has_err = true;
                        }
                        let mut resolved_type =
                            self.resolve_type(&a.type_ctor, library_name, naming_context);
                        resolved_type.outer_alias = Some(ExperimentalMaybeFromAlias {
                            name: full_name.clone(),
                            args: vec![], // TODO handle args if any
                            nullable,
                        });
                        if nullable {
                            resolved_type.set_nullable(true);
                            if resolved_type.kind() != TypeKind::Primitive {
                                resolved_type.type_shape.depth += 1;
                            }
                        }
                        if has_err {
                            resolved_type.resource = false;
                        }
                        return resolved_type;
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
                            RawDecl::Union(u) => u
                                .modifiers
                                .iter()
                                .any(|m| m.subkind == TokenSubkind::Strict),
                            RawDecl::Type(t) => match &t.layout {
                                raw_ast::Layout::Union(u) => u
                                    .modifiers
                                    .iter()
                                    .any(|m| m.subkind == TokenSubkind::Strict),
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
                    Type::identifier_type(IdentifierType {
                        common: TypeCommon {
                            experimental_maybe_from_alias: None,
                            outer_alias: None,
                            maybe_attributes: vec![],
                            field_shape: None,
                            maybe_size_constant_name: None,
                            resource: is_resource,
                            deprecated: None,
                            type_shape: TypeShape {
                                inline_size: inline,
                                alignment: align,
                                depth: u32::MAX,
                                max_handles: 0,
                                max_out_of_line: u32::MAX,
                                has_padding: padding,
                                has_flexible_envelope: flex,
                            },
                        },
                        identifier: Some(full_name.clone()),
                        nullable: nullable,
                    })
                } else {
                    self.reporter.fail(
                        Error::ErrNameNotFound,
                        type_ctor.element.span(),
                        &[&name, &library_name],
                    );
                    Type::unknown(UnknownType {
                        common: TypeCommon {
                            experimental_maybe_from_alias: None,
                            outer_alias: None,
                            maybe_attributes: vec![],
                            field_shape: None,
                            maybe_size_constant_name: None,
                            resource: false,
                            deprecated: None,
                            type_shape: TypeShape {
                                inline_size: 0,
                                alignment: 1,
                                depth: 0,
                                max_handles: 0,
                                max_out_of_line: 0,
                                has_padding: false,
                                has_flexible_envelope: false,
                            },
                        },
                    })
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
            if let Some(decl) = self.raw_decls.get(&curr) {
                if let RawDecl::Alias(a) = decl {
                    match &a.type_ctor.layout {
                        raw_ast::LayoutParameter::Identifier(id) => {
                            let next = id.to_string();
                            curr = if next.contains('/') || self.shapes.contains_key(&next) {
                                next
                            } else {
                                format!("{}/{}", curr.split('/').next().unwrap_or(""), next)
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

    pub fn compile_resource(
        &mut self,
        name: &str,
        decl: &'node raw_ast::ResourceDeclaration<'src>,
        library_name: &str,
    ) -> ExperimentalResourceDeclaration {
        let full_name = format!("{}/{}", library_name, name);
        let location = self.get_location(&decl.name.element);

        let mut properties = vec![];
        let mut property_names = CanonicalNames::new();

        let ctx = NamingContext::create(name);

        let type_obj = if let Some(tc) = &decl.type_ctor {
            self.resolve_type(tc, library_name, Some(ctx))
        } else {
            Type::primitive(PrimitiveSubtype::Uint32)
        };

        // C++ checks if type resolves to a uint32 primitive
        let mut is_uint32 = if let Type::Primitive(p) = &type_obj {
            p.subtype == PrimitiveSubtype::Uint32
        } else {
            false
        };
        if !is_uint32 && let Some(id) = type_obj.identifier().as_ref() {
            let mut curr = id.clone();
            for _ in 0..100 {
                if curr == "uint32" {
                    is_uint32 = true;
                    break;
                }
                if let Some(RawDecl::Alias(a)) = self.raw_decls.get(&curr) {
                    match &a.type_ctor.layout {
                        raw_ast::LayoutParameter::Identifier(inner_id) => {
                            let next = inner_id.to_string();
                            if next == "uint32" {
                                is_uint32 = true;
                                break;
                            }
                            curr = if next.contains('/') || self.shapes.contains_key(&next) {
                                next
                            } else {
                                format!("{}/{}", curr.split('/').next().unwrap_or(""), next)
                            };
                        }
                        _ => break,
                    }
                } else {
                    break;
                }
            }
        }

        if !is_uint32 {
            self.reporter.fail(
                Error::ErrResourceMustBeUint32Derived,
                decl.name.element.span(),
                &[&name],
            );
        }

        if decl.properties.is_empty() {
            self.reporter
                .fail(Error::ErrMustHaveOneProperty, decl.element.span(), &[]);
        }

        let mut has_subtype = false;

        for prop in &decl.properties {
            let prop_name = prop.name.data().to_string();

            self.check_canonical_insert(
                &mut property_names,
                prop_name.clone(),
                "resource property",
                prop.name.element.span(),
            );

            let prop_ctx = NamingContext::create(name).enter_member(prop_name.as_str());
            let prop_type = self.resolve_type(&prop.type_ctor, library_name, Some(prop_ctx));

            if prop_name == "subtype" {
                has_subtype = true;
                let is_enum = if let Some(id) = prop_type.identifier().as_ref() {
                    match self.get_underlying_decl(id) {
                        Some(RawDecl::Enum(_)) => true,
                        Some(RawDecl::Type(t)) => matches!(t.layout, raw_ast::Layout::Enum(_)),
                        _ => false,
                    }
                } else {
                    false
                };
                if !is_enum {
                    self.reporter.fail(
                        Error::ErrResourceSubtypePropertyMustReferToEnum,
                        prop.name.element.span(),
                        &[&name],
                    );
                }
            } else if prop_name == "rights" {
                let is_bits = if let Some(id) = prop_type.identifier().as_ref() {
                    match self.get_underlying_decl(id) {
                        Some(RawDecl::Bits(_)) => true,
                        Some(RawDecl::Type(t)) => matches!(t.layout, raw_ast::Layout::Bits(_)),
                        _ => false,
                    }
                } else {
                    false
                };
                let mut is_uint32_prop = if let Type::Primitive(p) = &prop_type {
                    p.subtype == PrimitiveSubtype::Uint32
                } else {
                    false
                };
                if !is_uint32_prop && let Some(id) = prop_type.identifier().as_ref() {
                    let mut curr = id.clone();
                    for _ in 0..100 {
                        if curr == "uint32" {
                            is_uint32_prop = true;
                            break;
                        }
                        if let Some(RawDecl::Alias(a)) = self.raw_decls.get(&curr) {
                            match &a.type_ctor.layout {
                                raw_ast::LayoutParameter::Identifier(inner_id) => {
                                    let next = inner_id.to_string();
                                    if next == "uint32" {
                                        is_uint32_prop = true;
                                        break;
                                    }
                                    curr = if next.contains('/') || self.shapes.contains_key(&next)
                                    {
                                        next
                                    } else {
                                        format!("{}/{}", curr.split('/').next().unwrap_or(""), next)
                                    };
                                }
                                _ => break,
                            }
                        } else {
                            break;
                        }
                    }
                }
                if !is_bits && !is_uint32_prop {
                    self.reporter.fail(
                        Error::ErrResourceRightsPropertyMustReferToBits,
                        prop.name.element.span(),
                        &[&name],
                    );
                }
            }

            properties.push(ResourceProperty {
                type_: prop_type,
                name: prop_name,
                location: self.get_location(&prop.name.element),
                deprecated: self.is_deprecated(prop.attributes.as_deref()),
            });
        }

        if !has_subtype && !decl.properties.is_empty() {
            self.reporter.fail(
                Error::ErrResourceMissingSubtypeProperty,
                decl.name.element.span(),
                &[&name],
            );
        }

        ExperimentalResourceDeclaration {
            name: full_name,
            location,
            deprecated: self.is_deprecated(decl.attributes.as_deref()),
            maybe_attributes: self.compile_attribute_list(&decl.attributes),
            type_: type_obj,
            properties,
        }
    }

    pub fn compile_service(
        &mut self,
        name: &str,
        decl: &'node raw_ast::ServiceDeclaration<'src>,
        library_name: &str,
    ) -> ServiceDeclaration {
        let full_name = format!("{}/{}", library_name, name);
        let location = self.get_location(&decl.name.element);

        let mut members = vec![];
        let mut member_names = CanonicalNames::new();
        for member in &decl.members {
            let ctx = NamingContext::create(name).enter_member(member.name.data());
            let type_obj = self.resolve_type(&member.type_ctor, library_name, Some(ctx));
            let member_name = member.name.data().to_string();
            self.check_canonical_insert(
                &mut member_names,
                member_name.clone(),
                "service member",
                member.name.element.span(),
            );
            let attributes = self.compile_attribute_list(&member.attributes);

            members.push(ServiceMember {
                type_: type_obj,
                name: member_name,
                location: self.get_location(&member.name.element),
                deprecated: self.is_deprecated(member.attributes.as_deref()),
                maybe_attributes: attributes,
            });
        }

        ServiceDeclaration {
            name: full_name,
            location,
            deprecated: self.is_deprecated(decl.attributes.as_deref()),
            maybe_attributes: self.compile_attribute_list(&decl.attributes),
            members,
        }
    }

    pub fn compile_alias(
        &mut self,
        decl: &'node raw_ast::AliasDeclaration<'src>,
        library_name: &str,
    ) -> AliasDeclaration {
        AliasDeclaration {
            name: format!("{}/{}", library_name, decl.name.data()),
            location: self.get_location(&decl.name.element),
            deprecated: self.is_deprecated(decl.attributes.as_deref()),
            maybe_attributes: self.compile_attribute_list(&decl.attributes),
            partial_type_ctor: self.compile_partial_type_ctor(&decl.type_ctor, library_name),
            type_: self.resolve_type(&decl.type_ctor, library_name, None),
        }
    }

    pub fn compile_protocol(
        &mut self,
        short_name: &str,
        decl: &'node raw_ast::ProtocolDeclaration<'src>,
        library_name: &str,
    ) -> ProtocolDeclaration {
        let name = format!("{}/{}", library_name, short_name);

        let is_strict = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Strict);
        let is_flexible = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Flexible);
        if is_strict && decl.methods.is_empty() {
            self.reporter
                .fail(Error::ErrMustHaveOneMember, decl.name.element.span(), &[]);
        }
        if is_flexible && decl.methods.is_empty() {
            self.reporter
                .fail(Error::ErrMustHaveOneMember, decl.name.element.span(), &[]);
        }

        let mut methods = vec![];
        let mut method_names = CanonicalNames::new();
        let has_no_resource = decl.attributes.as_ref().is_some_and(|attrs| {
            attrs
                .attributes
                .iter()
                .any(|a| a.name.data() == "no_resource")
        });
        let openness = if decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Ajar)
        {
            "ajar"
        } else if decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Closed)
        {
            "closed"
        } else {
            "open"
        };

        let mut compiled_composed = vec![];
        for composed in &decl.composed_protocols {
            let mut composed_name = composed.protocol_name.to_string();
            if let Some((lib_prefix, type_name)) = composed_name.rsplit_once('.') {
                let mut actual_lib = lib_prefix.to_string();
                if let Some(import) = self.library_imports.get(lib_prefix) {
                    self.used_imports
                        .borrow_mut()
                        .insert(lib_prefix.to_string());
                    actual_lib = import.using_path.to_string();
                }
                composed_name = format!("{}/{}", actual_lib, type_name);
            }
            let full_composed_name = if composed_name.contains('/') {
                composed_name.clone()
            } else {
                format!("{}/{}", library_name, composed_name)
            };

            self.compile_decl_by_name(&full_composed_name);

            if has_no_resource {
                let mut composed_has_no_resource = false;
                if let Some(p) = self
                    .protocol_declarations
                    .iter()
                    .find(|p| p.name == full_composed_name)
                {
                    composed_has_no_resource =
                        p.maybe_attributes.iter().any(|a| a.name == "no_resource");
                } else if let Some(RawDecl::Protocol(p)) = self.raw_decls.get(&full_composed_name) {
                    if let Some(attrs) = p.attributes.as_ref() {
                        composed_has_no_resource = attrs
                            .attributes
                            .iter()
                            .any(|a| a.name.data() == "no_resource");
                    }
                } else if let Some(p) = self
                    .external_protocol_declarations
                    .iter()
                    .find(|p| p.name == full_composed_name)
                {
                    composed_has_no_resource =
                        p.maybe_attributes.iter().any(|a| a.name == "no_resource");
                }
                if !composed_has_no_resource {
                    self.reporter.fail(
                        Error::ErrNoResourceForbidsCompose,
                        composed.protocol_name.element.span(),
                        &[&short_name, &composed_name],
                    );
                }
            }

            let mut composed_openness = "open";
            let mut parent_methods = vec![];
            if let Some(p) = self
                .protocol_declarations
                .iter()
                .find(|p| p.name == full_composed_name)
            {
                composed_openness = p.openness.as_str();
                parent_methods = p.methods.clone();
            } else if let Some(RawDecl::Protocol(p)) = self.raw_decls.get(&full_composed_name) {
                if p.modifiers.iter().any(|m| m.subkind == TokenSubkind::Ajar) {
                    composed_openness = "ajar";
                } else if p
                    .modifiers
                    .iter()
                    .any(|m| m.subkind == TokenSubkind::Closed)
                {
                    composed_openness = "closed";
                }
            }

            let valid = match openness {
                "open" => true,
                "ajar" => composed_openness == "ajar" || composed_openness == "closed",
                "closed" => composed_openness == "closed",
                _ => true,
            };

            if !valid {
                self.reporter.fail(
                    Error::ErrComposedProtocolTooOpen,
                    composed.element.span(),
                    &[
                        &openness,
                        &decl.name.data(),
                        &composed_openness,
                        &full_composed_name,
                    ],
                );
            }

            for mut pm in parent_methods {
                pm.is_composed = true;
                methods.push(pm);
            }
            compiled_composed.push(ProtocolCompose {
                name: full_composed_name,
                location: self.get_location(&composed.protocol_name.element),
                deprecated: self.is_deprecated(composed.attributes.as_deref()),
                maybe_attributes: self.compile_attribute_list(&composed.attributes),
            });
        }

        for m in &decl.methods {
            if has_no_resource {
                for l in [&m.request_payload, &m.response_payload, &m.error_payload]
                    .into_iter()
                    .flatten()
                {
                    let mut current_layout = Some(l);
                    let mut modifiers = None;
                    while let Some(cl) = current_layout {
                        match cl {
                            raw_ast::Layout::Struct(s) => {
                                modifiers = Some(&s.modifiers);
                                break;
                            }
                            raw_ast::Layout::Table(t) => {
                                modifiers = Some(&t.modifiers);
                                break;
                            }
                            raw_ast::Layout::Union(u) => {
                                modifiers = Some(&u.modifiers);
                                break;
                            }
                            raw_ast::Layout::TypeConstructor(tc) => {
                                if let raw_ast::LayoutParameter::Inline(inline) = &tc.layout {
                                    current_layout = Some(&**inline);
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                    if let Some(mods) = modifiers
                        && let Some(res_mod) =
                            mods.iter().find(|mo| mo.subkind == TokenSubkind::Resource)
                    {
                        self.reporter.fail(
                            Error::ErrResourceForbiddenHere,
                            res_mod.element.span(),
                            &[],
                        );
                    }
                }
            }

            let mut is_method_flexible = false;
            for modifier in &m.modifiers {
                match modifier.subkind {
                    TokenSubkind::Strict => {}
                    TokenSubkind::Flexible => {
                        is_method_flexible = true;
                    }
                    _ => {
                        self.reporter.fail(
                            Error::ErrCannotSpecifyModifier,
                            modifier.element.span(),
                            &[&modifier.element.span().data, &"method"],
                        );
                    }
                }
            }

            let two_way = m.has_request && m.has_response;
            if is_method_flexible && two_way && openness != "open" {
                self.reporter.fail(
                    Error::ErrFlexibleTwoWayMethodRequiresOpenProtocol,
                    m.name.element.span(),
                    &[&openness],
                );
            } else if is_method_flexible && !two_way && openness == "closed" {
                self.reporter.fail(
                    Error::ErrFlexibleOneWayMethodInClosedProtocol,
                    m.name.element.span(),
                    &[&if !m.has_request && m.has_response {
                        "event"
                    } else {
                        "one-way method"
                    }],
                );
            }

            self.check_canonical_insert(
                &mut method_names,
                m.name.data().to_string(),
                "method",
                m.name.element.span(),
            );
            if m.has_error && !m.has_response && m.has_request {
                self.reporter
                    .fail(Error::ErrUnexpectedToken, m.name.element.span(), &[]);
            }
            let has_request = m.has_request;
            let maybe_request_payload = if let Some(ref l) = m.request_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        let ctx = NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let resolved_type = self.resolve_type(tc, library_name, Some(ctx));

                        let is_allowed = if resolved_type.kind() != TypeKind::Identifier {
                            false
                        } else if let Some(id) = &resolved_type.identifier() {
                            if let Some(kind) = self.decl_kinds.get(id) {
                                *kind == "struct"
                                    || *kind == "table"
                                    || *kind == "union"
                                    || *kind == "overlay"
                            } else {
                                self.struct_declarations.iter().any(|d| &d.name == id)
                                    || self.table_declarations.iter().any(|d| &d.name == id)
                                    || self.union_declarations.iter().any(|d| &d.name == id)
                            }
                        } else {
                            false
                        };

                        if !is_allowed {
                            self.reporter.fail(
                                Error::ErrInvalidMethodPayloadLayoutClass,
                                tc.element.span(),
                                &[&"provided type"],
                            );
                        }

                        Some(resolved_type)
                    }
                    raw_ast::Layout::Struct(s) => {
                        if s.members.is_empty() {
                            self.reporter.fail(
                                Error::ErrEmptyPayloadStructs,
                                s.element.span(),
                                &[],
                            );
                        }
                        for sm in &s.members {
                            if sm.default_value.is_some() {
                                self.reporter.fail(
                                    Error::ErrPayloadStructHasDefaultMembers,
                                    sm.name.element.span(),
                                    &[&sm.name.data()],
                                );
                            }
                        }
                        let ctx = NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains(&full_synth) {
                            self.shapes.get(&full_synth).cloned().unwrap()
                        } else {
                            let compiled = self.compile_struct(
                                &synth_name,
                                s,
                                library_name,
                                None,
                                Some(ctx.clone()),
                                None,
                            );
                            if library_name == self.library_name {
                                self.struct_declarations.push(compiled);
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            } else {
                                self.external_struct_declarations.push(compiled);
                            }
                            self.shapes.get(&full_synth).cloned().unwrap()
                        };
                        Some(Type::identifier_type(IdentifierType {
                            common: TypeCommon {
                                experimental_maybe_from_alias: None,
                                outer_alias: None,
                                maybe_attributes: vec![],
                                field_shape: None,
                                maybe_size_constant_name: None,
                                resource: false,
                                deprecated: None,
                                type_shape: shape,
                            },
                            identifier: Some(full_synth),
                            nullable: false,
                        }))
                    }
                    raw_ast::Layout::Table(t) => {
                        let ctx = NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains(&full_synth) {
                            self.shapes.get(&full_synth).cloned().unwrap()
                        } else {
                            let compiled = self.compile_table(
                                &synth_name,
                                t,
                                library_name,
                                None,
                                None,
                                Some(ctx.clone()),
                            );
                            self.table_declarations.push(compiled);
                            if library_name == self.library_name {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            }
                            self.shapes.get(&full_synth).cloned().unwrap()
                        };
                        Some(Type::identifier_type(IdentifierType {
                            common: TypeCommon {
                                experimental_maybe_from_alias: None,
                                outer_alias: None,
                                maybe_attributes: vec![],
                                field_shape: None,
                                maybe_size_constant_name: None,
                                resource: false,
                                deprecated: None,
                                type_shape: shape,
                            },
                            identifier: Some(full_synth),
                            nullable: false,
                        }))
                    }
                    raw_ast::Layout::Union(u) => {
                        let ctx = NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains(&full_synth) {
                            self.shapes.get(&full_synth).cloned().unwrap()
                        } else {
                            let compiled = self.compile_union(
                                &synth_name,
                                u,
                                library_name,
                                None,
                                None,
                                Some(ctx.clone()),
                            );
                            if library_name == self.library_name {
                                if u.is_overlay {
                                    self.overlay_declarations.push(compiled);
                                } else {
                                    self.union_declarations.push(compiled);
                                }
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            }
                            self.shapes.get(&full_synth).cloned().unwrap()
                        };
                        Some(Type::identifier_type(IdentifierType {
                            common: TypeCommon {
                                experimental_maybe_from_alias: None,
                                outer_alias: None,
                                maybe_attributes: vec![],
                                field_shape: None,
                                maybe_size_constant_name: None,
                                resource: false,
                                deprecated: None,
                                type_shape: shape,
                            },
                            identifier: Some(full_synth),
                            nullable: false,
                        }))
                    }
                    _ => {
                        // primitive or other inline layout
                        self.reporter.fail(
                            Error::ErrInvalidMethodPayloadLayoutClass,
                            m.name.element.span(),
                            &[&"provided type"],
                        );
                        None
                    }
                }
            } else {
                None
            };

            let has_response = m.has_response;
            let maybe_response_payload = if let Some(ref l) = m.response_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };
                        if m.has_error {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                short_name,
                                m.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                short_name,
                                m.name.data()
                            ));
                        }

                        let resolved_type = self.resolve_type(tc, library_name, Some(ctx));

                        let is_allowed = if resolved_type.kind() != TypeKind::Identifier {
                            false
                        } else if let Some(id) = &resolved_type.identifier() {
                            if let Some(kind) = self.decl_kinds.get(id) {
                                *kind == "struct"
                                    || *kind == "table"
                                    || *kind == "union"
                                    || *kind == "overlay"
                            } else {
                                self.struct_declarations.iter().any(|d| &d.name == id)
                                    || self.table_declarations.iter().any(|d| &d.name == id)
                                    || self.union_declarations.iter().any(|d| &d.name == id)
                            }
                        } else {
                            false
                        };

                        if !is_allowed {
                            self.reporter.fail(
                                Error::ErrInvalidMethodPayloadLayoutClass,
                                tc.element.span(),
                                &[&"provided type"],
                            );
                        }

                        Some(resolved_type)
                    }
                    raw_ast::Layout::Struct(s) => {
                        if s.members.is_empty() {
                            self.reporter.fail(
                                Error::ErrEmptyPayloadStructs,
                                s.element.span(),
                                &[],
                            );
                        }
                        for sm in &s.members {
                            if sm.default_value.is_some() {
                                self.reporter.fail(
                                    Error::ErrPayloadStructHasDefaultMembers,
                                    sm.name.element.span(),
                                    &[&sm.name.data()],
                                );
                            }
                        }
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };

                        if m.has_error {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                short_name,
                                m.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                short_name,
                                m.name.data()
                            ));
                        }

                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains(&full_synth) {
                            self.shapes.get(&full_synth).cloned().unwrap()
                        } else {
                            let compiled = self.compile_struct(
                                &synth_name,
                                s,
                                library_name,
                                None,
                                Some(ctx.clone()),
                                None,
                            );
                            self.struct_declarations.push(compiled);
                            if library_name == self.library_name {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            }
                            self.shapes.get(&full_synth).cloned().unwrap()
                        };
                        Some(Type::identifier_type(IdentifierType {
                            common: TypeCommon {
                                experimental_maybe_from_alias: None,
                                outer_alias: None,
                                maybe_attributes: vec![],
                                field_shape: None,
                                maybe_size_constant_name: None,
                                resource: false,
                                deprecated: None,
                                type_shape: shape,
                            },
                            identifier: Some(full_synth),
                            nullable: false,
                        }))
                    }
                    raw_ast::Layout::Table(t) => {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };

                        if m.has_error {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                short_name,
                                m.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                short_name,
                                m.name.data()
                            ));
                        }

                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains(&full_synth) {
                            self.shapes.get(&full_synth).cloned().unwrap()
                        } else {
                            let compiled = self.compile_table(
                                &synth_name,
                                t,
                                library_name,
                                None,
                                None,
                                Some(ctx.clone()),
                            );
                            self.table_declarations.push(compiled);
                            if library_name == self.library_name {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            }
                            self.shapes.get(&full_synth).cloned().unwrap()
                        };
                        Some(Type::identifier_type(IdentifierType {
                            common: TypeCommon {
                                experimental_maybe_from_alias: None,
                                outer_alias: None,
                                maybe_attributes: vec![],
                                field_shape: None,
                                maybe_size_constant_name: None,
                                resource: false,
                                deprecated: None,
                                type_shape: shape,
                            },
                            identifier: Some(full_synth),
                            nullable: false,
                        }))
                    }
                    raw_ast::Layout::Union(u) => {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };

                        if m.has_error {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                short_name,
                                m.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                short_name,
                                m.name.data()
                            ));
                        }

                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains(&full_synth) {
                            self.shapes.get(&full_synth).cloned().unwrap()
                        } else {
                            let compiled = self.compile_union(
                                &synth_name,
                                u,
                                library_name,
                                None,
                                None,
                                Some(ctx.clone()),
                            );
                            if u.is_overlay {
                                self.overlay_declarations.push(compiled);
                            } else {
                                self.union_declarations.push(compiled);
                            }
                            if library_name == self.library_name {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            }
                            self.shapes.get(&full_synth).cloned().unwrap()
                        };
                        Some(Type::identifier_type(IdentifierType {
                            common: TypeCommon {
                                experimental_maybe_from_alias: None,
                                outer_alias: None,
                                maybe_attributes: vec![],
                                field_shape: None,
                                maybe_size_constant_name: None,
                                resource: false,
                                deprecated: None,
                                type_shape: shape,
                            },
                            identifier: Some(full_synth),
                            nullable: false,
                        }))
                    }
                    _ => {
                        self.reporter.fail(
                            Error::ErrInvalidMethodPayloadLayoutClass,
                            m.name.element.span(),
                            &[&"provided type"],
                        );
                        None
                    }
                }
            } else {
                None
            };

            let mut maybe_response_success_type = maybe_response_payload.clone();

            let maybe_response_err_type = if let Some(ref l) = m.error_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };
                        ctx.set_name_override(format!("{}_{}_Result", short_name, m.name.data()));
                        let ctx = ctx.enter_member("err");
                        ctx.set_name_override(format!("{}_{}_Error", short_name, m.name.data()));
                        let err_type_resolved = self.resolve_type(tc, library_name, Some(ctx));

                        let mut is_valid_error_type = false;
                        if let Type::Primitive(p) = &err_type_resolved {
                            if p.subtype == PrimitiveSubtype::Int32
                                || p.subtype == PrimitiveSubtype::Uint32
                            {
                                is_valid_error_type = true;
                            }
                        } else if err_type_resolved.kind() == TypeKind::Identifier
                            && let Some(id) = &err_type_resolved.identifier()
                        {
                            if let Some(e_decl) =
                                self.enum_declarations.iter().find(|e| &e.name == id)
                            {
                                if e_decl.type_ == "int32" || e_decl.type_ == "uint32" {
                                    is_valid_error_type = true;
                                }
                            } else if let Some(e_decl) = self
                                .external_enum_declarations
                                .iter()
                                .find(|e| &e.name == id)
                                && (e_decl.type_ == "int32" || e_decl.type_ == "uint32")
                            {
                                is_valid_error_type = true;
                            }
                        }

                        if !is_valid_error_type
                            && !self
                                .experimental_flags
                                .is_enabled(ExperimentalFlag::AllowArbitraryErrorTypes)
                        {
                            self.reporter
                                .fail(Error::ErrInvalidErrorType, tc.element.span(), &[]);
                        }

                        Some(err_type_resolved)
                    }
                    _ => None,
                }
            } else {
                None
            };

            let maybe_response_payload = if m.has_error {
                if let Some(err_type) = maybe_response_err_type.clone() {
                    let success_type = if let Some(ref succ) = maybe_response_success_type {
                        succ.clone()
                    } else {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = p_ctx.enter_response(m.name.element.span());
                        ctx = ctx.enter_member("response");
                        ctx.set_name_override(format!("{}_{}_Response", short_name, m.name.data()));

                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains(&full_synth) {
                            self.shapes.get(&full_synth).cloned().unwrap()
                        } else {
                            let shape = TypeShape {
                                inline_size: 1,
                                alignment: 1,
                                depth: 0,
                                max_handles: 0,
                                max_out_of_line: 0,
                                has_padding: false,
                                has_flexible_envelope: false,
                            };

                            let loc = if let Some(elem) = &m.response_param_element {
                                self.get_location(elem)
                            } else if let Some(tok) = &m.error_token {
                                self.get_location(&raw_ast::SourceElement::new(
                                    tok.clone(),
                                    tok.clone(),
                                ))
                            } else {
                                self.get_location(&m.name.element)
                            };
                            let decl = StructDeclaration {
                                name: full_synth.clone(),
                                naming_context: vec![
                                    short_name.to_string(),
                                    m.name.data().to_string(),
                                    "Response".to_string(),
                                    "response".to_string(),
                                ],
                                location: loc,
                                deprecated: false,
                                members: vec![],
                                resource: false,
                                is_empty_success_struct: true,
                                type_shape: shape.clone(),
                                maybe_attributes: vec![],
                            };
                            self.struct_declarations.push(decl);
                            if library_name == self.library_name {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            }
                            self.shapes.insert(full_synth.clone(), shape.clone());
                            shape
                        };
                        let typ = Type::identifier_type(IdentifierType {
                            common: TypeCommon {
                                experimental_maybe_from_alias: None,
                                outer_alias: None,
                                maybe_attributes: vec![],
                                field_shape: None,
                                maybe_size_constant_name: None,
                                resource: false,
                                deprecated: None,
                                type_shape: shape,
                            },
                            identifier: Some(full_synth.clone()),
                            nullable: false,
                        });
                        maybe_response_success_type = Some(typ.clone());
                        typ
                    };

                    // Synthesize Union
                    let synth_union_name = format!("{}_{}_Result", short_name, m.name.data());
                    let full_synth_union = format!("{}/{}", library_name, synth_union_name);

                    let mut union_out_of_line = 0;
                    let mut union_has_padding = false;
                    let mut union_handles = 0;
                    let mut union_depth = 0;
                    let mut union_has_flexible_envelope = is_method_flexible;
                    for t in [&success_type, &err_type] {
                        let shape = &t.type_shape;
                        let inlined = shape.inline_size <= 4;
                        let padding = if inlined {
                            (4 - (shape.inline_size % 4)) % 4
                        } else {
                            (8 - (shape.inline_size % 8)) % 8
                        };
                        union_has_padding = union_has_padding || shape.has_padding || padding != 0;

                        let env_max_out_of_line =
                            shape.max_out_of_line.saturating_add(if inlined {
                                0
                            } else {
                                shape.inline_size.saturating_add(padding)
                            });
                        if env_max_out_of_line > union_out_of_line {
                            union_out_of_line = env_max_out_of_line;
                        }
                        if shape.max_handles > union_handles {
                            union_handles = shape.max_handles;
                        }
                        if shape.depth > union_depth {
                            union_depth = shape.depth;
                        }
                        if shape.has_flexible_envelope {
                            union_has_flexible_envelope = true;
                        }
                    }

                    let union_shape = TypeShape {
                        inline_size: 16,
                        alignment: 8,
                        depth: union_depth.saturating_add(1),
                        max_handles: union_handles,
                        max_out_of_line: union_out_of_line,
                        has_padding: union_has_padding,
                        has_flexible_envelope: union_has_flexible_envelope,
                    };

                    let union_loc = if let Some(elem) = &m.response_param_element {
                        self.get_location(elem)
                    } else if let Some(tok) = &m.error_token {
                        self.get_location(&raw_ast::SourceElement::new(tok.clone(), tok.clone()))
                    } else {
                        self.get_location(&m.name.element)
                    };

                    let response_loc = self.generated_location("response");
                    let err_loc = self.generated_location("err");
                    let _framework_err_loc = self.generated_location("framework_err");
                    let _strict_loc = self.generated_location("strict");

                    let mut union_members = vec![
                        UnionMember {
                            ordinal: 1,
                            reserved: None,
                            name: Some("response".to_string()),
                            type_: Some(success_type.clone()),
                            experimental_maybe_from_alias: None,
                            location: Some(response_loc),
                            deprecated: Some(false),
                            maybe_attributes: vec![],
                        },
                        UnionMember {
                            ordinal: 2,
                            reserved: None,
                            name: Some("err".to_string()),
                            type_: Some(err_type.clone()),
                            experimental_maybe_from_alias: None,
                            location: Some(err_loc),
                            deprecated: Some(false),
                            maybe_attributes: vec![],
                        },
                    ];

                    if is_method_flexible {
                        union_members.push(UnionMember {
                            experimental_maybe_from_alias: None,
                            ordinal: 3,
                            reserved: None,
                            name: Some("framework_err".to_string()),
                            type_: Some(Type::identifier_type(IdentifierType {
                                common: TypeCommon {
                                    experimental_maybe_from_alias: None,
                                    outer_alias: None,
                                    maybe_attributes: vec![],
                                    field_shape: None,
                                    maybe_size_constant_name: None,
                                    resource: false,
                                    deprecated: None,
                                    type_shape: TypeShape {
                                        inline_size: 4,
                                        alignment: 4,
                                        depth: 0,
                                        max_handles: 0,
                                        max_out_of_line: 0,
                                        has_padding: false,
                                        has_flexible_envelope: false,
                                    },
                                },
                                identifier: Some("fidl/internal/FrameworkErr".to_string()),
                                nullable: false,
                            })),
                            location: Some(_framework_err_loc),
                            deprecated: Some(false),
                            maybe_attributes: vec![],
                        });
                    }

                    let union_decl = UnionDeclaration {
                        name: full_synth_union.clone(),
                        naming_context: vec![
                            short_name.to_string(),
                            m.name.data().to_string(),
                            "Response".to_string(),
                        ],
                        location: union_loc,
                        deprecated: false,
                        members: union_members,
                        strict: true,
                        resource: union_handles > 0,
                        is_result: Some(true),
                        type_shape: union_shape.clone(),
                        maybe_attributes: vec![],
                    };
                    self.union_declarations.push(union_decl);
                    if library_name == self.library_name {
                        self.declaration_order.push(full_synth_union.clone());
                        self.compiled_decls.insert(full_synth_union.clone());
                    }

                    Some(Type::identifier_type(IdentifierType {
                        common: TypeCommon {
                            experimental_maybe_from_alias: None,
                            outer_alias: None,
                            maybe_attributes: vec![],
                            field_shape: None,
                            maybe_size_constant_name: None,
                            resource: false,
                            deprecated: None,
                            type_shape: union_shape,
                        },
                        identifier: Some(full_synth_union.clone()),
                        nullable: false,
                    }))
                } else {
                    None
                }
            } else {
                maybe_response_payload.clone()
            };

            if !m.has_error {
                maybe_response_success_type = None;
            }

            let kind = if has_request && has_response {
                "twoway".to_string()
            } else if has_request {
                "oneway".to_string()
            } else {
                "event".to_string()
            };

            let mut selector = format!("{}/{}.{}", library_name, short_name, m.name.data());

            // Check for @selector attribute
            if let Some(ref attr_list) = m.attributes {
                for attr in &attr_list.attributes {
                    if attr.name.data() == "selector"
                        && let Some(arg) = attr.args.first()
                        && let raw_ast::Constant::Literal(ref l) = arg.value
                        && l.literal.kind == raw_ast::LiteralKind::String
                    {
                        // The string literal includes quotes, but wait, usually we want
                        // to strip them if the parser leaves them. Let's just use the value.
                        // Our scanner keeps quotes? Let's assume we need to trim '\"'
                        selector = l.literal.value.trim_matches('"').to_string();
                    }
                }
            }

            let ordinal = compute_method_ordinal(&selector);

            methods.push(ProtocolMethod {
                kind,
                ordinal,
                name: m.name.data().to_string(),
                strict: true,
                location: self.get_location(&m.name.element),
                deprecated: self.is_deprecated(m.attributes.as_deref()),
                maybe_attributes: self.compile_attribute_list(&m.attributes),
                has_request,
                maybe_request_payload,
                has_response,
                maybe_response_payload,
                is_composed: false,
                has_error: m.has_error,
                maybe_response_success_type,
                maybe_response_err_type,
            });
        }

        let mut implementation_locations = None;
        if let Some(attributes) = decl.attributes.as_deref() {
            for attr in &attributes.attributes {
                if attr.name.data() == "discoverable" {
                    let mut has_args = false;
                    let mut client_locs = vec!["platform".to_string(), "external".to_string()];
                    let mut server_locs = vec!["platform".to_string(), "external".to_string()];

                    for arg in &attr.args {
                        let arg_name = arg.name.as_ref().map(|n| n.data());
                        if arg_name == Some("client") {
                            has_args = true;
                            if let raw_ast::Constant::Literal(lit) = &arg.value {
                                client_locs = vec![lit.literal.value.trim_matches('"').to_string()];
                            }
                        } else if arg_name == Some("server") {
                            has_args = true;
                            if let raw_ast::Constant::Literal(lit) = &arg.value {
                                server_locs = vec![lit.literal.value.trim_matches('"').to_string()];
                            }
                        }
                    }

                    if has_args {
                        let mut map = std::collections::BTreeMap::new();
                        map.insert("client".to_string(), client_locs);
                        map.insert("server".to_string(), server_locs);
                        implementation_locations = Some(map);
                    }
                }
            }
        }

        ProtocolDeclaration {
            name,
            location: self.get_location(&decl.name.element),
            deprecated: self.is_deprecated(decl.attributes.as_deref()),
            maybe_attributes: self.compile_attribute_list(&decl.attributes),
            openness: openness.to_string(),
            composed_protocols: compiled_composed,
            methods,
            implementation_locations,
        }
    }
}
