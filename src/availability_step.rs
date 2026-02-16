use crate::compiler::Compiler;
use crate::step::Step;
use crate::versioning_types::{Availability, InitArgs, Version, VersionRange};

pub struct AvailabilityStep;

impl<'node, 'src> Step<'node, 'src> for AvailabilityStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        let mut decl_availability = std::collections::HashMap::new();
        let selected_version = Version::HEAD;

        for (name, decl) in &compiler.raw_decls {
            let mut avail = Availability::unbounded();
            if let Some(attrs) = decl.attributes() {
                for attr in &attrs.attributes {
                    if attr.name.data() == "available" {
                        let mut added = None;
                        let mut deprecated = None;
                        let mut removed = None;
                        for arg in &attr.args {
                            let arg_name = arg.name.as_ref().map(|n| n.data()).unwrap_or("value");
                            let val_str = match &arg.value {
                                crate::raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                                crate::raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                                _ => "".to_string(),
                            };
                            if arg_name == "added" { added = Version::parse(&val_str); }
                            if arg_name == "deprecated" { deprecated = Version::parse(&val_str); }
                            if arg_name == "removed" { removed = Version::parse(&val_str); }
                        }
                        
                        let mut initial = Availability::new();
                        let _ = initial.init(InitArgs { added, deprecated, removed, replaced: false });
                        let _ = initial.inherit(&Availability::unbounded());
                        initial.narrow(VersionRange::new(selected_version, Version::POS_INF));
                        avail = initial;
                    }
                }
            }
            
            decl_availability.insert(name.clone(), avail);
        }

        compiler.decl_availability = decl_availability;
    }
}
