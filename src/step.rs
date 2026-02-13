use crate::compiler::Compiler;

pub trait Step<'node, 'src> {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>);
}
