use crate::compiler::{CanonicalNames, Compiler, DeclarationKind, MemberKind};
use crate::diagnostics::Error;
use crate::experimental_flags::ExperimentalFlag;
use crate::flat_ast::{
    Decl, DeclBase, DependencyDeclaration, EnumDeclaration, Openness, PrimitiveSubtype, ProtocolCompose,
    ProtocolDeclaration, ProtocolMethod, StructDeclaration, Type, TypeKind, TypeShape,
    UnionDeclaration, UnionMember,
};
use crate::name::NamingContext;
use crate::names::{OwnedLibraryName, OwnedQualifiedName};
use crate::raw_ast;
use crate::raw_ast::{AttributeProvenance, RawDecl};
use crate::token::TokenSubkind;
use sha2::{Digest, Sha256};

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

impl<'node, 'src> Compiler<'node, 'src> {
    pub fn compile_protocol(
        &mut self,
        short_name: &str,
        decl: &'node raw_ast::ProtocolDeclaration<'src>,
        library_name: &str,
    ) -> ProtocolDeclaration {
        let name = format!("{}/{}", library_name, short_name);

        let is_strict = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Strict && self.is_active(m.attributes.as_ref()));
        let is_flexible = decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Flexible && self.is_active(m.attributes.as_ref()));
        if is_strict && decl.methods.is_empty() {
            self.reporter
                .fail(Error::ErrMustHaveOneMember, decl.name.element.span(), &[]);
        }
        if is_flexible && decl.methods.is_empty() {
            self.reporter
                .fail(Error::ErrMustHaveOneMember, decl.name.element.span(), &[]);
        }

        let mut methods = vec![];
        let mut method_names = CanonicalNames::new();
        let has_no_resource = decl.attributes.as_ref().is_some_and(|attrs| {
            attrs
                .attributes
                .iter()
                .any(|a| a.name.data() == "no_resource")
        });
        let openness = if decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Ajar && self.is_active(m.attributes.as_ref()))
        {
            Openness::Ajar
        } else if decl
            .modifiers
            .iter()
            .any(|m| m.subkind == TokenSubkind::Closed && self.is_active(m.attributes.as_ref()))
        {
            Openness::Closed
        } else {
            Openness::Open
        };

        let mut compiled_composed = vec![];
        for composed in &decl.composed_protocols {
            let mut composed_name = composed.protocol_name.to_string();
            if let Some((lib_prefix, type_name)) = composed_name.rsplit_once('.') {
                let mut actual_lib = lib_prefix.to_string();
                if let Some(import) = self.library_imports.get(lib_prefix) {
                    self.used_imports
                        .borrow_mut()
                        .insert(OwnedLibraryName::new(lib_prefix.to_string()));
                    actual_lib = import.using_path.to_string();
                }
                composed_name = format!("{}/{}", actual_lib, type_name);
            }
            let full_composed_name = if composed_name.contains('/') {
                composed_name.clone()
            } else {
                format!("{}/{}", library_name, composed_name)
            };

            self.compile_decl_by_name(&full_composed_name);

            if has_no_resource {
                let mut composed_has_no_resource = false;
                if let Some(p) = self
                    .declarations
                    .protocols()
                    .find(|&p| p.name == full_composed_name)
                {
                    composed_has_no_resource =
                        p.maybe_attributes.iter().any(|a| a.name == "no_resource");
                } else if let Some(RawDecl::Protocol(p)) =
                    self.raw_decls.get::<str>(full_composed_name.as_ref())
                {
                    if let Some(attrs) = p.attributes.as_ref() {
                        composed_has_no_resource = attrs
                            .attributes
                            .iter()
                            .any(|a| a.name.data() == "no_resource");
                    }
                } else if let Some(p) = std::iter::empty::<&ProtocolDeclaration>()
                    .find(|&p| p.name == full_composed_name)
                {
                    composed_has_no_resource =
                        p.maybe_attributes.iter().any(|a| a.name == "no_resource");
                }
                if !composed_has_no_resource {
                    self.reporter.fail(
                        Error::ErrNoResourceForbidsCompose,
                        composed.protocol_name.element.span(),
                        &[&short_name, &composed_name],
                    );
                }
            }

            let mut composed_openness = Openness::Open;
            let mut parent_methods = vec![];
            if let Some(p) = self
                .declarations
                .protocols()
                .find(|&p| p.name == full_composed_name)
            {
                composed_openness = p.openness;
                parent_methods = p.methods.clone();
            } else if let Some(RawDecl::Protocol(p)) =
                self.raw_decls.get::<str>(full_composed_name.as_ref())
            {
                if p.modifiers.iter().any(|m| {
                    m.subkind == TokenSubkind::Ajar && self.is_active(m.attributes.as_ref())
                }) {
                    composed_openness = Openness::Ajar;
                } else if p.modifiers.iter().any(|m| {
                    m.subkind == TokenSubkind::Closed && self.is_active(m.attributes.as_ref())
                }) {
                    composed_openness = Openness::Closed;
                }
            }

            let valid = match openness {
                Openness::Open => true,
                Openness::Ajar => {
                    composed_openness == Openness::Ajar || composed_openness == Openness::Closed
                }
                Openness::Closed => composed_openness == Openness::Closed,
            };

            if !valid {
                self.reporter.fail(
                    Error::ErrComposedProtocolTooOpen,
                    composed.element.span(),
                    &[
                        &openness.to_string(),
                        &decl.name.data(),
                        &composed_openness.to_string(),
                        &full_composed_name,
                    ],
                );
            }

            for mut pm in parent_methods {
                pm.is_composed = true;
                methods.push(pm);
            }
            compiled_composed.push(ProtocolCompose {
                base: DeclBase {
                    name: full_composed_name.into(),
                    location: self.get_location(&composed.protocol_name.element),
                    deprecated: self.is_deprecated(composed.attributes.as_deref()),
                    maybe_attributes: self.compile_attribute_list(&composed.attributes),
                },
            });
        }

        for m in &decl.methods {
            if !self.is_member_active(m.element.span().data.as_ptr() as usize) {
                continue;
            }
            if has_no_resource {
                for l in [&m.request_payload, &m.response_payload, &m.error_payload]
                    .into_iter()
                    .flatten()
                {
                    let mut current_layout = Some(l);
                    let mut modifiers = None;
                    while let Some(cl) = current_layout {
                        match cl {
                            raw_ast::Layout::Struct(s) => {
                                modifiers = Some(&s.modifiers);
                                break;
                            }
                            raw_ast::Layout::Table(t) => {
                                modifiers = Some(&t.modifiers);
                                break;
                            }
                            raw_ast::Layout::Union(u) => {
                                modifiers = Some(&u.modifiers);
                                break;
                            }
                            raw_ast::Layout::TypeConstructor(tc) => {
                                if let raw_ast::LayoutParameter::Inline(inline) = &tc.layout {
                                    current_layout = Some(&**inline);
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                    if let Some(mods) = modifiers
                        && let Some(res_mod) =
                            mods.iter().find(|mo| mo.subkind == TokenSubkind::Resource)
                    {
                        self.reporter.fail(
                            Error::ErrResourceForbiddenHere,
                            res_mod.element.span(),
                            &[],
                        );
                    }
                }
            }

            let mut has_explicit_strict = false;
            let mut has_explicit_flexible = false;
            for modifier in &m.modifiers {
                if !self.is_active(modifier.attributes.as_ref()) {
                    continue;
                }
                match modifier.subkind {
                    TokenSubkind::Strict => has_explicit_strict = true,
                    TokenSubkind::Flexible => has_explicit_flexible = true,
                    _ => {
                        self.reporter.fail(
                            Error::ErrCannotSpecifyModifier,
                            modifier.element.span(),
                            &[&modifier.element.span().data, &"method"],
                        );
                    }
                }
            }

            let mut is_method_flexible = false;
            let two_way = m.has_request && m.has_response;

            if has_explicit_flexible
                || (!has_explicit_strict
                    && (openness == Openness::Open || (openness == Openness::Ajar && !two_way)))
            {
                is_method_flexible = true;
            }

            if is_method_flexible && two_way && openness != Openness::Open {
                self.reporter.fail(
                    Error::ErrFlexibleTwoWayMethodRequiresOpenProtocol,
                    m.name.element.span(),
                    &[&openness.to_string()],
                );
            } else if is_method_flexible && !two_way && openness == Openness::Closed {
                self.reporter.fail(
                    Error::ErrFlexibleOneWayMethodInClosedProtocol,
                    m.name.element.span(),
                    &[&if !m.has_request && m.has_response {
                        "event"
                    } else {
                        "one-way method"
                    }],
                );
            }

            self.check_canonical_insert(
                &mut method_names,
                m.name.data().to_string(),
                MemberKind::Method,
                m.name.element.span(),
                m.attributes.as_ref().is_some_and(|attrs| {
                    attrs.attributes.iter().any(|a| {
                        a.name.data() == "available"
                            || a.provenance == AttributeProvenance::ModifierAvailability
                    })
                }),
            );
            if m.has_error && !m.has_response && m.has_request {
                self.reporter
                    .fail(Error::ErrUnexpectedToken, m.name.element.span(), &[]);
            }
            let has_request = m.has_request;
            let maybe_request_payload = if let Some(ref l) = m.request_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        let ctx = NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let resolved_type = self.resolve_type(tc, library_name, Some(ctx));

                        let is_allowed = if resolved_type.kind() != TypeKind::Identifier {
                            false
                        } else if let Some(kind) = resolved_type
                            .identifier()
                            .and_then(|id| self.decl_kinds.get::<str>(id.as_ref()).copied())
                        {
                            kind == DeclarationKind::Struct
                                || kind == DeclarationKind::Table
                                || kind == DeclarationKind::Union
                                || kind == DeclarationKind::Overlay
                        } else if let Some(id) = &resolved_type.identifier() {
                            self.declarations.structs().any(|d| &d.name == id)
                                || self.declarations.tables().any(|d| &d.name == id)
                                || self.declarations.unions().any(|d| &d.name == id)
                        } else {
                            false
                        };

                        if !is_allowed {
                            self.reporter.fail(
                                Error::ErrInvalidMethodPayloadLayoutClass,
                                tc.element.span(),
                                &[&"provided type"],
                            );
                        }

                        Some(resolved_type)
                    }
                    raw_ast::Layout::Struct(s) => {
                        if s.members.is_empty() {
                            self.reporter.fail(
                                Error::ErrEmptyPayloadStructs,
                                s.element.span(),
                                &[],
                            );
                        }
                        for sm in &s.members {
                            if sm.default_value.is_some() {
                                self.reporter.fail(
                                    Error::ErrPayloadStructHasDefaultMembers,
                                    sm.name.element.span(),
                                    &[&sm.name.data()],
                                );
                            }
                        }
                        let ctx = NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains::<str>(full_synth.as_str()) {
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        } else {
                            let compiled = self.compile_struct(
                                &synth_name,
                                s,
                                library_name,
                                None,
                                Some(ctx.clone()),
                                None,
                            );
                            if library_name == self.library_name.to_string() {
                                self.declarations.push(Decl::Struct(compiled));
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls
                                    .insert(OwnedQualifiedName::from(full_synth.clone()));
                            } else {
                                self.declarations.push(Decl::Struct(compiled));
                            }
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        };
                        Some(Type::identifier_type(Some(full_synth), false, shape, false))
                    }
                    raw_ast::Layout::Table(t) => {
                        let ctx = NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains::<str>(full_synth.as_str()) {
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        } else {
                            let compiled = self.compile_table(
                                &synth_name,
                                t,
                                library_name,
                                None,
                                None,
                                Some(ctx.clone()),
                            );
                            self.declarations.push(Decl::Table(compiled));
                            if library_name == self.library_name.to_string() {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls
                                    .insert(OwnedQualifiedName::from(full_synth.clone()));
                            }
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        };
                        Some(Type::identifier_type(Some(full_synth), false, shape, false))
                    }
                    raw_ast::Layout::Union(u) => {
                        let ctx = NamingContext::create(decl.name.element.span())
                            .enter_request(m.name.element.span());
                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains::<str>(full_synth.as_str()) {
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        } else {
                            let compiled = self.compile_union(
                                &synth_name,
                                u,
                                library_name,
                                None,
                                None,
                                Some(ctx.clone()),
                            );
                            if library_name == self.library_name.to_string() {
                                if u.is_overlay {
                                    self.declarations.push(Decl::Overlay(compiled));
                                } else {
                                    self.declarations.push(Decl::Union(compiled));
                                }
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls
                                    .insert(OwnedQualifiedName::from(full_synth.clone()));
                            }
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        };
                        Some(Type::identifier_type(Some(full_synth), false, shape, false))
                    }
                    _ => {
                        // primitive or other inline layout
                        self.reporter.fail(
                            Error::ErrInvalidMethodPayloadLayoutClass,
                            m.name.element.span(),
                            &[&"provided type"],
                        );
                        None
                    }
                }
            } else {
                None
            };

            let maybe_response_payload = if let Some(ref l) = m.response_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };
                        if m.has_error || (is_method_flexible && m.has_request && m.has_response) {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                short_name,
                                m.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                short_name,
                                m.name.data()
                            ));
                        }

                        let resolved_type = self.resolve_type(tc, library_name, Some(ctx));

                        let is_allowed = if resolved_type.kind() != TypeKind::Identifier {
                            false
                        } else if let Some(kind) = resolved_type
                            .identifier()
                            .and_then(|id| self.decl_kinds.get::<str>(id.as_ref()).copied())
                        {
                            kind == DeclarationKind::Struct
                                || kind == DeclarationKind::Table
                                || kind == DeclarationKind::Union
                                || kind == DeclarationKind::Overlay
                        } else if let Some(id) = &resolved_type.identifier() {
                            self.declarations.structs().any(|d| &d.name == id)
                                || self.declarations.tables().any(|d| &d.name == id)
                                || self.declarations.unions().any(|d| &d.name == id)
                        } else {
                            false
                        };

                        if !is_allowed {
                            self.reporter.fail(
                                Error::ErrInvalidMethodPayloadLayoutClass,
                                tc.element.span(),
                                &[&"provided type"],
                            );
                        }

                        Some(resolved_type)
                    }
                    raw_ast::Layout::Struct(s) => {
                        if s.members.is_empty() {
                            self.reporter.fail(
                                Error::ErrEmptyPayloadStructs,
                                s.element.span(),
                                &[],
                            );
                        }
                        for sm in &s.members {
                            if sm.default_value.is_some() {
                                self.reporter.fail(
                                    Error::ErrPayloadStructHasDefaultMembers,
                                    sm.name.element.span(),
                                    &[&sm.name.data()],
                                );
                            }
                        }
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };

                        if m.has_error || (is_method_flexible && m.has_request && m.has_response) {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                short_name,
                                m.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                short_name,
                                m.name.data()
                            ));
                        }

                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains::<str>(full_synth.as_str()) {
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        } else {
                            let compiled = self.compile_struct(
                                &synth_name,
                                s,
                                library_name,
                                None,
                                Some(ctx.clone()),
                                None,
                            );
                            self.declarations.push(Decl::Struct(compiled));
                            if library_name == self.library_name.to_string() {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls
                                    .insert(OwnedQualifiedName::from(full_synth.clone()));
                            }
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        };
                        Some(Type::identifier_type(Some(full_synth), false, shape, false))
                    }
                    raw_ast::Layout::Table(t) => {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };

                        if m.has_error || (is_method_flexible && m.has_request && m.has_response) {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                short_name,
                                m.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                short_name,
                                m.name.data()
                            ));
                        }

                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains::<str>(full_synth.as_str()) {
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        } else {
                            let compiled = self.compile_table(
                                &synth_name,
                                t,
                                library_name,
                                None,
                                None,
                                Some(ctx.clone()),
                            );
                            self.declarations.push(Decl::Table(compiled));
                            if library_name == self.library_name.to_string() {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls
                                    .insert(OwnedQualifiedName::from(full_synth.clone()));
                            }
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        };
                        Some(Type::identifier_type(Some(full_synth), false, shape, false))
                    }
                    raw_ast::Layout::Union(u) => {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let mut ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };

                        if m.has_error || (is_method_flexible && m.has_request && m.has_response) {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                short_name,
                                m.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                short_name,
                                m.name.data()
                            ));
                        }

                        let synth_name = ctx.flattened_name().to_string();
                        let full_synth = format!("{}/{}", library_name, synth_name);

                        let shape = if self.compiled_decls.contains::<str>(full_synth.as_str()) {
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        } else {
                            let compiled = self.compile_union(
                                &synth_name,
                                u,
                                library_name,
                                None,
                                None,
                                Some(ctx.clone()),
                            );
                            if u.is_overlay {
                                self.declarations.push(Decl::Overlay(compiled));
                            } else {
                                self.declarations.push(Decl::Union(compiled));
                            }
                            if library_name == self.library_name.to_string() {
                                self.declaration_order.push(full_synth.clone());
                                self.compiled_decls
                                    .insert(OwnedQualifiedName::from(full_synth.clone()));
                            }
                            self.shapes
                                .get::<str>(full_synth.as_str())
                                .cloned()
                                .unwrap()
                        };
                        Some(Type::identifier_type(Some(full_synth), false, shape, false))
                    }
                    _ => {
                        self.reporter.fail(
                            Error::ErrInvalidMethodPayloadLayoutClass,
                            m.name.element.span(),
                            &[&"provided type"],
                        );
                        None
                    }
                }
            } else {
                None
            };

            let has_response = m.has_response;
            let two_way = has_request && has_response;
            let needs_result_union = m.has_error || (is_method_flexible && two_way);
            let mut maybe_response_success_type = maybe_response_payload.clone();

            let maybe_response_err_type = if let Some(ref l) = m.error_payload {
                match l {
                    raw_ast::Layout::TypeConstructor(tc) => {
                        let p_ctx = NamingContext::create(decl.name.element.span());
                        let ctx = if !m.has_request && !m.has_error {
                            p_ctx.enter_event(m.name.element.span())
                        } else {
                            p_ctx.enter_response(m.name.element.span())
                        };
                        ctx.set_name_override(format!("{}_{}_Result", short_name, m.name.data()));
                        let ctx = ctx.enter_member("err");
                        ctx.set_name_override(format!("{}_{}_Error", short_name, m.name.data()));
                        let err_type_resolved = self.resolve_type(tc, library_name, Some(ctx));

                        let mut is_valid_error_type = false;
                        if let Type::Primitive(p) = &err_type_resolved {
                            if p.subtype == PrimitiveSubtype::Int32
                                || p.subtype == PrimitiveSubtype::Uint32
                            {
                                is_valid_error_type = true;
                            }
                        } else if err_type_resolved.kind() == TypeKind::Identifier
                            && let Some(id) = &err_type_resolved.identifier()
                        {
                            if let Some(e_decl) = self.declarations.enums().find(|&e| e.name == *id)
                            {
                                if e_decl.type_ == "int32" || e_decl.type_ == "uint32" {
                                    is_valid_error_type = true;
                                }
                            } else if let Some(e_decl) =
                                std::iter::empty::<&EnumDeclaration>().find(|&e| e.name == *id)
                                && (e_decl.type_ == "int32" || e_decl.type_ == "uint32")
                            {
                                is_valid_error_type = true;
                            }
                        }

                        if !is_valid_error_type
                            && !self
                                .experimental_flags
                                .is_enabled(ExperimentalFlag::AllowArbitraryErrorTypes)
                        {
                            self.reporter
                                .fail(Error::ErrInvalidErrorType, tc.element.span(), &[]);
                        }

                        Some(err_type_resolved)
                    }
                    _ => None,
                }
            } else {
                None
            };

            let maybe_response_payload = if needs_result_union {
                let success_type = if let Some(ref succ) = maybe_response_success_type {
                    succ.clone()
                } else {
                    let p_ctx = NamingContext::create(decl.name.element.span());
                    let mut ctx = p_ctx.enter_response(m.name.element.span());
                    ctx = ctx.enter_member("response");
                    ctx.set_name_override(format!("{}_{}_Response", short_name, m.name.data()));

                    let synth_name = ctx.flattened_name().to_string();
                    let full_synth = format!("{}/{}", library_name, synth_name);

                    let shape = if self.compiled_decls.contains::<str>(full_synth.as_str()) {
                        self.shapes
                            .get::<str>(full_synth.as_str())
                            .cloned()
                            .unwrap()
                    } else {
                        let shape = TypeShape {
                            inline_size: 1,
                            alignment: 1,
                            depth: 0,
                            max_handles: 0,
                            max_out_of_line: 0,
                            has_padding: false,
                            has_flexible_envelope: false,
                        };

                        let loc = if let Some(elem) = &m.response_param_element {
                            self.get_location(elem)
                        } else if let Some(tok) = &m.error_token {
                            self.get_location(&raw_ast::SourceElement::new(
                                tok.clone(),
                                tok.clone(),
                            ))
                        } else {
                            self.get_location(&m.name.element)
                        };
                        let decl = StructDeclaration {
                            base: DeclBase {
                                name: full_synth.clone().into(),
                                location: loc,
                                deprecated: false,
                                maybe_attributes: vec![],
                            },
                            naming_context: vec![
                                short_name.to_string(),
                                m.name.data().to_string(),
                                "Response".to_string(),
                                "response".to_string(),
                            ],
                            members: vec![],
                            resource: false,
                            is_empty_success_struct: true,
                            type_shape: shape.clone(),
                        };
                        self.declarations.push(Decl::Struct(decl));
                        if library_name == self.library_name.to_string() {
                            self.declaration_order.push(full_synth.clone());
                            self.compiled_decls
                                .insert(OwnedQualifiedName::from(full_synth.clone()));
                        } else {
                            self.dependency_declarations
                                .entry(crate::names::OwnedLibraryName::new(library_name.to_string()))
                                .or_default()
                                .insert(
                                    full_synth.clone(),
                                    DependencyDeclaration {
                                        kind: crate::flat_ast::DeclarationKind::Struct,
                                        type_shape: Some(shape.clone()),
                                        resource: Some(false),
                                    },
                                );
                        }
                        self.shapes
                            .insert(OwnedQualifiedName::from(full_synth.clone()), shape.clone());
                        shape
                    };
                    let typ = Type::identifier_type(Some(full_synth.clone()), false, shape, false);
                    maybe_response_success_type = Some(typ.clone());
                    typ
                };

                // Synthesize Union
                let synth_union_name = format!("{}_{}_Result", short_name, m.name.data());
                let full_synth_union = format!("{}/{}", library_name, synth_union_name);

                let mut union_out_of_line = 0;
                let mut union_has_padding = false;
                let mut union_handles = 0;
                let mut union_depth = 0;
                let mut union_has_flexible_envelope = false;
                let mut member_types = vec![&success_type];
                if let Some(err_type) = &maybe_response_err_type {
                    member_types.push(err_type);
                }
                for t in member_types {
                    let shape = &t.type_shape;
                    let inlined = shape.inline_size <= 4;
                    let padding = if inlined {
                        (4 - (shape.inline_size % 4)) % 4
                    } else {
                        (8 - (shape.inline_size % 8)) % 8
                    };
                    union_has_padding = union_has_padding || shape.has_padding || padding != 0;

                    let env_max_out_of_line = shape.max_out_of_line.saturating_add(if inlined {
                        0
                    } else {
                        shape.inline_size.saturating_add(padding)
                    });
                    if env_max_out_of_line > union_out_of_line {
                        union_out_of_line = env_max_out_of_line;
                    }
                    if shape.max_handles > union_handles {
                        union_handles = shape.max_handles;
                    }
                    if shape.depth > union_depth {
                        union_depth = shape.depth;
                    }
                    if shape.has_flexible_envelope {
                        union_has_flexible_envelope = true;
                    }
                }

                let union_shape = TypeShape {
                    inline_size: 16,
                    alignment: 8,
                    depth: union_depth.saturating_add(1),
                    max_handles: union_handles,
                    max_out_of_line: union_out_of_line,
                    has_padding: union_has_padding,
                    has_flexible_envelope: union_has_flexible_envelope,
                };

                let union_loc = if let Some(elem) = &m.response_param_element {
                    self.get_location(elem)
                } else if let Some(tok) = &m.error_token {
                    self.get_location(&raw_ast::SourceElement::new(tok.clone(), tok.clone()))
                } else {
                    self.get_location(&m.name.element)
                };

                let response_loc = self.generated_location("response");
                let err_loc = self.generated_location("err");
                let _framework_err_loc = self.generated_location("framework_err");
                let _strict_loc = self.generated_location("strict");

                let mut union_members = vec![UnionMember {
                    ordinal: 1,
                    reserved: None,
                    type_: Some(success_type.clone()),
                    experimental_maybe_from_alias: None,
                    base: DeclBase {
                        name: "response".to_string().into(),
                        location: response_loc,
                        deprecated: false,
                        maybe_attributes: vec![],
                    },
                }];

                if let Some(err_type) = maybe_response_err_type.clone() {
                    union_members.push(UnionMember {
                        ordinal: 2,
                        reserved: None,
                        type_: Some(err_type.clone()),
                        experimental_maybe_from_alias: None,
                        base: DeclBase {
                            name: "err".to_string().into(),
                            location: err_loc,
                            deprecated: false,
                            maybe_attributes: vec![],
                        },
                    });
                }

                if is_method_flexible {
                    union_members.push(UnionMember {
                        experimental_maybe_from_alias: None,
                        ordinal: 3,
                        reserved: None,
                        type_: Some(Type::internal("framework_error".to_string())),
                        base: DeclBase {
                            name: "framework_err".to_string().into(),
                            location: _framework_err_loc,
                            deprecated: false,
                            maybe_attributes: vec![],
                        },
                    });
                }

                let union_decl = UnionDeclaration {
                    base: DeclBase {
                        name: full_synth_union.clone().into(),
                        location: union_loc,
                        deprecated: false,
                        maybe_attributes: vec![],
                    },
                    naming_context: vec![
                        short_name.to_string(),
                        m.name.data().to_string(),
                        "Response".to_string(),
                    ],
                    members: union_members,
                    strict: true,
                    resource: union_handles > 0,
                    is_result: Some(true),
                    type_shape: union_shape.clone(),
                };
                self.declarations.push(Decl::Union(union_decl));
                if library_name == self.library_name.to_string() {
                    self.compiled_decls
                        .insert(OwnedQualifiedName::from(full_synth_union.clone()));
                } else {
                    self.dependency_declarations
                        .entry(crate::names::OwnedLibraryName::new(library_name.to_string()))
                        .or_default()
                        .insert(
                            full_synth_union.clone(),
                            DependencyDeclaration {
                                kind: crate::flat_ast::DeclarationKind::Union,
                                type_shape: Some(union_shape.clone()),
                                resource: Some(union_handles > 0),
                            },
                        );
                }

                Some(Type::identifier_type(
                    Some(full_synth_union.clone()),
                    false,
                    union_shape,
                    false,
                ))
            } else {
                maybe_response_payload.clone()
            };

            if !needs_result_union {
                maybe_response_success_type = None;
            }

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
                    if attr.name.data() == "selector"
                        && let Some(arg) = attr.args.first()
                        && let raw_ast::Constant::Literal(ref l) = arg.value
                        && l.literal.kind == raw_ast::LiteralKind::String
                    {
                        // The string literal includes quotes, but wait, usually we want
                        // to strip them if the parser leaves them. Let's just use the value.
                        // Our scanner keeps quotes? Let's assume we need to trim '\"'
                        selector = l.literal.value.trim_matches('"').to_string();
                    }
                }
            }

            let ordinal = compute_method_ordinal(&selector);

            methods.push(ProtocolMethod {
                base: DeclBase {
                    name: m.name.data().to_string().into(),
                    location: self.get_location(&m.name.element),
                    deprecated: self.is_deprecated(m.attributes.as_deref()),
                    maybe_attributes: self.compile_attribute_list(&m.attributes),
                },
                kind,
                ordinal,
                strict: !is_method_flexible,
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

        let mut implementation_locations = None;
        if let Some(attributes) = decl.attributes.as_deref() {
            for attr in &attributes.attributes {
                if attr.name.data() == "discoverable" {
                    let mut has_args = false;
                    let mut client_locs = vec!["platform".to_string(), "external".to_string()];
                    let mut server_locs = vec!["platform".to_string(), "external".to_string()];

                    for arg in &attr.args {
                        let arg_name = arg.name.as_ref().map(|n| n.data());
                        if arg_name == Some("client") {
                            has_args = true;
                            if let raw_ast::Constant::Literal(lit) = &arg.value {
                                client_locs = vec![lit.literal.value.trim_matches('"').to_string()];
                            }
                        } else if arg_name == Some("server") {
                            has_args = true;
                            if let raw_ast::Constant::Literal(lit) = &arg.value {
                                server_locs = vec![lit.literal.value.trim_matches('"').to_string()];
                            }
                        }
                    }

                    if has_args {
                        let mut map = std::collections::BTreeMap::new();
                        map.insert("client".to_string(), client_locs);
                        map.insert("server".to_string(), server_locs);
                        implementation_locations = Some(map);
                    }
                }
            }
        }

        ProtocolDeclaration::new(
            name.into(),
            self.get_location(&decl.name.element),
            self.is_deprecated(decl.attributes.as_deref()),
            self.compile_attribute_list(&decl.attributes),
            openness,
            compiled_composed,
            methods,
            implementation_locations,
        )
    }
}
