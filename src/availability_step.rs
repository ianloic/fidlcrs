use crate::compiler::Compiler;
use crate::raw_ast;
use crate::step::Step;
use crate::versioning_types::{Availability, InitArgs, Version, VersionRange};

pub struct AvailabilityStep;

impl<'node, 'src> Step<'node, 'src> for AvailabilityStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        let mut decl_availability = std::collections::HashMap::new();
        let selected_version = Version::HEAD;

        let mut lib_avail = Availability::unbounded();
        if let Some(lib_decl) = &compiler.library_decl {
            if let Some(attrs) = &lib_decl.attributes {
                for attr in &attrs.attributes {
                    if attr.name.data() == "available" {
                        let mut added = None;
                        let mut deprecated = None;
                        let mut removed = None;
                        for arg in &attr.args {
                            let arg_name = arg.name.as_ref().map(|n| n.data()).unwrap_or("value");
                            let val_str = match &arg.value {
                                raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                                raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                                _ => "".to_string(),
                            };
                            if arg_name == "added" {
                                added = Version::parse(&val_str);
                            }
                            if arg_name == "deprecated" {
                                deprecated = Version::parse(&val_str);
                            }
                            if arg_name == "removed" {
                                removed = Version::parse(&val_str);
                            }
                        }
                        let mut initial = Availability::new();
                        if initial.init(InitArgs {
                            added,
                            deprecated,
                            removed,
                            replaced: false,
                        }) {
                            let mut head_avail = Availability::new();
                            let _ = head_avail.init(InitArgs {
                                added: Some(Version::HEAD),
                                deprecated: None,
                                removed: None,
                                replaced: false,
                            });
                            let _ = head_avail.inherit(&Availability::unbounded());
                            let _ = initial.inherit(&head_avail);
                            lib_avail = initial;
                        }
                    }
                }
            }
        }
        // Since Availability doesn't expose `added`, we assume lib_avail has an `added` set.
        // We know that `lib_avail` was initialized. If it wasn't (because no `@available` on library),
        // we should create one.
        if lib_avail.state() == crate::versioning_types::AvailabilityState::Inherited {
            // Wait, if it's Inherited, let's see if added == NEG_INF (from unbounded)
            // Actually, we can't easily check added. So we just assume it's good unless we didn't specify `@available`.
            // Wait, `AvailabilityStep` in fidlc handles this during library compilation.
            // For now let's just use `Availability::unbounded()` as the parent for `lib_avail` to make it Inherited.
        }

        let mut final_lib_avail = Availability::new();
        let _ = final_lib_avail.init(InitArgs {
            added: Some(Version::HEAD),
            deprecated: None,
            removed: None,
            replaced: false,
        });
        let _ = final_lib_avail.inherit(&Availability::unbounded());
        if lib_avail.state() == crate::versioning_types::AvailabilityState::Inherited {
            // Use lib_avail if it comes from an attribute
            final_lib_avail = lib_avail.clone();
            // but actually, we need
        }

        for (name, decl) in &compiler.raw_decls {
            let mut avail = final_lib_avail.clone();
            if let Some(attrs) = decl.attributes() {
                for attr in &attrs.attributes {
                    if attr.name.data() == "available" {
                        let mut added = None;
                        let mut deprecated = None;
                        let mut removed = None;
                        for arg in &attr.args {
                            let arg_name = arg.name.as_ref().map(|n| n.data()).unwrap_or("value");
                            let val_str = match &arg.value {
                                raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                                raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                                _ => "".to_string(),
                            };
                            if arg_name == "added" {
                                added = Version::parse(&val_str);
                            }
                            if arg_name == "deprecated" {
                                deprecated = Version::parse(&val_str);
                            }
                            if arg_name == "removed" {
                                removed = Version::parse(&val_str);
                            }
                        }

                        let mut initial = Availability::new();
                        if initial.init(InitArgs {
                            added,
                            deprecated,
                            removed,
                            replaced: false,
                        }) {
                            let _ = initial.inherit(&final_lib_avail);
                            if initial.state()
                                == crate::versioning_types::AvailabilityState::Inherited
                            {
                                initial
                                    .narrow(VersionRange::new(selected_version, Version::POS_INF));
                            }
                            avail = initial;
                        }
                    }
                }
            }

            decl_availability.insert(name.clone(), avail);
        }

        compiler.decl_availability = decl_availability;
    }
}
