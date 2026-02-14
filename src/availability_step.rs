use crate::compiler::Compiler;
use crate::step::Step;
use crate::versioning_types::Availability;

pub struct AvailabilityStep;

impl<'node, 'src> Step<'node, 'src> for AvailabilityStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        // Just dummy logic for now to bootstrap it.
        // It will examine `@available` attributes and resolve inheritance.

        let mut decl_availability = std::collections::HashMap::new();

        // For now, everything gets default unversioned availability
        for name in compiler.raw_decls.keys() {
            decl_availability.insert(name.clone(), Availability::default());
        }

        compiler.decl_availability = decl_availability;
    }
}
