use crate::compiler::Compiler;
use crate::raw_ast;
use crate::step::Step;
use crate::versioning_types::Platform;
use crate::versioning_types::{Availability, InitArgs, Version};

pub struct AvailabilityStep;

impl AvailabilityStep {
    fn extract_availability<'src>(
        attrs: Option<&raw_ast::AttributeList<'src>>,
        parent_avail: &Availability,
    ) -> Availability {
        let mut avail = parent_avail.clone();
        if let Some(attrs) = attrs {
            for attr in &attrs.attributes {
                if attr.name.data() == "available"
                    || attr.provenance == crate::raw_ast::AttributeProvenance::ModifierAvailability
                {
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
                    let did_init = initial.init(InitArgs {
                        added,
                        deprecated,
                        removed,
                        replaced: false,
                    });
                    if did_init {
                        let _res = initial.inherit(parent_avail);
                        avail = initial;
                    }
                }
            }
        }
        avail
    }
}

impl<'node, 'src> Step<'node, 'src> for AvailabilityStep {
    fn run(&mut self, compiler: &mut Compiler<'node, 'src>) {
        let mut decl_availability = std::collections::HashMap::new();

        let mut platform_name = compiler
            .library_decl
            .as_ref()
            .map(|l| l.path.components[0].data().to_string())
            .unwrap_or_else(|| "unversioned".to_string());

        let mut lib_avail = Availability::unbounded();
        if let Some(lib_decl) = &compiler.library_decl
            && let Some(attrs) = &lib_decl.attributes
        {
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
        let final_lib_avail = lib_avail;

        let platform = Platform::parse(&platform_name).unwrap_or_else(Platform::unversioned);
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
            if let Some(avail) = decl_availability.get(name)
                && !avail.set().contains(selected_version)
            {
                any_decl_removed = true;
                return false;
            }
            true
        });

        if any_decl_removed || !final_lib_avail.set().contains(selected_version) {
            compiler.allow_unused_imports = true;
        }

        // Validate modifiers
        for (name, decl) in &compiler.raw_decls {
            let decl_avail = decl_availability.get(name).unwrap();
            decl.for_each_modifier_list(|modifiers| {
                let mut by_kind: std::collections::HashMap<
                    u8,
                    Vec<(&raw_ast::Modifier<'src>, Availability)>,
                > = std::collections::HashMap::new();

                for modifier in modifiers {
                    let avail =
                        Self::extract_availability(modifier.attributes.as_ref(), decl_avail);

                    let kind = match modifier.subkind {
                        crate::token::TokenSubkind::Strict
                        | crate::token::TokenSubkind::Flexible => 1,
                        crate::token::TokenSubkind::Open
                        | crate::token::TokenSubkind::Ajar
                        | crate::token::TokenSubkind::Closed => 2,
                        crate::token::TokenSubkind::Resource => 3,
                        _ => 0,
                    };

                    let same_kind = by_kind.entry(kind).or_default();
                    for (other_mod, other_avail) in same_kind.iter() {
                        if avail.set().overlap(&other_avail.set()) {
                            println!(
                                "Overlap between {:?} and {:?}",
                                avail.set(),
                                other_avail.set()
                            );
                            if modifier.subkind == other_mod.subkind {
                                compiler.reporter.fail(
                                    crate::diagnostics::Error::ErrDuplicateModifier,
                                    modifier.element.span(),
                                    &[&modifier.element.span().data.to_string()],
                                );
                            } else {
                                compiler.reporter.fail(
                                    crate::diagnostics::Error::ErrConflictingModifier,
                                    modifier.element.span(),
                                    &[
                                        &modifier.element.span().data.to_string(),
                                        &other_mod.element.span().data.to_string(),
                                    ],
                                );
                            }
                            break;
                        }
                    }
                    same_kind.push((modifier, avail));
                }
            });
        }
    }
}
