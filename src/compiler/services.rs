use crate::compiler::{CanonicalNames, Compiler, MemberKind};
use crate::diagnostics::Error;
use crate::flat_ast::{DeclBase, ServiceDeclaration, ServiceMember, Type, TypeKind};
use crate::name::NamingContext;
use crate::raw_ast;
use crate::raw_ast::AttributeProvenance;

impl<'node, 'src> Compiler<'node, 'src> {
    pub fn compile_service(
        &mut self,
        name: &str,
        decl: &'node raw_ast::ServiceDeclaration<'src>,
        library_name: &str,
    ) -> ServiceDeclaration {
        let full_name = format!("{}/{}", library_name, name);
        let location = self.get_location(&decl.name.element);

        let mut members = vec![];
        let mut member_names = CanonicalNames::new();
        let mut associated_transport = String::new();
        let mut first_member_with_that_transport = String::new();

        for member in &decl.members {
            let ctx = NamingContext::create(name).enter_member(member.name.data());
            let type_obj = self.resolve_type(&member.type_ctor, library_name, Some(ctx));
            let member_name = member.name.data().to_string();
            self.check_canonical_insert(
                &mut member_names,
                member_name.clone(),
                MemberKind::ServiceMember,
                member.name.element.span(),
                member.attributes.as_ref().is_some_and(|attrs| {
                    attrs.attributes.iter().any(|a| {
                        a.name.data() == "available"
                            || a.provenance == AttributeProvenance::ModifierAvailability
                    })
                }),
            );
            let attributes = self.compile_attribute_list(&member.attributes);

            if type_obj.kind() != TypeKind::Endpoint {
                self.reporter.fail(
                    Error::ErrOnlyClientEndsInServices,
                    member.name.element.span(),
                );
            } else if let Type::Endpoint(e) = &type_obj {
                if let Some(role) = &e.role
                    && role != "client"
                {
                    self.reporter.fail(
                        Error::ErrOnlyClientEndsInServices,
                        member.name.element.span(),
                    );
                }
                if e.nullable {
                    self.reporter
                        .fail(Error::ErrOptionalServiceMember, member.name.element.span());
                }
                let transport = e.protocol_transport.as_deref().unwrap_or("Channel");
                if associated_transport.is_empty() {
                    associated_transport = transport.to_string();
                    first_member_with_that_transport = member_name.clone();
                } else if associated_transport != transport {
                    self.reporter.fail(
                        Error::ErrMismatchedTransportInServices(
                            format!("{}", &member_name),
                            format!("{}", &transport),
                            format!("{}", &first_member_with_that_transport.as_str()),
                            format!("{}", &associated_transport.as_str()),
                        ),
                        member.name.element.span(),
                    );
                }
            }

            members.push(ServiceMember {
                type_: type_obj,
                base: DeclBase {
                    name: member_name.into(),
                    location: self.get_location(&member.name.element),
                    deprecated: self.is_deprecated(member.attributes.as_deref()),
                    maybe_attributes: attributes,
                },
            });
        }

        ServiceDeclaration::new(
            full_name.clone().into(),
            location,
            self.is_deprecated(decl.attributes.as_deref()),
            self.compile_attribute_list(&decl.attributes),
            members,
        )
    }
}
