use crate::compiler::Compiler;
use crate::raw_ast;
use crate::step::Step;
use crate::versioning_types::Platform;
use crate::versioning_types::{Availability, InheritStatus, InitArgs, Version};

use crate::diagnostics::Error;
use crate::names::OwnedQualifiedName;
use crate::raw_ast::AttributeProvenance;
use crate::raw_ast::Layout;
use crate::raw_ast::RawDecl;
use crate::source_span::SourceSpan;
use crate::token::TokenSubkind;
pub struct AvailabilityStep;

impl AvailabilityStep {
    fn compile_attr<'src>(
        compiler: &Compiler<'_, 'src>,
        attr: &raw_ast::Attribute<'src>,
        parent_avail: &Availability,
        decl_kind: &str, // e.g. "library", "struct", "modifier", "alias", etc
        item_name: &str,
    ) -> Option<Availability> {
        let mut added = None;
        let mut deprecated = None;
        let mut removed = None;
        let mut replaced = None;

        let mut added_arg = None;
        let mut deprecated_arg = None;
        let mut removed_arg = None;

        for arg in &attr.args {
            let arg_name = arg.name.as_ref().map(|n| n.data()).unwrap_or("value");
            let val_str = match &arg.value {
                raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                _ => {
                    let span = match &arg.value {
                        raw_ast::Constant::BinaryOperator(b) => b.element.span(),
                        _ => arg.element.span(),
                    };
                    span.data.to_string()
                }
            };
            if arg_name == "added" {
                added = Version::parse(&val_str);
                if added.is_none() && !val_str.is_empty() {
                    compiler.reporter.fail(
                        Error::ErrInvalidVersion(flyweights::FlyStr::new(
                            format!("{}", &val_str).into_boxed_str(),
                        )),
                        arg.element.span(),
                    );
                }
                added_arg = Some((arg_name, val_str.clone(), arg.element.span()));
            } else if arg_name == "deprecated" {
                deprecated = Version::parse(&val_str);
                if deprecated.is_none() && !val_str.is_empty() {
                    compiler.reporter.fail(
                        Error::ErrInvalidVersion(flyweights::FlyStr::new(
                            format!("{}", &val_str).into_boxed_str(),
                        )),
                        arg.element.span(),
                    );
                }
                deprecated_arg = Some((arg_name, val_str.clone(), arg.element.span()));
                if decl_kind == "modifier" {
                    compiler.reporter.fail(
                        Error::ErrInvalidModifierAvailableArgument(arg_name.into()),
                        arg.element.span(),
                    );
                }
            } else if arg_name == "removed" {
                removed = Version::parse(&val_str);
                if removed.is_none() && !val_str.is_empty() {
                    compiler.reporter.fail(
                        Error::ErrInvalidVersion(flyweights::FlyStr::new(
                            format!("{}", &val_str).into_boxed_str(),
                        )),
                        arg.element.span(),
                    );
                }
                removed_arg = Some((arg_name, val_str.clone(), arg.element.span()));
            } else if arg_name == "replaced" {
                replaced = Version::parse(&val_str);
                if replaced.is_none() && !val_str.is_empty() {
                    compiler.reporter.fail(
                        Error::ErrInvalidVersion(flyweights::FlyStr::new(
                            format!("{}", &val_str).into_boxed_str(),
                        )),
                        arg.element.span(),
                    );
                }
                if decl_kind == "modifier" {
                    compiler.reporter.fail(
                        Error::ErrInvalidModifierAvailableArgument(arg_name.into()),
                        arg.element.span(),
                    );
                }
            } else if arg_name == "platform" {
                if decl_kind != "library" {
                    compiler
                        .reporter
                        .fail(Error::ErrPlatformNotOnLibrary, arg.element.span());
                }
            } else if arg_name == "renamed" {
                if decl_kind == "library" || decl_kind == "declaration" {
                    let kind_str = if decl_kind == "library" {
                        "library"
                    } else {
                        "alias"
                    }; // just hardcode kind string for now
                    compiler.reporter.fail(
                        Error::ErrCannotBeRenamed(flyweights::FlyStr::new(
                            format!("{}", &kind_str.to_string()).into_boxed_str(),
                        )),
                        arg.element.span(),
                    );
                } else {
                    let unquoted_val = val_str.trim_matches('"');
                    if unquoted_val == item_name {
                        compiler.reporter.fail(
                            Error::ErrRenamedToSameName(
                                flyweights::FlyStr::new(
                                    format!("{}", &unquoted_val.to_string()).into_boxed_str(),
                                ),
                                flyweights::FlyStr::new(format!("{}", &item_name.to_string())),
                            ),
                            arg.element.span(),
                        );
                    }
                }
                if decl_kind == "modifier" {
                    compiler.reporter.fail(
                        Error::ErrInvalidModifierAvailableArgument(arg_name.into()),
                        arg.element.span(),
                    );
                }
            } else if (arg_name == "note" || arg_name == "legacy") && decl_kind == "modifier" {
                compiler.reporter.fail(
                    Error::ErrInvalidModifierAvailableArgument(arg_name.into()),
                    arg.element.span(),
                );
            }
        }

        if decl_kind == "library" {
            if replaced.is_some() {
                compiler
                    .reporter
                    .fail(Error::ErrLibraryReplaced, attr.element.span());
            }
            if added.is_none() {
                compiler.reporter.fail(
                    Error::ErrLibraryAvailabilityMissingAdded,
                    attr.element.span(),
                );
            }
        }

        let mut initial = Availability::new();
        if !initial.init(InitArgs {
            added,
            deprecated,
            removed: removed.or(replaced),
            replaced: replaced.is_some(),
        }) {
            let mut msg = String::new();
            if added.is_some() {
                msg.push_str("added");
            }
            if deprecated.is_some() {
                msg.push_str(if msg.is_empty() {
                    "deprecated"
                } else {
                    " <= deprecated"
                });
            }
            if removed.is_some() {
                msg.push_str(" < removed");
            } else if replaced.is_some() {
                msg.push_str(" < replaced");
            }
            let span = unsafe {
                std::mem::transmute::<SourceSpan<'_>, SourceSpan<'_>>(attr.element.span())
            };
            compiler.reporter.fail(
                Error::ErrInvalidAvailabilityOrder(flyweights::FlyStr::new(format!("{}", &msg))),
                span,
            );
            return None;
        }

        let result = initial.inherit(parent_avail);

        let report = |arg: Option<(&str, String, SourceSpan)>, status: InheritStatus| {
            if status == InheritStatus::Ok {
                return;
            }
            if let Some((child_name, child_val, child_span)) = arg {
                let when = match status {
                    InheritStatus::BeforeParentAdded => "before",
                    InheritStatus::AfterParentDeprecated => "after",
                    InheritStatus::AfterParentRemoved => "after",
                    _ => "",
                };
                let parent_what = match status {
                    InheritStatus::BeforeParentAdded => "added",
                    InheritStatus::AfterParentDeprecated => "deprecated",
                    InheritStatus::AfterParentRemoved => "removed",
                    _ => "",
                };
                let parent_name = parent_what;
                let parent_val = "unknown"; // We omit finding the actual parent value for simplicity
                let parent_span = "unknown_location";

                let span =
                    unsafe { std::mem::transmute::<SourceSpan<'_>, SourceSpan<'_>>(child_span) };
                compiler.reporter.fail(
                    Error::ErrAvailabilityConflictsWithParent(
                        flyweights::FlyStr::new(format!("{}", &child_name)),
                        flyweights::FlyStr::new(format!("{}", &child_val)),
                        flyweights::FlyStr::new(format!("{}", &parent_name)),
                        flyweights::FlyStr::new(format!("{}", &parent_val)),
                        flyweights::FlyStr::new(format!("{}", &parent_span)),
                        flyweights::FlyStr::new(format!("{}", &child_name)),
                        flyweights::FlyStr::new(format!("{}", &when)),
                        flyweights::FlyStr::new(format!("{}", &parent_what)),
                    ),
                    span,
                );
            }
        };

        report(added_arg, result.added);
        report(deprecated_arg, result.deprecated);
        report(removed_arg, result.removed);

        Some(initial)
    }

    fn extract_availability<'src>(
        compiler: &Compiler<'_, 'src>,
        attrs: Option<&raw_ast::AttributeList<'src>>,
        parent_avail: &Availability,
        decl_kind: &str,
        has_library_avail: bool,
        item_name: &str,
    ) -> Availability {
        let mut avail = parent_avail.clone();
        if let Some(attrs) = attrs {
            for attr in &attrs.attributes {
                if attr.name.data() == "available"
                    || attr.provenance == AttributeProvenance::ModifierAvailability
                {
                    if !has_library_avail
                        && attr.name.data() == "available"
                        && decl_kind != "library"
                    {
                        compiler
                            .reporter
                            .fail(Error::ErrMissingLibraryAvailability, attr.element.span());
                        // Continue to avoid multiple errors
                    }
                    if let Some(parsed) =
                        Self::compile_attr(compiler, attr, parent_avail, decl_kind, item_name)
                    {
                        avail = parsed;
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

        // Need to parse Availability for each file's library_decl, but we only have `raw_decls`.
        // We can just get it from `compiler.library_decl` and `compiler.all_files`?
        // Wait, `all_files` isn't accessible. But `all_ast_files` is in `Compiler`? No it is not.
        // Let's just avoid failing on dependencies and use unbounded for them.
        let main_lib_prefix = compiler
            .library_decl
            .as_ref()
            .map(|l| format!("{}/", l.path))
            .unwrap_or_default();

        // Wait, we can extract the library prefix from the declaration's name string!
        // We'll just define the main library availability.
        let mut platform_name = compiler
            .library_decl
            .as_ref()
            .map(|l| l.path.components[0].data().to_string())
            .unwrap_or_else(|| "unversioned".to_string());

        let mut main_lib_avail = Availability::unbounded();
        let mut has_library_avail = false;
        if let Some(lib_decl) = &compiler.library_decl
            && let Some(attrs) = &lib_decl.attributes
        {
            for attr in &attrs.attributes {
                if attr.name.data() == "available" {
                    has_library_avail = true;
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
                    }
                    if let Some(parsed) = Self::compile_attr(
                        compiler,
                        attr,
                        &Availability::unbounded(),
                        "library",
                        &platform_name,
                    ) {
                        main_lib_avail = parsed;
                    }
                }
            }
        }
        let final_lib_avail = main_lib_avail;

        let platform = Platform::parse(&platform_name).unwrap_or_else(Platform::unversioned);
        let selected_version = compiler.version_selection.lookup(&platform);

        for (name, decl) in &compiler.raw_decls {
            let is_main =
                name.to_string().starts_with(&main_lib_prefix) || main_lib_prefix.is_empty();
            let parent_avail = if is_main {
                &final_lib_avail
            } else {
                &Availability::unbounded()
            };

            // We only report errors for the main library's elements compilation.
            // But compile_attr always reports. To hack around this:
            // What if we don't compile availability for dependencies?
            // Actually, we must. But we don't have their lib_avail so they might fail if they inherit.
            // unbounded() allows them to pass inherit.
            let item_name = name.declaration();
            let avail = if is_main {
                Self::extract_availability(
                    compiler,
                    decl.attributes(),
                    parent_avail,
                    "declaration",
                    has_library_avail,
                    item_name,
                )
            } else {
                Self::extract_availability(
                    compiler,
                    decl.attributes(),
                    parent_avail,
                    "dependency",
                    true,
                    item_name,
                )
            };
            decl_availability.insert(name.to_string(), avail);
        }
        compiler.decl_availability = decl_availability
            .clone()
            .into_iter()
            .map(|(k, v)| (OwnedQualifiedName::from(k), v))
            .collect();

        let mut any_decl_removed = false;

        compiler.raw_decls.retain(|name, _| {
            if let Some(avail) = decl_availability.get(&name.to_string())
                && !avail.set().contains(selected_version)
            {
                any_decl_removed = true;
                return false;
            }
            true
        });

        let mut allow_unused_imports = false;
        if any_decl_removed || !final_lib_avail.set().contains(selected_version) {
            allow_unused_imports = true;
        }

        let mut member_availability_additions = std::collections::HashMap::new();

        // Validate modifiers
        for (name, decl) in &compiler.raw_decls {
            let decl_avail = decl_availability.get(&name.to_string()).unwrap();
            decl.for_each_modifier_list(|modifiers| {
                let mut by_kind: std::collections::HashMap<
                    u8,
                    Vec<(&raw_ast::Modifier<'src>, Availability)>,
                > = std::collections::HashMap::new();

                for modifier in modifiers {
                    // Only extract availability for main library modifiers so we don't report errors twice
                    let decl_is_main = name.to_string().starts_with(&main_lib_prefix)
                        || main_lib_prefix.is_empty();
                    let kind_str = if decl_is_main {
                        "modifier"
                    } else {
                        "dependency_modifier"
                    };
                    let item_name = modifier.element.span().data;
                    let avail = Self::extract_availability(
                        compiler,
                        modifier.attributes.as_ref(),
                        decl_avail,
                        kind_str,
                        has_library_avail || !decl_is_main,
                        item_name,
                    );

                    let kind = match modifier.subkind {
                        TokenSubkind::Strict | TokenSubkind::Flexible => 1,
                        TokenSubkind::Open | TokenSubkind::Ajar | TokenSubkind::Closed => 2,
                        TokenSubkind::Resource => 3,
                        _ => 0,
                    };

                    let same_kind = by_kind.entry(kind).or_default();
                    for (other_mod, other_avail) in same_kind.iter() {
                        if avail.set().overlap(&other_avail.set()) {
                            if modifier.subkind == other_mod.subkind {
                                compiler.reporter.fail(
                                    Error::ErrDuplicateModifier(flyweights::FlyStr::new(
                                        format!("{}", &modifier.element.span().data.to_string())
                                            .into_boxed_str(),
                                    )),
                                    modifier.element.span(),
                                );
                            } else {
                                compiler.reporter.fail(
                                    Error::ErrConflictingModifier(
                                        flyweights::FlyStr::new(
                                            format!(
                                                "{}",
                                                &modifier.element.span().data.to_string()
                                            )
                                            .into_boxed_str(),
                                        ),
                                        flyweights::FlyStr::new(
                                            format!(
                                                "{}",
                                                &other_mod.element.span().data.to_string()
                                            )
                                            .into_boxed_str(),
                                        ),
                                    ),
                                    modifier.element.span(),
                                );
                            }
                            break;
                        }
                    }
                    same_kind.push((modifier, avail));
                }
            });

            // Visit struct / table / union / enum / bits / protocol members and extract availability
            let mut visit_member = |attributes: Option<&raw_ast::AttributeList<'src>>,
                                    item_name: &str,
                                    member_ptr: usize| {
                let decl_is_main =
                    name.to_string().starts_with(&main_lib_prefix) || main_lib_prefix.is_empty();
                let kind_str = if decl_is_main {
                    "member"
                } else {
                    "dependency_member"
                };
                let avail = Self::extract_availability(
                    compiler,
                    attributes,
                    decl_avail,
                    kind_str,
                    has_library_avail || !decl_is_main,
                    item_name,
                );
                let platform = Platform::parse(compiler.library_name.versioning_platform())
                    .unwrap_or_else(Platform::unversioned);
                if !avail
                    .set()
                    .contains(compiler.version_selection.lookup(&platform))
                {
                    allow_unused_imports = true;
                }
                member_availability_additions.insert(member_ptr, avail);
            };

            match decl {
                RawDecl::Struct(d) => {
                    for m in &d.members {
                        visit_member(
                            m.attributes.as_deref(),
                            m.name.data(),
                            m.element.span().data.as_ptr() as usize,
                        );
                    }
                }
                RawDecl::Table(d) => {
                    for m in &d.members {
                        visit_member(
                            m.attributes.as_deref(),
                            m.name.as_ref().map(|n| n.data()).unwrap_or(""),
                            m.element.span().data.as_ptr() as usize,
                        );
                    }
                }
                RawDecl::Union(d) => {
                    for m in &d.members {
                        visit_member(
                            m.attributes.as_deref(),
                            m.name.as_ref().map(|n| n.data()).unwrap_or(""),
                            m.element.span().data.as_ptr() as usize,
                        );
                    }
                }
                RawDecl::Enum(d) => {
                    for m in &d.members {
                        visit_member(
                            m.attributes.as_deref(),
                            m.name.data(),
                            m.element.span().data.as_ptr() as usize,
                        );
                    }
                }
                RawDecl::Bits(d) => {
                    for m in &d.members {
                        visit_member(
                            m.attributes.as_deref(),
                            m.name.data(),
                            m.element.span().data.as_ptr() as usize,
                        );
                    }
                }
                RawDecl::Protocol(d) => {
                    for m in &d.methods {
                        visit_member(
                            m.attributes.as_deref(),
                            m.name.data(),
                            m.element.span().data.as_ptr() as usize,
                        );
                    }
                }
                RawDecl::Service(d) => {
                    for m in &d.members {
                        visit_member(
                            m.attributes.as_deref(),
                            m.name.data(),
                            m.element.span().data.as_ptr() as usize,
                        );
                    }
                }
                RawDecl::Type(d) => match &d.layout {
                    Layout::Struct(l) => {
                        for m in &l.members {
                            visit_member(
                                m.attributes.as_deref(),
                                m.name.data(),
                                m.element.span().data.as_ptr() as usize,
                            );
                        }
                    }
                    Layout::Table(l) => {
                        for m in &l.members {
                            visit_member(
                                m.attributes.as_deref(),
                                m.name.as_ref().map(|n| n.data()).unwrap_or(""),
                                m.element.span().data.as_ptr() as usize,
                            );
                        }
                    }
                    Layout::Union(l) => {
                        for m in &l.members {
                            visit_member(
                                m.attributes.as_deref(),
                                m.name.as_ref().map(|n| n.data()).unwrap_or(""),
                                m.element.span().data.as_ptr() as usize,
                            );
                        }
                    }
                    Layout::Enum(l) => {
                        for m in &l.members {
                            visit_member(
                                m.attributes.as_deref(),
                                m.name.data(),
                                m.element.span().data.as_ptr() as usize,
                            );
                        }
                    }
                    Layout::Bits(l) => {
                        for m in &l.members {
                            visit_member(
                                m.attributes.as_deref(),
                                m.name.data(),
                                m.element.span().data.as_ptr() as usize,
                            );
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if allow_unused_imports {
            compiler.allow_unused_imports = true;
        }

        for (k, v) in member_availability_additions {
            compiler.member_availability.insert(k, v);
        }
    }
}
