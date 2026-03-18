use crate::compiler::{CanonicalNames, Compiler, MemberKind};
use crate::diagnostics::Error;
use crate::flat_ast::Location;
use crate::flat_ast::{DeclBase, TableDeclaration, TableMember, TypeKind, TypeShape};
use crate::name::NamingContext;
use crate::names::OwnedQualifiedName;
use crate::raw_ast;
use crate::raw_ast::{AttributeProvenance, RawDecl};
use crate::token::TokenSubkind;

impl<'node, 'src> Compiler<'node, 'src> {
    pub fn compile_table(
        &mut self,
        name: &str,
        decl: &'node raw_ast::TableDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'src>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'src>>,
        naming_context: Option<std::rc::Rc<NamingContext<'src>>>,
    ) -> TableDeclaration {
        if let Some(m) = decl.modifiers.iter().find(|m| {
            (m.subkind == TokenSubkind::Strict || m.subkind == TokenSubkind::Flexible)
                && self.is_active(m.attributes.as_ref())
        }) {
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
            if !self.is_member_active(member.element.span().data.as_ptr() as usize) {
                continue;
            }
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
                let is_versioned = member.attributes.as_ref().is_some_and(|attrs| {
                    attrs.attributes.iter().any(|a| {
                        a.name.data() == "available"
                            || a.provenance == AttributeProvenance::ModifierAvailability
                    })
                });
                let prev_versioned = prev.maybe_attributes.iter().any(|a| a.name == "available");
                if !(is_versioned && prev_versioned) {
                    let location_str = format!(
                        "{}:{}:{}",
                        prev.location.filename, prev.location.line, prev.location.column
                    );
                    self.reporter.fail(
                        Error::ErrDuplicateTableFieldOrdinal,
                        member.ordinal.as_ref().unwrap().element.span(),
                        &[&location_str],
                    );
                }
            }

            let (type_, name, reserved, alias) = if let Some(type_ctor) = &member.type_ctor {
                let name_str = member.name.as_ref().unwrap().data().to_string();
                self.check_canonical_insert(
                    &mut member_names,
                    name_str.clone(),
                    MemberKind::TableField,
                    member.name.as_ref().unwrap().element.span(),
                    member.attributes.as_ref().is_some_and(|attrs| {
                        attrs.attributes.iter().any(|a| {
                            a.name.data() == "available"
                                || a.provenance == AttributeProvenance::ModifierAvailability
                        })
                    }),
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
                    let is_table = if let Some(decl) =
                        self.raw_decls.get(&OwnedQualifiedName::from(
                            type_obj.identifier().as_deref().unwrap_or("").to_string(),
                        )) {
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
                    && !decl.modifiers.iter().any(|m| {
                        m.subkind == TokenSubkind::Resource && self.is_active(m.attributes.as_ref())
                    })
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
                base: DeclBase {
                    name: name.unwrap_or_default().into(),
                    location: member
                        .name
                        .as_ref()
                        .map(|n| self.get_location(&n.element))
                        .unwrap_or_else(|| Location {
                            filename: String::new(),
                            line: 0,
                            column: 0,
                            length: 0,
                        }),
                    deprecated: self.is_deprecated(member.attributes.as_deref()),
                    maybe_attributes: attributes,
                },
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

        self.shapes.insert(
            OwnedQualifiedName::from(full_name.clone()),
            type_shape.clone(),
        );

        TableDeclaration::new(
            full_name.clone().into(),
            location,
            self.is_deprecated(decl.attributes.as_deref())
                || self.is_deprecated(inherited_attributes),
            {
                let mut attrs = self.compile_attribute_list(&decl.attributes);
                if let Some(inherited) = inherited_attributes {
                    let extra = self.compile_attributes_from_ref(inherited);
                    attrs.extend(extra);
                }
                attrs
            },
            naming_context
                .map(|ctx| ctx.context())
                .unwrap_or_else(|| vec![name.to_string()]),
            members,
            false,
            decl.modifiers.iter().any(|m| {
                m.subkind == TokenSubkind::Resource && self.is_active(m.attributes.as_ref())
            }),
            type_shape,
        )
    }
}
