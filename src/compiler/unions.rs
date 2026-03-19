use crate::compiler::{CanonicalNames, Compiler, MemberKind};
use crate::diagnostics::Error;
use crate::flat_ast::Location;
use crate::flat_ast::{DeclBase, TypeKind, TypeShape, UnionDeclaration, UnionMember};
use crate::name::NamingContext;
use crate::names::OwnedQualifiedName;
use crate::raw_ast;
use crate::raw_ast::AttributeProvenance;
use crate::token::TokenSubkind;

impl<'node, 'src> Compiler<'node, 'src> {
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
            if !self.is_member_active(member.element.span().data.as_ptr() as usize) {
                continue;
            }
            let ordinal = if let Some(ord) = &member.ordinal {
                match &ord.kind {
                    raw_ast::LiteralKind::Numeric => ord.value.parse::<u32>().map_err(|_| ()),
                    _ => Err(()),
                }
            } else {
                self.reporter
                    .fail(Error::ErrMissingOrdinalBeforeMember, member.element.span());
                Ok(0)
            };

            let ordinal = match ordinal {
                Ok(o) => {
                    if o == 0
                        && let Some(ord) = &member.ordinal
                    {
                        self.reporter
                            .fail(Error::ErrOrdinalsMustStartAtOne, ord.element.span());
                    }
                    o
                }
                Err(_) => {
                    self.reporter.fail(
                        Error::ErrOrdinalOutOfBound,
                        member.ordinal.as_ref().unwrap().element.span(),
                    );
                    0
                }
            };

            if let Some(prev) = members.iter().find(|m: &&UnionMember| m.ordinal == ordinal)
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
                        Error::ErrDuplicateUnionMemberOrdinal(flyweights::FlyStr::new(
                            format!("{}", &location_str).into_boxed_str(),
                        )),
                        member.ordinal.as_ref().unwrap().element.span(),
                    );
                }
            }

            if let Some(n_name) = &member.name {
                let member_name = n_name.data();
                self.check_canonical_insert(
                    &mut member_names,
                    member_name.to_string(),
                    MemberKind::UnionMember,
                    n_name.element.span(),
                    member.attributes.as_ref().is_some_and(|attrs| {
                        attrs.attributes.iter().any(|a| {
                            a.name.data() == "available"
                                || a.provenance == AttributeProvenance::ModifierAvailability
                        })
                    }),
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
                    && !decl.modifiers.iter().any(|m| {
                        m.subkind == TokenSubkind::Resource && self.is_active(m.attributes.as_ref())
                    })
                {
                    let member_name = member.name.as_ref().unwrap().data().to_string();
                    let n = name.to_string();
                    self.reporter.fail(
                        Error::ErrTypeMustBeResource(
                            flyweights::FlyStr::new(format!("{}", &"union")),
                            flyweights::FlyStr::new(format!("{}", &n)),
                            flyweights::FlyStr::new(format!("{}", &member_name)),
                            flyweights::FlyStr::new(format!("{}", &"union")),
                            flyweights::FlyStr::new(format!("{}", &"union")),
                            flyweights::FlyStr::new(format!("{}", &n)),
                        ),
                        type_ctor.element.span(),
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
                    self.reporter
                        .fail(Error::ErrOptionalUnionMember, type_ctor.element.span());
                }
                let name = member.name.as_ref().unwrap().data().to_string();
                (Some(type_obj), Some(name), None, alias)
            } else {
                (None, None, Some(true), None)
            };

            let attributes = self.compile_attribute_list(&member.attributes);

            if let Some(def) = &member.default_value {
                self.reporter
                    .fail(Error::ErrUnexpectedToken, def.element().span());
            }

            if attributes.iter().any(|a| a.name == "selector") {
                self.reporter.fail(
                    Error::ErrInvalidAttributePlacement(flyweights::FlyStr::new(
                        format!("{}", &"selector").into_boxed_str(),
                    )),
                    member.element.span(),
                );
            }

            members.push(UnionMember {
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

        let strict = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Strict && self.is_active(m.attributes.as_ref()));

        if strict && members.is_empty() {
            self.reporter
                .fail(Error::ErrMustHaveOneMember, decl.element.span());
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

        self.shapes.insert(
            OwnedQualifiedName::from(full_name.clone()),
            type_shape.clone(),
        );

        UnionDeclaration::new(
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
            strict,
            decl.modifiers.iter().any(|m| {
                m.subkind == TokenSubkind::Resource && self.is_active(m.attributes.as_ref())
            }),
            if decl.is_overlay { None } else { Some(false) }, // TODO: detect result unions
            type_shape,
        )
    }
}
