use crate::compiler::{CanonicalNames, Compiler, MemberKind};
use crate::diagnostics::Error;
use crate::flat_ast::{BitsDeclaration, BitsMember, DeclBase, PrimitiveSubtype, Type};
use crate::name::NamingContext;
use crate::names::{OwnedLibraryName, OwnedQualifiedName};
use crate::raw_ast;
use crate::raw_ast::{AttributeProvenance, RawDecl};
use crate::token::TokenSubkind;

impl<'node, 'src> Compiler<'node, 'src> {
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
                Error::ErrCannotSpecifyModifier(
                    format!("{}", &"resource".to_string()),
                    format!("{}", &"bits".to_string()),
                ),
                m.element.span(),
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
                    if !full_name.contains('/') && !self.shapes.contains_key::<str>(&current) {
                        let fqn = OwnedLibraryName::new(library_name.to_string())
                            .with_declaration(&current);
                        if self.raw_decls.contains_key(&fqn) {
                            full_name = fqn.as_string();
                        } else if let Some((lib, name)) = current.rsplit_once('.') {
                            let dep_fqn =
                                OwnedLibraryName::new(lib.to_string()).with_declaration(name);
                            if self.raw_decls.contains_key(&dep_fqn) {
                                full_name = dep_fqn.as_string();
                            } else {
                                full_name = fqn.as_string();
                            }
                        } else {
                            full_name = fqn.as_string();
                        }
                    }
                    if let Some(RawDecl::Alias(alias)) =
                        self.raw_decls.get::<str>(full_name.as_ref())
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
                    .fail(Error::ErrInvalidWrappedType, sc.element.span());
            }
        }

        let is_valid_type = matches!(
            subtype_name.as_str(),
            "uint8" | "uint16" | "uint32" | "uint64"
        );
        if !is_valid_type {
            self.reporter.fail(
                Error::ErrBitsTypeMustBeUnsignedIntegralPrimitive(format!("{}", &subtype_name)),
                decl.name
                    .as_ref()
                    .map_or_else(|| decl.element.start_token.span, |id| id.element.span()),
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
            );
        }

        let mut members = vec![];
        let mut mask: u64 = 0;
        let mut member_names = CanonicalNames::new();

        for member in &decl.members {
            if !self.is_member_active(member.element.span().data.as_ptr() as usize) {
                continue;
            }
            let attributes = self.compile_attribute_list(&member.attributes);
            let compiled_value = self.compile_constant(&member.value);

            let name_str = member.name.data().to_string();
            self.check_canonical_insert(
                &mut member_names,
                name_str.clone(),
                MemberKind::Member,
                member.name.element.span(),
                member.attributes.as_ref().is_some_and(|attrs| {
                    attrs.attributes.iter().any(|a| {
                        a.name.data() == "available"
                            || a.provenance == AttributeProvenance::ModifierAvailability
                    })
                }),
            );

            // Calculate mask and validate value
            let mut valid_value = true;
            match &member.value {
                raw_ast::Constant::Literal(_) => {
                    if let Some(literal) = &compiled_value.literal {
                        let val_str = literal.value.trim_matches('"');
                        if let Ok(val) = val_str.parse::<u64>() {
                            if val != 0 && (val & (val - 1)) != 0 {
                                self.reporter.fail(
                                    Error::ErrBitsMemberMustBePowerOfTwo,
                                    member.value.element().span(),
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
                                    Error::ErrConstantOverflowsType(
                                        format!("{}", &val_str),
                                        format!("{}", &subtype_name),
                                    ),
                                    member.value.element().span(),
                                );
                                valid_value = false;
                            }

                            if valid_value {
                                if (mask & val) != 0 {
                                    self.reporter.fail(
                                        Error::ErrDuplicateMemberValue(
                                            format!("{}", &"bits"),
                                            format!("{}", &name_str),
                                            format!("{}", &"unknown"),
                                            format!("{}", &name_str),
                                        ),
                                        member.value.element().span(),
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
                                );
                            } else {
                                self.reporter.fail(
                                    Error::ErrConstantOverflowsType(
                                        format!("{}", &val_str),
                                        format!("{}", &subtype_name),
                                    ),
                                    member.value.element().span(),
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
                            );
                        } else if (mask & val) != 0 {
                            self.reporter.fail(
                                Error::ErrDuplicateMemberValue(
                                    format!("{}", &"bits"),
                                    format!("{}", &name_str),
                                    format!("{}", &"unknown"),
                                    format!("{}", &name_str),
                                ),
                                member.value.element().span(),
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
                        );
                    }
                } // No other Constant variants
            }

            members.push(BitsMember {
                base: DeclBase {
                    name: name_str.into(),
                    location: self.get_location(&member.name.element),
                    deprecated: self.is_deprecated(member.attributes.as_deref()),
                    maybe_attributes: attributes,
                },
                value: compiled_value,
            });
        }

        let subtype = subtype_name.parse().unwrap_or(PrimitiveSubtype::Uint32);
        let primitive = Type::primitive(subtype);

        self.shapes.insert(
            OwnedQualifiedName::from(full_name.clone()),
            primitive.type_shape.clone(),
        );

        BitsDeclaration::new(
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
            primitive,
            mask.to_string(),
            members,
            strict,
        )
    }
}
