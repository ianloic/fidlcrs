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

    pub fn fail(&self, def: Error, span: SourceSpan<'a>, args: &[&dyn std::fmt::Debug]) {
        // Simple formatting for now.
        // We handle {} placeholders.
        let mut msg = def.msg().to_string();
        for arg in args {
            // Debug fmt
            msg = msg.replacen("{}", &format!("{:?}", arg), 1);
        }
        // Handle {:x} or similar?
        // Rust's format! is macro based, difficult to use with dyn args and templates at runtime.
        // For now, let's just append args if message has placeholders, or rely on simple replacement.
        // The C++ code uses fmt::format which supports positional args.
        // Here we can use a simpler approach for the initial port.

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
