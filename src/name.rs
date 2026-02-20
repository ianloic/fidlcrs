use crate::compiler::to_camel_case;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Decl,
    LayoutMember,
    MethodRequest,
    MethodResponse,
}

#[derive(Clone, Debug)]
pub struct NamingContext {
    pub name: String,
    pub kind: Kind,
    pub parent: Option<Rc<NamingContext>>,
    pub flattened_name: String,
    pub flattened_name_override: std::cell::RefCell<Option<String>>,
}

impl NamingContext {
    pub fn create(decl_name: &str) -> Rc<Self> {
        Rc::new(Self::new(decl_name, Kind::Decl, None))
    }

    fn new(name: &str, kind: Kind, parent: Option<Rc<NamingContext>>) -> Self {
        let flattened_name = Self::build_flattened_name(name, kind, &parent);
        Self {
            name: name.to_string(),
            kind,
            parent,
            flattened_name,
            flattened_name_override: std::cell::RefCell::new(None),
        }
    }

    fn build_flattened_name(name: &str, kind: Kind, parent: &Option<Rc<NamingContext>>) -> String {
        match kind {
            Kind::Decl => name.to_string(),
            Kind::LayoutMember => to_camel_case(name),
            Kind::MethodRequest => {
                let p = parent.as_ref().unwrap();
                format!(
                    "{}{}{}Request",
                    to_camel_case(&p.name),
                    if to_camel_case(name).is_empty() {
                        "".to_string()
                    } else {
                        to_camel_case(name)
                    },
                    ""
                )
            }
            Kind::MethodResponse => {
                let p = parent.as_ref().unwrap();
                format!(
                    "{}{}{}Response",
                    to_camel_case(&p.name),
                    if to_camel_case(name).is_empty() {
                        "".to_string()
                    } else {
                        to_camel_case(name)
                    },
                    ""
                )
            }
        }
    }

    pub fn enter_request(self: &Rc<Self>, method_name: &str) -> Rc<Self> {
        assert_eq!(self.kind, Kind::Decl, "request must follow protocol");
        self.push(method_name, Kind::MethodRequest)
    }

    pub fn enter_event(self: &Rc<Self>, method_name: &str) -> Rc<Self> {
        assert_eq!(self.kind, Kind::Decl, "event must follow protocol");
        self.push(method_name, Kind::MethodRequest)
    }

    pub fn enter_response(self: &Rc<Self>, method_name: &str) -> Rc<Self> {
        assert_eq!(self.kind, Kind::Decl, "response must follow protocol");
        self.push(method_name, Kind::MethodResponse)
    }

    pub fn enter_member(self: &Rc<Self>, member_name: &str) -> Rc<Self> {
        self.push(member_name, Kind::LayoutMember)
    }

    fn push(self: &Rc<Self>, name: &str, kind: Kind) -> Rc<Self> {
        Rc::new(Self::new(name, kind, Some(self.clone())))
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
            names.push(ctx.name.clone());
            current = ctx.parent.as_deref();
        }
        names.reverse();
        names
    }
}
