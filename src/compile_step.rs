use crate::compiler::Compiler;
use crate::step::Step;

pub struct CompileStep;

impl<'node, 'src> Step<'node, 'src> for CompileStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        let sorted_names = compiler.sorted_names.clone();
        for name in &sorted_names {
            compiler.compile_decl_by_name(name);
        }
    }
}
