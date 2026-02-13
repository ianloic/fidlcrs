use crate::json_generator::*;
use crate::raw_ast;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};

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

use crate::source_file::SourceFile;

pub struct Compiler<'src> {
    // Compiled shapes for types
    shapes: HashMap<String, TypeShapeV2>,
    source_file: Option<&'src SourceFile>,
}

#[derive(Clone)]
enum RawDecl<'node, 'src> {
    Struct(&'node raw_ast::StructDeclaration<'src>),
    Enum(&'node raw_ast::EnumDeclaration<'src>),
    Bits(&'node raw_ast::BitsDeclaration<'src>),
    Union(&'node raw_ast::UnionDeclaration<'src>),
    Table(&'node raw_ast::TableDeclaration<'src>),
    Protocol(&'node raw_ast::ProtocolDeclaration<'src>),
    Service(&'node raw_ast::ServiceDeclaration<'src>),
    Const(&'node raw_ast::ConstDeclaration<'src>),
    Type(&'node raw_ast::TypeDeclaration<'src>),
}

impl<'src> Compiler<'src> {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            source_file: None,
        }
    }

    pub fn compile(
        &mut self,
        file: raw_ast::File<'src>,
        source_file: &'src SourceFile,
    ) -> JsonRoot {
        self.source_file = Some(source_file);
        let library_name = file
            .library_decl
            .as_ref()
            .map(|l| l.path.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // 1. Collect all declarations
        // We let rust infer lifetimes for the keys/values, or explicit if needed.
        let mut raw_decls: HashMap<String, RawDecl<'_, 'src>> = HashMap::new();

        for decl in &file.type_decls {
            let name = format!("{}/{}", library_name, decl.name.data());
            raw_decls.insert(name, RawDecl::Type(decl));
        }

        for decl in &file.struct_decls {
            let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
            let full_name = format!("{}/{}", library_name, name);
            raw_decls.insert(full_name, RawDecl::Struct(decl));
        }

        for decl in &file.enum_decls {
            let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
            let full_name = format!("{}/{}", library_name, name);
            raw_decls.insert(full_name, RawDecl::Enum(decl));
        }

        for decl in &file.bits_decls {
            let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
            let full_name = format!("{}/{}", library_name, name);
            raw_decls.insert(full_name, RawDecl::Bits(decl));
        }

        for decl in &file.union_decls {
            let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
            let full_name = format!("{}/{}", library_name, name);
            raw_decls.insert(full_name, RawDecl::Union(decl));
        }

        for decl in &file.table_decls {
            let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
            let full_name = format!("{}/{}", library_name, name);
            raw_decls.insert(full_name, RawDecl::Table(decl));
        }

        for decl in &file.protocol_decls {
            let name = decl.name.data();
            let full_name = format!("{}/{}", library_name, name);
            raw_decls.insert(full_name, RawDecl::Protocol(decl));

            for method in &decl.methods {
                let method_name_camel = format!(
                    "{}{}",
                    method.name.data().chars().next().unwrap().to_uppercase(),
                    &method.name.data()[1..]
                );
                if let Some(raw_ast::Layout::Struct(s)) = &method.request_payload {
                    let synth_name = format!("{}Request", method_name_camel);
                    let full_synth =
                        format!("{}/{}", library_name, format!("{}{}", name, synth_name));
                    raw_decls.insert(full_synth, RawDecl::Struct(s));
                }
                if let Some(raw_ast::Layout::Struct(s)) = &method.response_payload {
                    let synth_name = format!("{}Response", method_name_camel);
                    let full_synth =
                        format!("{}/{}", library_name, format!("{}{}", name, synth_name));
                    raw_decls.insert(full_synth, RawDecl::Struct(s));
                }
            }
        }

        for decl in &file.service_decls {
            let name = decl.name.data();
            let full_name = format!("{}/{}", library_name, name);
            raw_decls.insert(full_name, RawDecl::Service(decl));
        }

        for decl in &file.const_decls {
            let name = decl.name.data();
            let full_name = format!("{}/{}", library_name, name);
            raw_decls.insert(full_name, RawDecl::Const(decl));
        }

        // 2. Build Dependency Graph
        let mut decl_kinds = HashMap::new();
        for (name, decl) in &raw_decls {
            let kind = match decl {
                RawDecl::Struct(_) => "struct",
                RawDecl::Union(_) => "union",
                RawDecl::Table(_) => "table",
                RawDecl::Protocol(_) => "protocol",
                RawDecl::Service(_) => "service",
                RawDecl::Const(_) => "const",
                RawDecl::Enum(_) => "enum",
                RawDecl::Bits(_) => "bits",
                RawDecl::Type(t) => match t.layout {
                    raw_ast::Layout::Struct(_) => "struct",
                    raw_ast::Layout::Union(_) => "union",
                    raw_ast::Layout::Table(_) => "table",
                    raw_ast::Layout::Enum(_) => "enum",
                    raw_ast::Layout::Bits(_) => "bits",
                    _ => "unknown",
                },
            };
            decl_kinds.insert(name.clone(), kind);
        }

        let sorted_names = self.topological_sort(&raw_decls, &library_name, &decl_kinds, false);

        // 3. Compile in order
        let mut struct_declarations = vec![];
        let mut enum_declarations = vec![];
        let mut bits_declarations = vec![];
        let mut const_declarations = vec![];
        let mut union_declarations = vec![];
        let mut table_declarations = vec![];
        let mut protocol_declarations = vec![];
        let mut service_declarations = vec![];
        let mut declarations_ignored = indexmap::IndexMap::new();

        for name in &sorted_names {
            if let Some(decl) = raw_decls.get(name) {
                // Determine if it's a struct and compile it
                match decl {
                    RawDecl::Type(t) => {
                        if let raw_ast::Layout::Struct(ref s) = t.layout {
                            // It is a struct defined via type alias syntax: type S = struct { ... };
                            let compiled = self.compile_struct(
                                t.name.data(),
                                s,
                                &library_name,
                                Some(&t.name.element),
                                None,
                                t.attributes.as_deref(),
                            );
                            struct_declarations.push(compiled);
                        } else if let raw_ast::Layout::Enum(ref e) = t.layout {
                            let compiled = self.compile_enum(
                                t.name.data(),
                                e,
                                &library_name,
                                Some(&t.name.element),
                                t.attributes.as_deref(),
                            );
                            enum_declarations.push(compiled);
                        } else if let raw_ast::Layout::Bits(ref b) = t.layout {
                            let compiled = self.compile_bits(
                                t.name.data(),
                                b,
                                &library_name,
                                Some(&t.name.element),
                                t.attributes.as_deref(),
                            );
                            bits_declarations.push(compiled);
                        } else if let raw_ast::Layout::Table(ref ta) = t.layout {
                            let compiled = self.compile_table(
                                t.name.data(),
                                ta,
                                &library_name,
                                Some(&t.name.element),
                                t.attributes.as_deref(),
                            );
                            table_declarations.push(compiled);
                        } else if let raw_ast::Layout::Union(ref u) = t.layout {
                            let compiled = self.compile_union(
                                t.name.data(),
                                u,
                                &library_name,
                                Some(&t.name.element),
                                t.attributes.as_deref(),
                            );
                            union_declarations.push(compiled);
                        }
                    }
                    RawDecl::Struct(s) => {
                        if s.name.is_none() {
                            // It's an anonymous/synthetic struct, compile_protocol will compile it!
                            continue;
                        }
                        // It is a struct defined via struct S { ... };
                        // name is already full name
                        // We need short name for compile_struct
                        let short_name = s.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = self.compile_struct(
                            short_name,
                            s,
                            &library_name,
                            None,
                            None,
                            s.attributes.as_deref(),
                        );
                        if s.name.is_some() {
                            struct_declarations.push(compiled);
                        }
                    }
                    RawDecl::Enum(e) => {
                        let short_name = e.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = self.compile_enum(
                            short_name,
                            e,
                            &library_name,
                            None,
                            e.attributes.as_deref(),
                        );
                        if e.name.is_some() {
                            enum_declarations.push(compiled);
                        }
                    }
                    RawDecl::Bits(b) => {
                        let short_name = b.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = self.compile_bits(
                            short_name,
                            b,
                            &library_name,
                            None,
                            b.attributes.as_deref(),
                        );
                        if b.name.is_some() {
                            bits_declarations.push(compiled);
                        }
                    }
                    RawDecl::Union(u) => {
                        let short_name = u.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = self.compile_union(
                            short_name,
                            u,
                            &library_name,
                            None,
                            u.attributes.as_deref(),
                        );
                        if u.name.is_some() {
                            union_declarations.push(compiled);
                        }
                    }
                    RawDecl::Table(t) => {
                        let short_name = t.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = self.compile_table(
                            short_name,
                            t,
                            &library_name,
                            None,
                            t.attributes.as_deref(),
                        );
                        if t.name.is_some() {
                            table_declarations.push(compiled);
                        }
                    }
                    RawDecl::Protocol(p) => {
                        let short_name = p.name.data();
                        let compiled = self.compile_protocol(
                            short_name,
                            p,
                            &library_name,
                            &mut struct_declarations,
                            &mut declarations_ignored,
                        );
                        protocol_declarations.push(compiled);
                    }
                    RawDecl::Service(s) => {
                        let short_name = s.name.data();
                        let compiled = self.compile_service(short_name, s, &library_name);
                        service_declarations.push(compiled);
                    }
                    RawDecl::Const(c) => {
                        let compiled = self.compile_const(c, &library_name);
                        const_declarations.push(compiled);
                    }
                }
            }
        }

        // Sort declarations by name to match fidlc output order (alphabetical)
        bits_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        const_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        enum_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        protocol_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        service_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        struct_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        table_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        union_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        // declaration_order.sort(); // Don't sort declaration_order, allow native order

        let mut declarations = indexmap::IndexMap::new();
        for decl in &bits_declarations {
            declarations.insert(decl.name.clone(), "bits".to_string());
        }
        for decl in &const_declarations {
            declarations.insert(decl.name.clone(), "const".to_string());
        }
        for decl in &enum_declarations {
            declarations.insert(decl.name.clone(), "enum".to_string());
        }
        for decl in &protocol_declarations {
            declarations.insert(decl.name.clone(), "protocol".to_string());
        }
        for decl in &service_declarations {
            declarations.insert(decl.name.clone(), "service".to_string());
        }
        for decl in &struct_declarations {
            declarations.insert(decl.name.clone(), "struct".to_string());
        }
        for decl in &table_declarations {
            declarations.insert(decl.name.clone(), "table".to_string());
        }
        for decl in &union_declarations {
            declarations.insert(decl.name.clone(), "union".to_string());
        }

        let declaration_order = self.topological_sort(&raw_decls, &library_name, &decl_kinds, true);

        JsonRoot {
            name: library_name,
            platform: "unversioned".to_string(),
            available: Some(BTreeMap::from([
                ("fuchsia".to_string(), vec!["HEAD".to_string()]),
                ("test".to_string(), vec!["HEAD".to_string()]),
            ])),
            maybe_attributes: file
                .library_decl
                .as_ref()
                .map_or(vec![], |decl| self.compile_attribute_list(&decl.attributes)),
            experiments: vec!["output_index_json".to_string()],
            library_dependencies: vec![],
            bits_declarations,
            const_declarations,
            enum_declarations,
            experimental_resource_declarations: vec![],
            protocol_declarations,
            service_declarations,
            struct_declarations,
            external_struct_declarations: vec![],
            table_declarations,
            union_declarations,
            // bits_declarations.sort_by(|a, b| a.name.cmp(&b.name)); // when implemented
            alias_declarations: vec![],
            new_type_declarations: vec![],
            declaration_order,
            declarations,
        }
    }

    fn topological_sort<'node>(
        &self,
        decls: &HashMap<String, RawDecl<'node, 'src>>,
        library_name: &str,
        decl_kinds: &HashMap<String, &str>,
        skip_optional: bool,
    ) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut sorted = vec![];
        let mut temp_mark = HashSet::new(); // for cycle detection

        let mut keys: Vec<&String> = decls.keys().collect();
        keys.sort();

        fn visit(
            name: &str,
            decls: &HashMap<String, RawDecl<'_, '_>>,
            library_name: &str,
            visited: &mut HashSet<String>,
            temp_mark: &mut HashSet<String>,
            sorted: &mut Vec<String>,
            decl_kinds: &HashMap<String, &str>,
            skip_optional: bool,
        ) {
            if visited.contains(name) {
                return;
            }
            if temp_mark.contains(name) {
                eprintln!("Cycle detected involving {}", name);
                return;
            }
            temp_mark.insert(name.to_string());

            if let Some(decl) = decls.get(name) {
                let deps = get_dependencies(decl, library_name, decl_kinds, skip_optional);
                // Sort dependencies by name to ensure deterministic order if needed, but they are in AST order
                for dep in deps {
                    visit(
                        &dep,
                        decls,
                        library_name,
                        visited,
                        temp_mark,
                        sorted,
                        decl_kinds,
                        skip_optional,
                    );
                }
            }

            temp_mark.remove(name);
            visited.insert(name.to_string());
            if decls.contains_key(name) {
                sorted.push(name.to_string());
            }
        }

        for name in keys {
            visit(
                name,
                decls,
                library_name,
                &mut visited,
                &mut temp_mark,
                &mut sorted,
                decl_kinds,
                skip_optional,
            );
        }

        sorted
    }

    fn compile_enum(
        &mut self,
        name: &str,
        decl: &raw_ast::EnumDeclaration<'_>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'_>>,
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
                if let Ok(val) = compiled_value.literal.value.get().trim_matches('"').parse::<u32>() {
                    maybe_unknown_value = Some(val);
                }
            }

            members.push(EnumMember {
                name: member.name.data().to_string(),
                location: self.get_location(&member.name.element),
                deprecated: false,
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
        let strict =
            decl.strictness.unwrap_or(raw_ast::Strictness::Flexible) == raw_ast::Strictness::Strict;

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
            naming_context: vec![name.to_string()],
            location,
            deprecated: false,
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

    fn compile_bits(
        &mut self,
        name: &str,
        decl: &raw_ast::BitsDeclaration<'_>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'_>>,
    ) -> BitsDeclaration {
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
        let mut mask: u64 = 0;

        for member in &decl.members {
            let attributes = self.compile_attribute_list(&member.attributes);
            let compiled_value = self.compile_constant(&member.value);

            // Calculate mask
            if let Ok(val) = compiled_value.literal.value.get().trim_matches('"').parse::<u64>() {
                mask |= val;
            }
            // TODO: Handle non-u64 values if needed?

            members.push(BitsMember {
                name: member.name.data().to_string(),
                location: self.get_location(&member.name.element),
                deprecated: false,
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

        // Strictness default: Flexible?
        let strict =
            decl.strictness.unwrap_or(raw_ast::Strictness::Flexible) == raw_ast::Strictness::Strict;

        BitsDeclaration {
            name: full_name,
            naming_context: vec![name.to_string()],
            location,
            deprecated: false,
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            type_: Type {
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
                type_shape_v2,
            },
            mask: mask.to_string(),
            members,
            strict,
        }
    }

    fn compile_table(
        &mut self,
        name: &str,
        decl: &raw_ast::TableDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'src>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'src>>,
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

            let (type_, name, reserved) = if let Some(type_ctor) = &member.type_ctor {
                let type_obj = self.resolve_type(type_ctor, library_name);
                let name = member.name.as_ref().unwrap().data().to_string();
                (Some(type_obj), Some(name), None)
            } else {
                (None, None, Some(true))
            };

            let attributes = self.compile_attribute_list(&member.attributes);

            members.push(TableMember {
                ordinal,
                reserved,
                type_,
                name,
                location: member.name.as_ref().map(|n| self.get_location(&n.element)),
                deprecated: Some(false),
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

        let type_shape_v2 = TypeShapeV2 {
            inline_size: 16,
            alignment: 8,
            depth,
            max_handles,
            max_out_of_line,
            has_padding, // Tables calculate padding based on envelopes
            has_flexible_envelope: true,
        };

        self.shapes.insert(full_name.clone(), type_shape_v2.clone());

        TableDeclaration {
            name: full_name,
            naming_context: vec![name.to_string()],
            location,
            deprecated: false,
            members,
            strict: false,
            resource: decl.is_resource,
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

    fn compile_union(
        &mut self,
        name: &str,
        decl: &raw_ast::UnionDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'src>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'src>>,
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
                    raw_ast::LiteralKind::Numeric => ord.value.parse::<u32>().unwrap_or(0),
                    _ => 0,
                }
            } else {
                0
            };

            let (type_, name, reserved) = if let Some(type_ctor) = &member.type_ctor {
                let type_obj = self.resolve_type(type_ctor, library_name);
                let name = member.name.as_ref().unwrap().data().to_string();
                (Some(type_obj), Some(name), None)
            } else {
                (None, None, Some(true))
            };

            let attributes = self.compile_attribute_list(&member.attributes);

            members.push(UnionMember {
                ordinal,
                reserved,
                name,
                type_,
                location: member.name.as_ref().map(|n| self.get_location(&n.element)),
                deprecated: Some(false),
                maybe_attributes: attributes,
            });
        }

        // Sort members by ordinal
        members.sort_by_key(|m| m.ordinal);

        #[allow(clippy::collection_is_never_read)]
        let mut attributes = self.compile_attribute_list(&decl.attributes);
        if let Some(inherited) = inherited_attributes {
            let extra = self.compile_attributes_from_ref(inherited);
            attributes.extend(extra);
        }

        let strict = decl.strictness == raw_ast::Strictness::Strict;

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

        let type_shape_v2 = TypeShapeV2 {
            inline_size: 16,
            alignment: 8,
            depth,
            max_handles,
            max_out_of_line: max_out_of_line as u32,
            has_padding,
            has_flexible_envelope: !strict,
        };

        self.shapes.insert(full_name.clone(), type_shape_v2);

        UnionDeclaration {
            name: full_name,
            naming_context: vec![name.to_string()],
            location,
            deprecated: false,
            members,
            strict,
            resource: decl.is_resource,
            is_result: false, // TODO: detect result unions
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            type_shape_v2: TypeShapeV2 {
                inline_size: 16,
                alignment: 8,
                depth,
                max_handles,
                max_out_of_line,
                has_padding,
                has_flexible_envelope: !strict,
            },
        }
    }

    fn compile_struct(
        &mut self,
        name: &str,
        decl: &raw_ast::StructDeclaration<'_>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        naming_context: Option<Vec<String>>,
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
            let type_obj = self.resolve_type(&member.type_ctor, library_name);
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

            members.push(StructMember {
                type_: type_obj,
                name: member.name.data().to_string(),
                location,
                deprecated: false,
                maybe_attributes: self.compile_attribute_list(&member.attributes),
                field_shape_v2: FieldShapeV2 {
                    offset: field_offset,
                    padding: 0,
                },
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

        StructDeclaration {
            name: full_name,
            naming_context: naming_context.unwrap_or_else(|| vec![name.to_string()]),
            location,
            deprecated: false,
            maybe_attributes: {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            members,
            resource: decl.is_resource,
            is_empty_success_struct: false,
            type_shape_v2: type_shape,
        }
    }

    fn resolve_type(&self, type_ctor: &raw_ast::TypeConstructor<'_>, library_name: &str) -> Type {
        let name = match &type_ctor.layout {
            raw_ast::LayoutParameter::Identifier(id) => id.to_string(),
            raw_ast::LayoutParameter::Literal(_) => {
                panic!("Literal layout not supported in resolve_type")
            }
            raw_ast::LayoutParameter::Type(_) => {
                panic!("Type layout not supported in resolve_type yet")
            }
        };

        let mut nullable = type_ctor.nullable;
        if !nullable {
            // Check constraints for "optional"
            for constraint in &type_ctor.constraints {
                if let raw_ast::Constant::Identifier(id) = constraint {
                    if id.identifier.to_string() == "optional" {
                        nullable = true;
                        break;
                    }
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
                    type_shape_v2: TypeShapeV2 {
                        inline_size: 16,
                        alignment: 8,
                        depth: 1,
                        max_handles: 0,
                        max_out_of_line: max_len,
                        has_padding: true,
                        has_flexible_envelope: false,
                    },
                }
            }
            "vector" => {
                if type_ctor.parameters.is_empty() {
                    // Error handling?
                    return Type {
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
                let inner_type = self.resolve_type(inner, library_name);

                let max_count = if let Some(c) = type_ctor.constraints.first() {
                    self.eval_constant_usize(c).unwrap_or(u32::MAX as usize) as u32
                } else {
                    u32::MAX
                };

                let new_depth = inner_type.type_shape_v2.depth.saturating_add(1);

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
                    type_shape_v2: TypeShapeV2 {
                        inline_size: 16,
                        alignment: 8,
                        depth: new_depth,
                        max_handles,
                        max_out_of_line: max_ool,
                        has_padding: inner_type.type_shape_v2.has_padding
                            || (inner_type.type_shape_v2.inline_size % 8 != 0),
                        has_flexible_envelope: false,
                    },
                }
            }
            "array" => {
                if type_ctor.parameters.len() < 2 {
                    return Type {
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
                let size_param = &type_ctor.parameters[1];
                let count = self.eval_type_constant_usize(size_param).unwrap_or(0) as u32;

                let inner_type = self.resolve_type(inner, library_name);

                let total_size = count * inner_type.type_shape_v2.inline_size;
                let max_ool = count * inner_type.type_shape_v2.max_out_of_line;

                Type {
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
                    type_shape_v2: TypeShapeV2 {
                        inline_size: total_size,
                        alignment: inner_type.type_shape_v2.alignment,
                        depth: inner_type.type_shape_v2.depth,
                        max_handles: count * inner_type.type_shape_v2.max_handles,
                        max_out_of_line: max_ool,
                        has_padding: inner_type.type_shape_v2.has_padding,
                        has_flexible_envelope: false,
                    },
                }
            }
            "handle" => {
                let subtype = if let Some(param) = type_ctor.parameters.first() {
                    if let raw_ast::LayoutParameter::Identifier(id) = &param.layout {
                        Some(id.to_string())
                    } else {
                        Some("handle".to_string())
                    }
                } else {
                    Some("handle".to_string())
                };

                Type {
                    kind_v2: "handle".to_string(),
                    subtype,
                    identifier: None,
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
                        if proto_name.contains('/') {
                            protocol = proto_name;
                        } else {
                            protocol = format!("{}/{}", library_name, proto_name);
                        }
                    }
                } else if let Some(param) = type_ctor.parameters.first() {
                    if let raw_ast::LayoutParameter::Identifier(id) = &param.layout {
                        let proto_name = id.to_string();
                        if proto_name.contains('/') {
                            protocol = proto_name;
                        } else {
                            protocol = format!("{}/{}", library_name, proto_name);
                        }
                    }
                }

                Type {
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
            _ => {
                // Try to resolve identifier
                // 1. Check if name exists directly
                // 2. Check if name exists with library prefix
                let full_name = if self.shapes.contains_key(&name) {
                    name.clone()
                } else {
                    format!("{}/{}", library_name, name)
                };

                if let Some(shape) = self.shapes.get(&full_name) {
                    Type {
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
                        type_shape_v2: shape.clone(),
                    }
                } else {
                    eprintln!("Warning: Type not found: {} (tried {})", name, full_name);
                    eprintln!("Available shapes: {:?}", self.shapes.keys());
                    Type {
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

    fn eval_constant_usize(&self, constant: &raw_ast::Constant<'_>) -> Option<usize> {
        match constant {
            raw_ast::Constant::Literal(lit) => match &lit.literal.kind {
                raw_ast::LiteralKind::Numeric => lit.literal.value.parse::<usize>().ok(),
                _ => None,
            },
            raw_ast::Constant::Identifier(id) => {
                if id.identifier.to_string() == "MAX" {
                    Some(u32::MAX as usize)
                } else {
                    None // TODO lookup const
                }
            }
            _ => None,
        }
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
        if let Some(source) = self.source_file {
            let start_span = element.start_token.span;
            let end_span = element.end_token.span;

            // Calculate length from start of start_token to end of end_token
            // This is annoying because SourceElement stores Tokens, not full span range directly.
            // But we can approximate or use the tokens.
            // Ideally SourceElement should have a method to get full string.
            // For now, let's just use the start token's location and length of the whole element?
            // Wait, SourceElement::span() is not implemented fully in raw_ast.rs?
            // Let's use start_token.

            let view = start_span.data;
            if let Some((_, pos)) = source.line_containing(view) {
                // Calculate true length: end_token.span.data.as_ptr() + end_token.span.data.len() - start_token.span.data.as_ptr()
                // Assuming they are in the same buffer.
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

    fn compile_attributes_from_ref(
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
                            type_: value.literal.kind.clone(), // This isn't generally correct natively but good enough for now
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

    fn compile_doc_comments(&self, doc_comments: &[&raw_ast::Attribute<'_>]) -> Attribute {
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
                    value: serde_json::value::RawValue::from_string(serde_json::to_string(&combined_value).unwrap()).unwrap(),
                    expression: serde_json::value::RawValue::from_string(serde_json::to_string(&combined_expression).unwrap()).unwrap(),
                    literal: Literal {
                        kind: "string".to_string(),
                        value: serde_json::value::RawValue::from_string(serde_json::to_string(&combined_value).unwrap()).unwrap(),
                        expression: serde_json::value::RawValue::from_string(serde_json::to_string(&combined_expression).unwrap()).unwrap(),
                    },
                },
                location: loc.clone(),
            }],
            location: loc,
        }
    }

    fn compile_attribute_list(
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
                                    if hc == '}' { break; }
                                    hex.push(hc);
                                    chars.next();
                                }
                                chars.next(); // consume }
                                if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                    if let Some(ch) = char::from_u32(code) {
                                        let mut b = [0; 2];
                                        for u in ch.encode_utf16(&mut b) {
                                            out.push_str(&format!("\\u{:04x}", u));
                                        }
                                        continue;
                                    }
                                }
                                out.push_str("\\u{"); out.push_str(&hex); out.push('}');
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
            } else {
                if c == '"' { out.push_str("\\\""); }
                else if c == '\\' { out.push_str("\\\\"); }
                else if c == '\n' { out.push_str("\\n"); }
                else if c == '\r' { out.push_str("\\r"); }
                else if c == '\t' { out.push_str("\\t"); }
                else { out.push(c); }
            }
        }
        out.push('"');
        out
    }

    fn compile_constant(&self, constant: &raw_ast::Constant<'_>) -> Constant {
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
                        } else {
                            val.clone()
                        };
                        ("numeric", serde_json::to_string(&n_str).unwrap(), serde_json::to_string(&val).unwrap())
                    }
                    raw_ast::LiteralKind::Bool(b) => {
                        let s = b.to_string();
                        ("bool", serde_json::to_string(&s).unwrap(), serde_json::to_string(&s).unwrap())
                    }
                    raw_ast::LiteralKind::DocComment => {
                        ("doc_comment", "\"\"".to_string(), "\"\"".to_string())
                    }
                };

                Constant {
                    kind: "literal".to_string(),
                    value: serde_json::value::RawValue::from_string(value_json.clone()).unwrap(),
                    expression: serde_json::value::RawValue::from_string(expr_json.clone()).unwrap(),
                    literal: Literal {
                        kind: kind.to_string(),
                        value: serde_json::value::RawValue::from_string(value_json).unwrap(),
                        expression: serde_json::value::RawValue::from_string(expr_json).unwrap(),
                    },
                }
            }
            raw_ast::Constant::Identifier(_) => Constant {
                kind: "identifier".to_string(),
                value: serde_json::value::RawValue::from_string("\"0\"".to_string()).unwrap(),
                expression: serde_json::value::RawValue::from_string("\"0\"".to_string()).unwrap(),
                literal: Literal {
                    kind: "numeric".to_string(),
                    value: serde_json::value::RawValue::from_string("\"0\"".to_string()).unwrap(),
                    expression: serde_json::value::RawValue::from_string("\"0\"".to_string()).unwrap(),
                },
            },
            raw_ast::Constant::BinaryOperator(_) => Constant {
                kind: "binary_operator".to_string(),
                value: serde_json::value::RawValue::from_string("\"0\"".to_string()).unwrap(),
                expression: serde_json::value::RawValue::from_string("\"0\"".to_string()).unwrap(),
                literal: Literal {
                    kind: "numeric".to_string(),
                    value: serde_json::value::RawValue::from_string("\"0\"".to_string()).unwrap(),
                    expression: serde_json::value::RawValue::from_string("\"0\"".to_string()).unwrap(),
                },
            },
        }
    }
}

fn get_dependencies<'node, 'src>(
    decl: &RawDecl<'node, 'src>,
    library_name: &str,
    _decl_kinds: &HashMap<String, &str>,
    skip_optional: bool,
) -> Vec<String> {
    let mut deps = vec![];
    match decl {
        RawDecl::Struct(s) => {
            for member in &s.members {
                collect_deps_from_ctor(&member.type_ctor, library_name, &mut deps, skip_optional);
            }
        }
        RawDecl::Enum(e) => {
            if let Some(ref subtype) = e.subtype {
                collect_deps_from_ctor(subtype, library_name, &mut deps, skip_optional);
            }
        }
        RawDecl::Bits(b) => {
            if let Some(ref subtype) = b.subtype {
                collect_deps_from_ctor(subtype, library_name, &mut deps, skip_optional);
            }
        }
        RawDecl::Union(u) => {
            for member in &u.members {
                if let Some(ref ctor) = member.type_ctor {
                    collect_deps_from_ctor(ctor, library_name, &mut deps, skip_optional);
                }
            }
        }
        RawDecl::Table(t) => {
            for member in &t.members {
                if let Some(ref ctor) = member.type_ctor {
                    collect_deps_from_ctor(ctor, library_name, &mut deps, skip_optional);
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
                    );
                }
            } else if let Some(e) = option_layout_as_enum(&t.layout) {
                if let Some(ref subtype) = e.subtype {
                    collect_deps_from_ctor(subtype, library_name, &mut deps, skip_optional);
                }
            } else if let Some(b) = option_layout_as_bits(&t.layout) {
                if let Some(ref subtype) = b.subtype {
                    collect_deps_from_ctor(subtype, library_name, &mut deps, skip_optional);
                }
            } else if let Some(u) = option_layout_as_union(&t.layout) {
                for member in &u.members {
                    if let Some(ref ctor) = member.type_ctor {
                        collect_deps_from_ctor(ctor, library_name, &mut deps, skip_optional);
                    }
                }
            } else if let Some(ta) = option_layout_as_table(&t.layout) {
                for member in &ta.members {
                    if let Some(ref ctor) = member.type_ctor {
                        collect_deps_from_ctor(ctor, library_name, &mut deps, skip_optional);
                    }
                }
            }
        }
        RawDecl::Protocol(p) => {
            for m in &p.methods {
                let _method_name_camel = format!(
                    "{}{}",
                    m.name.data().chars().next().unwrap().to_uppercase(),
                    &m.name.data()[1..]
                );
                if let Some(ref req) = m.request_payload {
                    if let raw_ast::Layout::Struct(_) = req {
                        let synth_name = format!(
                            "{}{}{}Request",
                            p.name.data(),
                            m.name.data().chars().next().unwrap().to_uppercase(),
                            &m.name.data()[1..]
                        );
                        deps.push(format!("{}/{}", library_name, synth_name));
                    }
                    collect_deps_from_layout(req, library_name, &mut deps, skip_optional);
                }
                if let Some(ref res) = m.response_payload {
                    if let raw_ast::Layout::Struct(_) = res {
                        let synth_name = format!(
                            "{}{}{}Response",
                            p.name.data(),
                            m.name.data().chars().next().unwrap().to_uppercase(),
                            &m.name.data()[1..]
                        );
                        deps.push(format!("{}/{}", library_name, synth_name));
                    }
                    collect_deps_from_layout(res, library_name, &mut deps, skip_optional);
                }
                if let Some(ref err) = m.error_payload {
                    collect_deps_from_layout(err, library_name, &mut deps, skip_optional);
                }
            }
        }
        RawDecl::Service(_) => {}
        RawDecl::Const(_) => {}
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
) {
    if skip_optional {
        // Nullable types (e.g., box, optional unions) are placed behind pointers or
        // envelopes and don't strict layout dependency cycles.
        if ctor.nullable {
            return;
        }

        // Check if it has an `:optional` constraint
        for constraint in &ctor.constraints {
            if let raw_ast::Constant::Identifier(id_const) = constraint {
                if id_const.identifier.to_string() == "optional" {
                    return;
                }
            }
        }
    }

    if let raw_ast::LayoutParameter::Identifier(ref id) = ctor.layout {
        let name = id.to_string();
        match name.as_str() {
            "bool" | "int8" | "uint8" | "int16" | "uint16" | "int32" | "uint32" | "int64"
            | "uint64" | "string" => {}
            _ => {
                deps.push(format!("{}/{}", library_name, name));
            }
        }
    }

    for param in &ctor.parameters {
        collect_deps_from_ctor(param, library_name, deps, skip_optional);
    }
}

fn collect_deps_from_layout(
    layout: &raw_ast::Layout<'_>,
    library_name: &str,
    deps: &mut Vec<String>,
    skip_optional: bool,
) {
    match layout {
        raw_ast::Layout::TypeConstructor(tc) => {
            collect_deps_from_ctor(tc, library_name, deps, skip_optional);
        }
        raw_ast::Layout::Struct(s) => {
            for member in &s.members {
                collect_deps_from_ctor(&member.type_ctor, library_name, deps, skip_optional);
            }
        }
        raw_ast::Layout::Union(u) => {
            for member in &u.members {
                if let Some(ref ctor) = member.type_ctor {
                    collect_deps_from_ctor(ctor, library_name, deps, skip_optional);
                }
            }
        }
        raw_ast::Layout::Table(t) => {
            for member in &t.members {
                if let Some(ref ctor) = member.type_ctor {
                    collect_deps_from_ctor(ctor, library_name, deps, skip_optional);
                }
            }
        }
        _ => {}
    }
}

impl<'src> Compiler<'src> {
    fn compile_service(
        &mut self,
        name: &str,
        decl: &raw_ast::ServiceDeclaration<'src>,
        library_name: &str,
    ) -> ServiceDeclaration {
        let full_name = format!("{}/{}", library_name, name);
        let location = self.get_location(&decl.name.element);

        let mut members = vec![];
        for member in &decl.members {
            let type_obj = self.resolve_type(&member.type_ctor, library_name);
            let member_name = member.name.data().to_string();
            let attributes = self.compile_attribute_list(&member.attributes);

            members.push(ServiceMember {
                type_: type_obj,
                name: member_name,
                location: self.get_location(&member.name.element),
                deprecated: false,
                maybe_attributes: attributes,
            });
        }

        ServiceDeclaration {
            name: full_name,
            location,
            deprecated: false,
            maybe_attributes: self.compile_attribute_list(&decl.attributes),
            members,
        }
    }

    fn compile_const(
        &mut self,
        decl: &raw_ast::ConstDeclaration<'src>,
        library_name: &str,
    ) -> ConstDeclaration {
        let name = decl.name.data();
        let full_name = format!("{}/{}", library_name, name);
        let location = self.get_location(&decl.name.element);
        
        let type_obj = self.resolve_type(&decl.type_ctor, library_name);
        let constant = self.compile_constant(&decl.value);

        ConstDeclaration {
            name: full_name,
            location,
            deprecated: false,
            maybe_attributes: self.compile_attribute_list(&decl.attributes),
            type_: type_obj,
            value: constant,
        }
    }

    pub fn compile_protocol(
        &mut self,
        short_name: &str,
        decl: &raw_ast::ProtocolDeclaration<'src>,
        library_name: &str,
        struct_declarations: &mut Vec<StructDeclaration>,
        declarations: &mut indexmap::IndexMap<String, String>,
    ) -> ProtocolDeclaration {
        let name = format!("{}/{}", library_name, short_name);

        let mut methods = vec![];
        for m in &decl.methods {
            let has_request = m.has_request;
            let maybe_request_payload = if let Some(ref l) = m.request_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        Some(self.resolve_type(tc, library_name))
                    }
                    raw_ast::Layout::Struct(s) => {
                        let method_name_camel = format!(
                            "{}{}",
                            m.name.data().chars().next().unwrap().to_uppercase(),
                            &m.name.data()[1..]
                        );
                        let synth_name = format!(
                            "{}{}{}Request",
                            short_name,
                            m.name.data().chars().next().unwrap().to_uppercase(),
                            &m.name.data()[1..]
                        );
                        let compiled = self.compile_struct(
                            &synth_name,
                            s,
                            library_name,
                            None,
                            Some(vec![
                                short_name.to_string(),
                                method_name_camel.clone(),
                                "Request".to_string(),
                            ]),
                            None,
                        );
                        struct_declarations.push(compiled);
                        let full_synth = format!("{}/{}", library_name, synth_name);
                        declarations.insert(full_synth.clone(), "struct".to_string());
                        let shape = self.shapes.get(&full_synth).cloned().unwrap();
                        Some(Type {
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
                            type_shape_v2: shape,
                        })
                    }
                    _ => None,
                }
            } else {
                None
            };

            let has_response = m.has_response;
            let maybe_response_payload = if let Some(ref l) = m.response_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        Some(self.resolve_type(tc, library_name))
                    }
                    raw_ast::Layout::Struct(s) => {
                        let method_name_camel = format!(
                            "{}{}",
                            m.name.data().chars().next().unwrap().to_uppercase(),
                            &m.name.data()[1..]
                        );
                        let synth_name = format!(
                            "{}{}{}Response",
                            short_name,
                            m.name.data().chars().next().unwrap().to_uppercase(),
                            &m.name.data()[1..]
                        );
                        let compiled = self.compile_struct(
                            &synth_name,
                            s,
                            library_name,
                            None,
                            Some(vec![
                                short_name.to_string(),
                                method_name_camel.clone(),
                                "Response".to_string(),
                            ]),
                            None,
                        );
                        struct_declarations.push(compiled);
                        let full_synth = format!("{}/{}", library_name, synth_name);
                        declarations.insert(full_synth.clone(), "struct".to_string());
                        let shape = self.shapes.get(&full_synth).cloned().unwrap();
                        Some(Type {
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
                            type_shape_v2: shape,
                        })
                    }
                    _ => None,
                }
            } else {
                None
            };

            let maybe_response_success_type = if m.has_error {
                maybe_response_payload.clone()
            } else {
                None
            };

            let maybe_response_err_type = if let Some(ref l) = m.error_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        Some(self.resolve_type(tc, library_name))
                    }
                    _ => None,
                }
            } else {
                None
            };

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
                    if attr.name.data() == "selector" {
                        if let Some(arg) = attr.args.first() {
                            if let raw_ast::Constant::Literal(ref l) = arg.value {
                                if l.literal.kind == raw_ast::LiteralKind::String {
                                    // The string literal includes quotes, but wait, usually we want
                                    // to strip them if the parser leaves them. Let's just use the value.
                                    // Our scanner keeps quotes? Let's assume we need to trim '\"'
                                    selector = l.literal.value.trim_matches('"').to_string();
                                }
                            }
                        }
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
                deprecated: false,
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

        ProtocolDeclaration {
            name,
            location: self.get_location(&decl.name.element),
            deprecated: false,
            maybe_attributes: self.compile_attribute_list(&decl.attributes),
            openness: "closed".to_string(),
            composed_protocols: vec![],
            methods,
        }
    }
}
