use crate::json_generator::*;
use crate::raw_ast;
use std::collections::{HashMap, HashSet};

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
    Type(&'node raw_ast::TypeDeclaration<'src>),
}

impl<'src> Compiler<'src> {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            source_file: None,
        }
    }

    pub fn compile(&mut self, file: raw_ast::File<'src>, source_file: &'src SourceFile) -> JsonRoot {
        self.source_file = Some(source_file);
        let library_name =
            file.library_decl.as_ref().map(|l| l.path.to_string()).unwrap_or_else(|| "unknown".to_string());

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

        // 2. Build Dependency Graph
        let mut decl_kinds = HashMap::new();
        for (name, decl) in &raw_decls {
            let kind = match decl {
                RawDecl::Struct(_) => "struct",
                RawDecl::Union(_) => "union",
                RawDecl::Table(_) => "table",
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

        let sorted_names = self.topological_sort(&raw_decls, &library_name, &decl_kinds);

        // 3. Compile in order
        let mut struct_declarations = vec![];
        let mut enum_declarations = vec![];
        let mut bits_declarations = vec![];
        let mut union_declarations = vec![];
        let mut table_declarations = vec![];
        let mut declarations = HashMap::new();
        let mut declaration_order = vec![];

        for name in &sorted_names {
            if let Some(decl) = raw_decls.get(name) {
                // Determine if it's a struct and compile it
                match decl {
                    RawDecl::Type(t) => {
                        if let raw_ast::Layout::Struct(ref s) = t.layout {
                             // It is a struct defined via type alias syntax: type S = struct { ... };
                             let compiled = self.compile_struct(t.name.data(), s, &library_name, Some(&t.name.element));
                             struct_declarations.push(compiled);
                             declarations.insert(name.clone(), "struct".to_string());
                             declaration_order.push(name.clone());
                        } else if let raw_ast::Layout::Enum(ref e) = t.layout {
                             let compiled = self.compile_enum(t.name.data(), e, &library_name, Some(&t.name.element));
                             enum_declarations.push(compiled);
                             declarations.insert(name.clone(), "enum".to_string());
                             declaration_order.push(name.clone());

                        } else if let raw_ast::Layout::Bits(ref b) = t.layout {
                             let compiled = self.compile_bits(t.name.data(), b, &library_name, Some(&t.name.element));
                             bits_declarations.push(compiled);
                             declarations.insert(name.clone(), "bits".to_string());
                             declaration_order.push(name.clone());
                        } else if let raw_ast::Layout::Table(ref ta) = t.layout {
                             let compiled = self.compile_table(t.name.data(), ta, &library_name, Some(&t.name.element), t.attributes.as_deref());
                             table_declarations.push(compiled);
                             declarations.insert(name.clone(), "table".to_string());
                             declaration_order.push(name.clone());
                        } else if let raw_ast::Layout::Union(ref u) = t.layout {
                             let compiled = self.compile_union(t.name.data(), u, &library_name, Some(&t.name.element), t.attributes.as_deref());
                             union_declarations.push(compiled);
                             declarations.insert(name.clone(), "union".to_string());
                             declaration_order.push(name.clone());
                        }
                    }
                    RawDecl::Struct(s) => {
                        // It is a struct defined via struct S { ... };
                        // name is already full name
                        // We need short name for compile_struct
                        // The key in map is full_name.
                        let short_name = s.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = self.compile_struct(short_name, s, &library_name, None);
                        if s.name.is_some() {
                            struct_declarations.push(compiled);
                            declarations.insert(name.clone(), "struct".to_string());
                            declaration_order.push(name.clone());
                        }
                    }
                    RawDecl::Enum(e) => {
                        let short_name = e.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = self.compile_enum(short_name, e, &library_name, None);
                        if e.name.is_some() {
                            enum_declarations.push(compiled);
                            declarations.insert(name.clone(), "enum".to_string());
                            declaration_order.push(name.clone());
                        }
                    }
                    RawDecl::Bits(b) => {
                         let short_name = b.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                         let compiled = self.compile_bits(short_name, b, &library_name, None);
                         if b.name.is_some() {
                             bits_declarations.push(compiled);
                             declarations.insert(name.clone(), "bits".to_string());
                             declaration_order.push(name.clone());
                         }
                    },
                    RawDecl::Union(u) => {
                         let short_name = u.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                         let compiled = self.compile_union(short_name, u, &library_name, None, None);
                         if u.name.is_some() {
                             union_declarations.push(compiled);
                             declarations.insert(format!("{}/{}", library_name, u.name.as_ref().unwrap().data()), "union".to_string());
                             declaration_order.push(format!("{}/{}", library_name, u.name.as_ref().unwrap().data()));
                         }
                    },
                    RawDecl::Table(t) => {
                         let short_name = t.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                         let compiled = self.compile_table(short_name, t, &library_name, None, None);
                         if t.name.is_some() {
                             table_declarations.push(compiled);
                             declarations.insert(format!("{}/{}", library_name, t.name.as_ref().unwrap().data()), "table".to_string());
                             declaration_order.push(format!("{}/{}", library_name, t.name.as_ref().unwrap().data()));
                         }
                    },
                }
            }
        }

        // Sort declarations by name to match fidlc output order (alphabetical)
        struct_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        table_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        union_declarations.sort_by(|a, b| a.name.cmp(&b.name));
        // declaration_order.sort(); // Don't sort declaration_order, allow native order

        JsonRoot {
            name: library_name,
            platform: "unversioned".to_string(),
            available: Some(HashMap::from([
                ("fuchsia".to_string(), vec!["HEAD".to_string()]),
                ("test".to_string(), vec!["HEAD".to_string()]),
            ])),
            experiments: vec!["output_index_json".to_string()],
            library_dependencies: vec![],
            bits_declarations,
            const_declarations: vec![],
            enum_declarations,
            experimental_resource_declarations: vec![],
            protocol_declarations: vec![],
            service_declarations: vec![],
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

    fn topological_sort<'node>(&self, decls: &HashMap<String, RawDecl<'node, 'src>>, library_name: &str, decl_kinds: &HashMap<String, &str>) -> Vec<String> {
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
                let deps = get_dependencies(decl, library_name, decl_kinds);
                for dep in deps {
                     visit(&dep, decls, library_name, visited, temp_mark, sorted, decl_kinds);
                }
            }

            temp_mark.remove(name);
            visited.insert(name.to_string());
            sorted.push(name.to_string());
        }

        for name in keys {
            visit(name, decls, library_name, &mut visited, &mut temp_mark, &mut sorted, decl_kinds);
        }

        sorted
    }

    fn compile_enum(
        &mut self,
        name: &str,
        decl: &raw_ast::EnumDeclaration<'_>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
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
                 if let Ok(val) = compiled_value.value.parse::<u32>() {
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

        self.shapes.insert(full_name.clone(), TypeShapeV2 {
             inline_size,
             alignment,
             depth: 0,
             max_handles: 0,
             max_out_of_line: 0,
             has_padding: false,
             has_flexible_envelope: false,
        });

        // Strictness default: Flexible?
        let strict = decl.strictness.unwrap_or(raw_ast::Strictness::Flexible) == raw_ast::Strictness::Strict;

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
        }
    }

    fn compile_bits(
        &mut self,
        name: &str,
        decl: &raw_ast::BitsDeclaration<'_>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
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
             if let Ok(val) = compiled_value.value.parse::<u64>() {
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
        let strict = decl.strictness.unwrap_or(raw_ast::Strictness::Flexible) == raw_ast::Strictness::Strict;

        BitsDeclaration {
            name: full_name,
            naming_context: vec![name.to_string()],
            location,
            deprecated: false,
            type_: Type {
                kind_v2: "primitive".to_string(),
                subtype: Some(subtype_name),
                identifier: None,
                nullable: None,
                element_type: None,
                element_count: None,
                maybe_element_count: None,
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
                 location: Some(self.get_location(&member.element)),
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

                 // Content is out of line of the envelope.
                 // Content is aligned to 8 bytes.
                 let content_size = shape.inline_size.saturating_add(shape.max_out_of_line);
                 let content_aligned = if content_size % 8 == 0 { content_size } else { content_size.saturating_add(8u32 - (content_size % 8u32)) };
                 max_out_of_line = max_out_of_line.saturating_add(content_aligned);

                 if shape.depth > depth {
                     depth = shape.depth;
                 }
            }
        }

        // Depth is 1 (table vector) + 1 (table envelope) if items present.
        // If empty, just vector depth (1).
        depth += if max_ordinal > 0 { 2 } else { 1 };

        let type_shape_v2 = TypeShapeV2 {
             inline_size: 16,
             alignment: 8,
             depth,
             max_handles,
             max_out_of_line,
             has_padding: false,
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
                 location: Some(self.get_location(&member.element)),
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

        for member in &members {
            if let Some(type_obj) = &member.type_ {
                 let shape = &type_obj.type_shape_v2;
                 if shape.max_handles > max_handles {
                     max_handles = shape.max_handles;
                 }

                 // Union payload is potentially out of line.
                 // If payload fits in envelope (<= 4 bytes), it's inline.
                 // Otherwise it's out of line.
                 // Wait, FIDL V2 union: 16 bytes inline.
                 // Tag (8) + Envelope (8).
                 // Envelope contains inlined data if <= 4 bytes.
                 // Else contains pointer.
                 // So max_out_of_line depends on if it's inlined.

                 let is_inlined = shape.inline_size <= 4;
                 if !is_inlined {
                     // 8 byte alignment for OOL content.
                     let content_size = shape.inline_size.saturating_add(shape.max_out_of_line);
                     let content_aligned = if content_size % 8 == 0 { content_size } else { content_size.saturating_add(8u32 - (content_size % 8u32)) };
                     if content_aligned > max_out_of_line {
                         max_out_of_line = content_aligned;
                     }
                 } else {
                     // If inlined, does it contribute to OOL? No.
                     // But does it have OOL children? Yes.
                     if shape.max_out_of_line > max_out_of_line {
                         max_out_of_line = shape.max_out_of_line;
                     }
                 }

                 if shape.depth > depth {
                     depth = shape.depth;
                 }
            }
        }

        // Union depth is 1 + max(member depth).
        // Union depth is 1 + max(member depth).
        depth = if members.is_empty() { 0 } else { depth + 1 };

        let type_shape_v2 = TypeShapeV2 {
             inline_size: 16,
             alignment: 8,
             depth,
             max_handles,
             max_out_of_line: max_out_of_line as u32,
             has_padding: false,
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
                has_padding: false, // Unions in V2 are 16 bytes (8 tag + 8 envelope), no inline padding.
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
            let current_end = members[i].field_shape_v2.offset + members[i].type_.type_shape_v2.inline_size;
            members[i].field_shape_v2.padding = next_offset - current_end;
        }

        let type_shape = TypeShapeV2 {
            inline_size: total_size,
            alignment,
            depth,
            max_handles,
            max_out_of_line,
            has_padding: final_padding > 0 || members.iter().any(|m| m.field_shape_v2.padding > 0 || m.type_.type_shape_v2.has_padding),
            has_flexible_envelope: members.iter().any(|m| m.type_.type_shape_v2.has_flexible_envelope),
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
            naming_context: vec![name.to_string()],
            location,
            deprecated: false,
            members,
            resource: decl.is_resource,
            is_empty_success_struct: false,
            type_shape_v2: type_shape,
        }
    }

    fn resolve_type(
        &self,
        type_ctor: &raw_ast::TypeConstructor<'_>,
        library_name: &str,
    ) -> Type {
        let name = match &type_ctor.layout {
            raw_ast::LayoutParameter::Identifier(id) => id.to_string(),
            raw_ast::LayoutParameter::Literal(_) => panic!("Literal layout not supported in resolve_type"),
            raw_ast::LayoutParameter::Type(_) => panic!("Type layout not supported in resolve_type yet"),
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
            "bool" | "int8" | "int16" | "int32" | "int64" | "uint8" | "uint16" | "uint32" | "uint64" | "float32" | "float64" => {
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
                        has_flexible_envelope: false
                    }
                }
            },
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
                    element_type: None,
                    element_count: None,
                    maybe_element_count: if max_len == u32::MAX { None } else { Some(max_len) },
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
                        has_flexible_envelope: false
                    }
                }
            },
            "vector" => {
                 if type_ctor.parameters.is_empty() {
                     // Error handling?
                     return Type { kind_v2: "unknown".to_string(), subtype: None, identifier: None, nullable: None, element_type: None, element_count: None, maybe_element_count: None, deprecated: None, maybe_attributes: vec![], field_shape_v2: None, type_shape_v2: TypeShapeV2 { inline_size: 0, alignment: 1, depth: 0, max_handles: 0, max_out_of_line: 0, has_padding: false, has_flexible_envelope: false } };
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
                 let max_ool = if content_size % 8 == 0 { content_size } else { content_size.saturating_add(8 - (content_size % 8)) };

                 let max_handles = max_count.saturating_mul(inner_type.type_shape_v2.max_handles);

                 Type {
                     kind_v2: "vector".to_string(),
                     subtype: None,
                     identifier: None,
                     nullable: Some(nullable),
                     element_type: Some(Box::new(inner_type)),
                     element_count: None,
                     maybe_element_count: if max_count == u32::MAX { None } else { Some(max_count) },
                     deprecated: None,
                     maybe_attributes: vec![],
                     field_shape_v2: None,
                     type_shape_v2: TypeShapeV2 {
                         inline_size: 16,
                         alignment: 8,
                         depth: new_depth,
                         max_handles,
                         max_out_of_line: max_ool,
                         has_padding: true,
                         has_flexible_envelope: false
                     }
                 }
            },
            "array" => {
                 if type_ctor.parameters.len() < 2 {
                     return Type { kind_v2: "unknown".to_string(), subtype: None, identifier: None, nullable: None, element_type: None, element_count: None, maybe_element_count: None, deprecated: None, maybe_attributes: vec![], field_shape_v2: None, type_shape_v2: TypeShapeV2 { inline_size: 0, alignment: 1, depth: 0, max_handles: 0, max_out_of_line: 0, has_padding: false, has_flexible_envelope: false } };
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
                         has_flexible_envelope: false
                     }
                 }
            },
            "handle" | "client_end" | "server_end" => {
                let subtype = if name == "handle" {
                     if let Some(param) = type_ctor.parameters.first() {
                         if let raw_ast::LayoutParameter::Identifier(id) = &param.layout {
                             Some(id.to_string())
                         } else {
                             Some("handle".to_string())
                         }
                     } else {
                         Some("handle".to_string())
                     }
                } else {
                     Some("channel".to_string())
                };

                Type {
                    kind_v2: "handle".to_string(),
                    subtype,
                    identifier: None,
                    nullable: Some(nullable),
                    element_type: None,
                    element_count: None,
                    maybe_element_count: None,
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
                    }
                }
            },
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
                         }
                     }
                }
            }
        }
    }

    fn eval_constant_usize(&self, constant: &raw_ast::Constant<'_>) -> Option<usize> {
        match constant {
            raw_ast::Constant::Literal(lit) => {
                match &lit.literal.kind {
                    raw_ast::LiteralKind::Numeric => lit.literal.value.parse::<usize>().ok(),
                    _ => None,
                }
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
             raw_ast::LayoutParameter::Literal(lit) => {
                match &lit.literal.kind {
                    raw_ast::LiteralKind::Numeric => lit.literal.value.parse::<usize>().ok(),
                    _ => None,
                }
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
             _ => None
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
                     filename: source.filename().to_string(),
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

    fn compile_attributes_from_ref(&self, attributes: &raw_ast::AttributeList<'_>) -> Vec<Attribute> {
        attributes.attributes.iter().map(|attr| {
            Attribute {
                name: attr.name.data().to_string(),
                arguments: vec![], // TODO: compile arguments
                location: self.get_location(&attr.element),
            }
        }).collect()
    }

    fn compile_attribute_list(&self, attributes: &Option<Box<raw_ast::AttributeList<'_>>>) -> Vec<Attribute> {
        if let Some(list) = attributes {
            self.compile_attributes_from_ref(list)
        } else {
            vec![]
        }
    }

    fn compile_constant(&self, constant: &raw_ast::Constant<'_>) -> Constant {
        match constant {
            raw_ast::Constant::Literal(lit) => {
                let (kind, value, expression) = match &lit.literal.kind {
                    raw_ast::LiteralKind::String => ("string", lit.literal.value.clone(), lit.literal.value.clone()),
                    raw_ast::LiteralKind::Numeric => {
                        let val = lit.literal.value.clone();
                        if val.starts_with("0x") || val.starts_with("0X") {
                            let without_prefix = &val[2..];
                            if let Ok(n) = u64::from_str_radix(without_prefix, 16) {
                                ("numeric", n.to_string(), val)
                            } else {
                                ("numeric", val.clone(), val)
                            }
                        } else {
                            ("numeric", val.clone(), val)
                        }
                    },
                    raw_ast::LiteralKind::Bool(b) => ("bool", b.to_string(), b.to_string()),
                    raw_ast::LiteralKind::DocComment => ("doc_comment", "".to_string(), "".to_string()), // Should not happen in constants
                };

                Constant {
                    kind: "literal".to_string(),
                    value: value.clone(),
                    expression: expression.clone(),
                    literal: Literal {
                        kind: kind.to_string(),
                        value: value,
                        expression: expression,
                    },
                }
            }
            raw_ast::Constant::Identifier(_) => {
                 Constant {
                     kind: "identifier".to_string(),
                     value: "0".to_string(),
                     expression: "0".to_string(),
                     literal: Literal {
                         kind: "numeric".to_string(),
                         value: "0".to_string(),
                         expression: "0".to_string(),
                     },
                 }
            }
            raw_ast::Constant::BinaryOperator(_) => {
                 Constant {
                     kind: "binary_operator".to_string(),
                     value: "0".to_string(),
                     expression: "0".to_string(),
                     literal: Literal {
                         kind: "numeric".to_string(),
                         value: "0".to_string(),
                         expression: "0".to_string(),
                     },
                 }
            }
        }
    }
}

fn get_dependencies(decl: &RawDecl<'_, '_>, library_name: &str, _decl_kinds: &HashMap<String, &str>) -> Vec<String> {
        let mut deps = vec![];
    match decl {
        RawDecl::Struct(s) => {
             for member in &s.members {
                 collect_deps_from_ctor(&member.type_ctor, library_name, &mut deps);
             }
        }
        RawDecl::Enum(e) => {
             if let Some(ref subtype) = e.subtype {
                 collect_deps_from_ctor(subtype, library_name, &mut deps);
             }
        }
        RawDecl::Bits(b) => {
              if let Some(ref subtype) = b.subtype {
                  collect_deps_from_ctor(subtype, library_name, &mut deps);
              }
        }
        RawDecl::Union(u) => {
             for member in &u.members {
                 if let Some(ref ctor) = member.type_ctor {
                      collect_deps_from_ctor(ctor, library_name, &mut deps);
                 }
             }
        }
        RawDecl::Table(t) => {
             for member in &t.members {
                 if let Some(ref ctor) = member.type_ctor {
                      collect_deps_from_ctor(ctor, library_name, &mut deps);
                 }
             }
        }
        RawDecl::Type(t) => {
             if let Some(s) = option_layout_as_struct(&t.layout) {
                  for member in &s.members {
                       collect_deps_from_ctor(&member.type_ctor, library_name, &mut deps);
                  }
             } else if let Some(e) = option_layout_as_enum(&t.layout) {
                  if let Some(ref subtype) = e.subtype {
                      collect_deps_from_ctor(subtype, library_name, &mut deps);
                  }
             } else if let Some(b) = option_layout_as_bits(&t.layout) {
                  if let Some(ref subtype) = b.subtype {
                      collect_deps_from_ctor(subtype, library_name, &mut deps);
                  }
             } else if let Some(u) = option_layout_as_union(&t.layout) {
                  for member in &u.members {
                      if let Some(ref ctor) = member.type_ctor {
                           collect_deps_from_ctor(ctor, library_name, &mut deps);
                      }
                  }
             } else if let Some(ta) = option_layout_as_table(&t.layout) {
                  for member in &ta.members {
                      if let Some(ref ctor) = member.type_ctor {
                           collect_deps_from_ctor(ctor, library_name, &mut deps);
                      }
                  }
             }
        }
    }

        deps
    }

fn option_layout_as_struct<'a>(layout: &'a raw_ast::Layout<'a>) -> Option<&'a raw_ast::StructDeclaration<'a>> {
    if let raw_ast::Layout::Struct(s) = layout {
        Some(s)
    } else {
        None
    }
}

fn option_layout_as_enum<'a>(layout: &'a raw_ast::Layout<'a>) -> Option<&'a raw_ast::EnumDeclaration<'a>> {
    if let raw_ast::Layout::Enum(e) = layout {
        Some(e)
    } else {
        None
    }
}

fn option_layout_as_bits<'a>(layout: &'a raw_ast::Layout<'a>) -> Option<&'a raw_ast::BitsDeclaration<'a>> {
    if let raw_ast::Layout::Bits(b) = layout {
        Some(b)
    } else {
        None
    }
}

fn option_layout_as_union<'a>(layout: &'a raw_ast::Layout<'a>) -> Option<&'a raw_ast::UnionDeclaration<'a>> {
    if let raw_ast::Layout::Union(u) = layout {
        Some(u)
    } else {
        None
    }
}

fn option_layout_as_table<'a>(layout: &'a raw_ast::Layout<'a>) -> Option<&'a raw_ast::TableDeclaration<'a>> {
    if let raw_ast::Layout::Table(t) = layout {
        Some(t)
    } else {
        None
    }
}



fn collect_deps_from_ctor(ctor: &raw_ast::TypeConstructor<'_>, library_name: &str, deps: &mut Vec<String>) {
    if let raw_ast::LayoutParameter::Identifier(ref id) = ctor.layout {
        let name = id.to_string();
        match name.as_str() {
            "bool" | "int8" | "uint8" | "int16" | "uint16" | "int32" | "uint32" | "int64" | "uint64" | "string" => {}
            _ => {
                deps.push(format!("{}/{}", library_name, name));
            }
        }
    }

    for param in &ctor.parameters {
        collect_deps_from_ctor(param, library_name, deps);
    }
}
