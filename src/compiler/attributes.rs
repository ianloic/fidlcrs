use crate::compiler::RawDecl;
use crate::diagnostics::Error;
use crate::flat_ast::*;
use crate::raw_ast;
use crate::source_span::SourceSpan;
use crate::versioning_types::Version;
impl<'node, 'src> super::Compiler<'node, 'src> {
    pub(crate) fn get_location(&self, element: &raw_ast::SourceElement<'_>) -> Location {
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

                if attr.name.element.start_token.span.data == "transitional" {
                    let span = attr.name.element.span().clone();
                    // Bypass the '_ lifetime issue by recreating the span with 'src
                    let transmuted_span: SourceSpan<'src> = unsafe { std::mem::transmute(span) };
                    self.reporter.fail(
                        Error::ErrDeprecatedAttribute,
                        transmuted_span,
                        &[&"transitional".to_string()],
                    );
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
                        AttributeArg {
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
            arguments: vec![AttributeArg {
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
                                raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                                raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                                _ => continue,
                            };
                            let d = Version::parse(&val_str).unwrap_or(Version::POS_INF);
                            let is_depr = d <= Version::HEAD;
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
                            raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                            raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                            _ => continue,
                        };
                        let r = Version::parse(&val_str).unwrap_or(Version::POS_INF);
                        if r <= Version::HEAD {
                            return false;
                        }
                    } else if arg_name == "added" {
                        let val_str = match &arg.value {
                            raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                            raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                            _ => continue,
                        };
                        let a = Version::parse(&val_str).unwrap_or(Version::POS_INF);
                        if a > Version::HEAD {
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

    pub fn verify_attributes(&self) {
        let schemas = &self.attribute_schemas;

        // 1. Validate library attributes
        if let Some(libr) = &self.library_decl {
            if let Some(attrs) = &libr.attributes {
                schemas.validate(self, "library", false, attrs);
            }
        }

        // 2. Validate all declarations and their members
        for (_name, raw_decl) in &self.raw_decls {
            let kind = match raw_decl {
                RawDecl::Struct(_) => "struct",
                RawDecl::Enum(_) => "enum",
                RawDecl::Bits(_) => "bits",
                RawDecl::Union(_) => "union",
                RawDecl::Table(_) => "table",
                RawDecl::Protocol(_) => "protocol",
                RawDecl::Service(_) => "service",
                RawDecl::Resource(_) => "resource",
                RawDecl::Const(_) => "const",
                RawDecl::Alias(_) => "alias",
                RawDecl::Type(d) => match &d.layout {
                    raw_ast::Layout::Struct(_) => "struct",
                    raw_ast::Layout::Enum(_) => "enum",
                    raw_ast::Layout::Bits(_) => "bits",
                    raw_ast::Layout::Union(_) => "union",
                    raw_ast::Layout::Table(_) => "table",
                    raw_ast::Layout::TypeConstructor(_) => "type",
                },
            };
            // is_anon is determined by checking if the RawDecl has an optional name set.
            let is_anon = match raw_decl {
                RawDecl::Struct(d) => d.name.is_none(),
                RawDecl::Enum(d) => d.name.is_none(),
                RawDecl::Bits(d) => d.name.is_none(),
                RawDecl::Union(d) => d.name.is_none(),
                RawDecl::Table(d) => d.name.is_none(),
                _ => false,
            };
            if let Some(attrs) = raw_decl.attributes() {
                schemas.validate(self, kind, is_anon, attrs);
            }

            // 3. Verify members recursively
            match raw_decl {
                RawDecl::Struct(d) => {
                    for member in &d.members {
                        if let Some(attrs) = &member.attributes {
                            schemas.validate(self, "struct_member", false, attrs);
                        }
                    }
                }
                RawDecl::Protocol(d) => {
                    for method in &d.methods {
                        if let Some(attrs) = &method.attributes {
                            schemas.validate(self, "protocol_method", is_anon, attrs);
                        }
                    }
                }
                RawDecl::Enum(d) => {
                    for member in &d.members {
                        if let Some(attrs) = &member.attributes {
                            schemas.validate(self, "enum_member", false, attrs);
                        }
                    }
                }
                RawDecl::Bits(d) => {
                    for member in &d.members {
                        if let Some(attrs) = &member.attributes {
                            schemas.validate(self, "bits_member", false, attrs);
                        }
                    }
                }
                RawDecl::Union(d) => {
                    for member in &d.members {
                        if let Some(attrs) = &member.attributes {
                            schemas.validate(self, "union_member", false, attrs);
                        }
                    }
                }
                RawDecl::Table(d) => {
                    for member in &d.members {
                        if let Some(attrs) = &member.attributes {
                            schemas.validate(self, "table_member", false, attrs);
                        }
                    }
                }
                RawDecl::Service(d) => {
                    for member in &d.members {
                        if let Some(attrs) = &member.attributes {
                            schemas.validate(self, "service_member", false, attrs);
                        }
                    }
                }
                RawDecl::Resource(d) => {
                    for member in &d.properties {
                        if let Some(attrs) = &member.attributes {
                            schemas.validate(self, "resource_property", false, attrs);
                        }
                    }
                }
                RawDecl::Type(d) => match &d.layout {
                    raw_ast::Layout::Struct(s) => {
                        for member in &s.members {
                            if let Some(attrs) = &member.attributes {
                                schemas.validate(self, "struct_member", is_anon, attrs);
                            }
                        }
                    }
                    raw_ast::Layout::Enum(e) => {
                        for member in &e.members {
                            if let Some(attrs) = &member.attributes {
                                schemas.validate(self, "enum_member", is_anon, attrs);
                            }
                        }
                    }
                    raw_ast::Layout::Bits(b) => {
                        for member in &b.members {
                            if let Some(attrs) = &member.attributes {
                                schemas.validate(self, "bits_member", is_anon, attrs);
                            }
                        }
                    }
                    raw_ast::Layout::Table(t) => {
                        for member in &t.members {
                            if let Some(attrs) = &member.attributes {
                                schemas.validate(self, "table_member", is_anon, attrs);
                            }
                        }
                    }
                    raw_ast::Layout::Union(u) => {
                        for member in &u.members {
                            if let Some(attrs) = &member.attributes {
                                schemas.validate(self, "union_member", is_anon, attrs);
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
