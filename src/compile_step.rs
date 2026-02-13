use crate::compiler::{Compiler, RawDecl};
use crate::raw_ast;
use crate::step::Step;

pub struct CompileStep;

impl<'node, 'src> Step<'node, 'src> for CompileStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        let sorted_names = compiler.sorted_names.clone();
        for name in &sorted_names {
            if let Some(decl) = compiler.raw_decls.get(name).cloned() {
                let library_name = compiler.library_name.clone();
                match decl {
                    RawDecl::Type(t) => {
                        if let raw_ast::Layout::Struct(ref s) = t.layout {
                            let compiled = compiler.compile_struct(
                                t.name.data(),
                                s,
                                &library_name,
                                Some(&t.name.element),
                                None,
                                t.attributes.as_deref(),
                            );
                            compiler.struct_declarations.push(compiled);
                        } else if let raw_ast::Layout::Enum(ref e) = t.layout {
                            let compiled = compiler.compile_enum(
                                t.name.data(),
                                e,
                                &library_name,
                                Some(&t.name.element),
                                t.attributes.as_deref(),
                            );
                            compiler.enum_declarations.push(compiled);
                        } else if let raw_ast::Layout::Bits(ref b) = t.layout {
                            let compiled = compiler.compile_bits(
                                t.name.data(),
                                b,
                                &library_name,
                                Some(&t.name.element),
                                t.attributes.as_deref(),
                            );
                            compiler.bits_declarations.push(compiled);
                        } else if let raw_ast::Layout::Table(ref ta) = t.layout {
                            let compiled = compiler.compile_table(
                                t.name.data(),
                                ta,
                                &library_name,
                                Some(&t.name.element),
                                t.attributes.as_deref(),
                            );
                            compiler.table_declarations.push(compiled);
                        } else if let raw_ast::Layout::Union(ref u) = t.layout {
                            let compiled = compiler.compile_union(
                                t.name.data(),
                                u,
                                &library_name,
                                Some(&t.name.element),
                                t.attributes.as_deref(),
                            );
                            compiler.union_declarations.push(compiled);
                        }
                    }
                    RawDecl::Struct(s) => {
                        if s.name.is_none() {
                            continue;
                        }
                        let short_name = s.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = compiler.compile_struct(
                            short_name,
                            s,
                            &library_name,
                            None,
                            None,
                            s.attributes.as_deref(),
                        );
                        if s.name.is_some() {
                            compiler.struct_declarations.push(compiled);
                        }
                    }
                    RawDecl::Enum(e) => {
                        let short_name = e.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = compiler.compile_enum(
                            short_name,
                            e,
                            &library_name,
                            None,
                            e.attributes.as_deref(),
                        );
                        if e.name.is_some() {
                            compiler.enum_declarations.push(compiled);
                        }
                    }
                    RawDecl::Bits(b) => {
                        let short_name = b.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = compiler.compile_bits(
                            short_name,
                            b,
                            &library_name,
                            None,
                            b.attributes.as_deref(),
                        );
                        if b.name.is_some() {
                            compiler.bits_declarations.push(compiled);
                        }
                    }
                    RawDecl::Union(u) => {
                        let short_name = u.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = compiler.compile_union(
                            short_name,
                            u,
                            &library_name,
                            None,
                            u.attributes.as_deref(),
                        );
                        if u.name.is_some() {
                            compiler.union_declarations.push(compiled);
                        }
                    }
                    RawDecl::Table(t) => {
                        let short_name = t.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                        let compiled = compiler.compile_table(
                            short_name,
                            t,
                            &library_name,
                            None,
                            t.attributes.as_deref(),
                        );
                        if t.name.is_some() {
                            compiler.table_declarations.push(compiled);
                        }
                    }
                    RawDecl::Protocol(p) => {
                        let short_name = p.name.data();
                        let compiled = compiler.compile_protocol(short_name, p, &library_name);
                        compiler.protocol_declarations.push(compiled);
                    }
                    RawDecl::Service(s) => {
                        let short_name = s.name.data();
                        let compiled = compiler.compile_service(short_name, s, &library_name);
                        compiler.service_declarations.push(compiled);
                    }
                    RawDecl::Const(c) => {
                        let compiled = compiler.compile_const(c, &library_name);
                        compiler.const_declarations.push(compiled);
                    }
                }
            }
        }
    }
}
