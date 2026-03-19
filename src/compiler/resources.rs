use crate::compiler::{CanonicalNames, Compiler, MemberKind};
use crate::diagnostics::Error;
use crate::flat_ast::{ExperimentalResourceDeclaration, PrimitiveSubtype, ResourceProperty, Type};
use crate::name::NamingContext;
use crate::names::OwnedQualifiedName;
use crate::raw_ast;
use crate::raw_ast::{AttributeProvenance, RawDecl};

impl<'node, 'src> Compiler<'node, 'src> {
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
                if let Some(RawDecl::Alias(a)) = self.raw_decls.get::<str>(curr.as_ref()) {
                    match &a.type_ctor.layout {
                        raw_ast::LayoutParameter::Identifier(inner_id) => {
                            let next = inner_id.to_string();
                            if next == "uint32" {
                                is_uint32 = true;
                                break;
                            }
                            curr = if next.contains('/') || self.shapes.contains_key::<str>(&next) {
                                next
                            } else {
                                let curr_fqn = OwnedQualifiedName::parse(&curr);
                                format!("{}/{}", curr_fqn.library(), next)
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
                Error::ErrResourceMustBeUint32Derived(format!("{}", &name)),
                decl.name.element.span(),
            );
        }

        if decl.properties.is_empty() {
            self.reporter
                .fail(Error::ErrMustHaveOneProperty, decl.element.span());
        }

        let mut has_subtype = false;

        for prop in &decl.properties {
            let prop_name = prop.name.data().to_string();

            self.check_canonical_insert(
                &mut property_names,
                prop_name.clone(),
                MemberKind::ResourceProperty,
                prop.name.element.span(),
                prop.attributes.as_ref().is_some_and(|attrs| {
                    attrs.attributes.iter().any(|a| {
                        a.name.data() == "available"
                            || a.provenance == AttributeProvenance::ModifierAvailability
                    })
                }),
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
                        Error::ErrResourceSubtypePropertyMustReferToEnum(format!("{}", &name)),
                        prop.name.element.span(),
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
                        if let Some(RawDecl::Alias(a)) = self.raw_decls.get::<str>(curr.as_ref()) {
                            match &a.type_ctor.layout {
                                raw_ast::LayoutParameter::Identifier(inner_id) => {
                                    let next = inner_id.to_string();
                                    if next == "uint32" {
                                        is_uint32_prop = true;
                                        break;
                                    }
                                    curr = if next.contains('/')
                                        || self.shapes.contains_key::<str>(&next)
                                    {
                                        next
                                    } else {
                                        let curr_fqn = OwnedQualifiedName::parse(&curr);
                                        format!("{}/{}", curr_fqn.library(), next)
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
                        Error::ErrResourceRightsPropertyMustReferToBits(format!("{}", &name)),
                        prop.name.element.span(),
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
                Error::ErrResourceMissingSubtypeProperty(format!("{}", &name)),
                decl.name.element.span(),
            );
        }

        ExperimentalResourceDeclaration::new(
            full_name.clone().into(),
            location,
            self.is_deprecated(decl.attributes.as_deref()),
            self.compile_attribute_list(&decl.attributes),
            type_obj,
            properties,
        )
    }
}
