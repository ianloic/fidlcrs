use crate::compiler::to_camel_case;
use crate::source_span::SourceSpan;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Decl,
    LayoutMember,
    MethodRequest,
    MethodResponse,
}

#[derive(Clone, Debug)]
pub enum ContextName<'a> {
    Sourced(SourceSpan<'a>),
    Generated(String),
}

impl<'a> ContextName<'a> {
    pub fn data(&self) -> &str {
        match self {
            Self::Sourced(s) => s.data,
            Self::Generated(s) => s.as_str(),
        }
    }
}

impl<'a> From<SourceSpan<'a>> for ContextName<'a> {
    fn from(s: SourceSpan<'a>) -> Self {
        Self::Sourced(s)
    }
}

impl<'a> From<&str> for ContextName<'a> {
    fn from(s: &str) -> Self {
        Self::Generated(s.to_string())
    }
}

impl<'a> From<String> for ContextName<'a> {
    fn from(s: String) -> Self {
        Self::Generated(s)
    }
}

#[derive(Clone, Debug)]
pub struct NamingContext<'a> {
    pub name: ContextName<'a>,
    pub kind: Kind,
    pub parent: Option<Rc<NamingContext<'a>>>,
    pub flattened_name: String,
    pub flattened_name_override: RefCell<Option<String>>,
}

impl<'a> NamingContext<'a> {
    pub fn create(name: impl Into<ContextName<'a>>) -> Rc<Self> {
        Self::new(name.into(), Kind::Decl, None)
    }

    pub fn from_name(decl_name: &Name<'a>) -> Rc<Self> {
        match decl_name {
            Name::Sourced { span } => Self::create(*span),
            // Assuming created from sourced name primarily
            _ => panic!("cannot create naming context from non-sourced name"),
        }
    }

    fn new(name: ContextName<'a>, kind: Kind, parent: Option<Rc<NamingContext<'a>>>) -> Rc<Self> {
        let flattened_name = Self::build_flattened_name(&name, kind, &parent);
        Rc::new(Self {
            name,
            kind,
            parent,
            flattened_name,
            flattened_name_override: RefCell::new(None),
        })
    }

    fn build_flattened_name(
        name: &ContextName<'a>,
        kind: Kind,
        parent: &Option<Rc<NamingContext<'a>>>,
    ) -> String {
        match kind {
            Kind::Decl => name.data().to_string(),
            Kind::LayoutMember => to_camel_case(name.data()),
            Kind::MethodRequest => {
                let p = parent.as_ref().unwrap();
                format!(
                    "{}{}Request",
                    to_camel_case(p.name.data()),
                    to_camel_case(name.data())
                )
            }
            Kind::MethodResponse => {
                let p = parent.as_ref().unwrap();
                format!(
                    "{}{}Response",
                    to_camel_case(p.name.data()),
                    to_camel_case(name.data())
                )
            }
        }
    }

    pub fn enter_request(self: &Rc<Self>, method_name: impl Into<ContextName<'a>>) -> Rc<Self> {
        assert_eq!(self.kind, Kind::Decl, "request must follow protocol");
        self.push(method_name.into(), Kind::MethodRequest)
    }

    pub fn enter_event(self: &Rc<Self>, method_name: impl Into<ContextName<'a>>) -> Rc<Self> {
        assert_eq!(self.kind, Kind::Decl, "event must follow protocol");
        // in fidlc, event uses MethodRequest kind
        self.push(method_name.into(), Kind::MethodRequest)
    }

    pub fn enter_response(self: &Rc<Self>, method_name: impl Into<ContextName<'a>>) -> Rc<Self> {
        assert_eq!(self.kind, Kind::Decl, "response must follow protocol");
        self.push(method_name.into(), Kind::MethodResponse)
    }

    pub fn enter_member(self: &Rc<Self>, member_name: impl Into<ContextName<'a>>) -> Rc<Self> {
        self.push(member_name.into(), Kind::LayoutMember)
    }

    fn push(self: &Rc<Self>, name: ContextName<'a>, kind: Kind) -> Rc<Self> {
        Self::new(name, kind, Some(self.clone()))
    }

    pub fn set_name_override(&self, value: String) {
        *self.flattened_name_override.borrow_mut() = Some(value);
    }

    pub fn flattened_name(&self) -> String {
        if let Some(ref override_name) = *self.flattened_name_override.borrow() {
            override_name.clone()
        } else {
            self.flattened_name.clone()
        }
    }

    pub fn context(&self) -> Vec<String> {
        let mut names = Vec::new();
        let mut current = Some(self);
        while let Some(ctx) = current {
            match ctx.kind {
                Kind::MethodRequest => names.push("Request".to_string()),
                Kind::MethodResponse => names.push("Response".to_string()),
                Kind::Decl | Kind::LayoutMember => {}
            }
            names.push(ctx.name.data().to_string());
            current = ctx.parent.as_deref();
        }
        names.reverse();
        names
    }

    pub fn to_name(self: &Rc<Self>, declaration_span: SourceSpan<'a>) -> Name<'a> {
        if self.parent.is_none() {
            match self.name {
                ContextName::Sourced(span) => Name::Sourced { span },
                ContextName::Generated(ref n) => Name::Intrinsic { name: n.clone() },
            }
        } else {
            Name::Anonymous {
                context: self.clone(),
                provenance: Provenance::AnonymousLayout,
                span: declaration_span,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Provenance {
    AnonymousLayout,
    GeneratedResultUnion,
    GeneratedEmptySuccessStruct,
}

#[derive(Clone, Debug)]
pub enum Name<'a> {
    Sourced {
        span: SourceSpan<'a>,
    },
    Anonymous {
        context: Rc<NamingContext<'a>>,
        provenance: Provenance,
        span: SourceSpan<'a>,
    },
    Intrinsic {
        name: String,
    },
}

impl<'a> Name<'a> {
    pub fn create_sourced(span: SourceSpan<'a>) -> Self {
        Name::Sourced { span }
    }

    pub fn create_anonymous(
        context: Rc<NamingContext<'a>>,
        provenance: Provenance,
        span: SourceSpan<'a>,
    ) -> Self {
        Name::Anonymous {
            context,
            provenance,
            span,
        }
    }

    pub fn create_intrinsic(name: &str) -> Self {
        Name::Intrinsic {
            name: name.to_string(),
        }
    }

    pub fn span(&self) -> Option<SourceSpan<'a>> {
        match self {
            Name::Sourced { span } => Some(*span),
            Name::Anonymous { span, .. } => Some(*span),
            Name::Intrinsic { .. } => None,
        }
    }

    pub fn decl_name(&self) -> String {
        match self {
            Name::Sourced { span } => span.data.to_string(),
            Name::Anonymous { context, .. } => context.flattened_name(),
            Name::Intrinsic { name } => name.clone(),
        }
    }
}
