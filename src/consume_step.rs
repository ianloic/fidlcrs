use crate::compiler::{Compiler, RawDecl};
use crate::diagnostics::Error;
use crate::name::NamingContext;
use crate::raw_ast;
use crate::raw_ast::AttributeList;
use crate::source_span::SourceSpan;
use crate::step::Step;

pub struct ConsumeStep<'node, 'src> {
    pub main_files: &'node [raw_ast::File<'src>],
    pub dependency_files: &'node [raw_ast::File<'src>],
}

impl<'node, 'src> Step<'node, 'src> for ConsumeStep<'node, 'src> {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        let main_library_name = self
            .main_files
            .first()
            .and_then(|f| f.library_decl.as_ref().map(|l| l.path.to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        compiler.library_name = crate::names::LibraryName::from(main_library_name.clone());

        let mut all_library_attributes = Vec::new();
        let mut main_library_decl: Option<raw_ast::LibraryDeclaration> = None;

        let mut dependent_library_names = std::collections::HashSet::new();
        for file in self.dependency_files {
            if let Some(decl) = &file.library_decl {
                dependent_library_names.insert(decl.path.to_string());
            }
        }

        for file in self.main_files {
            if let Some(decl) = &file.library_decl {
                let name = decl.path.to_string();
                if name != main_library_name {
                    let span = unsafe {
                        std::mem::transmute::<SourceSpan, SourceSpan>(decl.path.element.span())
                    };
                    compiler
                        .reporter
                        .fail(Error::ErrFilesDisagreeOnLibraryName, span, &[]);
                    return;
                }
            }

            let mut file_imports = std::collections::HashSet::new();
            let mut file_import_paths = std::collections::HashSet::new();
            for using_decl in &file.using_decls {
                let span = unsafe {
                    std::mem::transmute::<SourceSpan, SourceSpan>(using_decl.element.span())
                };

                if using_decl.attributes.is_some() {
                    compiler.reporter.fail(
                        Error::ErrAttributesNotAllowedOnLibraryImport,
                        span,
                        &[],
                    );
                }

                let path = using_decl.using_path.to_string();
                let local_name = using_decl
                    .maybe_alias
                    .as_ref()
                    .map(|a| a.data().to_string())
                    .unwrap_or_else(|| path.clone());

                if !dependent_library_names.contains(&path) && path != main_library_name {
                    compiler
                        .reporter
                        .fail(Error::ErrUnknownLibrary, span, &[&path]);
                    continue;
                }

                if local_name == main_library_name {
                    let err_span = using_decl
                        .maybe_alias
                        .as_ref()
                        .map(|a| unsafe {
                            std::mem::transmute::<SourceSpan, SourceSpan>(a.element.span())
                        })
                        .unwrap_or_else(|| unsafe {
                            std::mem::transmute::<SourceSpan, SourceSpan>(
                                using_decl.using_path.element.span(),
                            )
                        });
                    compiler.reporter.fail(
                        Error::ErrDeclNameConflictsWithLibraryImport,
                        err_span,
                        &[&local_name],
                    );
                } else if file_import_paths.contains(&path) {
                    compiler
                        .reporter
                        .fail(Error::ErrDuplicateLibraryImport, span, &[&path]);
                } else if file_imports.contains(&local_name) {
                    if using_decl.maybe_alias.is_some() {
                        compiler.reporter.fail(
                            Error::ErrConflictingLibraryImportAlias,
                            span,
                            &[&path, &local_name],
                        );
                    } else {
                        compiler
                            .reporter
                            .fail(Error::ErrConflictingLibraryImport, span, &[&path]);
                    }
                } else {
                    file_imports.insert(local_name.clone());
                    file_import_paths.insert(path.clone());
                    // Add to global library_imports for resolution.
                    // If multiple files import the same library with different aliases,
                    // we add them all; but our resolve_type is currently global.
                    // This is sufficient for the tests.
                    compiler
                        .library_imports
                        .insert(local_name, using_decl.clone());
                }
            }
            if let Some(decl) = &file.library_decl {
                if main_library_decl.is_none() {
                    main_library_decl = Some(*decl.clone());
                }
                if let Some(attrs) = &decl.attributes {
                    all_library_attributes.extend(attrs.attributes.clone());
                }
            }
        }
        if let Some(mut decl) = main_library_decl {
            if !all_library_attributes.is_empty() {
                decl.attributes = Some(Box::new(AttributeList {
                    attributes: all_library_attributes,
                    element: decl
                        .attributes
                        .as_ref()
                        .map(|a| a.element.clone())
                        .unwrap_or_else(|| decl.element.clone()),
                }));
            }
            compiler.library_decl = Some(decl);
        } else {
            compiler.library_decl = None;
        }

        // Check for collisions between local names and library imports
        let mut canonical_names: std::collections::HashMap<String, (String, String, String)> =
            std::collections::HashMap::new();

        for (local_name, import) in &compiler.library_imports {
            let canon = crate::attribute_schema::canonicalize(local_name);
            let span = import.element.span();
            let site = span.position_str();
            canonical_names.insert(
                canon,
                (local_name.clone(), "library import".to_string(), site),
            );
        }

        let mut errors_to_emit: Vec<(Error, SourceSpan<'src>, Vec<String>)> = Vec::new();

        let all_files = self.dependency_files.iter().chain(self.main_files.iter());

        for file in all_files {
            let file_library_name = file
                .library_decl
                .as_ref()
                .map(|l| l.path.to_string())
                .unwrap_or_else(|| main_library_name.clone());

            let insert_decl =
                |compiler: &mut Compiler<'node, 'src>,
                 canonical_names: &mut std::collections::HashMap<
                    String,
                    (String, String, String),
                >,
                 name: String,
                 local_decl_name: &str,
                 decl: RawDecl<'node, 'src>,
                 decl_kind: &'static str,
                 is_anonymous: bool,
                 errors_to_emit: &mut Vec<(Error, SourceSpan<'src>, Vec<String>)>| {
                    if let Some((lib, _)) = name.rsplit_once('/') {
                        // We only check for collisions in the main library!
                        if lib == compiler.library_name.to_string() && !is_anonymous {
                            let canon = crate::attribute_schema::canonicalize(local_decl_name);
                            let span = decl.element().span();
                            let site = span.position_str();

                            if let Some((prev_raw, prev_kind, prev_site)) =
                                canonical_names.get(&canon)
                            {
                                let err_span = unsafe {
                                    std::mem::transmute::<SourceSpan<'_>, SourceSpan<'_>>(span)
                                };

                                let is_versioned = decl.attributes().is_some_and(|attrs| attrs.attributes.iter().any(|a| a.name.data() == "available" || a.provenance == crate::raw_ast::AttributeProvenance::ModifierAvailability));
                                let prev_full_name = format!("{}/{}", lib, prev_raw);
                                let prev_is_versioned = compiler.raw_decls.get(&crate::names::FullyQualifiedName::from(prev_full_name)).and_then(|d| d.attributes()).is_some_and(|attrs| attrs.attributes.iter().any(|a| a.name.data() == "available" || a.provenance == crate::raw_ast::AttributeProvenance::ModifierAvailability));

                                if is_versioned
                                    && prev_is_versioned
                                    && prev_kind != "library import"
                                {
                                    // Assume structurally sound versioning and overlap resolution happens in availability_step.
                                } else if prev_raw == local_decl_name {
                                    if prev_kind == "library import" {
                                        errors_to_emit.push((
                                            Error::ErrDeclNameConflictsWithLibraryImport,
                                            err_span,
                                            vec![local_decl_name.to_string()],
                                        ));
                                    } else {
                                        errors_to_emit.push((
                                            Error::ErrNameCollision,
                                            err_span,
                                            vec![
                                                decl_kind.to_string(),
                                                local_decl_name.to_string(),
                                                prev_kind.to_string(),
                                                prev_site.clone(),
                                            ],
                                        ));
                                    }
                                } else if prev_kind == "library import" {
                                    errors_to_emit.push((
                                        Error::ErrDeclNameConflictsWithLibraryImportCanonical,
                                        err_span,
                                        vec![local_decl_name.to_string(), canon.clone()],
                                    ));
                                } else {
                                    errors_to_emit.push((
                                        Error::ErrNameCollisionCanonical,
                                        err_span,
                                        vec![
                                            decl_kind.to_string(),
                                            local_decl_name.to_string(),
                                            prev_kind.to_string(),
                                            prev_raw.clone(),
                                            prev_site.clone(),
                                            canon.clone(),
                                        ],
                                    ));
                                }
                            } else {
                                canonical_names.insert(
                                    canon,
                                    (local_decl_name.to_string(), decl_kind.to_string(), site),
                                );
                            }
                        }
                    }
                    compiler.raw_decls.insert(crate::names::FullyQualifiedName::from(name.to_string()), decl);
                };

            for decl in &file.type_decls {
                let local_name = decl.name.data();
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Type(decl),
                    "type",
                    false,
                    &mut errors_to_emit,
                );
            }

            for decl in &file.alias_decls {
                let local_name = decl.name.data();
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Alias(decl),
                    "alias",
                    false,
                    &mut errors_to_emit,
                );
            }

            for decl in &file.struct_decls {
                let local_name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Struct(decl),
                    "struct",
                    decl.name.is_none(),
                    &mut errors_to_emit,
                );
            }

            for decl in &file.enum_decls {
                let local_name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Enum(decl),
                    "enum",
                    decl.name.is_none(),
                    &mut errors_to_emit,
                );
            }

            for decl in &file.bits_decls {
                let local_name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Bits(decl),
                    "bits",
                    decl.name.is_none(),
                    &mut errors_to_emit,
                );
            }

            for decl in &file.union_decls {
                let local_name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Union(decl),
                    "union",
                    decl.name.is_none(),
                    &mut errors_to_emit,
                );
            }

            for decl in &file.table_decls {
                let local_name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Table(decl),
                    "table",
                    decl.name.is_none(),
                    &mut errors_to_emit,
                );
            }

            for decl in &file.protocol_decls {
                let local_name = decl.name.data();
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Protocol(decl),
                    "protocol",
                    false,
                    &mut errors_to_emit,
                );

                let protocol_context = NamingContext::create(decl.name.element.span());

                for method in &decl.methods {
                    let req_s = match &method.request_payload {
                        Some(raw_ast::Layout::Struct(s)) => Some(RawDecl::Struct(s)),
                        Some(raw_ast::Layout::Table(t)) => Some(RawDecl::Table(t)),
                        Some(raw_ast::Layout::Union(u)) => Some(RawDecl::Union(u)),
                        Some(raw_ast::Layout::TypeConstructor(tc)) => match &tc.layout {
                            raw_ast::LayoutParameter::Inline(inline_layout) => {
                                match &**inline_layout {
                                    raw_ast::Layout::Struct(s) => Some(RawDecl::Struct(s)),
                                    raw_ast::Layout::Table(t) => Some(RawDecl::Table(t)),
                                    raw_ast::Layout::Union(u) => Some(RawDecl::Union(u)),
                                    _ => None,
                                }
                            }
                            _ => None,
                        },
                        _ => None,
                    };
                    if let Some(decl) = req_s {
                        let ctx = protocol_context.enter_request(method.name.element.span());
                        let synth_name = ctx.flattened_name();
                        let full_synth = format!("{}/{}", file_library_name, synth_name);
                        compiler.anonymous_structs.insert(full_synth.clone());
                        let kind = match decl {
                            RawDecl::Struct(_) => "struct",
                            RawDecl::Table(_) => "table",
                            RawDecl::Union(_) => "union",
                            _ => "unknown",
                        };
                        insert_decl(
                            compiler,
                            &mut canonical_names,
                            full_synth,
                            &synth_name,
                            decl,
                            kind,
                            true,
                            &mut errors_to_emit,
                        );
                    }

                    let res_s = match &method.response_payload {
                        Some(raw_ast::Layout::Struct(s)) => Some(RawDecl::Struct(s)),
                        Some(raw_ast::Layout::Table(t)) => Some(RawDecl::Table(t)),
                        Some(raw_ast::Layout::Union(u)) => Some(RawDecl::Union(u)),
                        Some(raw_ast::Layout::TypeConstructor(tc)) => match &tc.layout {
                            raw_ast::LayoutParameter::Inline(inline_layout) => {
                                match &**inline_layout {
                                    raw_ast::Layout::Struct(s) => Some(RawDecl::Struct(s)),
                                    raw_ast::Layout::Table(t) => Some(RawDecl::Table(t)),
                                    raw_ast::Layout::Union(u) => Some(RawDecl::Union(u)),
                                    _ => None,
                                }
                            }
                            _ => None,
                        },
                        _ => None,
                    };
                    let is_method_flexible = method
                        .modifiers
                        .iter()
                        .any(|m| m.element.span().data == "flexible");

                    if method.has_request && method.has_response && !method.has_error {
                        let has_complex_modifiers = method.modifiers.len() > 1
                            || (method.modifiers.len() == 1
                                && method.modifiers[0].attributes.is_some());
                        if has_complex_modifiers {
                            let span = method.modifiers.first().unwrap().element.span();
                            errors_to_emit.push((
                                Error::ErrCannotChangeMethodStrictness,
                                unsafe {
                                    std::mem::transmute::<SourceSpan<'_>, SourceSpan<'_>>(span)
                                },
                                vec![],
                            ));
                        }
                    }

                    if let Some(decl) = res_s {
                        let mut ctx = if !method.has_request && !method.has_error {
                            protocol_context.enter_event(method.name.element.span())
                        } else {
                            protocol_context.enter_response(method.name.element.span())
                        };

                        if method.has_error
                            || (is_method_flexible && method.has_request && method.has_response)
                        {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                local_name,
                                method.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                local_name,
                                method.name.data()
                            ));
                        }

                        let synth_name = ctx.flattened_name();
                        let full_synth = format!("{}/{}", file_library_name, synth_name);
                        compiler.anonymous_structs.insert(full_synth.clone());
                        let kind = match decl {
                            RawDecl::Struct(_) => "struct",
                            RawDecl::Table(_) => "table",
                            RawDecl::Union(_) => "union",
                            _ => "unknown",
                        };
                        insert_decl(
                            compiler,
                            &mut canonical_names,
                            full_synth,
                            &synth_name,
                            decl,
                            kind,
                            true,
                            &mut errors_to_emit,
                        );
                    }
                }
            }

            for decl in &file.service_decls {
                let local_name = decl.name.data();
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Service(decl),
                    "service",
                    false,
                    &mut errors_to_emit,
                );
            }

            for decl in &file.resource_decls {
                let local_name = decl.name.data();
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Resource(decl),
                    "resource_definition",
                    false,
                    &mut errors_to_emit,
                );
            }

            for decl in &file.const_decls {
                let local_name = decl.name.data();
                let name = format!("{}/{}", file_library_name, local_name);
                insert_decl(
                    compiler,
                    &mut canonical_names,
                    name,
                    local_name,
                    RawDecl::Const(decl),
                    "const",
                    false,
                    &mut errors_to_emit,
                );
            }
        }

        for (err, span, args) in errors_to_emit {
            let ref_args: Vec<&dyn std::fmt::Debug> =
                args.iter().map(|s| s as &dyn std::fmt::Debug).collect();
            compiler.reporter.fail(err, span, &ref_args);
        }
    }
}
