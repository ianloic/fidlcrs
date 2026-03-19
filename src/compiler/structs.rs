use crate::compiler::{CanonicalNames, Compiler, MemberKind};
use crate::diagnostics::Error;
use crate::flat_ast::{DeclBase, FieldShape, StructDeclaration, StructMember, TypeKind, TypeShape};
use crate::name::NamingContext;
use crate::names::OwnedQualifiedName;
use crate::raw_ast;
use crate::raw_ast::AttributeProvenance;
use crate::token::TokenSubkind;

impl<'node, 'src> Compiler<'node, 'src> {
    pub fn compile_struct(
        &mut self,
        name: &str,
        decl: &'node raw_ast::StructDeclaration<'src>,
        library_name: &str,
        name_element: Option<&raw_ast::SourceElement<'_>>,
        naming_context: Option<std::rc::Rc<NamingContext<'src>>>,
        inherited_attributes: Option<&raw_ast::AttributeList<'_>>,
    ) -> StructDeclaration {
        if let Some(m) = decl.modifiers.iter().find(|m| {
            (m.subkind == TokenSubkind::Strict || m.subkind == TokenSubkind::Flexible)
                && self.is_active(m.attributes.as_ref())
        }) {
            self.reporter.fail(
                Error::ErrCannotSpecifyModifier(
                    flyweights::FlyStr::new(format!("{}", &m.element.span().data.to_string())),
                    flyweights::FlyStr::new(format!("{}", &"struct".to_string())),
                ),
                m.element.span(),
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
            if !self.is_member_active(member.element.span().data.as_ptr() as usize) {
                continue;
            }
            let member_name = member.name.data();
            self.check_canonical_insert(
                &mut member_names,
                member_name.to_string(),
                MemberKind::StructMember,
                member.name.element.span(),
                member.attributes.as_ref().is_some_and(|attrs| {
                    attrs.attributes.iter().any(|a| {
                        a.name.data() == "available"
                            || a.provenance == AttributeProvenance::ModifierAvailability
                    })
                }),
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
                && !decl.modifiers.iter().any(|m| {
                    m.subkind == TokenSubkind::Resource && self.is_active(m.attributes.as_ref())
                })
            {
                let member_name = member.name.data().to_string();
                let n = name.to_string();
                self.reporter.fail(
                    Error::ErrTypeMustBeResource(
                        flyweights::FlyStr::new(format!("{}", &"struct")),
                        flyweights::FlyStr::new(format!("{}", &n)),
                        flyweights::FlyStr::new(format!("{}", &member_name)),
                        flyweights::FlyStr::new(format!("{}", &"struct")),
                        flyweights::FlyStr::new(format!("{}", &"struct")),
                        flyweights::FlyStr::new(format!("{}", &n)),
                    ),
                    member.type_ctor.element.span(),
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
                );
            }

            let mut maybe_default_value = None;
            if let Some(def_val) = &member.default_value {
                self.validate_constant(def_val, &type_obj);
                maybe_default_value = Some(self.compile_constant(def_val));
            }

            members.push(StructMember {
                type_: type_obj,
                base: DeclBase {
                    name: member.name.data().to_string().into(),
                    location,
                    deprecated: self.is_deprecated(member.attributes.as_deref()),
                    maybe_attributes: self.compile_attribute_list(&member.attributes),
                },
                experimental_maybe_from_alias: alias,
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
        self.shapes.insert(
            OwnedQualifiedName::from(full_name.clone()),
            type_shape.clone(),
        );

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
                Error::ErrInlineSizeExceedsLimit(
                    flyweights::FlyStr::new(format!("{}", &name)),
                    flyweights::FlyStr::new(format!("{}", &total_size.to_string())),
                    flyweights::FlyStr::new(format!("{}", &"65535".to_string())),
                ),
                span,
            );
        }

        StructDeclaration::new(
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
            decl.modifiers.iter().any(|m| {
                m.subkind == TokenSubkind::Resource && self.is_active(m.attributes.as_ref())
            }),
            false,
            type_shape,
        )
    }
}
