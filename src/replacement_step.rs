use crate::compiler::Compiler;
use crate::step::Step;

pub struct ReplacementStep;

impl<'node, 'src> Step<'node, 'src> for ReplacementStep {
    fn run(&mut self, _compiler: &mut Compiler<'node, 'src>) {
        // Just dummy logic for now to bootstrap it.
        // It will examine `@available(removed=X, replaced=Y)`
        // and check correctness based on Availability from AvailabilityStep.
    }
}
