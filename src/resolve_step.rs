use crate::compiler::{Compiler, RawDecl};
use crate::raw_ast;
use crate::step::Step;

pub struct ResolveStep;

impl<'node, 'src> Step<'node, 'src> for ResolveStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        for (name, decl) in &compiler.raw_decls {
            let kind = match decl {
                RawDecl::Struct(_) => "struct",
                RawDecl::Union(_) => "union",
                RawDecl::Table(_) => "table",
                RawDecl::Protocol(_) => "protocol",
                RawDecl::Service(_) => "service",
                RawDecl::Const(_) => "const",
                RawDecl::Enum(_) => "enum",
                RawDecl::Bits(_) => "bits",
                RawDecl::Alias(_) => "alias",
                RawDecl::Type(t) => match t.layout {
                    raw_ast::Layout::Struct(_) => "struct",
                    raw_ast::Layout::Union(_) => "union",
                    raw_ast::Layout::Table(_) => "table",
                    raw_ast::Layout::Enum(_) => "enum",
                    raw_ast::Layout::Bits(_) => "bits",
                    _ => "unknown",
                },
            };
            compiler.decl_kinds.insert(name.clone(), kind);
        }

        compiler.sorted_names = compiler.topological_sort(true);
    }
}
