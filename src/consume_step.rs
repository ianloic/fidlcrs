use crate::compiler::{Compiler, RawDecl};
use crate::name::NamingContext;
use crate::raw_ast;
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

        compiler.library_name = main_library_name.clone();
        
        let mut all_library_attributes = Vec::new();
        let mut main_library_decl: Option<raw_ast::LibraryDeclaration> = None;
        for file in self.main_files {
            for using_decl in &file.using_decls {
                if using_decl.attributes.is_some() {
                    let span = unsafe {
                        std::mem::transmute::<
                            crate::source_span::SourceSpan,
                            crate::source_span::SourceSpan,
                        >(using_decl.element.span().clone())
                    };
                    compiler.reporter.fail(
                        crate::diagnostics::Error::ErrAttributesNotAllowedOnLibraryImport,
                        span,
                        &[],
                    );
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
                decl.attributes = Some(Box::new(crate::raw_ast::AttributeList {
                    attributes: all_library_attributes,
                    element: decl.attributes.as_ref().map(|a| a.element.clone()).unwrap_or_else(|| decl.element.clone()),
                }));
            }
            compiler.library_decl = Some(decl);
        } else {
            compiler.library_decl = None;
        }

        let all_files = self.dependency_files.iter().chain(self.main_files.iter());

        for file in all_files {
            let file_library_name = file
                .library_decl
                .as_ref()
                .map(|l| l.path.to_string())
                .unwrap_or_else(|| main_library_name.clone());

            for decl in &file.type_decls {
                let name = format!("{}/{}", file_library_name, decl.name.data());
                compiler.raw_decls.insert(name, RawDecl::Type(decl));
            }

            for decl in &file.alias_decls {
                let name = format!("{}/{}", file_library_name, decl.name.data());
                compiler.raw_decls.insert(name, RawDecl::Alias(decl));
            }

            for decl in &file.struct_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Struct(decl));
            }

            for decl in &file.enum_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Enum(decl));
            }

            for decl in &file.bits_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Bits(decl));
            }

            for decl in &file.union_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Union(decl));
            }

            for decl in &file.table_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Table(decl));
            }

            for decl in &file.protocol_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", file_library_name, name);
                compiler
                    .raw_decls
                    .insert(full_name, RawDecl::Protocol(decl));

                let protocol_context = NamingContext::create(decl.name.element.span());

                for method in &decl.methods {
                    let req_s = match &method.request_payload {
                        Some(raw_ast::Layout::Struct(s)) => Some(s),
                        Some(raw_ast::Layout::TypeConstructor(tc)) => match &tc.layout {
                            raw_ast::LayoutParameter::Inline(inline_layout) => {
                                match &**inline_layout {
                                    raw_ast::Layout::Struct(s) => Some(s),
                                    _ => None,
                                }
                            }
                            _ => None,
                        },
                        _ => None,
                    };
                    if let Some(s) = req_s {
                        let ctx = protocol_context.enter_request(method.name.element.span());
                        let full_synth = format!("{}/{}", file_library_name, ctx.flattened_name());
                        compiler.anonymous_structs.insert(full_synth.clone());
                        compiler.raw_decls.insert(full_synth, RawDecl::Struct(s));
                    }

                    let res_s = match &method.response_payload {
                        Some(raw_ast::Layout::Struct(s)) => Some(s),
                        Some(raw_ast::Layout::TypeConstructor(tc)) => match &tc.layout {
                            raw_ast::LayoutParameter::Inline(inline_layout) => {
                                match &**inline_layout {
                                    raw_ast::Layout::Struct(s) => Some(s),
                                    _ => None,
                                }
                            }
                            _ => None,
                        },
                        _ => None,
                    };
                    if let Some(s) = res_s {
                        let mut ctx = if !method.has_request && !method.has_error {
                            protocol_context.enter_event(method.name.element.span())
                        } else {
                            protocol_context.enter_response(method.name.element.span())
                        };

                        if method.has_error {
                            ctx.set_name_override(format!(
                                "{}_{}_Result",
                                name,
                                method.name.data()
                            ));
                            ctx = ctx.enter_member("response");
                            ctx.set_name_override(format!(
                                "{}_{}_Response",
                                name,
                                method.name.data()
                            ));
                        }

                        let full_synth = format!("{}/{}", file_library_name, ctx.flattened_name());
                        compiler.anonymous_structs.insert(full_synth.clone());
                        compiler.raw_decls.insert(full_synth, RawDecl::Struct(s));
                    }
                }
            }

            for decl in &file.service_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Service(decl));
            }

            for decl in &file.resource_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", file_library_name, name);
                compiler
                    .raw_decls
                    .insert(full_name, RawDecl::Resource(decl));
            }

            for decl in &file.const_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Const(decl));
            }
        }
    }
}
