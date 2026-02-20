use crate::compiler::{Compiler, RawDecl};
use crate::raw_ast;
use crate::step::Step;

pub struct ConsumeStep<'node, 'src> {
    pub files: &'node [raw_ast::File<'src>],
}

impl<'node, 'src> Step<'node, 'src> for ConsumeStep<'node, 'src> {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        let main_library_name = self
            .files
            .last()
            .and_then(|f| f.library_decl.as_ref().map(|l| l.path.to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        compiler.library_name = main_library_name.clone();
        compiler.library_decl = self
            .files
            .last()
            .and_then(|f| f.library_decl.as_ref().map(|l| *l.clone()));

        for file in self.files {
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

                let protocol_context = crate::name::NamingContext::create(name);

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
                        let ctx = protocol_context.enter_request(method.name.data());
                        let full_synth = format!("{}/{}", file_library_name, ctx.flattened_name());
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
                        let mut ctx = protocol_context.enter_response(method.name.data());
                        if method.has_error {
                            let mut ctx_val = (*ctx).clone();
                            ctx_val.set_name_override(format!(
                                "{}_{}_Result",
                                name,
                                method.name.data()
                            ));
                            ctx = std::rc::Rc::new(ctx_val);
                            ctx = ctx.enter_member("response");
                            let mut ctx_val = (*ctx).clone();
                            ctx_val.set_name_override(format!(
                                "{}_{}_Response",
                                name,
                                method.name.data()
                            ));
                            ctx = std::rc::Rc::new(ctx_val);
                        } else if !method.has_request {
                            ctx = protocol_context.enter_event(method.name.data());
                        }

                        let full_synth = format!("{}/{}", file_library_name, ctx.flattened_name());
                        compiler.raw_decls.insert(full_synth, RawDecl::Struct(s));
                    }
                }
            }

            for decl in &file.service_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Service(decl));
            }

            for decl in &file.const_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", file_library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Const(decl));
            }
        }
    }
}
