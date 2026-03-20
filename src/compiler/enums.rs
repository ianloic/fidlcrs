use crate::compiler::{CanonicalNames, Compiler, MemberKind};
use crate::diagnostics::Error;
use crate::flat_ast::{DeclBase, EnumDeclaration, EnumMember, PrimitiveSubtype, Type, TypeShape};
use crate::name::NamingContext;
use crate::names::{OwnedLibraryName, OwnedQualifiedName};
use crate::raw_ast;
use crate::raw_ast::{AttributeProvenance, RawDecl};
use crate::source_span::SourceSpan;
use crate::token::TokenSubkind;

impl<'node, 'src> Compiler<'node, 'src> {
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
            .find(|m| m.subkind == TokenSubkind::Resource && self.is_active(m.attributes.as_ref()))
        {
            self.reporter.fail(
                Error::ErrCannotSpecifyModifier(
                    flyweights::FlyStr::new(format!("{}", &"resource".to_string())),
                    flyweights::FlyStr::new(format!("{}", &"enum".to_string())),
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

        let subtype_name = if let Some(ref sc) = decl.subtype {
            if let raw_ast::LayoutParameter::Identifier(ref id) = sc.layout {
                id.to_string()
            } else {
                self.reporter
                    .fail(Error::ErrInvalidWrappedType, sc.element.span());
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
                    let fqn =
                        OwnedLibraryName::new(library_name.to_string()).with_declaration(&current);
                    if self.raw_decls.contains_key(&fqn) {
                        full_name = fqn.as_string();
                    } else if let Some((lib, name)) = current.rsplit_once('.') {
                        let dep_fqn = OwnedLibraryName::new(lib.to_string()).with_declaration(name);
                        if self.raw_decls.contains_key(&dep_fqn) {
                            full_name = dep_fqn.as_string();
                        } else {
                            full_name = fqn.as_string();
                        }
                    } else {
                        full_name = fqn.as_string();
                    }
                }
                if let Some(RawDecl::Alias(alias)) = self.raw_decls.get::<str>(full_name.as_ref())
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
                Error::ErrEnumTypeMustBeIntegralPrimitive(flyweights::FlyStr::new(
                    format!("{}", &subtype_name).into_boxed_str(),
                )),
                if let Some(sc) = &decl.subtype {
                    sc.element.start_token.span
                } else {
                    decl.name.as_ref().unwrap().element.span()
                },
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
            if !self.is_member_active(member.element.span().data.as_ptr() as usize) {
                continue;
            }
            let attributes = self.compile_attribute_list(&member.attributes);
            self.validate_constant(&member.value, &expected_type, library_name);
            let compiled_value = self.compile_constant(&member.value, library_name);

            let name_str = member.name.data().to_string();
            self.check_canonical_insert(
                &mut member_names,
                name_str.clone(),
                MemberKind::EnumValue,
                member.name.element.span(),
                member.attributes.as_ref().is_some_and(|attrs| {
                    attrs.attributes.iter().any(|a| {
                        a.name.data() == "available"
                            || a.provenance == AttributeProvenance::ModifierAvailability
                    })
                }),
            );

            if let Some(eval_val) = self.eval_constant_value(&member.value, library_name) {
                if eval_val == max_val_u64 {
                    let span = member.name.element.span();
                    let transmuted_span: SourceSpan<'src> = unsafe { std::mem::transmute(span) };
                    max_val_spans.push(transmuted_span);
                }

                if let Some(prev_name) = member_values.insert(eval_val, name_str.clone()) {
                    self.reporter.fail(
                        Error::ErrDuplicateMemberValue(
                            flyweights::FlyStr::new(format!("{}", &"enum")),
                            flyweights::FlyStr::new(format!("{}", &name_str)),
                            flyweights::FlyStr::new(format!("{}", &prev_name)),
                            flyweights::FlyStr::new(format!("{}", &prev_name)),
                        ),
                        member.name.element.span(),
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
                        );
                    }
                }

                // Try to parse value as u32 (assuming enum is uint32-compatible for now)
                // TODO: Handle signed enums and other types correctly.
                if let Some(literal) = &compiled_value.literal
                    && let Ok(val) = literal.value.trim_matches('"').parse::<u64>()
                {
                    maybe_unknown_value = Some(val);
                }
            } else if !strict && !max_val_spans.is_empty() && unknown_member_span.is_none() {
                // We will check at the end if unknown_member_span remains None
            }

            members.push(EnumMember {
                base: DeclBase {
                    name: member.name.data().to_string().into(),
                    location: self.get_location(&member.name.element),
                    deprecated: self.is_deprecated(member.attributes.as_deref()),
                    maybe_attributes: attributes,
                },
                value: compiled_value,
            });
        }

        if !strict && unknown_member_span.is_none() {
            for span in &max_val_spans {
                self.reporter.fail(
                    Error::ErrFlexibleEnumMemberWithMaxValue(flyweights::FlyStr::new(
                        format!("{}", &max_val_u64.to_string()).into_boxed_str(),
                    )),
                    *span,
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
            OwnedQualifiedName::from(full_name.clone()),
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
            );
        }

        if !strict && maybe_unknown_value.is_none() {
            maybe_unknown_value = match subtype_name.as_str() {
                "int8" => Some(((1u64 << 7) - 1) as u64),
                "uint8" => Some(u8::MAX as u64),
                "int16" => Some(((1u64 << 15) - 1) as u64),
                "uint16" => Some(u16::MAX as u64),
                "int32" => Some(((1u64 << 31) - 1) as u64),
                "uint32" => Some(u32::MAX as u64),
                // TODO: Handle u64 and signed types correctly (requires changing EnumDeclaration to support u64/i64)
                "int64" => Some(((1u64 << 63) - 1) as u64),
                _ => Some(u64::MAX),
            };
        }

        EnumDeclaration::new(
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
            subtype_name,
            members,
            strict,
            maybe_unknown_value,
        )
    }
}
