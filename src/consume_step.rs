use crate::compiler::{Compiler, RawDecl};
use crate::raw_ast;
use crate::step::Step;

pub struct ConsumeStep<'node, 'src> {
    pub files: &'node [raw_ast::File<'src>],
}

impl<'node, 'src> Step<'node, 'src> for ConsumeStep<'node, 'src> {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        compiler.library_name = self.files.iter()
            .find_map(|f| f.library_decl.as_ref().map(|l| l.path.to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        for file in self.files {
            for decl in &file.type_decls {
                let name = format!("{}/{}", compiler.library_name, decl.name.data());
                compiler.raw_decls.insert(name, RawDecl::Type(decl));
            }

            for decl in &file.struct_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", compiler.library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Struct(decl));
            }

            for decl in &file.enum_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", compiler.library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Enum(decl));
            }

            for decl in &file.bits_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", compiler.library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Bits(decl));
            }

            for decl in &file.union_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", compiler.library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Union(decl));
            }

            for decl in &file.table_decls {
                let name = decl.name.as_ref().map(|n| n.data()).unwrap_or("anonymous");
                let full_name = format!("{}/{}", compiler.library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Table(decl));
            }

            for decl in &file.protocol_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", compiler.library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Protocol(decl));

                for method in &decl.methods {
                    let method_name_camel = format!(
                        "{}{}",
                        method.name.data().chars().next().unwrap().to_uppercase(),
                        &method.name.data()[1..]
                    );
                    if let Some(raw_ast::Layout::Struct(s)) = &method.request_payload {
                        let synth_name = format!("{}Request", method_name_camel);
                        let full_synth =
                            format!("{}/{}", compiler.library_name, format!("{}{}", name, synth_name));
                        compiler.raw_decls.insert(full_synth, RawDecl::Struct(s));
                    }
                    if let Some(raw_ast::Layout::Struct(s)) = &method.response_payload {
                        let (synth_name, full_synth) = if method.has_error {
                            let sn = format!("_{}_Response", method.name.data());
                            (sn.clone(), format!("{}/{}", compiler.library_name, format!("{}{}", name, sn)))
                        } else {
                            let sn = format!("{}Response", method_name_camel);
                            (sn.clone(), format!("{}/{}", compiler.library_name, format!("{}{}", name, sn)))
                        };
                        compiler.raw_decls.insert(full_synth, RawDecl::Struct(s));
                    }
                }
            }

            for decl in &file.service_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", compiler.library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Service(decl));
            }

            for decl in &file.const_decls {
                let name = decl.name.data();
                let full_name = format!("{}/{}", compiler.library_name, name);
                compiler.raw_decls.insert(full_name, RawDecl::Const(decl));
            }
        }
    }
}
