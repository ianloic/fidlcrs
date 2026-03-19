use crate::diagnostics::ErrorKind;
use crate::diagnostics::{Diagnostic, Error};
use crate::source_span::SourceSpan;
use std::cell::RefCell;

pub struct Reporter<'a> {
    diagnostics: RefCell<Vec<Diagnostic<'a>>>,
    pub warnings_as_errors: bool,
}

impl<'a> Default for Reporter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Reporter<'a> {
    pub fn new() -> Self {
        Self {
            diagnostics: RefCell::new(Vec::new()),
            warnings_as_errors: false,
        }
    }

    pub fn fail(&self, def: Error, span: SourceSpan<'a>) {
        let msg = def.msg();

        self.diagnostics.borrow_mut().push(Diagnostic {
            def,
            message: msg,
            span: Some(span),
        });
    }

    // Helper to accept varargs?
    // In Rust, macros.

    pub fn diagnostics(&self) -> std::cell::Ref<'_, Vec<Diagnostic<'a>>> {
        self.diagnostics.borrow()
    }

    pub fn print_reports(&self) {
        for diag in self.diagnostics.borrow().iter() {
            let prefix = match diag.def.kind() {
                ErrorKind::Warning => "warning",
                _ => "error",
            };
            if let Some(span) = &diag.span {
                println!("{}: {}: {}", span.position_str(), prefix, diag.message);
            } else {
                println!("{}: {}", prefix, diag.message);
            }
        }
    }
}
