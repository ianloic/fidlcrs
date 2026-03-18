use crate::compiler::Compiler;
use crate::step::Step;

use crate::names::OwnedQualifiedName;
pub struct ResolveStep;

impl<'node, 'src> Step<'node, 'src> for ResolveStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        for (name, decl) in &compiler.raw_decls {
            compiler.decl_kinds.insert(name.clone(), decl.kind());
        }
        compiler.sorted_names = compiler
            .topological_sort(true)
            .into_iter()
            .map(OwnedQualifiedName::from)
            .collect();
    }
}
