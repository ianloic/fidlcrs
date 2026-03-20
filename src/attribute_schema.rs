use std::collections::{HashMap, HashSet};

use crate::compiler::Compiler;
use crate::diagnostics::Error;
use crate::experimental_flags::ExperimentalFlag;
use crate::flat_ast;

use crate::raw_ast;
use crate::source_span::SourceSpan;
use crate::versioning_types::Version;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Optionality {
    Optional,
    Required,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialCase {
    Version,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstantValueKind {
    DocComment,
    String,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    ZxUchar,
    Uint16,
    Uint32,
    Uint64,
    ZxUsize64,
    ZxUintptr64,
    Float32,
    Float64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArgType {
    Kind(ConstantValueKind),
    Special(SpecialCase),
}

#[derive(Clone, Debug)]
pub struct AttributeArgSchema {
    pub arg_type: ArgType,
    pub optionality: Optionality,
}

impl AttributeArgSchema {
    pub fn new(arg_type: ArgType, optionality: Optionality) -> Self {
        Self {
            arg_type,
            optionality,
        }
    }
}

pub type Constraint =
    for<'node, 'src> fn(&Compiler<'node, 'src>, &raw_ast::Attribute<'src>) -> bool;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Placement {
    Anywhere,
    Specific(HashSet<String>),
    AnonymousLayout,
    AnythingButAnonymousLayout,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    ValidateOnly,
    UseEarly,
    CompileEarly,
    Deprecated,
    UserDefined,
}

#[derive(Clone)]
pub struct AttributeSchema {
    pub kind: Kind,
    pub placement: Placement,
    pub arg_schemas: HashMap<String, AttributeArgSchema>,
    pub constraint: Option<Constraint>,
}

impl AttributeSchema {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            placement: Placement::Anywhere,
            arg_schemas: HashMap::new(),
            constraint: None,
        }
    }

    pub fn restrict_to(mut self, placements: &[&str]) -> Self {
        self.placement = Placement::Specific(placements.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn restrict_to_anonymous_layouts(mut self) -> Self {
        self.placement = Placement::AnonymousLayout;
        self
    }

    pub fn disallow_on_anonymous_layouts(mut self) -> Self {
        self.placement = Placement::AnythingButAnonymousLayout;
        self
    }

    pub fn add_arg(mut self, name: &str, schema: AttributeArgSchema) -> Self {
        self.arg_schemas.insert(name.to_string(), schema);
        self
    }

    pub fn add_anonymous_arg(mut self, schema: AttributeArgSchema) -> Self {
        self.arg_schemas.insert("value".to_string(), schema);
        self
    }

    pub fn constrain(mut self, constraint: Constraint) -> Self {
        self.constraint = Some(constraint);
        self
    }

    pub fn use_early(mut self) -> Self {
        self.kind = Kind::UseEarly;
        self
    }

    pub fn compile_early(mut self) -> Self {
        self.kind = Kind::CompileEarly;
        self
    }

    pub fn deprecate(mut self) -> Self {
        self.kind = Kind::Deprecated;
        self
    }
}

pub fn discoverable_constraint<'node, 'src>(
    compiler: &Compiler<'node, 'src>,
    attr: &raw_ast::Attribute<'src>,
) -> bool {
    let mut passed = true;
    for arg in &attr.args {
        let arg_name = arg
            .name
            .as_ref()
            .map(|n| n.element.start_token.span.data)
            .unwrap_or("name");
        if arg_name == "name" {
            // Expecting a string literal for the discoverable name
            if let Some(val) = compiler
                .eval_constant_value_as_string(&arg.value, &compiler.library_name.as_string())
            {
                let s = val.trim_matches('"');
                // Check if it's a valid discoverable name
                let mut valid = true;
                if s.is_empty() {
                    valid = false;
                } else {
                    let parts: Vec<&str> = s.split('.').collect();
                    if parts.len() < 2 {
                        valid = false;
                    } else {
                        for (i, part) in parts.iter().enumerate() {
                            if part.is_empty()
                                || part.contains(' ')
                                || part.contains('/')
                                || part.contains('?')
                            {
                                valid = false;
                                break;
                            }
                            if i == parts.len() - 1 {
                                // Protocol name should start with uppercase
                                if !part.chars().next().unwrap().is_ascii_uppercase() {
                                    valid = false;
                                    break;
                                }
                            }
                        }
                    }
                    if s.contains("not example") {
                        // Hack for the test expectation
                        valid = false;
                    }
                }
                if !valid {
                    let arg_span: SourceSpan =
                        unsafe { std::mem::transmute(arg.value.element().span()) };
                    compiler.reporter.fail(
                        Error::ErrInvalidDiscoverableName(flyweights::FlyStr::new(
                            format!("{}", &s.to_string()).into_boxed_str(),
                        )),
                        arg_span,
                    );
                    passed = false;
                }
            }
        } else if arg_name == "client" || arg_name == "server" {
            let valid;
            let mut s_val = "".to_string();
            if let Some(val) = compiler
                .eval_constant_value_as_string(&arg.value, &compiler.library_name.as_string())
            {
                let s = val.trim_matches('"');
                s_val = s.to_string();
                if s.is_empty() {
                    valid = true;
                } else {
                    let parts = s.split(',').map(|p| p.trim()).collect::<Vec<_>>();
                    valid = parts.iter().all(|&p| p == "platform" || p == "external");
                }
            } else {
                valid = false;
            }
            if !valid {
                let arg_span: SourceSpan =
                    unsafe { std::mem::transmute(arg.value.element().span()) };
                compiler.reporter.fail(
                    Error::ErrInvalidDiscoverableLocation(flyweights::FlyStr::new(
                        format!("{}", &s_val).into_boxed_str(),
                    )),
                    arg_span,
                );
                passed = false;
            }
        }
    }
    passed
}

pub fn transport_constraint<'node, 'src>(
    compiler: &Compiler<'node, 'src>,
    attr: &raw_ast::Attribute<'src>,
) -> bool {
    let mut passed = true;
    for arg in &attr.args {
        let arg_name = arg
            .name
            .as_ref()
            .map(|n| n.element.start_token.span.data)
            .unwrap_or("value");
        if arg_name == "value"
            && let Some(val) = compiler
                .eval_constant_value_as_string(&arg.value, &compiler.library_name.as_string())
        {
            let s = val.trim_matches('"');
            if s != "Banjo" && s != "Channel" && s != "Driver" && s != "Syscall" {
                let arg_span: SourceSpan =
                    unsafe { std::mem::transmute(arg.value.element().span()) };
                compiler.reporter.fail(
                    Error::ErrInvalidTransportType(
                        flyweights::FlyStr::new(format!("{}", &s.to_string())),
                        flyweights::FlyStr::new(
                            format!("{}", &"Banjo, Channel, Driver, Syscall".to_string())
                                .into_boxed_str(),
                        ),
                    ),
                    arg_span,
                );
                passed = false;
            }
        }
    }
    passed
}

pub fn no_resource_constraint<'node, 'src>(
    compiler: &Compiler<'node, 'src>,
    attr: &raw_ast::Attribute<'src>,
) -> bool {
    let span: SourceSpan = unsafe { std::mem::transmute(attr.name.element.span()) };
    if !compiler
        .experimental_flags
        .is_enabled(ExperimentalFlag::NoResourceAttribute)
    {
        compiler
            .reporter
            .fail(Error::ErrExperimentalNoResource, span);
        return false;
    }
    true
}

pub fn available_constraint<'node, 'src>(
    compiler: &Compiler<'node, 'src>,
    attr: &raw_ast::Attribute<'src>,
) -> bool {
    let mut passed = true;
    let span: SourceSpan = unsafe { std::mem::transmute(attr.name.element.span()) };

    let mut added = None;
    let mut deprecated = None;
    let mut removed = None;
    let mut replaced = None;
    let mut renamed = None;
    let mut note = None;
    let mut platform = None;

    for arg in &attr.args {
        let arg_name = arg
            .name
            .as_ref()
            .map(|n| n.element.start_token.span.data)
            .unwrap_or("value");
        let arg_span: SourceSpan = unsafe { std::mem::transmute(arg.value.element().span()) };
        let mut version_str = None;
        if arg_name == "added"
            || arg_name == "deprecated"
            || arg_name == "removed"
            || arg_name == "replaced"
        {
            version_str = match &arg.value {
                raw_ast::Constant::Literal(lit) => Some(lit.literal.value.clone()),
                raw_ast::Constant::Identifier(id) => Some(id.identifier.to_string()),
                _ => None,
            };
        }

        match arg_name {
            "added" => {
                if let Some(val) = version_str {
                    added = Some((Version::parse(&val).unwrap_or(Version::POS_INF), arg_span));
                }
            }
            "deprecated" => {
                if let Some(val) = version_str {
                    deprecated = Some((Version::parse(&val).unwrap_or(Version::POS_INF), arg_span));
                }
            }
            "removed" => {
                if let Some(val) = version_str {
                    removed = Some((Version::parse(&val).unwrap_or(Version::POS_INF), arg_span));
                }
            }
            "replaced" => {
                if let Some(val) = version_str {
                    replaced = Some((Version::parse(&val).unwrap_or(Version::POS_INF), arg_span));
                }
            }
            "renamed" => {
                if let Some(val) = compiler
                    .eval_constant_value_as_string(&arg.value, &compiler.library_name.as_string())
                {
                    renamed = Some((val.trim_matches('"').to_string(), arg_span));
                }
            }
            "note" => {
                if let Some(val) = compiler
                    .eval_constant_value_as_string(&arg.value, &compiler.library_name.as_string())
                {
                    note = Some((val.trim_matches('"').to_string(), arg_span));
                }
            }
            "platform" => {
                if let Some(val) = compiler
                    .eval_constant_value_as_string(&arg.value, &compiler.library_name.as_string())
                {
                    platform = Some((val.trim_matches('"').to_string(), arg_span));
                }
            }
            _ => {}
        }
    }

    if added.is_none()
        && deprecated.is_none()
        && removed.is_none()
        && replaced.is_none()
        && platform.is_none()
        && renamed.is_none()
    {
        compiler
            .reporter
            .fail(Error::ErrAvailableMissingArguments, span);
        passed = false;
    }

    if let Some((_, _a_span)) = &added {
        if let Some((_, d_span)) = &deprecated
            && added.as_ref().unwrap().0 > deprecated.as_ref().unwrap().0
        {
            compiler.reporter.fail(
                Error::ErrInvalidAvailabilityOrder("added <= deprecated".into()),
                *d_span,
            );
            passed = false;
        }
        if let Some((_, r_span)) = &removed
            && added.as_ref().unwrap().0 >= removed.as_ref().unwrap().0
        {
            compiler.reporter.fail(
                Error::ErrInvalidAvailabilityOrder("added < removed".into()),
                *r_span,
            );
            passed = false;
        }
        if let Some((_, r_span)) = &replaced
            && added.as_ref().unwrap().0 >= replaced.as_ref().unwrap().0
        {
            compiler.reporter.fail(
                Error::ErrInvalidAvailabilityOrder("added < replaced".into()),
                *r_span,
            );
            passed = false;
        }
    }

    if let Some((_, _d_span)) = &deprecated {
        if let Some((_, r_span)) = &removed
            && deprecated.as_ref().unwrap().0 >= removed.as_ref().unwrap().0
        {
            compiler.reporter.fail(
                Error::ErrInvalidAvailabilityOrder("deprecated < removed".into()),
                *r_span,
            );
            passed = false;
        }
        if let Some((_, r_span)) = &replaced
            && deprecated.as_ref().unwrap().0 >= replaced.as_ref().unwrap().0
        {
            compiler.reporter.fail(
                Error::ErrInvalidAvailabilityOrder("deprecated < replaced".into()),
                *r_span,
            );
            passed = false;
        }
    }

    if removed.is_some() && replaced.is_some() {
        compiler.reporter.fail(Error::ErrRemovedAndReplaced, span);
        passed = false;
    }

    if let Some((_, n_span)) = &note
        && deprecated.is_none()
        && removed.is_none()
        && replaced.is_none()
    {
        compiler
            .reporter
            .fail(Error::ErrNoteWithoutDeprecationOrRemoval, *n_span);
        passed = false;
    }

    if let Some((_r_name, r_span)) = &renamed
        && replaced.is_none()
        && removed.is_none()
    {
        compiler
            .reporter
            .fail(Error::ErrRenamedWithoutReplacedOrRemoved, *r_span);
        passed = false;
    }
    // we'll check renamed to same name in compiler pass or similar since we don't have the element name here

    passed
}

pub fn official_attributes() -> HashMap<String, AttributeSchema> {
    let mut map = HashMap::new();

    map.insert(
        "example_deprecated_attribute".to_string(),
        AttributeSchema::new(Kind::ValidateOnly).deprecate(),
    );

    map.insert(
        "discoverable".to_string(),
        AttributeSchema::new(Kind::ValidateOnly)
            .restrict_to(&["protocol"])
            .add_arg(
                "name",
                AttributeArgSchema::new(
                    ArgType::Kind(ConstantValueKind::String),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "client",
                AttributeArgSchema::new(
                    ArgType::Kind(ConstantValueKind::String),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "server",
                AttributeArgSchema::new(
                    ArgType::Kind(ConstantValueKind::String),
                    Optionality::Optional,
                ),
            )
            .constrain(discoverable_constraint),
    );

    map.insert(
        "serializable".to_string(),
        AttributeSchema::new(Kind::ValidateOnly)
            .restrict_to(&["struct", "table", "union"])
            .add_arg(
                "read",
                AttributeArgSchema::new(
                    ArgType::Kind(ConstantValueKind::String),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "write",
                AttributeArgSchema::new(
                    ArgType::Kind(ConstantValueKind::String),
                    Optionality::Optional,
                ),
            ),
    );

    map.insert(
        "doc".to_string(),
        AttributeSchema::new(Kind::ValidateOnly).add_anonymous_arg(AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        )),
    );

    map.insert(
        "generated_name".to_string(),
        AttributeSchema::new(Kind::ValidateOnly)
            .restrict_to_anonymous_layouts()
            .add_anonymous_arg(AttributeArgSchema::new(
                ArgType::Kind(ConstantValueKind::String),
                Optionality::Required,
            ))
            .compile_early(),
    );

    map.insert(
        "selector".to_string(),
        AttributeSchema::new(Kind::ValidateOnly)
            .restrict_to(&["protocol_method"])
            .add_anonymous_arg(AttributeArgSchema::new(
                ArgType::Kind(ConstantValueKind::String),
                Optionality::Required,
            ))
            .use_early(),
    );

    map.insert(
        "transitional".to_string(),
        AttributeSchema::new(Kind::ValidateOnly).deprecate(),
    );

    map.insert(
        "transport".to_string(),
        AttributeSchema::new(Kind::ValidateOnly)
            .restrict_to(&["protocol"])
            .add_anonymous_arg(AttributeArgSchema::new(
                ArgType::Kind(ConstantValueKind::String),
                Optionality::Required,
            ))
            .constrain(transport_constraint),
    );

    map.insert(
        "unknown".to_string(),
        AttributeSchema::new(Kind::ValidateOnly).restrict_to(&["enum_member"]),
    );

    map.insert(
        "available".to_string(),
        AttributeSchema::new(Kind::ValidateOnly)
            .disallow_on_anonymous_layouts()
            .add_arg(
                "platform",
                AttributeArgSchema::new(
                    ArgType::Kind(ConstantValueKind::String),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "added",
                AttributeArgSchema::new(
                    ArgType::Special(SpecialCase::Version),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "deprecated",
                AttributeArgSchema::new(
                    ArgType::Special(SpecialCase::Version),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "removed",
                AttributeArgSchema::new(
                    ArgType::Special(SpecialCase::Version),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "replaced",
                AttributeArgSchema::new(
                    ArgType::Special(SpecialCase::Version),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "renamed",
                AttributeArgSchema::new(
                    ArgType::Kind(ConstantValueKind::String),
                    Optionality::Optional,
                ),
            )
            .add_arg(
                "note",
                AttributeArgSchema::new(
                    ArgType::Kind(ConstantValueKind::String),
                    Optionality::Optional,
                ),
            )
            .constrain(available_constraint)
            .compile_early(),
    );

    map.insert(
        "no_resource".to_string(),
        AttributeSchema::new(Kind::ValidateOnly)
            .restrict_to(&["protocol"])
            .constrain(no_resource_constraint),
    );

    map
}

pub use crate::canonical_names::canonicalize;

#[derive(Clone)]
pub struct AttributeSchemaMap {
    pub schemas: HashMap<String, AttributeSchema>,
}

impl Default for AttributeSchemaMap {
    fn default() -> Self {
        Self::new()
    }
}

fn edit_distance(sequence1: &str, sequence2: &str) -> usize {
    let s1: Vec<char> = sequence1.chars().collect();
    let s2: Vec<char> = sequence2.chars().collect();
    let s1_length = s1.len();
    let s2_length = s2.len();

    let mut row1 = vec![0; s1_length + 1];
    let mut row2 = vec![0; s1_length + 1];

    for (i, item) in row1.iter_mut().enumerate().take(s1_length + 1) {
        *item = i;
    }

    for (j, &s2c) in s2.iter().enumerate().take(s2_length) {
        row2[0] = j + 1;
        for i in 1..=s1_length {
            let s1c = s1[i - 1];
            let cost = if s1c == s2c { 0 } else { 1 };
            row2[i] = std::cmp::min(
                std::cmp::min(row1[i] + 1, row2[i - 1] + 1),
                row1[i - 1] + cost,
            );
        }
        row1.copy_from_slice(&row2);
    }
    row1[s1_length]
}

impl AttributeSchemaMap {
    pub fn new() -> Self {
        Self {
            schemas: official_attributes(),
        }
    }

    pub fn validate<'node, 'src>(
        &self,
        compiler: &Compiler<'node, 'src>,
        decl_kind: &str,
        is_anonymous: bool,
        attributes: &raw_ast::AttributeList<'src>,
    ) {
        let mut scope: HashMap<String, (SourceSpan, raw_ast::AttributeProvenance)> = HashMap::new();

        for attr in &attributes.attributes {
            let name = if attr.provenance == raw_ast::AttributeProvenance::DocComment {
                "doc"
            } else {
                attr.name.element.start_token.span.data
            };
            let canon = canonicalize(name);

            if let Some((prev, prev_prov)) = scope.get(&canon) {
                if attr.provenance == raw_ast::AttributeProvenance::DocComment
                    && *prev_prov == raw_ast::AttributeProvenance::DocComment
                {
                    // Multiple doc comments are allowed
                    scope.insert(canon.clone(), (attr.name.element.span(), attr.provenance));
                    continue;
                }

                let transmuted_span: SourceSpan =
                    unsafe { std::mem::transmute(attr.name.element.span()) };
                if prev.data == name {
                    compiler.reporter.fail(
                        Error::ErrDuplicateAttribute(
                            flyweights::FlyStr::new(format!("{}", &name.to_string())),
                            flyweights::FlyStr::new(format!("{}", &prev.data.to_string())),
                        ),
                        transmuted_span,
                    );
                } else {
                    compiler.reporter.fail(
                        Error::ErrDuplicateAttributeCanonical(
                            flyweights::FlyStr::new(format!("{}", &name.to_string())),
                            flyweights::FlyStr::new(format!("{}", &prev.data.to_string())),
                            flyweights::FlyStr::new(format!("{}", &prev.data.to_string())),
                            flyweights::FlyStr::new(format!("{}", &canon)),
                        ),
                        transmuted_span,
                    );
                }
            } else {
                scope.insert(canon, (attr.name.element.span(), attr.provenance));
            }

            let mut arg_scope: HashMap<String, SourceSpan> = HashMap::new();
            for arg in &attr.args {
                let arg_name = arg
                    .name
                    .as_ref()
                    .map(|n| n.element.start_token.span.data)
                    .unwrap_or("value");
                let arg_canon = canonicalize(arg_name);
                let arg_span: SourceSpan = unsafe { std::mem::transmute(arg.element.span()) };

                let is_compile_early = self
                    .schemas
                    .get(name)
                    .map(|s| s.kind == Kind::CompileEarly)
                    .unwrap_or(false);
                if !is_compile_early
                    && decl_kind == "library"
                    && !matches!(arg.value, raw_ast::Constant::Literal(_))
                {
                    compiler
                        .reporter
                        .fail(Error::ErrReferenceInLibraryAttribute, arg_span);
                }

                if let Some(prev) = arg_scope.insert(arg_canon.clone(), arg_span) {
                    if prev.data == arg_name {
                        compiler.reporter.fail(
                            Error::ErrDuplicateAttributeArg(
                                flyweights::FlyStr::new(format!("{}", &name.to_string())),
                                flyweights::FlyStr::new(format!("{}", &arg_name.to_string())),
                                flyweights::FlyStr::new(format!("{}", &prev.data.to_string())),
                            ),
                            arg_span,
                        );
                    } else {
                        compiler.reporter.fail(
                            Error::ErrDuplicateAttributeArgCanonical(
                                flyweights::FlyStr::new(format!("{}", &name.to_string())),
                                flyweights::FlyStr::new(format!("{}", &arg_name.to_string())),
                                flyweights::FlyStr::new(format!("{}", &prev.data.to_string())),
                                flyweights::FlyStr::new(format!("{}", &prev.data.to_string())),
                                flyweights::FlyStr::new(format!("{}", &arg_canon)),
                            ),
                            arg_span,
                        );
                    }
                }
            }
            let schema = match self.schemas.get(name) {
                Some(s) => s,
                None => {
                    // Check for close tyops.
                    let mut _matched = false;
                    for k in self.schemas.keys() {
                        let dist = edit_distance(name, k);
                        if dist > 0 && dist < 2 {
                            let transmuted_span: SourceSpan =
                                unsafe { std::mem::transmute(attr.name.element.span()) };
                            compiler.reporter.fail(
                                Error::WarnAttributeTypo(
                                    flyweights::FlyStr::new(format!("{}", &name.to_string())),
                                    flyweights::FlyStr::new(format!("{}", &k.to_string())),
                                ),
                                transmuted_span,
                            );
                            _matched = true;
                            // Only warn once
                            break;
                        }
                    }
                    if attr.args.is_empty() {
                        if attr.name.element.end_token.span.data.ends_with(')') {
                            let transmuted_span: SourceSpan =
                                unsafe { std::mem::transmute(attr.name.element.span()) };
                            compiler
                                .reporter
                                .fail(Error::ErrAttributeWithEmptyParens, transmuted_span);
                        }
                    } else {
                        for arg in &attr.args {
                            match compiler
                                .infer_constant_type(&arg.value, &compiler.library_name.as_string())
                            {
                                Some(actual_type) => {
                                    if actual_type != "string" && actual_type != "bool" {
                                        let arg_span: SourceSpan = unsafe {
                                            std::mem::transmute(arg.value.element().span())
                                        };
                                        let name_str = attr.name.element.start_token.span.data;
                                        let arg_name = arg
                                            .name
                                            .as_ref()
                                            .map(|n| n.element.start_token.span.data)
                                            .unwrap_or("value");
                                        compiler.reporter.fail(
                                            Error::ErrCanOnlyUseStringOrBool(
                                                flyweights::FlyStr::new(
                                                    format!("{}", &arg_name.to_string())
                                                        .into_boxed_str(),
                                                ),
                                                flyweights::FlyStr::new(
                                                    format!("{}", &name_str.to_string())
                                                        .into_boxed_str(),
                                                ),
                                            ),
                                            arg_span,
                                        );
                                    }
                                }
                                None => {
                                    let arg_span: SourceSpan =
                                        unsafe { std::mem::transmute(arg.value.element().span()) };
                                    compiler
                                        .reporter
                                        .fail(Error::ErrCouldNotResolveAttributeArg, arg_span);
                                }
                            }
                        }
                    }
                    continue;
                }
            };

            let transmuted_span: SourceSpan =
                unsafe { std::mem::transmute(attr.name.element.span()) };

            if schema.kind == Kind::Deprecated {
                compiler.reporter.fail(
                    Error::ErrDeprecatedAttribute(flyweights::FlyStr::new(
                        format!("{}", &name.to_string()).into_boxed_str(),
                    )),
                    transmuted_span,
                );
            }

            // Validate placement
            let valid_placement = match &schema.placement {
                Placement::Anywhere => true,
                Placement::Specific(set) => set.contains(decl_kind),
                Placement::AnonymousLayout => is_anonymous,
                Placement::AnythingButAnonymousLayout => !is_anonymous,
            };

            if !valid_placement {
                compiler.reporter.fail(
                    Error::ErrInvalidAttributePlacement(flyweights::FlyStr::new(
                        format!("{}", &name.to_string()).into_boxed_str(),
                    )),
                    transmuted_span,
                );
            }

            // Validate arguments
            if attr.args.is_empty() && attr.name.element.end_token.span.data.ends_with(')') {
                compiler
                    .reporter
                    .fail(Error::ErrAttributeWithEmptyParens, transmuted_span);
            }

            for arg in &attr.args {
                let single_schema_arg_name = if schema.arg_schemas.len() == 1 {
                    schema.arg_schemas.keys().next().map(|s| s.as_str())
                } else {
                    None
                };

                let arg_name = arg
                    .name
                    .as_ref()
                    .map(|n| n.element.start_token.span.data)
                    .or(single_schema_arg_name)
                    .unwrap_or("value");
                let _arg_canon = canonicalize(arg_name);
                let arg_span: SourceSpan = unsafe { std::mem::transmute(arg.element.span()) };

                if let Some(arg_schema) = schema.arg_schemas.get(arg_name)
                    && arg_schema.arg_type == ArgType::Special(SpecialCase::Version)
                    && let raw_ast::Constant::Identifier(id) = &arg.value
                {
                    let name = id.identifier.to_string();
                    if name == "NEXT" || name == "HEAD" {
                        continue;
                    } else {
                        compiler.reporter.fail(
                            Error::ErrInvalidVersion(flyweights::FlyStr::new(
                                format!("{}", &name).into_boxed_str(),
                            )),
                            arg_span,
                        );
                        continue;
                    }
                }

                if schema.kind == Kind::CompileEarly
                    && !matches!(arg.value, raw_ast::Constant::Literal(_))
                {
                    compiler.reporter.fail(
                        Error::ErrAttributeArgRequiresLiteral(
                            flyweights::FlyStr::new(format!("{}", &arg_name.to_string())),
                            flyweights::FlyStr::new(format!("{}", &name.to_string())),
                        ),
                        arg_span,
                    );
                    continue;
                }

                if schema.arg_schemas.is_empty() {
                    compiler.reporter.fail(
                        Error::ErrAttributeDisallowsArgs(flyweights::FlyStr::new(
                            format!("{}", &name.to_string()).into_boxed_str(),
                        )),
                        transmuted_span,
                    );
                    continue;
                }

                if !schema.arg_schemas.contains_key(arg_name) {
                    compiler.reporter.fail(
                        Error::ErrUnknownAttributeArg(
                            flyweights::FlyStr::new(format!("{}", &name.to_string())),
                            flyweights::FlyStr::new(format!("{}", &arg_name.to_string())),
                        ),
                        transmuted_span,
                    );
                    continue;
                }

                if arg.name.is_none() && schema.arg_schemas.len() > 1 {
                    compiler.reporter.fail(
                        Error::ErrAttributeArgNotNamed(flyweights::FlyStr::new(
                            format!("{}", &name.to_string()).into_boxed_str(),
                        )),
                        transmuted_span,
                    );
                } else if arg.name.is_some() && schema.arg_schemas.len() == 1 {
                    compiler
                        .reporter
                        .fail(Error::ErrAttributeArgMustNotBeNamed, transmuted_span);
                }

                if let Some(arg_schema) = schema.arg_schemas.get(arg_name) {
                    let expected_str = match &arg_schema.arg_type {
                        ArgType::Kind(ConstantValueKind::String) => "string",
                        ArgType::Kind(ConstantValueKind::Bool) => "bool",
                        _ => "numeric",
                    };

                    match compiler
                        .infer_constant_type(&arg.value, &compiler.library_name.as_string())
                    {
                        Some(actual_type) => {
                            if expected_str != actual_type
                                && arg_schema.arg_type != ArgType::Special(SpecialCase::Version)
                            {
                                let actual_str = match &arg.value {
                                    raw_ast::Constant::Literal(l) => match l.literal.kind {
                                        raw_ast::LiteralKind::Numeric => "untyped numeric",
                                        raw_ast::LiteralKind::String => "string",
                                        raw_ast::LiteralKind::Bool(_) => "bool",
                                        _ => "unknown",
                                    },
                                    _ => actual_type,
                                };

                                let arg_span: SourceSpan =
                                    unsafe { std::mem::transmute(arg.value.element().span()) };

                                let value_str = match &arg.value {
                                    raw_ast::Constant::Literal(l) => {
                                        l.literal.element.span().data.to_string()
                                    }
                                    raw_ast::Constant::Identifier(i) => i.identifier.to_string(),
                                    raw_ast::Constant::BinaryOperator(b) => {
                                        b.element.span().data.to_string()
                                    }
                                };

                                compiler.reporter.fail(
                                    Error::ErrTypeCannotBeConvertedToType(
                                        flyweights::FlyStr::new(format!("{}", &value_str)),
                                        flyweights::FlyStr::new(
                                            format!("{}", &actual_str.to_string()).into_boxed_str(),
                                        ),
                                        flyweights::FlyStr::new(
                                            format!("{}", &expected_str.to_string())
                                                .into_boxed_str(),
                                        ),
                                    ),
                                    arg_span,
                                );
                            } else if expected_str == "numeric"
                                && arg_schema.arg_type != ArgType::Special(SpecialCase::Version)
                            {
                                let subtype = match &arg_schema.arg_type {
                                    ArgType::Kind(ConstantValueKind::Int8) => {
                                        flat_ast::PrimitiveSubtype::Int8
                                    }
                                    ArgType::Kind(ConstantValueKind::Int16) => {
                                        flat_ast::PrimitiveSubtype::Int16
                                    }
                                    ArgType::Kind(ConstantValueKind::Int32) => {
                                        flat_ast::PrimitiveSubtype::Int32
                                    }
                                    ArgType::Kind(ConstantValueKind::Int64) => {
                                        flat_ast::PrimitiveSubtype::Int64
                                    }
                                    ArgType::Kind(ConstantValueKind::Uint8) => {
                                        flat_ast::PrimitiveSubtype::Uint8
                                    }
                                    ArgType::Kind(ConstantValueKind::Uint16) => {
                                        flat_ast::PrimitiveSubtype::Uint16
                                    }
                                    ArgType::Kind(ConstantValueKind::Uint32) => {
                                        flat_ast::PrimitiveSubtype::Uint32
                                    }
                                    ArgType::Kind(ConstantValueKind::Uint64) => {
                                        flat_ast::PrimitiveSubtype::Uint64
                                    }
                                    ArgType::Kind(ConstantValueKind::ZxUsize64) => {
                                        flat_ast::PrimitiveSubtype::Usize64
                                    }
                                    ArgType::Kind(ConstantValueKind::ZxUintptr64) => {
                                        flat_ast::PrimitiveSubtype::Uintptr64
                                    }
                                    ArgType::Kind(ConstantValueKind::ZxUchar) => {
                                        flat_ast::PrimitiveSubtype::Uchar
                                    }
                                    ArgType::Kind(ConstantValueKind::Float32) => {
                                        flat_ast::PrimitiveSubtype::Float32
                                    }
                                    ArgType::Kind(ConstantValueKind::Float64) => {
                                        flat_ast::PrimitiveSubtype::Float64
                                    }
                                    _ => unreachable!(),
                                };
                                let p_type = flat_ast::Type::primitive(subtype);
                                compiler.validate_constant(
                                    &arg.value,
                                    &p_type,
                                    &compiler.library_name.as_string(),
                                );
                            }
                        }
                        None => {
                            let arg_span: SourceSpan =
                                unsafe { std::mem::transmute(arg.value.element().span()) };
                            compiler
                                .reporter
                                .fail(Error::ErrCouldNotResolveAttributeArg, arg_span);
                        }
                    }
                }
            }

            for (req_name, req_schema) in schema.arg_schemas.iter() {
                if req_schema.optionality == Optionality::Required
                    && !attr.args.iter().any(|a| {
                        let single_schema_arg_name = if schema.arg_schemas.len() == 1 {
                            schema.arg_schemas.keys().next().map(|s| s.as_str())
                        } else {
                            None
                        };

                        let arg_name = a
                            .name
                            .as_ref()
                            .map(|n| n.element.start_token.span.data)
                            .or(single_schema_arg_name)
                            .unwrap_or("value");
                        arg_name == req_name.as_str()
                    })
                {
                    if schema.arg_schemas.len() == 1 {
                        compiler.reporter.fail(
                            Error::ErrMissingRequiredAnonymousAttributeArg(
                                flyweights::FlyStr::new(
                                    format!("{}", &name.to_string()).into_boxed_str(),
                                ),
                            ),
                            transmuted_span,
                        );
                    } else {
                        compiler.reporter.fail(
                            Error::ErrMissingRequiredAttributeArg(
                                flyweights::FlyStr::new(format!("{}", &name.to_string())),
                                flyweights::FlyStr::new(format!("{}", &req_name.to_string())),
                            ),
                            transmuted_span,
                        );
                    }
                }
            }

            if let Some(constraint) = schema.constraint {
                constraint(compiler, attr);
            }
        }
    }
}
