use crate::compiler::Compiler;
use crate::raw_ast;
use crate::step::Step;
use crate::versioning_types::{Availability, InitArgs, Version};

pub struct AvailabilityStep;

impl<'node, 'src> Step<'node, 'src> for AvailabilityStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        let mut decl_availability = std::collections::HashMap::new();

        let mut platform_name = compiler
            .library_decl
            .as_ref()
            .map(|l| l.path.components[0].data().to_string())
            .unwrap_or_else(|| "unversioned".to_string());

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
                            if arg_name == "platform" {
                                platform_name = val_str.trim_matches('"').to_string();
                            }
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
                            let _ = initial.inherit(&Availability::unbounded());
                            lib_avail = initial;
                        }
                    }
                }
            }
        }
        let final_lib_avail = lib_avail;

        let platform = crate::versioning_types::Platform::parse(&platform_name)
            .unwrap_or_else(crate::versioning_types::Platform::unversioned);
        let selected_version = compiler.version_selection.lookup(&platform);

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
                            avail = initial;
                        }
                    }
                }
            }

            decl_availability.insert(name.clone(), avail);
        }
        compiler.decl_availability = decl_availability.clone();

        let mut any_decl_removed = false;

        compiler.raw_decls.retain(|name, _| {
            if let Some(avail) = decl_availability.get(name) {
                if !avail.set().contains(selected_version) {
                    any_decl_removed = true;
                    return false;
                }
            }
            true
        });

        if any_decl_removed || !final_lib_avail.set().contains(selected_version) {
            compiler.allow_unused_imports = true;
        }
    }
}
