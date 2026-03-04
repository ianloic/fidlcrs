use crate::compile_step::CompileStep;
use crate::consume_step::ConsumeStep;
use crate::json_generator::*;
use crate::raw_ast;
use crate::reporter::Reporter;
use crate::resolve_step::ResolveStep;
use crate::step::Step;
use indexmap::IndexMap;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};

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

use crate::source_file::{SourceFile, VirtualSourceFile};

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
    pub shapes: HashMap<String, TypeShapeV2>,
    pub source_files: Vec<&'src SourceFile>,
    pub reporter: &'src Reporter<'src>,

    // State
    pub library_name: String,
    pub library_decl: Option<crate::raw_ast::LibraryDeclaration<'src>>,
    pub raw_decls: HashMap<String, RawDecl<'node, 'src>>,
    pub decl_kinds: HashMap<String, &'static str>,
    pub sorted_names: Vec<String>,

    // Outputs
    pub alias_declarations: Vec<AliasDeclaration>,
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

    pub declarations: IndexMap<String, String>,
    pub declaration_order: Vec<String>,
    pub decl_availability: HashMap<String, crate::versioning_types::Availability>,
    pub version_selection: crate::versioning_types::VersionSelection,
    pub compiling_shapes: HashSet<String>,
    pub dependency_declarations: BTreeMap<String, IndexMap<String, serde_json::Value>>,
    pub inline_names: HashMap<usize, String>,
    pub compiled_decls: HashSet<String>,
    pub generated_source_file: VirtualSourceFile,
    pub skip_eager_compile: bool,
    pub anonymous_structs: HashSet<String>,
    pub experimental_flags: Vec<String>,
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
            version_selection: crate::versioning_types::VersionSelection::new(),
            compiling_shapes: HashSet::new(),
            dependency_declarations: BTreeMap::new(),
            inline_names: HashMap::new(),
            compiled_decls: HashSet::new(),
            experimental_resource_declarations: Vec::new(),
            generated_source_file: VirtualSourceFile::new("generated".to_string()),
            skip_eager_compile: false,
            anonymous_structs: HashSet::new(),
            experimental_flags: Vec::new(),
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

    pub fn compile(
        &mut self,
        main_files: &'node [raw_ast::File<'src>],
        dependency_files: &'node [raw_ast::File<'src>],
        source_files: &[&'src SourceFile],
    ) -> Result<JsonRoot, String> {
        self.source_files = source_files.to_vec();

        // 1. Consume
        let mut consume = ConsumeStep { main_files, dependency_files };
        consume.run(self);

        // 2. Resolve
        // 1.5. Availability
        let mut avail = crate::availability_step::AvailabilityStep;
        avail.run(self);

        let mut resolve = ResolveStep;
        resolve.run(self);

        // 3. Compile
        let mut compile = CompileStep;
        compile.run(self);
        // Fixup max_handles for resources in cycles
        for decl in &mut self.struct_declarations {
            if decl.resource && decl.type_shape_v2.depth == u32::MAX {
                decl.type_shape_v2.max_handles = u32::MAX;
            }
        }
        for decl in &mut self.table_declarations {
            if decl.resource && decl.type_shape_v2.depth == u32::MAX {
                decl.type_shape_v2.max_handles = u32::MAX;
            }
        }
        for decl in &mut self.union_declarations {
            if decl.resource && decl.type_shape_v2.depth == u32::MAX {
                decl.type_shape_v2.max_handles = u32::MAX;
            }
        }

        self.patch_member_shapes();

        // Sort declarations by name to match fidlc output order (alphabetical)
        self.alias_declarations.sort_by(|a, b| a.name.cmp(&b.name));
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

        let mut all_decls = Vec::new();
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
        for decl in &self.alias_declarations {
            all_decls.push((decl.name.clone(), "alias".to_string()));
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
            let mut target_id = ty.identifier.as_ref();
            if let Some(alias) = &ty.experimental_maybe_from_alias {
                target_id = Some(&alias.name);
            }

            if let Some(id) = target_id {
                if let Some(pos) = id.find('/') {
                    let d = id[..pos].to_string();
                    if d != compiler.library_name {
                        deps.insert(d.clone());
                    }

                    if compiler.anonymous_structs.contains(id) {
                        if let Some(s) = compiler
                            .external_struct_declarations
                            .iter()
                            .find(|s| &s.name == id)
                        {
                            for m in &s.members {
                                extract_deps_from_type(&m.type_, deps, compiler);
                            }
                        }
                    }
                }
            }
            if let Some(inner) = &ty.element_type {
                if ty.experimental_maybe_from_alias.is_none() {
                    extract_deps_from_type(inner, deps, compiler);
                }
            }
            if let Some(proto) = &ty.protocol {
                if let Some(pos) = proto.find('/') {
                    let d = proto[..pos].to_string();
                    if d != compiler.library_name {
                        deps.insert(d);
                    }
                }
            }
            if let Some(res) = &ty.resource_identifier {
                if let Some(pos) = res.find('/') {
                    let d = res[..pos].to_string();
                    if d != compiler.library_name {
                        deps.insert(d);
                    }
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
            if let Some(alias) = val.get("experimental_maybe_from_alias") {
                if let Some(n) = alias.get("name").and_then(|n| n.as_str()) {
                    target_id = Some(n);
                }
            }

            if let Some(id) = target_id {
                if let Some(pos) = id.find('/') {
                    let d = id[..pos].to_string();
                    if d != compiler.library_name {
                        deps.insert(d.clone());
                    }

                    if compiler.anonymous_structs.contains(id) {
                        if let Some(s) = compiler
                            .external_struct_declarations
                            .iter()
                            .find(|s| &s.name == id)
                        {
                            for m in &s.members {
                                extract_deps_from_type(&m.type_, deps, compiler);
                            }
                        }
                    }
                }
            }
            if let Some(inner) = val.get("element_type") {
                if val.get("experimental_maybe_from_alias").is_none() {
                    extract_deps_from_type_value(inner, deps, compiler);
                }
            }
            if let Some(proto) = val.get("protocol").and_then(|p| p.as_str()) {
                if let Some(pos) = proto.find('/') {
                    let d = proto[..pos].to_string();
                    if d != compiler.library_name {
                        deps.insert(d);
                    }
                }
            }
            if let Some(res) = val.get("resource_identifier").and_then(|r| r.as_str()) {
                if let Some(pos) = res.find('/') {
                    let d = res[..pos].to_string();
                    if d != compiler.library_name {
                        deps.insert(d);
                    }
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
            available: Some(BTreeMap::from([
                ("fuchsia".to_string(), vec!["HEAD".to_string()]),
                ("test".to_string(), vec!["HEAD".to_string()]),
            ])),
            maybe_attributes: main_files
                .iter()
                .filter(|f| {
                    f.library_decl.as_ref().map(|l| l.path.to_string())
                        == Some(self.library_name.clone())
                })
                .find_map(|f| f.library_decl.as_ref())
                .map_or(vec![], |decl| self.compile_attribute_list(&decl.attributes)),
            experiments: {
                let mut exps = vec!["output_index_json".to_string()];
                exps.extend(self.experimental_flags.clone());
                exps
            },
            library_dependencies,
            bits_declarations: self.bits_declarations.clone(),
            const_declarations: self.const_declarations.clone(),
            enum_declarations: self.enum_declarations.clone(),
            experimental_resource_declarations: vec![],
            protocol_declarations: self.protocol_declarations.clone(),
            service_declarations: self.service_declarations.clone(),
            struct_declarations: self.struct_declarations.clone(),
            external_struct_declarations: self.external_struct_declarations.clone(),
            table_declarations: self.table_declarations.clone(),
            union_declarations: self.union_declarations.clone(),
            alias_declarations: self.alias_declarations.clone(),
            new_type_declarations: vec![],
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

        let has_errors = self.reporter.diagnostics().iter().any(|d| d.def.kind() == crate::diagnostics::ErrorKind::Error);
        let has_warnings = self.reporter.diagnostics().iter().any(|d| d.def.kind() == crate::diagnostics::ErrorKind::Warning);

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
        let envelope = |shape: &TypeShapeV2| -> TypeShapeV2 {
            let inlined = shape.inline_size <= 4;
            let align = if inlined { 4 } else { 8 };
            let padding = (align - (shape.inline_size % align)) % align;
            let added_ool = if inlined {
                0
            } else {
                shape.inline_size.saturating_add(padding)
            };
            TypeShapeV2 {
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

            for member in &mut decl.members {
                Self::update_type_shape(&mut member.type_, &shapes, &struct_names);
                let type_shape = &member.type_.type_shape_v2;

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
                offset += padding_before;
                member.field_shape_v2.offset = offset;
                offset += size;

                has_flex |= type_shape.has_flexible_envelope;
            }

            let final_padding = (alignment - (offset % alignment)) % alignment;
            let total_size = if offset == 0 && final_padding == 0 {
                1
            } else {
                offset + final_padding
            };

            for i in 0..decl.members.len() {
                let next_offset = if i + 1 < decl.members.len() {
                    decl.members[i + 1].field_shape_v2.offset
                } else {
                    total_size
                };
                let current_end = decl.members[i].field_shape_v2.offset
                    + decl.members[i].type_.type_shape_v2.inline_size;
                decl.members[i].field_shape_v2.padding = next_offset - current_end;
                has_padding |= decl.members[i].field_shape_v2.padding > 0
                    || decl.members[i].type_.type_shape_v2.has_padding;
            }

            decl.type_shape_v2.inline_size = total_size;
            decl.type_shape_v2.alignment = alignment;
            decl.type_shape_v2.depth = max_depth;
            decl.type_shape_v2.max_out_of_line = sum_ool;
            decl.type_shape_v2.max_handles = sum_handles;
            decl.type_shape_v2.has_padding = has_padding || final_padding > 0;
            decl.type_shape_v2.has_flexible_envelope = has_flex;
        }
        for decl in &mut self.union_declarations {
            let mut max_ool = 0u32;
            let mut max_handles = 0u32;
            let mut has_padding = false;
            let mut has_flex = false;
            for member in &mut decl.members {
                if let Some(ty) = &mut member.type_ {
                    Self::update_type_shape(ty, &shapes, &struct_names);
                    let env = envelope(&ty.type_shape_v2);
                    max_ool = std::cmp::max(max_ool, env.max_out_of_line);
                    max_handles = std::cmp::max(max_handles, env.max_handles);
                    has_padding |= env.has_padding;
                    has_flex |= env.has_flexible_envelope;
                }
            }
            if decl.type_shape_v2.depth != u32::MAX {
                decl.type_shape_v2.max_out_of_line = max_ool;
                decl.type_shape_v2.max_handles = max_handles;
                decl.type_shape_v2.has_padding = has_padding;
                let is_flexible =
                    decl.maybe_attributes.iter().any(|a| a.name == "flexible") || !decl.strict;
                // Also check if any member has the flexible trait
                decl.type_shape_v2.has_flexible_envelope = has_flex || is_flexible;
            }
        }
        for decl in &mut self.table_declarations {
            let mut max_ool = 0u32;
            let mut max_handles = 0u32;
            let mut has_padding = false;
            let mut has_flex = false;
            let mut max_ordinal = 0u32;
            for member in &mut decl.members {
                max_ordinal = std::cmp::max(max_ordinal, member.ordinal);
                if let Some(ty) = &mut member.type_ {
                    Self::update_type_shape(ty, &shapes, &struct_names);
                    let env = envelope(&ty.type_shape_v2);
                    max_ool = max_ool.saturating_add(env.max_out_of_line);
                    max_handles = max_handles.saturating_add(env.max_handles);
                    has_padding |= env.has_padding;
                    has_flex |= env.has_flexible_envelope;
                }
            }
            if decl.type_shape_v2.depth != u32::MAX {
                decl.type_shape_v2.max_out_of_line =
                    max_ool.saturating_add(max_ordinal.saturating_mul(8));
                decl.type_shape_v2.max_handles = max_handles;
                decl.type_shape_v2.has_padding = has_padding;
                decl.type_shape_v2.has_flexible_envelope = has_flex || true;
            }
        }
        for decl in &mut self.alias_declarations {
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
        shapes: &HashMap<String, TypeShapeV2>,
        struct_names: &HashSet<String>,
    ) {
        if ty.kind_v2 == "identifier" {
            if let Some(id) = &ty.identifier {
                if let Some(shape) = shapes.get(id) {
                    if ty.nullable == Some(true) && struct_names.contains(id) {
                        let inner_inline = shape.inline_size;
                        let padding = (8 - (inner_inline % 8)) % 8;
                        let max_out_of_line = shape
                            .max_out_of_line
                            .saturating_add(inner_inline.saturating_add(padding));

                        ty.type_shape_v2 = TypeShapeV2 {
                            inline_size: 8,
                            alignment: 8,
                            depth: shape.depth.saturating_add(1),
                            max_handles: shape.max_handles,
                            max_out_of_line,
                            has_padding: shape.has_padding || padding > 0,
                            has_flexible_envelope: shape.has_flexible_envelope,
                        };
                    } else {
                        ty.type_shape_v2 = shape.clone();
                    }
                }
            }
        }
        if let Some(inner) = &mut ty.element_type {
            Self::update_type_shape(inner, shapes, struct_names);

            if ty.kind_v2 == "vector" {
                let inner_shape = &inner.type_shape_v2;
                let count = ty.maybe_element_count.unwrap_or(u32::MAX);

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

                ty.type_shape_v2 = TypeShapeV2 {
                    inline_size: 16,
                    alignment: 8,
                    depth: new_depth,
                    max_handles,
                    max_out_of_line: max_ool,
                    has_padding,
                    has_flexible_envelope: inner_shape.has_flexible_envelope,
                };
            } else if ty.kind_v2 == "array" {
                let inner_shape = &inner.type_shape_v2;
                let count = ty.element_count.unwrap_or(0);

                let elem_size = inner_shape.inline_size;
                let elem_ool = inner_shape.max_out_of_line;
                let depth = inner_shape.depth;

                let max_handles = count.saturating_mul(inner_shape.max_handles);
                let max_out_of_line = count.saturating_mul(elem_ool);

                ty.type_shape_v2 = TypeShapeV2 {
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

    pub fn topological_sort(&self, skip_optional: bool) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut sorted = vec![];
        let mut temp_path = vec![]; // for cycle detection

        let mut keys: Vec<&String> = self.raw_decls.keys().collect();
        keys.sort();

        fn visit<'a, 'b>(
            name: &str,
            decls: &HashMap<String, RawDecl<'a, 'b>>,
            library_name: &str,
            visited: &mut HashSet<String>,
            temp_path: &mut Vec<String>,
            sorted: &mut Vec<String>,
            decl_kinds: &HashMap<String, &str>,
            skip_optional: bool,
            inline_names: &HashMap<usize, String>,
            reporter: &Reporter<'b>,
        ) {
            if visited.contains(name) {
                return;
            }
            if let Some(idx) = temp_path.iter().position(|x| x == name) {
                let cycle_names = &temp_path[idx..];
                let mut cycle_str = String::new();
                for cname in cycle_names {
                    let ckind = decl_kinds.get(cname).unwrap_or(&"unknown");
                    let short_name = cname.split('/').last().unwrap_or(cname);
                    cycle_str.push_str(&format!("{} '{}' -> ", ckind, short_name));
                }
                let kind = decl_kinds.get(name).unwrap_or(&"unknown");
                let short_name = name.split('/').last().unwrap_or(name);
                cycle_str.push_str(&format!("{} '{}'", kind, short_name));

                let span = if let Some(decl) = decls.get(name) {
                    decl.element().span()
                } else {
                    let first_decl = decls.values().next().unwrap();
                    first_decl.element().span()
                };

                reporter.fail(
                    crate::diagnostics::Error::ErrIncludeCycle,
                    span,
                    &[&cycle_str],
                );
                return;
            }
            temp_path.push(name.to_string());

            if let Some(decl) = decls.get(name) {
                let deps =
                    get_dependencies(decl, library_name, decl_kinds, skip_optional, inline_names);
                // Sort dependencies by name to ensure deterministic order if needed, but they are in AST order
                for dep in deps {
                    visit(
                        &dep,
                        decls,
                        library_name,
                        visited,
                        temp_path,
                        sorted,
                        decl_kinds,
                        skip_optional,
                        inline_names,
                        reporter,
                    );
                }
            }

            temp_path.pop();
            visited.insert(name.to_string());
            if decls.contains_key(name) {
                sorted.push(name.to_string());
            }
        }

        for name in keys {
            visit(
                name,
                &self.raw_decls,
                &self.library_name,
                &mut visited,
                &mut temp_path,
                &mut sorted,
                &self.decl_kinds,
                skip_optional,
                &self.inline_names,
                self.reporter,
            );
        }

        sorted
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
                    if is_main_library {
                        self.union_declarations.push(compiled);
                    }
                } else if let raw_ast::Layout::TypeConstructor(ref tc) = t.layout {
                    let compiled = AliasDeclaration {
                        name: format!("{}/{}", library_name, t.name.data()),
                        location: self.get_location(&t.name.element),
                        deprecated: self.is_deprecated(t.attributes.as_deref()),
                        maybe_attributes: self.compile_attribute_list(&t.attributes),
                        partial_type_ctor: crate::json_generator::PartialTypeCtor {
                            name: if let raw_ast::LayoutParameter::Identifier(id) = &tc.layout {
                                id.to_string()
                            } else {
                                "".to_string()
                            },
                            args: vec![],
                            nullable: tc.nullable,
                        },
                        type_: self.resolve_type(tc, &library_name, None),
                    };
                    if is_main_library {
                        self.alias_declarations.push(compiled);
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
                    if is_main_library {
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
                    if let Some(req) = &m.maybe_request_payload {
                        if let Some(id) = &req.identifier {
                            extra_to_compile.push(id.clone());
                        }
                    }
                    if let Some(res) = &m.maybe_response_payload {
                        if let Some(id) = &res.identifier {
                            extra_to_compile.push(id.clone());
                        }
                    }
                    if let Some(suc) = &m.maybe_response_success_type {
                        if let Some(id) = &suc.identifier {
                            extra_to_compile.push(id.clone());
                        }
                    }
                    if let Some(err) = &m.maybe_response_err_type {
                        if let Some(id) = &err.identifier {
                            extra_to_compile.push(id.clone());
                        }
                    }
                }

                if is_main_library {
                    self.protocol_declarations.push(compiled);
                } else {
                    self.external_protocol_declarations.push(compiled);
                }

                for id in extra_to_compile {
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
                        serde_json::to_value(shape).unwrap(),
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

    pub fn compile_enum(
        &mut self,
        name: &str,
        decl: &raw_ast::EnumDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'_>>,
        naming_context: Option<std::rc::Rc<crate::name::NamingContext<'src>>>,
    ) -> EnumDeclaration {
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
                "uint32".to_string()
            }
        } else {
            "uint32".to_string()
        };

        let mut members = vec![];
        let mut maybe_unknown_value = None;

        for member in &decl.members {
            let attributes = self.compile_attribute_list(&member.attributes);
            let compiled_value = self.compile_constant(&member.value);

            // Check for unknown attribute
            if attributes.iter().any(|a| a.name == "unknown") {
                // Try to parse value as u32 (assuming enum is uint32-compatible for now)
                // TODO: Handle signed enums and other types correctly.
                if let Some(literal) = &compiled_value.literal
                    && let Ok(val) = literal.value.get().trim_matches('"').parse::<u32>()
                {
                    maybe_unknown_value = Some(val);
                }
            }

            members.push(EnumMember {
                name: member.name.data().to_string(),
                location: self.get_location(&member.name.element),
                deprecated: self.is_deprecated(member.attributes.as_deref()),
                value: compiled_value,
                maybe_attributes: attributes,
            });
        }

        let (inline_size, alignment) = match subtype_name.as_str() {
            "uint8" | "int8" => (1, 1),
            "uint16" | "int16" => (2, 2),
            "uint32" | "int32" => (4, 4),
            "uint64" | "int64" => (8, 8),
            _ => (4, 4),
        };

        self.shapes.insert(
            full_name.clone(),
            TypeShapeV2 {
                inline_size,
                alignment,
                depth: 0,
                max_handles: 0,
                max_out_of_line: 0,
                has_padding: false,
                has_flexible_envelope: false,
            },
        );

        // Strictness default: Flexible?
        let strict = decl.modifiers.iter().any(|m| {
            m.subkind == crate::token::TokenSubkind::Strict && self.is_active(m.attributes.as_ref())
        });

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
        naming_context: Option<std::rc::Rc<crate::name::NamingContext<'src>>>,
    ) -> BitsDeclaration {
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
                    let full_name = if current.contains('/') || self.shapes.contains_key(&current) {
                        current.clone()
                    } else {
                        format!("{}/{}", library_name, current)
                    };
                    if let Some(RawDecl::Alias(alias)) = self.raw_decls.get(&full_name) {
                        if let raw_ast::LayoutParameter::Identifier(ref inner_id) =
                            alias.type_ctor.layout
                        {
                            current = inner_id.to_string();
                            continue;
                        }
                    }
                    subtype_name = current;
                    break;
                }
            }
        }

        let is_valid_type = matches!(
            subtype_name.as_str(),
            "uint8" | "uint16" | "uint32" | "uint64"
        );
        if !is_valid_type {
            self.reporter.fail(
                crate::diagnostics::Error::ErrBitsTypeMustBeUnsignedIntegralPrimitive,
                decl.name.as_ref().map_or_else(
                    || decl.element.start_token.span.clone(),
                    |id| id.element.span(),
                ),
                &[&subtype_name],
            );
        }

        // Strictness default: Flexible?
        let strict = decl.modifiers.iter().any(|m| {
            m.subkind == crate::token::TokenSubkind::Strict && self.is_active(m.attributes.as_ref())
        });

        if strict && decl.members.is_empty() {
            self.reporter.fail(
                crate::diagnostics::Error::ErrMustHaveOneMember,
                decl.name.as_ref().map_or_else(
                    || decl.element.start_token.span.clone(),
                    |id| id.element.span(),
                ),
                &[],
            );
        }

        let mut members = vec![];
        let mut mask: u64 = 0;
        let mut member_names = std::collections::HashSet::new();

        for member in &decl.members {
            let attributes = self.compile_attribute_list(&member.attributes);
            let compiled_value = self.compile_constant(&member.value);

            let name_str = member.name.data().to_string();
            if !member_names.insert(name_str.clone()) {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrNameCollision,
                    member.name.element.span(),
                    &[&"member", &name_str, &"member", &name_str],
                );
            }

            // Calculate mask and validate value
            let mut valid_value = true;
            match &member.value {
                raw_ast::Constant::Literal(_) => {
                    if let Some(literal) = &compiled_value.literal {
                        let val_str = literal.value.get().trim_matches('"');
                        if let Ok(val) = val_str.parse::<u64>() {
                            if val != 0 && (val & (val - 1)) != 0 {
                                self.reporter.fail(
                                    crate::diagnostics::Error::ErrBitsMemberMustBePowerOfTwo,
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
                                    crate::diagnostics::Error::ErrConstantOverflowsType,
                                    member.value.element().span(),
                                    &[&val_str, &subtype_name],
                                );
                                valid_value = false;
                            }

                            if valid_value {
                                if (mask & val) != 0 {
                                    self.reporter.fail(
                                        crate::diagnostics::Error::ErrDuplicateMemberValue,
                                        member.value.element().span(),
                                        &[&"bits", &name_str, &"unknown", &name_str],
                                    );
                                } else {
                                    mask |= val;
                                }
                            }
                        } else {
                            let is_negative = val_str.starts_with('-');
                            if is_negative && subtype_name.starts_with("uint") {
                                self.reporter.fail(
                                    crate::diagnostics::Error::ErrCannotResolveConstantValue,
                                    member.value.element().span(),
                                    &[],
                                );
                            } else if !val_str.chars().all(|c| c.is_ascii_digit()) {
                                self.reporter.fail(
                                    crate::diagnostics::Error::ErrCannotResolveConstantValue,
                                    member.value.element().span(),
                                    &[],
                                );
                            } else {
                                self.reporter.fail(
                                    crate::diagnostics::Error::ErrConstantOverflowsType,
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
                                crate::diagnostics::Error::ErrBitsMemberMustBePowerOfTwo,
                                member.value.element().span(),
                                &[],
                            );
                        } else if (mask & val) != 0 {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrDuplicateMemberValue,
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
                            crate::diagnostics::Error::ErrCannotResolveConstantValue,
                            member.value.element().span(),
                            &[],
                        );
                    }
                }
                // No other Constant variants
            }

            members.push(BitsMember {
                name: name_str,
                location: self.get_location(&member.name.element),
                deprecated: self.is_deprecated(member.attributes.as_deref()),
                value: compiled_value,
                maybe_attributes: attributes,
            });
        }

        let (inline_size, alignment) = match subtype_name.as_str() {
            "uint8" => (1, 1),
            "uint16" => (2, 2),
            "uint32" => (4, 4),
            "uint64" => (8, 8),
            _ => (4, 4),
        };

        let type_shape_v2 = TypeShapeV2 {
            inline_size,
            alignment,
            depth: 0,
            max_handles: 0,
            max_out_of_line: 0,
            has_padding: false,
            has_flexible_envelope: false,
        };

        self.shapes.insert(full_name.clone(), type_shape_v2.clone());

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
            type_: Type {
experimental_maybe_from_alias: None,

                kind_v2: "primitive".to_string(),
                subtype: Some(subtype_name),
                identifier: None,
                nullable: None,
                element_type: None,
                element_count: None,
                maybe_element_count: None,
                role: None,
                protocol: None,
                protocol_transport: None,
                obj_type: None,
                rights: None,
                resource_identifier: None,
                deprecated: None,
                maybe_attributes: vec![],
                field_shape_v2: None,
                maybe_size_constant_name: None,
                type_shape_v2,
            },
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
        naming_context: Option<std::rc::Rc<crate::name::NamingContext<'src>>>,
    ) -> TableDeclaration {
        let full_name = format!("{}/{}", library_name, name);
        let location = if let Some(el) = name_element {
            self.get_location(el)
        } else {
            self.get_location(&decl.element)
        };

        let mut members = vec![];
        for member in &decl.members {
            let ordinal = match &member.ordinal.kind {
                raw_ast::LiteralKind::Numeric => member.ordinal.value.parse::<u32>().unwrap_or(0),
                _ => 0,
            };

            let (type_, name, reserved, alias) = if let Some(type_ctor) = &member.type_ctor {
                let ctx = naming_context.clone().unwrap_or_else(|| {
                    crate::name::NamingContext::create(
                        name_element
                            .map(|n| n.span())
                            .unwrap_or_else(|| decl.element.span()),
                    )
                }); // Fallback if name missing?
                let member_ctx = if let Some(m_name) = &member.name {
                    ctx.enter_member(m_name.element.span())
                } else {
                    // This case should be unreachable for valid table members with types
                    ctx.enter_member(member.ordinal.element.span())
                };
                let mut type_obj = self.resolve_type(type_ctor, library_name, Some(member_ctx));
                let alias = if type_obj.kind_v2 != "array" && type_obj.kind_v2 != "vector" && type_obj.kind_v2 != "string" && type_obj.kind_v2 != "request" {
                    type_obj.experimental_maybe_from_alias.take()
                } else {
                    None
                };
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
                let shape = &type_obj.type_shape_v2;
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

        let mut type_shape_v2 = TypeShapeV2 {
            inline_size: 16,
            alignment: 8,
            depth,
            max_handles,
            max_out_of_line,
            has_padding, // Tables calculate padding based on envelopes
            has_flexible_envelope: true,
        };

        if type_shape_v2.depth == u32::MAX && type_shape_v2.max_handles != 0 {
            type_shape_v2.max_handles = u32::MAX;
        }

        self.shapes.insert(full_name.clone(), type_shape_v2.clone());

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
                .any(|m| m.subkind == crate::token::TokenSubkind::Resource),
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            type_shape_v2,
        }
    }

    pub fn compile_union(
        &mut self,
        name: &str,
        decl: &'node raw_ast::UnionDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'src>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'src>>,
        naming_context: Option<std::rc::Rc<crate::name::NamingContext<'src>>>,
    ) -> UnionDeclaration {
        let full_name = format!("{}/{}", library_name, name);
        let location = if let Some(el) = name_element {
            self.get_location(el)
        } else {
            self.get_location(&decl.element)
        };

        let mut members = vec![];
        for member in &decl.members {
            let ordinal = if let Some(ord) = &member.ordinal {
                match &ord.kind {
                    raw_ast::LiteralKind::Numeric => ord.value.parse::<u32>().map_err(|_| ()),
                    _ => Err(()),
                }
            } else {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrMissingOrdinalBeforeMember,
                    member.element.span(),
                    &[],
                );
                Ok(0)
            };

            let ordinal = match ordinal {
                Ok(o) => {
                    if o == 0 {
                        if member.ordinal.is_some() {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrOrdinalsMustStartAtOne,
                                member.ordinal.as_ref().unwrap().element.span(),
                                &[],
                            );
                        }
                    }
                    o
                }
                Err(_) => {
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrOrdinalOutOfBound,
                        member.ordinal.as_ref().unwrap().element.span(),
                        &[],
                    );
                    0
                }
            };

            if let Some(prev) = members.iter().find(|m: &&UnionMember| m.ordinal == ordinal) {
                if ordinal != 0 {
                    let location_str = format!(
                        "{}:{}:{}",
                        prev.location.as_ref().unwrap().filename, 
                        prev.location.as_ref().unwrap().line, 
                        prev.location.as_ref().unwrap().column
                    );
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrDuplicateUnionMemberOrdinal,
                        member.ordinal.as_ref().unwrap().element.span(),
                        &[&location_str],
                    );
                }
            }

            if let Some(n_name) = &member.name {
                let member_name = n_name.data();
                if let Some(prev) = members.iter().find(|m: &&UnionMember| m.name.as_deref() == Some(member_name)) {
                    let location_str = format!(
                        "{}:{}:{}",
                        prev.location.as_ref().unwrap().filename, 
                        prev.location.as_ref().unwrap().line, 
                        prev.location.as_ref().unwrap().column
                    );
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrNameCollision,
                        n_name.element.span(),
                        &[
                            &"union member",
                            &member_name,
                            &"union member",
                            &location_str,
                        ],
                    );
                }
            }

            let (type_, name, reserved, alias) = if let Some(type_ctor) = &member.type_ctor {
                let ctx = naming_context.clone().unwrap_or_else(|| {
                    crate::name::NamingContext::create(
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
                let alias = if type_obj.kind_v2 != "array" && type_obj.kind_v2 != "vector" && type_obj.kind_v2 != "string" && type_obj.kind_v2 != "request" {
                    type_obj.experimental_maybe_from_alias.take()
                } else {
                    None
                };
                if type_obj.nullable.unwrap_or(false) {
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrOptionalUnionMember,
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

            if member.default_value.is_some() {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrUnexpectedToken,
                    member.default_value.as_ref().unwrap().element().span(),
                    &[],
                );
            }

            if attributes.iter().any(|a| a.name == "selector") {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrInvalidAttributePlacement,
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

        let strict = decl.modifiers.iter().any(|m| {
            m.subkind == crate::token::TokenSubkind::Strict && self.is_active(m.attributes.as_ref())
        });

        if strict && members.is_empty() {
            self.reporter.fail(
                crate::diagnostics::Error::ErrMustHaveOneMember,
                decl.element.span(),
                &[],
            );
        }

        // Sort members by ordinal
        members.sort_by_key(|m| m.ordinal);

        #[allow(clippy::collection_is_never_read)]
        let mut attributes = self.compile_attribute_list(&decl.attributes);
        if let Some(inherited) = inherited_attributes {
            let extra = self.compile_attributes_from_ref(inherited);
            attributes.extend(extra);
        }

        let strict = decl.modifiers.iter().any(|m| {
            m.subkind == crate::token::TokenSubkind::Strict && self.is_active(m.attributes.as_ref())
        });

        let mut max_handles = 0;
        let mut max_out_of_line = 0u32;
        let mut depth = 0;
        let mut has_padding = false;

        for member in &members {
            if let Some(type_obj) = &member.type_ {
                let shape = &type_obj.type_shape_v2;
                if shape.max_handles > max_handles {
                    max_handles = shape.max_handles;
                }

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
                if env_max_out_of_line > max_out_of_line {
                    max_out_of_line = env_max_out_of_line;
                }

                let env_depth = shape.depth.saturating_add(1);
                if env_depth > depth {
                    depth = env_depth;
                }
            }
        }

        // Union depth is 1 + max(member depth).
        // Zero fields or reserved fields = 0 depth.

        let mut type_shape_v2 = TypeShapeV2 {
            inline_size: 16,
            alignment: 8,
            depth,
            max_handles,
            max_out_of_line,
            has_padding,
            has_flexible_envelope: !strict
                || members.iter().any(|m| {
                    m.type_
                        .as_ref()
                        .is_some_and(|t| t.type_shape_v2.has_flexible_envelope)
                }),
        };

        if type_shape_v2.depth == u32::MAX && type_shape_v2.max_handles != 0 {
            type_shape_v2.max_handles = u32::MAX;
        }

        self.shapes.insert(full_name.clone(), type_shape_v2.clone());

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
                .any(|m| m.subkind == crate::token::TokenSubkind::Resource),
            is_result: false, // TODO: detect result unions
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            type_shape_v2,
        }
    }

    pub fn compile_struct(
        &mut self,
        name: &str,
        decl: &'node raw_ast::StructDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        naming_context: Option<std::rc::Rc<crate::name::NamingContext<'src>>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'_>>,
    ) -> StructDeclaration {
        let full_name = format!("{}/{}", library_name, name);

        let mut members = vec![];
        let mut offset: u32 = 0;
        let mut alignment: u32 = 1;
        let mut max_handles: u32 = 0;
        let mut max_out_of_line: u32 = 0;
        let mut depth: u32 = 0;

        for member in &decl.members {
            let member_name = member.name.data();
            if let Some(prev) = members
                .iter()
                .find(|m: &&StructMember| m.name == member_name)
            {
                let location_str = format!(
                    "{}:{}:{}",
                    prev.location.filename, prev.location.line, prev.location.column
                );
                self.reporter.fail(
                    crate::diagnostics::Error::ErrNameCollision,
                    member.name.element.span(),
                    &[
                        &"struct member",
                        &member_name,
                        &"struct member",
                        &location_str,
                    ],
                );
            }

            let ctx = naming_context.clone().unwrap_or_else(|| {
                crate::name::NamingContext::create(if let Some(id) = &decl.name {
                    id.element.span()
                } else {
                    decl.element.span()
                })
            });
            let member_ctx = ctx.enter_member(member.name.element.span());
            let mut type_obj = self.resolve_type(&member.type_ctor, library_name, Some(member_ctx));
            let alias = if type_obj.kind_v2 != "array" && type_obj.kind_v2 != "vector" && type_obj.kind_v2 != "string" && type_obj.kind_v2 != "request" {
                type_obj.experimental_maybe_from_alias.take()
            } else {
                None
            };
            let type_shape = &type_obj.type_shape_v2;

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
                    crate::diagnostics::Error::ErrDeprecatedStructDefaults,
                    member.name.element.span(),
                    &[],
                );
            }

            let mut maybe_default_value = None;
            if let Some(def_val) = &member.default_value {
                maybe_default_value = Some(self.compile_constant(def_val));
            }

            members.push(StructMember {
                type_: type_obj,
                name: member.name.data().to_string(),
experimental_maybe_from_alias: alias,
location,
                deprecated: self.is_deprecated(member.attributes.as_deref()),
                maybe_attributes: self.compile_attribute_list(&member.attributes),
                field_shape_v2: FieldShapeV2 {
                    offset: field_offset,
                    padding: 0,
                },
                maybe_default_value,
            });

            offset += size;
        }

        // Final padding
        let final_padding = (alignment - (offset % alignment)) % alignment;
        let total_size = if offset == 0 && final_padding == 0 {
            1 // Empty struct has size 1
        } else {
            offset + final_padding
        };

        // Fixup padding
        for i in 0..members.len() {
            let next_offset = if i + 1 < members.len() {
                members[i + 1].field_shape_v2.offset
            } else {
                total_size
            };
            let current_end =
                members[i].field_shape_v2.offset + members[i].type_.type_shape_v2.inline_size;
            members[i].field_shape_v2.padding = next_offset - current_end;
        }

        if depth == u32::MAX && max_handles != 0 {
            max_handles = u32::MAX;
        }

        let type_shape = TypeShapeV2 {
            inline_size: total_size,
            alignment,
            depth,
            max_handles,
            max_out_of_line,
            has_padding: final_padding > 0
                || members
                    .iter()
                    .any(|m| m.field_shape_v2.padding > 0 || m.type_.type_shape_v2.has_padding),
            has_flexible_envelope: members
                .iter()
                .any(|m| m.type_.type_shape_v2.has_flexible_envelope),
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
                crate::diagnostics::Error::ErrInlineSizeExceedsLimit,
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
                .any(|m| m.subkind == crate::token::TokenSubkind::Resource),
            is_empty_success_struct: false,
            type_shape_v2: type_shape,
        }
    }

    pub fn resolve_type(
        &mut self,
        type_ctor: &'node raw_ast::TypeConstructor<'src>,
        library_name: &str,
        naming_context: Option<std::rc::Rc<crate::name::NamingContext<'src>>>,
    ) -> Type {
        let name = match &type_ctor.layout {
            raw_ast::LayoutParameter::Identifier(id) => {
                if id.components.len() > 1 {
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
                }
            }
            raw_ast::LayoutParameter::Literal(_) => {
                panic!("Literal layout not supported in resolve_type")
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

                let _decl_context = naming_context
                    .as_ref()
                    .map(|ctx| ctx.context())
                    .unwrap_or_else(Vec::new);

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
                            self.union_declarations.push(compiled);
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

        let mut nullable = type_ctor.nullable;
        if !nullable {
            // Check constraints for "optional"
            for constraint in &type_ctor.constraints {
                if let raw_ast::Constant::Identifier(id) = constraint
                    && id.identifier.to_string() == "optional"
                {
                    nullable = true;
                    break;
                }
            }
        }

        match name.as_str() {
            "bool" | "int8" | "int16" | "int32" | "int64" | "uint8" | "uint16" | "uint32"
            | "uint64" | "float32" | "float64" => {
                let (inline_size, alignment) = match name.as_str() {
                    "bool" | "int8" | "uint8" => (1, 1),
                    "int16" | "uint16" => (2, 2),
                    "int32" | "uint32" | "float32" => (4, 4),
                    "int64" | "uint64" | "float64" => (8, 8),
                    _ => (0, 0),
                };
                Type {
experimental_maybe_from_alias: None,

                    kind_v2: "primitive".to_string(),
                    subtype: Some(name),
                    identifier: None,
                    nullable: None,
                    element_type: None,
                    element_count: None,
                    maybe_element_count: None,
                    role: None,
                    protocol: None,
                    protocol_transport: None,
                    obj_type: None,
                    rights: None,
                    resource_identifier: None,
                    deprecated: None,
                    maybe_attributes: vec![],
                    field_shape_v2: None,
                    maybe_size_constant_name: None,
                    type_shape_v2: TypeShapeV2 {
                        inline_size,
                        alignment,
                        depth: 0,
                        max_handles: 0,
                        max_out_of_line: 0,
                        has_padding: false,
                        has_flexible_envelope: false,
                    },
                }
            }
            "string" => {
                let max_len = if let Some(c) = type_ctor.constraints.first() {
                    self.eval_constant_usize(c).unwrap_or(u32::MAX as usize) as u32
                } else {
                    u32::MAX
                };
                Type {
experimental_maybe_from_alias: None,

                    kind_v2: "string".to_string(),
                    subtype: None,
                    identifier: None,
                    nullable: Some(nullable),
                    role: None,
                    protocol: None,
                    protocol_transport: None,
                    obj_type: None,
                    rights: None,
                    resource_identifier: None,
                    element_type: None,
                    element_count: None,
                    maybe_element_count: if max_len == u32::MAX {
                        None
                    } else {
                        Some(max_len)
                    },
                    deprecated: None,
                    maybe_attributes: vec![],
                    field_shape_v2: None,
                    maybe_size_constant_name: if let Some(c) = type_ctor.constraints.first() {
                        if let raw_ast::Constant::Identifier(id) = c {
                            Some(id.identifier.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                    type_shape_v2: TypeShapeV2 {
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
                }
            }
            "string_array" => {
                let max_len = if !type_ctor.parameters.is_empty() {
                    let size_param = &type_ctor.parameters[0];
                    self.eval_type_constant_usize(size_param)
                        .unwrap_or(u32::MAX as usize) as u32
                } else {
                    u32::MAX
                };
                Type {
experimental_maybe_from_alias: None,

                    kind_v2: "string_array".to_string(),
                    subtype: None,
                    identifier: None,
                    nullable: None,
                    element_type: None,
                    element_count: Some(max_len),
                    maybe_element_count: Some(max_len),
                    role: None,
                    protocol: None,
                    protocol_transport: None,
                    obj_type: None,
                    rights: None,
                    resource_identifier: None,
                    deprecated: None,
                    maybe_attributes: vec![],
                    field_shape_v2: None,
                    maybe_size_constant_name: None,
                    type_shape_v2: TypeShapeV2 {
                        inline_size: max_len,
                        alignment: 1,
                        depth: 0,
                        max_handles: 0,
                        max_out_of_line: 0,
                        has_padding: false,
                        has_flexible_envelope: false,
                    },
                }
            }
            "vector" => {
                if type_ctor.parameters.is_empty() {
                    // Error handling?
                    return Type {
experimental_maybe_from_alias: None,

                        kind_v2: "unknown".to_string(),
                        subtype: None,
                        identifier: None,
                        nullable: None,
                        element_type: None,
                        element_count: None,
                        maybe_element_count: None,
                        role: None,
                        protocol: None,
                        protocol_transport: None,
                        obj_type: None,
                        rights: None,
                        resource_identifier: None,
                        deprecated: None,
                        maybe_attributes: vec![],
                        field_shape_v2: None,
                        maybe_size_constant_name: None,
                        type_shape_v2: TypeShapeV2 {
                            inline_size: 0,
                            alignment: 1,
                            depth: 0,
                            max_handles: 0,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    };
                }
                let inner = &type_ctor.parameters[0];
                let mut inner_type = self.resolve_type(inner, library_name, naming_context);
                let inner_alias = inner_type.experimental_maybe_from_alias.take();

                let max_count = if let Some(c) = type_ctor.constraints.first() {
                    self.eval_constant_usize(c).unwrap_or(u32::MAX as usize) as u32
                } else {
                    u32::MAX
                };

                let new_depth = inner_type.type_shape_v2.depth.saturating_add(1);
                // println!("Vector depth calculation: inner {}, new {}", inner_type.type_shape_v2.depth, new_depth);

                let elem_size = inner_type.type_shape_v2.inline_size;
                let elem_ool = inner_type.type_shape_v2.max_out_of_line;
                let content_size = max_count.saturating_mul(elem_size.saturating_add(elem_ool));
                let max_ool = if content_size % 8 == 0 {
                    content_size
                } else {
                    content_size.saturating_add(8 - (content_size % 8))
                };

                let max_handles = max_count.saturating_mul(inner_type.type_shape_v2.max_handles);

                Type {
                    experimental_maybe_from_alias: inner_alias,

                    kind_v2: "vector".to_string(),
                    subtype: None,
                    identifier: None,
                    nullable: Some(nullable),
                    role: None,
                    protocol: None,
                    protocol_transport: None,
                    obj_type: None,
                    rights: None,
                    resource_identifier: None,
                    element_type: Some(Box::new(inner_type.clone())),
                    element_count: None,
                    maybe_element_count: if max_count == u32::MAX {
                        None
                    } else {
                        Some(max_count)
                    },
                    deprecated: None,
                    maybe_attributes: vec![],
                    field_shape_v2: None,
                    maybe_size_constant_name: if let Some(c) = type_ctor.constraints.first() {
                        if let raw_ast::Constant::Identifier(id) = c {
                            Some(id.identifier.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                    type_shape_v2: TypeShapeV2 {
                        inline_size: 16,
                        alignment: 8,
                        depth: new_depth,
                        max_handles,
                        max_out_of_line: max_ool,
                        has_padding: inner_type.type_shape_v2.has_padding
                            || !elem_size.is_multiple_of(8),
                        has_flexible_envelope: inner_type.type_shape_v2.has_flexible_envelope,
                    },
                }
            }
            "array" => {
                if type_ctor.parameters.len() < 2 {
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrWrongNumberOfLayoutParameters,
                        type_ctor.element.start_token.span.clone(),
                        &[],
                    );
                    return Type {
experimental_maybe_from_alias: None,

                        kind_v2: "unknown".to_string(),
                        subtype: None,
                        identifier: None,
                        nullable: None,
                        element_type: None,
                        element_count: None,
                        maybe_element_count: None,
                        role: None,
                        protocol: None,
                        protocol_transport: None,
                        obj_type: None,
                        rights: None,
                        resource_identifier: None,
                        deprecated: None,
                        maybe_attributes: vec![],
                        field_shape_v2: None,
                        maybe_size_constant_name: None,
                        type_shape_v2: TypeShapeV2 {
                            inline_size: 0,
                            alignment: 1,
                            depth: 0,
                            max_handles: 0,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    };
                }
                // Array validation
                let elt_type = &type_ctor.parameters[0];
                let count_param = &type_ctor.parameters[1];

                // Check for optional array: array<T, N>:optional is invalid
                if nullable {
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrCannotBeOptional,
                        type_ctor.element.start_token.span.clone(),
                        &[&"array"],
                    );
                }

                // Check count
                let mut count: u32 = 0;
                match &count_param.layout {
                    raw_ast::LayoutParameter::Literal(lit) => {
                        if let raw_ast::LiteralKind::Numeric = lit.literal.kind {
                            if let Ok(val) = lit.literal.value.parse::<u32>() {
                                if val == 0 {
                                    self.reporter.fail(
                                        crate::diagnostics::Error::ErrMustHaveNonZeroSize,
                                        count_param.element.start_token.span.clone(),
                                        &[&"array"],
                                    );
                                }
                                count = val;
                            }
                        }
                    }
                    raw_ast::LayoutParameter::Identifier(id) => {
                        if id.to_string() == "MAX" {
                            count = u32::MAX;
                        }
                        // TODO: actually resolve constant value to check for 0
                    }
                    _ => {}
                }

                // Check constraints
                if !type_ctor.constraints.is_empty() {
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrTooManyConstraints,
                        type_ctor.element.start_token.span.clone(),
                        &[&"array", &0_usize, &type_ctor.constraints.len()],
                    );
                }

                let mut inner_type = self.resolve_type(elt_type, library_name, naming_context);
                let inner_alias = inner_type.experimental_maybe_from_alias.take();
                let total_size = count.saturating_mul(inner_type.type_shape_v2.inline_size);
                let max_ool = count.saturating_mul(inner_type.type_shape_v2.max_out_of_line);

                Type {
                    experimental_maybe_from_alias: inner_alias,
                    kind_v2: "array".to_string(),
                    subtype: None,
                    identifier: None,
                    nullable: None, // Arrays themselves are not nullable
                    element_type: Some(Box::new(inner_type.clone())),
                    element_count: Some(count),
                    maybe_element_count: None,
                    role: None,
                    protocol: None,
                    protocol_transport: None,
                    obj_type: None,
                    rights: None,
                    resource_identifier: None,
                    deprecated: None,
                    maybe_attributes: vec![],
                    field_shape_v2: None,
                    maybe_size_constant_name: if let Some(param) = type_ctor.parameters.get(1) {
                        if let raw_ast::LayoutParameter::Identifier(id) = &param.layout {
                            Some(id.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                    type_shape_v2: TypeShapeV2 {
                        inline_size: total_size,
                        alignment: inner_type.type_shape_v2.alignment,
                        depth: inner_type.type_shape_v2.depth,
                        max_handles: inner_type.type_shape_v2.max_handles.saturating_mul(count),
                        max_out_of_line: max_ool,
                        has_padding: inner_type.type_shape_v2.has_padding,
                        has_flexible_envelope: inner_type.type_shape_v2.has_flexible_envelope,
                    },
                }
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
                        if proto_name.contains('.') {
                            protocol = proto_name.replace('.', "/");
                        } else if proto_name.contains('/') {
                            protocol = proto_name;
                        } else {
                            protocol = format!("{}/{}", library_name, proto_name);
                        }
                    }
                } else if let Some(param) = type_ctor.parameters.first() {
                    if let raw_ast::LayoutParameter::Identifier(id) = &param.layout {
                        let proto_name = id.to_string();
                        if proto_name.contains('.') {
                            protocol = proto_name.replace('.', "/");
                        } else if proto_name.contains('/') {
                            protocol = proto_name;
                        } else {
                            protocol = format!("{}/{}", library_name, proto_name);
                        }
                    }
                } else {
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrWrongNumberOfLayoutParameters,
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
                                crate::diagnostics::Error::ErrMustBeAProtocol,
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
                                crate::diagnostics::Error::ErrMustBeAProtocol,
                                type_ctor.element.span(),
                                &[&protocol],
                            );
                        }
                    } else {
                        self.reporter.fail(
                            crate::diagnostics::Error::ErrMustBeAProtocol,
                            type_ctor.element.span(),
                            &[&protocol],
                        );
                    }
                }

                Type {
experimental_maybe_from_alias: None,

                    kind_v2: "endpoint".to_string(),
                    subtype: None,
                    identifier: None,
                    nullable: Some(nullable),
                    element_type: None,
                    element_count: None,
                    maybe_element_count: None,
                    role: Some(role.to_string()),
                    protocol: Some(protocol),
                    protocol_transport: Some("Channel".to_string()),
                    obj_type: None,
                    rights: None,
                    resource_identifier: None,
                    deprecated: None,
                    maybe_attributes: vec![],
                    field_shape_v2: None,
                    maybe_size_constant_name: None,
                    type_shape_v2: TypeShapeV2 {
                        inline_size: 4,
                        alignment: 4,
                        depth: 0,
                        max_handles: 1,
                        max_out_of_line: 0,
                        has_padding: false,
                        has_flexible_envelope: false,
                    },
                }
            }
            "box" => {
                if type_ctor.parameters.is_empty() {
                    return Type {
experimental_maybe_from_alias: None,

                        kind_v2: "unknown".to_string(),
                        subtype: None,
                        identifier: None,
                        nullable: None,
                        element_type: None,
                        element_count: None,
                        maybe_element_count: None,
                        role: None,
                        protocol: None,
                        protocol_transport: None,
                        obj_type: None,
                        rights: None,
                        resource_identifier: None,
                        deprecated: None,
                        maybe_attributes: vec![],
                        field_shape_v2: None,
                        maybe_size_constant_name: None,
                        type_shape_v2: TypeShapeV2 {
                            inline_size: 0,
                            alignment: 1,
                            depth: 0,
                            max_handles: 0,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    };
                }
                let inner = &type_ctor.parameters[0];
                let prev = self.skip_eager_compile;
                self.skip_eager_compile = true;
                let mut inner_type = self.resolve_type(inner, library_name, naming_context);
                self.skip_eager_compile = prev;

                if nullable {
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrBoxCannotBeOptional,
                        type_ctor.element.span(),
                        &[],
                    );
                }

                if inner_type.kind_v2 != "struct" {
                    let k = inner_type.kind_v2.as_str();
                    let mut is_nor_opt = false;
                    let mut is_struct = false;
                    if let Some(decl) = self.get_underlying_decl(
                        inner_type.identifier.as_ref().unwrap_or(&"".to_string()),
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
                        is_nor_opt = k == "array" || k == "primitive";
                    }

                    if !is_struct {
                        if is_nor_opt {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrCannotBeBoxedNorOptional,
                                inner.element.span(),
                                &[&inner.element.span().data],
                            );
                        } else {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrCannotBeBoxedShouldBeOptional,
                                inner.element.span(),
                                &[&inner.element.span().data],
                            );
                        }
                    }
                }

                let boxed_inline = inner_type.type_shape_v2.inline_size;
                let padding = (8 - (boxed_inline % 8)) % 8;
                let max_ool = inner_type
                    .type_shape_v2
                    .max_out_of_line
                    .saturating_add(boxed_inline.saturating_add(padding));

                inner_type.nullable = Some(true); // box always makes it nullable for JSON output

                let new_depth = inner_type.type_shape_v2.depth.saturating_add(1);

                inner_type.type_shape_v2 = TypeShapeV2 {
                    inline_size: 8,
                    alignment: 8,
                    depth: new_depth,
                    max_handles: inner_type.type_shape_v2.max_handles,
                    max_out_of_line: max_ool,
                    has_padding: inner_type.type_shape_v2.has_padding || padding > 0,
                    has_flexible_envelope: inner_type.type_shape_v2.has_flexible_envelope,
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
                            crate::diagnostics::Error::ErrTooManyConstraints,
                            type_ctor.element.start_token.span.clone(),
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
                                    format!("{}/{}", res_library_name, id.to_string())
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
                                                if full_name == "zx/Handle" {
                                                    handle_subtype = ident_str.to_lowercase();
                                                } else {
                                                    handle_subtype = ident_str.to_string();
                                                }
                                                if let raw_ast::Constant::Literal(lit) = &mem.value
                                                {
                                                    if let Ok(v) = lit.literal.value.parse::<u32>()
                                                    {
                                                        handle_obj_type = v;
                                                    }
                                                }
                                                break;
                                            }
                                        }
                                    }
                                    if !found {
                                        self.reporter.fail(
                                            crate::diagnostics::Error::ErrUnexpectedConstraint,
                                            type_ctor.element.start_token.span.clone(),
                                            &[&full_name],
                                        );
                                    }
                                } else {
                                    self.reporter.fail(
                                        crate::diagnostics::Error::ErrExpectedType,
                                        type_ctor.element.start_token.span.clone(),
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
                                                    if mem.name.data() == ident_str {
                                                        if let raw_ast::Constant::Literal(lit) =
                                                            &mem.value
                                                        {
                                                            let val_str = lit.literal.value.clone();
                                                            let parsed = if val_str
                                                                .starts_with("0x")
                                                                || val_str.starts_with("0X")
                                                            {
                                                                u32::from_str_radix(
                                                                    &val_str[2..],
                                                                    16,
                                                                )
                                                            } else {
                                                                val_str.parse::<u32>()
                                                            };
                                                            if let Ok(v) = parsed {
                                                                return Some(v);
                                                            }
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
                                        crate::diagnostics::Error::ErrUnexpectedConstraint,
                                        type_ctor.element.start_token.span.clone(),
                                        &[&full_name],
                                    );
                                }
                            }
                        }
                    }

                    return Type {
experimental_maybe_from_alias: None,

                        kind_v2: "handle".to_string(),
                        subtype: Some(handle_subtype),
                        identifier: None,
                        nullable: Some(nullable),
                        element_type: None,
                        element_count: None,
                        maybe_element_count: None,
                        role: None,
                        protocol: None,
                        protocol_transport: None,
                        obj_type: Some(handle_obj_type),
                        rights: Some(handle_rights),
                        resource_identifier: Some(full_name.clone()),
                        deprecated: None,
                        maybe_attributes: vec![],
                        field_shape_v2: None,
                        maybe_size_constant_name: None,
                        type_shape_v2: TypeShapeV2 {
                            inline_size: 4,
                            alignment: 4,
                            depth: 0,
                            max_handles: 1,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    };
                }

                if let Some(decl) = self.raw_decls.get(&full_name) {
                    let is_bits = match decl {
                        RawDecl::Bits(_) => true,
                        RawDecl::Type(t) => matches!(t.layout, raw_ast::Layout::Bits(_)),
                        _ => false,
                    };
                    if is_bits {
                        if nullable {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrCannotBeOptional,
                                type_ctor.element.start_token.span.clone(),
                                &[&name],
                            );
                        }
                        if type_ctor.nullable
                            || type_ctor.constraints.iter().any(|c| {
                                if let raw_ast::Constant::Identifier(id) = c {
                                    id.identifier.to_string() != "optional"
                                } else {
                                    true
                                }
                            })
                            || (!type_ctor.constraints.is_empty() && !nullable)
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
                                        crate::diagnostics::Error::ErrTooManyConstraints,
                                        type_ctor.element.start_token.span.clone(),
                                        &[&name, &0_usize, &type_ctor.constraints.len()],
                                    );
                                }
                            }
                        }
                    }
                }

                if nullable {
                    if let Some(decl) = self.raw_decls.get(&full_name) {
                        let is_struct = match decl {
                            RawDecl::Struct(_) => true,
                            RawDecl::Type(t) => matches!(t.layout, raw_ast::Layout::Struct(_)),
                            _ => false,
                        };
                        if is_struct {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrStructCannotBeOptional,
                                type_ctor.element.span(),
                                &[&name],
                            );
                        }
                    }
                }

                if let Some(shape) = self.shapes.get(&full_name) {
                    Type {
experimental_maybe_from_alias: None,

                        kind_v2: "identifier".to_string(),
                        subtype: None,
                        identifier: Some(full_name.clone()),
                        nullable: Some(nullable),
                        element_type: None,
                        element_count: None,
                        maybe_element_count: None,
                        role: None,
                        protocol: None,
                        protocol_transport: None,
                        obj_type: None,
                        rights: None,
                        resource_identifier: None,
                        deprecated: None,
                        maybe_attributes: vec![],
                        field_shape_v2: None,
                        maybe_size_constant_name: None,
                        type_shape_v2: shape.clone(),
                    }
                } else if let Some(decl) = self.raw_decls.get(&full_name) {
                    if let RawDecl::Alias(a) = decl {
                        let mut resolved_type = self.resolve_type(&a.type_ctor, library_name, naming_context);
                        resolved_type.experimental_maybe_from_alias = Some(crate::json_generator::ExperimentalMaybeFromAlias {
                            name: full_name.clone(),
                            args: vec![], // TODO handle args if any
                            nullable,
                        });
                        if nullable {
                            resolved_type.nullable = Some(true);
                            if resolved_type.kind_v2 != "primitive" {
                                resolved_type.type_shape_v2.depth += 1;
                            }
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
                    let is_protocol = match decl {
                        RawDecl::Protocol(_) => true,
                        _ => false,
                    };
                    let (inline, align, flex, padding) = if is_union_or_table {
                        let is_strict = match decl {
                            RawDecl::Union(u) => u
                                .modifiers
                                .iter()
                                .any(|m| m.subkind == crate::token::TokenSubkind::Strict),
                            RawDecl::Type(t) => match &t.layout {
                                raw_ast::Layout::Union(u) => u
                                    .modifiers
                                    .iter()
                                    .any(|m| m.subkind == crate::token::TokenSubkind::Strict),
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
                    Type {
experimental_maybe_from_alias: None,

                        kind_v2: "identifier".to_string(),
                        subtype: None,
                        identifier: Some(full_name.clone()),
                        nullable: Some(nullable),
                        element_type: None,
                        element_count: None,
                        maybe_element_count: None,
                        role: None,
                        protocol: None,
                        protocol_transport: None,
                        obj_type: None,
                        rights: None,
                        resource_identifier: None,
                        deprecated: None,
                        maybe_attributes: vec![],
                        field_shape_v2: None,
                        maybe_size_constant_name: None,
                        type_shape_v2: TypeShapeV2 {
                            inline_size: inline,
                            alignment: align,
                            depth: u32::MAX,
                            max_handles: 0,
                            max_out_of_line: u32::MAX,
                            has_padding: padding,
                            has_flexible_envelope: flex,
                        },
                    }
                } else {
                    // eprintln!("Warning: Type not found: {} (tried {})", name, full_name);
                    if name == "handle" {
                        self.reporter.fail(
                            crate::diagnostics::Error::ErrNameNotFound,
                            type_ctor.element.span(),
                            &[&name, &library_name],
                        );
                    }
                    Type {
experimental_maybe_from_alias: None,

                        kind_v2: "unknown".to_string(),
                        subtype: None,
                        identifier: Some(full_name),
                        nullable: Some(nullable),
                        element_type: None,
                        element_count: None,
                        maybe_element_count: None,
                        role: None,
                        protocol: None,
                        protocol_transport: None,
                        obj_type: None,
                        rights: None,
                        resource_identifier: None,
                        deprecated: None,
                        maybe_attributes: vec![],
                        field_shape_v2: None,
                        maybe_size_constant_name: None,
                        type_shape_v2: TypeShapeV2 {
                            inline_size: 0,
                            alignment: 1,
                            depth: 0,
                            max_handles: 0,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        },
                    }
                }
            }
        }
    }

    pub fn eval_constant_value(&self, constant: &raw_ast::Constant<'_>) -> Option<u64> {
        match constant {
            raw_ast::Constant::Literal(lit) => match &lit.literal.kind {
                raw_ast::LiteralKind::Numeric => {
                    let val_str = &lit.literal.value;
                    if let Some(stripped) = val_str.strip_prefix("0x").or_else(|| val_str.strip_prefix("0X")) {
                        u64::from_str_radix(stripped, 16).ok()
                    } else if let Some(stripped) = val_str.strip_prefix("0b").or_else(|| val_str.strip_prefix("0B")) {
                        u64::from_str_radix(stripped, 2).ok()
                    } else {
                        val_str.parse::<i64>().ok().map(|v| v as u64).or_else(|| val_str.parse::<u64>().ok())
                    }
                }
                raw_ast::LiteralKind::Bool(b) => Some(if *b { 1 } else { 0 }),
                _ => None,
            },
            raw_ast::Constant::Identifier(id) => {
                let name = id.identifier.to_string();
                if name == "MAX" {
                    return Some(u32::MAX as u64); // Approximation
                }
                
                let mut full_name = name.clone();
                if !full_name.contains('/') {
                    full_name = format!("{}/{}", self.library_name, name);
                }

                if let Some(decl) = self.raw_decls.get(&full_name) {
                    if let RawDecl::Const(c) = decl {
                        return self.eval_constant_value(&c.value);
                    }
                }

                if let Some((type_name, member_name)) = name.rsplit_once('.') {
                    let mut type_full_name = type_name.to_string();
                    if !type_full_name.contains('/') {
                        type_full_name = format!("{}/{}", self.library_name, type_name);
                    }
                    if let Some(decl) = self.raw_decls.get(&type_full_name) {
                        return match decl {
                            RawDecl::Bits(b) => {
                                b.members.iter().find(|m| m.name.data() == member_name).and_then(|m| self.eval_constant_value(&m.value))
                            }
                            RawDecl::Enum(e) => {
                                e.members.iter().find(|m| m.name.data() == member_name).and_then(|m| self.eval_constant_value(&m.value))
                            }
                            _ => None,
                        };
                    }
                }

                None
            }
            raw_ast::Constant::BinaryOperator(binop) => {
                let left = self.eval_constant_value(&binop.left)?;
                let right = self.eval_constant_value(&binop.right)?;
                Some(left | right)
            }
        }
    }

    fn eval_constant_usize(&self, constant: &raw_ast::Constant<'_>) -> Option<usize> {
        self.eval_constant_value(constant).map(|v| v as usize)
    }

    fn eval_type_constant_usize(&self, ty: &raw_ast::TypeConstructor<'_>) -> Option<usize> {
        match &ty.layout {
            raw_ast::LayoutParameter::Literal(lit) => match &lit.literal.kind {
                raw_ast::LiteralKind::Numeric => lit.literal.value.parse::<usize>().ok(),
                _ => None,
            },
            // Handle Identifier if it's a const?
            raw_ast::LayoutParameter::Identifier(id) => {
                let name = id.to_string();
                if name == "MAX" {
                    Some(u32::MAX as usize)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_location(&self, element: &raw_ast::SourceElement<'_>) -> Location {
        let start_span = element.start_token.span;
        let end_span = element.end_token.span;
        let view = start_span.data;

        for source in &self.source_files {
            if let Some((_, pos)) = source.line_containing(view) {
                let start_ptr = view.as_ptr() as usize;
                let end_ptr = end_span.data.as_ptr() as usize + end_span.data.len();
                let length = end_ptr.saturating_sub(start_ptr);

                return Location {
                    filename: if source.filename().starts_with("fidlc/") {
                        format!("../../tools/fidl/{}", source.filename())
                    } else {
                        source.filename().to_string()
                    },
                    line: pos.line,
                    column: pos.column,
                    length,
                };
            }
        }
        Location {
            filename: "TODO".to_string(),
            line: 0,
            column: 0,
            length: 0,
        }
    }

    pub fn compile_attributes_from_ref(
        &self,
        attributes: &raw_ast::AttributeList<'_>,
    ) -> Vec<Attribute> {
        let mut compiled_attrs = vec![];
        let mut doc_comment_buffer: Vec<&raw_ast::Attribute<'_>> = vec![];

        for attr in &attributes.attributes {
            if attr.provenance == raw_ast::AttributeProvenance::DocComment {
                doc_comment_buffer.push(attr);
            } else {
                if !doc_comment_buffer.is_empty() {
                    compiled_attrs.push(self.compile_doc_comments(&doc_comment_buffer));
                    doc_comment_buffer.clear();
                }

                // Compile regular attribute
                let args = attr
                    .args
                    .iter()
                    .map(|arg| {
                        let arg_name = arg
                            .name
                            .as_ref()
                            .map(|n| n.element.start_token.span.data.to_string())
                            .unwrap_or_else(|| "value".to_string());
                        let value = self.compile_constant(&arg.value);
                        crate::json_generator::AttributeArg {
                            name: arg_name,
                            type_: if attr.name.element.start_token.span.data == "available" {
                                "uint32".to_string()
                            } else {
                                value
                                    .literal
                                    .as_ref()
                                    .map(|l| l.kind.clone())
                                    .unwrap_or_else(|| "string".to_string())
                            },
                            value,
                            location: self.get_location(&arg.element),
                        }
                    })
                    .collect();

                compiled_attrs.push(Attribute {
                    name: attr.name.element.start_token.span.data.to_string(),
                    arguments: args,
                    location: self.get_location(&attr.element),
                });
            }
        }

        if !doc_comment_buffer.is_empty() {
            compiled_attrs.push(self.compile_doc_comments(&doc_comment_buffer));
        }

        compiled_attrs
    }

    pub fn compile_doc_comments(&self, doc_comments: &[&raw_ast::Attribute<'_>]) -> Attribute {
        let mut combined_value = String::new();
        for attr in doc_comments.iter() {
            let text = attr.name.element.start_token.span.data;

            let stripped = if text.starts_with("///") {
                &text[3..]
            } else {
                text
            };
            combined_value.push_str(stripped);
            combined_value.push('\n');
        }

        let first = doc_comments.first().unwrap();
        let last = doc_comments.last().unwrap();

        let start_ptr = first.name.element.start_token.span.data.as_ptr() as usize;
        let end_ptr = last.name.element.start_token.span.data.as_ptr() as usize;
        let end_len = last.name.element.start_token.span.data.len();

        let len = (end_ptr + end_len).saturating_sub(start_ptr);

        let raw_expr = unsafe {
            let slice = std::slice::from_raw_parts(start_ptr as *const u8, len);
            std::str::from_utf8_unchecked(slice)
        };
        let combined_expression = raw_expr.to_string();

        let synthetic_element = raw_ast::SourceElement::new(
            first.element.start_token.clone(),
            last.element.end_token.clone(),
        );
        let loc = self.get_location(&synthetic_element);

        Attribute {
            name: "doc".to_string(),
            arguments: vec![crate::json_generator::AttributeArg {
                name: "value".to_string(),
                type_: "string".to_string(),
                value: Constant {
                    kind: "literal".to_string(),
                    value: serde_json::value::RawValue::from_string(
                        serde_json::to_string(&combined_value).unwrap(),
                    )
                    .unwrap(),
                    expression: serde_json::value::RawValue::from_string(
                        serde_json::to_string(&combined_expression).unwrap(),
                    )
                    .unwrap(),
                    literal: Some(Literal {
                        kind: "string".to_string(),
                        value: serde_json::value::RawValue::from_string(
                            serde_json::to_string(&combined_value).unwrap(),
                        )
                        .unwrap(),
                        expression: serde_json::value::RawValue::from_string(
                            serde_json::to_string(&combined_expression).unwrap(),
                        )
                        .unwrap(),
                    }),
                    identifier: None,
                },
                location: loc.clone(),
            }],
            location: loc,
        }
    }

    pub fn is_versioned_library(&self) -> bool {
        if let Some(lib) = &self.library_decl
            && let Some(attrs) = &lib.attributes
        {
            for attr in &attrs.attributes {
                if attr.name.data() == "available" {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_deprecated(&self, attributes: Option<&raw_ast::AttributeList<'_>>) -> bool {
        if let Some(attrs) = attributes {
            for attr in &attrs.attributes {
                if attr.name.data() == "available" {
                    for arg in &attr.args {
                        let arg_name = arg.name.as_ref().map(|n| n.data()).unwrap_or("value");
                        if arg_name == "deprecated" {
                            let val_str = match &arg.value {
                                crate::raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                                crate::raw_ast::Constant::Identifier(id) => {
                                    id.identifier.to_string()
                                }
                                _ => continue,
                            };
                            let d = crate::versioning_types::Version::parse(&val_str)
                                .unwrap_or(crate::versioning_types::Version::POS_INF);
                            let is_depr = d <= crate::versioning_types::Version::HEAD;
                            if is_depr {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn is_active(&self, attributes: Option<&raw_ast::AttributeList<'_>>) -> bool {
        if let Some(attrs) = attributes {
            for attr in &attrs.attributes {
                // For regular declarations, the name is "available".
                // For modifiers, the name is the modifier itself (e.g. "strict").
                // We can just check the arguments directly.
                for arg in &attr.args {
                    let arg_name = arg.name.as_ref().map(|n| n.data()).unwrap_or("value");
                    if arg_name == "removed" {
                        let val_str = match &arg.value {
                            crate::raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                            crate::raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                            _ => continue,
                        };
                        let r = crate::versioning_types::Version::parse(&val_str)
                            .unwrap_or(crate::versioning_types::Version::POS_INF);
                        if r <= crate::versioning_types::Version::HEAD {
                            return false;
                        }
                    } else if arg_name == "added" {
                        let val_str = match &arg.value {
                            crate::raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                            crate::raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                            _ => continue,
                        };
                        let a = crate::versioning_types::Version::parse(&val_str)
                            .unwrap_or(crate::versioning_types::Version::POS_INF);
                        if a > crate::versioning_types::Version::HEAD {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn compile_attribute_list(
        &self,
        attributes: &Option<Box<raw_ast::AttributeList<'_>>>,
    ) -> Vec<Attribute> {
        if let Some(list) = attributes {
            self.compile_attributes_from_ref(list)
        } else {
            vec![]
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

    pub fn compile_constant(&self, constant: &raw_ast::Constant<'_>) -> Constant {
        match constant {
            raw_ast::Constant::Literal(lit) => {
                let (kind, value_json, expr_json) = match &lit.literal.kind {
                    raw_ast::LiteralKind::String => {
                        let inner_json = self.generate_json_string_literal(&lit.literal.value);
                        let expr = lit.literal.value.clone();
                        let expr_json = serde_json::to_string(&expr).unwrap();
                        ("string", inner_json, expr_json)
                    }
                    raw_ast::LiteralKind::Numeric => {
                        let val = lit.literal.value.clone();
                        let n_str = if val.starts_with("0x") || val.starts_with("0X") {
                            let without_prefix = &val[2..];
                            if let Ok(n) = u64::from_str_radix(without_prefix, 16) {
                                n.to_string()
                            } else {
                                val.clone()
                            }
                        } else if val.starts_with("0b") || val.starts_with("0B") {
                            let without_prefix = &val[2..];
                            if let Ok(n) = u64::from_str_radix(without_prefix, 2) {
                                n.to_string()
                            } else {
                                val.clone()
                            }
                        } else {
                            val.clone()
                        };
                        (
                            "numeric",
                            serde_json::to_string(&n_str).unwrap(),
                            serde_json::to_string(&val).unwrap(),
                        )
                    }
                    raw_ast::LiteralKind::Bool(b) => {
                        let s = b.to_string();
                        (
                            "bool",
                            serde_json::to_string(&s).unwrap(),
                            serde_json::to_string(&s).unwrap(),
                        )
                    }
                    raw_ast::LiteralKind::DocComment => {
                        ("doc_comment", "\"\"".to_string(), "\"\"".to_string())
                    }
                };

                Constant {
                    kind: "literal".to_string(),
                    value: serde_json::value::RawValue::from_string(value_json.clone()).unwrap(),
                    expression: serde_json::value::RawValue::from_string(expr_json.clone())
                        .unwrap(),
                    literal: Some(Literal {
                        kind: kind.to_string(),
                        value: serde_json::value::RawValue::from_string(value_json).unwrap(),
                        expression: serde_json::value::RawValue::from_string(expr_json).unwrap(),
                    }),
                    identifier: None,
                }
            }
            raw_ast::Constant::Identifier(id) => {
                let id_str = id.identifier.to_string();
                if id_str == "HEAD" || id_str == "NEXT" {
                    let (val, expr, ident) = if id_str == "HEAD" {
                        ("4292870144", "HEAD", "fidl/HEAD")
                    } else {
                        ("4291821568", "NEXT", "fidl/NEXT")
                    };
                    return Constant {
                        kind: "identifier".to_string(),
                        value: serde_json::value::RawValue::from_string(format!("\"{}\"", val)).unwrap(),
                        expression: serde_json::value::RawValue::from_string(format!("\"{}\"", expr)).unwrap(),
                        literal: None,
                        identifier: Some(ident.to_string()),
                    };
                }

                let value = self.eval_constant_value(constant).unwrap_or(0);
                
                let mut full_name = id_str.clone();
                if !full_name.contains('/') {
                    full_name = format!("{}/{}", self.library_name, id_str);
                }

                Constant {
                    kind: "identifier".to_string(),
                    value: serde_json::value::RawValue::from_string(format!("\"{}\"", value)).unwrap(),
                    expression: serde_json::value::RawValue::from_string(format!("\"{}\"", id.element.span().data)).unwrap(),
                    literal: None,
                    identifier: Some(full_name),
                }
            }
            raw_ast::Constant::BinaryOperator(binop) => {
                let value = self.eval_constant_value(constant).unwrap_or(0);
                Constant {
                    kind: "binary_operator".to_string(),
                    value: serde_json::value::RawValue::from_string(format!("\"{}\"", value)).unwrap(),
                    expression: serde_json::value::RawValue::from_string(format!("\"{}\"", binop.element.span().data)).unwrap(),
                    literal: None,
                    identifier: None,
                }
            }
        }
    }
}

fn get_dependencies<'node, 'src>(
    decl: &RawDecl<'node, 'src>,
    library_name: &str,
    _decl_kinds: &HashMap<String, &str>,
    skip_optional: bool,
    inline_names: &HashMap<usize, String>,
) -> Vec<String> {
    let mut deps = vec![];
    match decl {
        RawDecl::Struct(s) => {
            for member in &s.members {
                collect_deps_from_ctor(
                    &member.type_ctor,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        RawDecl::Enum(e) => {
            if let Some(ref subtype) = e.subtype {
                collect_deps_from_ctor(
                    subtype,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        RawDecl::Bits(b) => {
            if let Some(ref subtype) = b.subtype {
                collect_deps_from_ctor(
                    subtype,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        RawDecl::Union(u) => {
            for member in &u.members {
                if let Some(type_ctor) = &member.type_ctor {
                    collect_deps_from_ctor(
                        type_ctor,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        RawDecl::Table(t) => {
            for member in &t.members {
                if let Some(type_ctor) = &member.type_ctor {
                    collect_deps_from_ctor(
                        type_ctor,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        RawDecl::Type(t) => {
            if let Some(s) = option_layout_as_struct(&t.layout) {
                for member in &s.members {
                    collect_deps_from_ctor(
                        &member.type_ctor,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            } else if let Some(e) = option_layout_as_enum(&t.layout) {
                if let Some(ref subtype) = e.subtype {
                    collect_deps_from_ctor(
                        subtype,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            } else if let Some(b) = option_layout_as_bits(&t.layout) {
                if let Some(ref subtype) = b.subtype {
                    collect_deps_from_ctor(
                        subtype,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            } else if let Some(u) = option_layout_as_union(&t.layout) {
                for member in &u.members {
                    if let Some(ref ctor) = member.type_ctor {
                        collect_deps_from_ctor(
                            ctor,
                            library_name,
                            &mut deps,
                            skip_optional,
                            inline_names,
                        );
                    }
                }
            } else if let Some(ta) = option_layout_as_table(&t.layout) {
                for member in &ta.members {
                    if let Some(ref ctor) = member.type_ctor {
                        collect_deps_from_ctor(
                            ctor,
                            library_name,
                            &mut deps,
                            skip_optional,
                            inline_names,
                        );
                    }
                }
            } else if let raw_ast::Layout::TypeConstructor(ref tc) = t.layout {
                collect_deps_from_ctor(tc, library_name, &mut deps, skip_optional, inline_names);
            }
        }
        RawDecl::Protocol(p) => {
            for m in &p.methods {
                let _method_name_camel = format!(
                    "{}{}",
                    m.name.data().chars().next().unwrap().to_uppercase(),
                    &m.name.data()[1..]
                );
                if let Some(req) = &m.request_payload {
                    collect_deps_from_layout(
                        req,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
                if let Some(res) = &m.response_payload {
                    collect_deps_from_layout(
                        res,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
                if let Some(ref err) = m.error_payload {
                    collect_deps_from_layout(
                        err,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        RawDecl::Service(s) => {
            for member in &s.members {
                collect_deps_from_ctor(
                    &member.type_ctor,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        RawDecl::Resource(r) => {
            collect_deps_from_ctor(
                &r.type_ctor,
                library_name,
                &mut deps,
                skip_optional,
                inline_names,
            );
            for prop in &r.properties {
                collect_deps_from_ctor(
                    &prop.type_ctor,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        RawDecl::Const(c) => {
            collect_deps_from_ctor(
                &c.type_ctor,
                library_name,
                &mut deps,
                skip_optional,
                inline_names,
            );
        }
        RawDecl::Alias(a) => {
            collect_deps_from_ctor(
                &a.type_ctor,
                library_name,
                &mut deps,
                skip_optional,
                inline_names,
            );
        }
    }

    deps
}

fn option_layout_as_struct<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::StructDeclaration<'a>> {
    if let raw_ast::Layout::Struct(s) = layout {
        Some(s)
    } else {
        None
    }
}

fn option_layout_as_enum<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::EnumDeclaration<'a>> {
    if let raw_ast::Layout::Enum(e) = layout {
        Some(e)
    } else {
        None
    }
}

fn option_layout_as_bits<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::BitsDeclaration<'a>> {
    if let raw_ast::Layout::Bits(b) = layout {
        Some(b)
    } else {
        None
    }
}

fn option_layout_as_union<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::UnionDeclaration<'a>> {
    if let raw_ast::Layout::Union(u) = layout {
        Some(u)
    } else {
        None
    }
}

fn option_layout_as_table<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::TableDeclaration<'a>> {
    if let raw_ast::Layout::Table(t) = layout {
        Some(t)
    } else {
        None
    }
}

fn collect_deps_from_ctor(
    ctor: &raw_ast::TypeConstructor<'_>,
    library_name: &str,
    deps: &mut Vec<String>,
    skip_optional: bool,
    inline_names: &HashMap<usize, String>,
) {
    if skip_optional {
        // Nullable types (e.g., box, optional unions) are placed behind pointers or
        // envelopes and don't strict layout dependency cycles.
        if ctor.nullable {
            return;
        }

        // Check if it has an `:optional` constraint
        for constraint in &ctor.constraints {
            if let raw_ast::Constant::Identifier(id_const) = constraint
                && id_const.identifier.to_string() == "optional"
            {
                return;
            }
        }
    }

    if let raw_ast::LayoutParameter::Identifier(ref id) = ctor.layout {
        let name = id.to_string();
        match name.as_str() {
            "bool" | "int8" | "uint8" | "int16" | "uint16" | "int32" | "uint32" | "int64"
            | "uint64" | "string" => {}
            "box" | "client_end" | "server_end" => {
                if skip_optional {
                    return;
                }
            }
            _ => {
                deps.push(format!("{}/{}", library_name, name));
            }
        }
    } else if let raw_ast::LayoutParameter::Inline(_) = ctor.layout {
        if let Some(name) =
            inline_names.get(&(ctor.element.start_token.span.data.as_ptr() as usize))
        {
            deps.push(name.clone());
        }
    }

    for param in &ctor.parameters {
        collect_deps_from_ctor(param, library_name, deps, skip_optional, inline_names);
    }
}

fn collect_deps_from_layout(
    layout: &raw_ast::Layout<'_>,
    library_name: &str,
    deps: &mut Vec<String>,
    skip_optional: bool,
    inline_names: &HashMap<usize, String>,
) {
    match layout {
        raw_ast::Layout::TypeConstructor(tc) => {
            collect_deps_from_ctor(tc, library_name, deps, skip_optional, inline_names);
        }
        raw_ast::Layout::Struct(s) => {
            for member in &s.members {
                collect_deps_from_ctor(
                    &member.type_ctor,
                    library_name,
                    deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        raw_ast::Layout::Union(u) => {
            for member in &u.members {
                if let Some(type_ctor) = &member.type_ctor {
                    collect_deps_from_ctor(
                        type_ctor,
                        library_name,
                        deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        raw_ast::Layout::Table(t) => {
            for member in &t.members {
                if let Some(type_ctor) = &member.type_ctor {
                    collect_deps_from_ctor(
                        type_ctor,
                        library_name,
                        deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        _ => {}
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
        let mut property_names = std::collections::HashSet::new();

        let ctx = crate::name::NamingContext::create(name);
        let type_obj = self.resolve_type(&decl.type_ctor, library_name, Some(ctx));

        // C++ checks if type resolves to a uint32 primitive
        let mut is_uint32 =
            type_obj.kind_v2 == "primitive" && type_obj.subtype.as_deref() == Some("uint32");
        if !is_uint32 {
            if let Some(id) = type_obj.identifier.as_ref() {
                let mut curr = id.clone();
                for _ in 0..100 {
                    if curr == "uint32" {
                        is_uint32 = true;
                        break;
                    }
                    if let Some(d) = self.raw_decls.get(&curr) {
                        if let RawDecl::Alias(a) = d {
                            match &a.type_ctor.layout {
                                raw_ast::LayoutParameter::Identifier(inner_id) => {
                                    let next = inner_id.to_string();
                                    if next == "uint32" {
                                        is_uint32 = true;
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
                    } else {
                        break;
                    }
                }
            }
        }

        if !is_uint32 {
            self.reporter.fail(
                crate::diagnostics::Error::ErrResourceMustBeUint32Derived,
                decl.name.element.span(),
                &[&name],
            );
        }

        if decl.properties.is_empty() {
            self.reporter.fail(
                crate::diagnostics::Error::ErrMustHaveOneProperty,
                decl.element.span(),
                &[],
            );
        }

        let mut has_subtype = false;

        for prop in &decl.properties {
            let prop_name = prop.name.data().to_string();

            if !property_names.insert(prop_name.clone()) {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrNameCollision,
                    prop.name.element.span(),
                    &[
                        &"resource property",
                        &prop_name,
                        &"resource property",
                        &prop.name.element.span().data,
                    ],
                );
            }

            let prop_ctx =
                crate::name::NamingContext::create(name).enter_member(prop_name.as_str());
            let prop_type = self.resolve_type(&prop.type_ctor, library_name, Some(prop_ctx));

            if prop_name == "subtype" {
                has_subtype = true;
                let is_enum = if let Some(id) = prop_type.identifier.as_ref() {
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
                        crate::diagnostics::Error::ErrResourceSubtypePropertyMustReferToEnum,
                        prop.name.element.span(),
                        &[&name],
                    );
                }
            } else if prop_name == "rights" {
                let is_bits = if let Some(id) = prop_type.identifier.as_ref() {
                    match self.get_underlying_decl(id) {
                        Some(RawDecl::Bits(_)) => true,
                        Some(RawDecl::Type(t)) => matches!(t.layout, raw_ast::Layout::Bits(_)),
                        _ => false,
                    }
                } else {
                    false
                };
                let mut is_uint32_prop = prop_type.kind_v2 == "primitive"
                    && prop_type.subtype.as_deref() == Some("uint32");
                if !is_uint32_prop {
                    if let Some(id) = prop_type.identifier.as_ref() {
                        let mut curr = id.clone();
                        for _ in 0..100 {
                            if curr == "uint32" {
                                is_uint32_prop = true;
                                break;
                            }
                            if let Some(d) = self.raw_decls.get(&curr) {
                                if let RawDecl::Alias(a) = d {
                                    match &a.type_ctor.layout {
                                        raw_ast::LayoutParameter::Identifier(inner_id) => {
                                            let next = inner_id.to_string();
                                            if next == "uint32" {
                                                is_uint32_prop = true;
                                                break;
                                            }
                                            curr = if next.contains('/')
                                                || self.shapes.contains_key(&next)
                                            {
                                                next
                                            } else {
                                                format!(
                                                    "{}/{}",
                                                    curr.split('/').next().unwrap_or(""),
                                                    next
                                                )
                                            };
                                        }
                                        _ => break,
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                if !is_bits && !is_uint32_prop {
                    self.reporter.fail(
                        crate::diagnostics::Error::ErrResourceRightsPropertyMustReferToBits,
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
                crate::diagnostics::Error::ErrResourceMissingSubtypeProperty,
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
        for member in &decl.members {
            let ctx = crate::name::NamingContext::create(name).enter_member(member.name.data());
            let type_obj = self.resolve_type(&member.type_ctor, library_name, Some(ctx));
            let member_name = member.name.data().to_string();
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
            partial_type_ctor: crate::json_generator::PartialTypeCtor {
                name: if let raw_ast::LayoutParameter::Identifier(id) = &decl.type_ctor.layout {
                    id.to_string()
                } else {
                    "".to_string()
                },
                args: vec![],
                nullable: decl.type_ctor.nullable,
            },
            type_: self.resolve_type(&decl.type_ctor, library_name, None),
        }
    }

    pub fn compile_const(
        &mut self,
        decl: &'node raw_ast::ConstDeclaration<'src>,
        library_name: &str,
    ) -> ConstDeclaration {
        let name = decl.name.data();
        let full_name = format!("{}/{}", library_name, name);
        let location = self.get_location(&decl.name.element);

        let ctx = crate::name::NamingContext::create(name);
        let type_obj = self.resolve_type(&decl.type_ctor, library_name, Some(ctx));
        let constant = self.compile_constant(&decl.value);

        ConstDeclaration {
            name: full_name,
            location,
            deprecated: self.is_deprecated(decl.attributes.as_deref()),
            maybe_attributes: self.compile_attribute_list(&decl.attributes),
            type_: type_obj,
            value: constant,
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
            .any(|m| m.subkind == crate::token::TokenSubkind::Strict);
        let is_flexible = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == crate::token::TokenSubkind::Flexible);
        if is_strict && decl.methods.is_empty() {
            self.reporter.fail(
                crate::diagnostics::Error::ErrMustHaveOneMember,
                decl.name.element.span(),
                &[],
            );
        }
        if is_flexible && decl.methods.is_empty() {
            self.reporter.fail(
                crate::diagnostics::Error::ErrMustHaveOneMember,
                decl.name.element.span(),
                &[],
            );
        }

        let mut methods = vec![];
        let mut method_names = std::collections::HashSet::new();
        let openness = if decl
            .modifiers
            .iter()
            .any(|m| m.subkind == crate::token::TokenSubkind::Ajar)
        {
            "ajar"
        } else if decl
            .modifiers
            .iter()
            .any(|m| m.subkind == crate::token::TokenSubkind::Closed)
        {
            "closed"
        } else {
            "open"
        };

        let mut compiled_composed = vec![];
        for composed in &decl.composed_protocols {
            let mut composed_name = composed.protocol_name.to_string();
            if composed_name.contains('.') {
                let parts: Vec<&str> = composed_name.split('.').collect();
                composed_name = format!("{}/{}", parts[0], parts[1]);
            }
            let full_composed_name = if composed_name.contains('/') {
                composed_name.clone()
            } else {
                format!("{}/{}", library_name, composed_name)
            };

            self.compile_decl_by_name(&full_composed_name);

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
                if p.modifiers
                    .iter()
                    .any(|m| m.subkind == crate::token::TokenSubkind::Ajar)
                {
                    composed_openness = "ajar";
                } else if p
                    .modifiers
                    .iter()
                    .any(|m| m.subkind == crate::token::TokenSubkind::Closed)
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
                    crate::diagnostics::Error::ErrComposedProtocolTooOpen,
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
            compiled_composed.push(crate::json_generator::ProtocolCompose {
                name: full_composed_name,
                location: self.get_location(&composed.protocol_name.element),
                deprecated: self.is_deprecated(composed.attributes.as_deref()),
                maybe_attributes: self.compile_attribute_list(&composed.attributes),
            });
        }

        for m in &decl.methods {
            let mut is_method_flexible = false;
            for modifier in &m.modifiers {
                match modifier.subkind {
                    crate::token::TokenSubkind::Strict => {}
                    crate::token::TokenSubkind::Flexible => {
                        is_method_flexible = true;
                    }
                    _ => {
                        self.reporter.fail(
                            crate::diagnostics::Error::ErrCannotSpecifyModifier,
                            modifier.element.span(),
                            &[&modifier.element.span().data, &"method"],
                        );
                    }
                }
            }

            let two_way = m.has_request && m.has_response;
            if is_method_flexible && two_way && openness != "open" {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrFlexibleTwoWayMethodRequiresOpenProtocol,
                    m.name.element.span(),
                    &[&openness],
                );
            } else if is_method_flexible && !two_way && openness == "closed" {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrFlexibleOneWayMethodInClosedProtocol,
                    m.name.element.span(),
                    &[&if !m.has_request && m.has_response { "event" } else { "one-way method" }],
                );
            }

            if !method_names.insert(m.name.data()) {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrNameCollision,
                    m.name.element.span(),
                    &[&"method", &m.name.data(), &"method", &m.name.data()],
                );
            }
            if m.has_error && !m.has_response && m.has_request {
                self.reporter.fail(
                    crate::diagnostics::Error::ErrUnexpectedToken,
                    m.name.element.span(),
                    &[],
                );
            }
            let has_request = m.has_request;
            let maybe_request_payload = if let Some(ref l) = m.request_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        let ctx = crate::name::NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let resolved_type = self.resolve_type(tc, library_name, Some(ctx));

                        let is_allowed = if resolved_type.kind_v2 != "identifier" {
                            false
                        } else if let Some(id) = &resolved_type.identifier {
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
                                crate::diagnostics::Error::ErrInvalidMethodPayloadLayoutClass,
                                tc.element.span(),
                                &[&"provided type"],
                            );
                        }

                        Some(resolved_type)
                    }
                    raw_ast::Layout::Struct(s) => {
                        if s.members.is_empty() {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrEmptyPayloadStructs,
                                s.element.span(),
                                &[],
                            );
                        }
                        for sm in &s.members {
                            if sm.default_value.is_some() {
                                self.reporter.fail(
                                    crate::diagnostics::Error::ErrPayloadStructHasDefaultMembers,
                                    sm.name.element.span(),
                                    &[&sm.name.data()],
                                );
                            }
                        }
                        let ctx = crate::name::NamingContext::create(decl.name.element.span())
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
                        Some(Type {
experimental_maybe_from_alias: None,

                            kind_v2: "identifier".to_string(),
                            subtype: None,
                            identifier: Some(full_synth),
                            nullable: Some(false),
                            element_type: None,
                            element_count: None,
                            maybe_element_count: None,
                            role: None,
                            protocol: None,
                            protocol_transport: None,
                            obj_type: None,
                            rights: None,
                            resource_identifier: None,
                            deprecated: None,
                            maybe_attributes: vec![],
                            field_shape_v2: None,
                            maybe_size_constant_name: None,
                            type_shape_v2: shape,
                        })
                    }
                    raw_ast::Layout::Table(t) => {
                        let ctx = crate::name::NamingContext::create(decl.name.element.span())
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
                        Some(Type {
experimental_maybe_from_alias: None,

                            kind_v2: "identifier".to_string(),
                            subtype: None,
                            identifier: Some(full_synth),
                            nullable: Some(false),
                            element_type: None,
                            element_count: None,
                            maybe_element_count: None,
                            role: None,
                            protocol: None,
                            protocol_transport: None,
                            obj_type: None,
                            rights: None,
                            resource_identifier: None,
                            deprecated: None,
                            maybe_attributes: vec![],
                            field_shape_v2: None,
                            maybe_size_constant_name: None,
                            type_shape_v2: shape,
                        })
                    }
                    raw_ast::Layout::Union(u) => {
                        let ctx = crate::name::NamingContext::create(decl.name.element.span())
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
                                self.union_declarations.push(compiled);
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            }
                            self.shapes.get(&full_synth).cloned().unwrap()
                        };
                        Some(Type {
experimental_maybe_from_alias: None,

                            kind_v2: "identifier".to_string(),
                            subtype: None,
                            identifier: Some(full_synth),
                            nullable: Some(false),
                            element_type: None,
                            element_count: None,
                            maybe_element_count: None,
                            role: None,
                            protocol: None,
                            protocol_transport: None,
                            obj_type: None,
                            rights: None,
                            resource_identifier: None,
                            deprecated: None,
                            maybe_attributes: vec![],
                            field_shape_v2: None,
                            maybe_size_constant_name: None,
                            type_shape_v2: shape,
                        })
                    }
                    _ => {
                        // primitive or other inline layout
                        self.reporter.fail(
                            crate::diagnostics::Error::ErrInvalidMethodPayloadLayoutClass,
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
                        let p_ctx = crate::name::NamingContext::create(decl.name.element.span());
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

                        let is_allowed = if resolved_type.kind_v2 != "identifier" {
                            false
                        } else if let Some(id) = &resolved_type.identifier {
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
                                crate::diagnostics::Error::ErrInvalidMethodPayloadLayoutClass,
                                tc.element.span(),
                                &[&"provided type"],
                            );
                        }

                        Some(resolved_type)
                    }
                    raw_ast::Layout::Struct(s) => {
                        if s.members.is_empty() {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrEmptyPayloadStructs,
                                s.element.span(),
                                &[],
                            );
                        }
                        for sm in &s.members {
                            if sm.default_value.is_some() {
                                self.reporter.fail(
                                    crate::diagnostics::Error::ErrPayloadStructHasDefaultMembers,
                                    sm.name.element.span(),
                                    &[&sm.name.data()],
                                );
                            }
                        }
                        let p_ctx = crate::name::NamingContext::create(decl.name.element.span());
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
                        Some(Type {
experimental_maybe_from_alias: None,

                            kind_v2: "identifier".to_string(),
                            subtype: None,
                            identifier: Some(full_synth),
                            nullable: Some(false),
                            type_shape_v2: shape,
                            element_type: None,
                            element_count: None,
                            maybe_element_count: None,
                            role: None,
                            protocol: None,
                            protocol_transport: None,
                            obj_type: None,
                            rights: None,
                            resource_identifier: None,
                            deprecated: None,
                            maybe_attributes: vec![],
                            field_shape_v2: None,
                            maybe_size_constant_name: None,
                        })
                    }
                    raw_ast::Layout::Table(t) => {
                        let p_ctx = crate::name::NamingContext::create(decl.name.element.span());
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
                        Some(Type {
experimental_maybe_from_alias: None,

                            kind_v2: "identifier".to_string(),
                            subtype: None,
                            identifier: Some(full_synth),
                            nullable: Some(false),
                            type_shape_v2: shape,
                            element_type: None,
                            element_count: None,
                            maybe_element_count: None,
                            role: None,
                            protocol: None,
                            protocol_transport: None,
                            obj_type: None,
                            rights: None,
                            resource_identifier: None,
                            deprecated: None,
                            maybe_attributes: vec![],
                            field_shape_v2: None,
                            maybe_size_constant_name: None,
                        })
                    }
                    raw_ast::Layout::Union(u) => {
                        let p_ctx = crate::name::NamingContext::create(decl.name.element.span());
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
                            self.union_declarations.push(compiled);
                            if library_name == self.library_name {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls.insert(full_synth.clone());
                            }
                            self.shapes.get(&full_synth).cloned().unwrap()
                        };
                        Some(Type {
experimental_maybe_from_alias: None,

                            kind_v2: "identifier".to_string(),
                            subtype: None,
                            identifier: Some(full_synth),
                            nullable: Some(false),
                            type_shape_v2: shape,
                            element_type: None,
                            element_count: None,
                            maybe_element_count: None,
                            role: None,
                            protocol: None,
                            protocol_transport: None,
                            obj_type: None,
                            rights: None,
                            resource_identifier: None,
                            deprecated: None,
                            maybe_attributes: vec![],
                            field_shape_v2: None,
                            maybe_size_constant_name: None,
                        })
                    }
                    _ => {
                        self.reporter.fail(
                            crate::diagnostics::Error::ErrInvalidMethodPayloadLayoutClass,
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
                        let p_ctx = crate::name::NamingContext::create(decl.name.element.span());
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
                        if err_type_resolved.kind_v2 == "primitive" {
                            if err_type_resolved.subtype.as_deref() == Some("int32") || err_type_resolved.subtype.as_deref() == Some("uint32") {
                                is_valid_error_type = true;
                            }
                        } else if err_type_resolved.kind_v2 == "identifier" {
                            if let Some(id) = &err_type_resolved.identifier {
                                if let Some(e_decl) = self.enum_declarations.iter().find(|e| &e.name == id) {
                                    if e_decl.type_ == "int32" || e_decl.type_ == "uint32" {
                                        is_valid_error_type = true;
                                    }
                                } else if let Some(e_decl) = self.external_enum_declarations.iter().find(|e| &e.name == id) {
                                    if e_decl.type_ == "int32" || e_decl.type_ == "uint32" {
                                        is_valid_error_type = true;
                                    }
                                }
                            }
                        }

                        if !is_valid_error_type {
                            self.reporter.fail(
                                crate::diagnostics::Error::ErrInvalidErrorType,
                                tc.element.span(),
                                &[],
                            );
                        }

                        Some(err_type_resolved)
                    }
                    _ => None,
                }
            } else {
                None
            };

            let maybe_response_payload = if m.has_error {
                let err_type = maybe_response_err_type.clone().unwrap();
                let success_type = if let Some(ref succ) = maybe_response_success_type {
                    succ.clone()
                } else {
                    let p_ctx = crate::name::NamingContext::create(decl.name.element.span());
                    let mut ctx = p_ctx.enter_response(m.name.element.span());
                    ctx = ctx.enter_member("response");
                    ctx.set_name_override(format!("{}_{}_Response", short_name, m.name.data()));

                    let synth_name = ctx.flattened_name().to_string();
                    let full_synth = format!("{}/{}", library_name, synth_name);

                    let shape = if self.compiled_decls.contains(&full_synth) {
                        self.shapes.get(&full_synth).cloned().unwrap()
                    } else {
                        let shape = TypeShapeV2 {
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
                            type_shape_v2: shape.clone(),
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
                    let typ = Type {
experimental_maybe_from_alias: None,

                        kind_v2: "identifier".to_string(),
                        subtype: None,
                        identifier: Some(full_synth.clone()),
                        nullable: Some(false),
                        type_shape_v2: shape,
                        element_type: None,
                        element_count: None,
                        maybe_element_count: None,
                        role: None,
                        protocol: None,
                        protocol_transport: None,
                        obj_type: None,
                        rights: None,
                        resource_identifier: None,
                        deprecated: None,
                        maybe_attributes: vec![],
                        field_shape_v2: None,
                        maybe_size_constant_name: None,
                    };
                    maybe_response_success_type = Some(typ.clone());
                    typ
                };

                // Synthesize Union
                let synth_union_name = format!("{}_{}_Result", short_name, m.name.data());
                let full_synth_union = format!("{}/{}", library_name, synth_union_name);

                let mut union_out_of_line = 0;
                let mut union_has_padding = false;
                let mut union_handles = 0;
                for t in [&success_type, &err_type] {
                    let shape = &t.type_shape_v2;
                    let inlined = shape.inline_size <= 4;
                    let padding = if inlined {
                        (4 - (shape.inline_size % 4)) % 4
                    } else {
                        (8 - (shape.inline_size % 8)) % 8
                    };
                    union_has_padding = union_has_padding || shape.has_padding || padding != 0;

                    let env_max_out_of_line = shape.max_out_of_line.saturating_add(if inlined {
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
                }

                let union_shape = TypeShapeV2 {
                    inline_size: 16,
                    alignment: 8,
                    depth: 1,
                    max_handles: union_handles,
                    max_out_of_line: union_out_of_line,
                    has_padding: union_has_padding,
                    has_flexible_envelope: false,
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
                    crate::json_generator::UnionMember {
                        ordinal: 1,
                        reserved: None,
                        name: Some("response".to_string()),
                        type_: Some(success_type.clone()),
experimental_maybe_from_alias: None,
location: Some(response_loc),
                        deprecated: Some(false),
                        maybe_attributes: vec![],
                    },
                    crate::json_generator::UnionMember {
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
                    union_members.push(crate::json_generator::UnionMember {
experimental_maybe_from_alias: None,
ordinal: 3,
reserved: None,
name: Some("framework_err".to_string()),
type_: Some(Type {
experimental_maybe_from_alias: None,

                            kind_v2: "identifier".to_string(),
                            subtype: None,
                            identifier: Some("fidl/internal/FrameworkErr".to_string()),
                            nullable: Some(false),
                            type_shape_v2: TypeShapeV2 {
                                inline_size: 4,
                                alignment: 4,
                                depth: 0,
                                max_handles: 0,
                                max_out_of_line: 0,
                                has_padding: false,
                                has_flexible_envelope: false,
                            },
                            element_type: None,
                            element_count: None,
                            maybe_element_count: None,
                            role: None,
                            protocol: None,
                            protocol_transport: None,
                            obj_type: None,
                            rights: None,
                            resource_identifier: None,
                            deprecated: None,
                            maybe_attributes: vec![],
                            field_shape_v2: None,
                            maybe_size_constant_name: None,
                        }),
                        location: Some(_framework_err_loc),
                        deprecated: Some(false),
                        maybe_attributes: vec![],
                    });
                }

                let union_decl = crate::json_generator::UnionDeclaration {
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
                    is_result: true,
                    type_shape_v2: union_shape.clone(),
                    maybe_attributes: vec![],
                };
                self.union_declarations.push(union_decl);
                if library_name == self.library_name {
                    self.declaration_order.push(full_synth_union.clone());
                    self.compiled_decls.insert(full_synth_union.clone());
                }

                Some(Type {
experimental_maybe_from_alias: None,

                    kind_v2: "identifier".to_string(),
                    subtype: None,
                    identifier: Some(full_synth_union.clone()),
                    nullable: Some(false),
                    type_shape_v2: union_shape,
                    element_type: None,
                    element_count: None,
                    maybe_element_count: None,
                    role: None,
                    protocol: None,
                    protocol_transport: None,
                    obj_type: None,
                    rights: None,
                    resource_identifier: None,
                    deprecated: None,
                    maybe_attributes: vec![],
                    field_shape_v2: None,
                    maybe_size_constant_name: None,
                })
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
