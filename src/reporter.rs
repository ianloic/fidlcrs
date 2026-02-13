use crate::diagnostics::{Diagnostic, ErrorDef};
use crate::source_span::SourceSpan;
use std::cell::RefCell;

pub struct Reporter<'a> {
    diagnostics: RefCell<Vec<Diagnostic<'a>>>,
    #[allow(dead_code)]
    warnings_as_errors: bool,
}

impl<'a> Reporter<'a> {
    pub fn new() -> Self {
        Self { diagnostics: RefCell::new(Vec::new()), warnings_as_errors: false }
    }

    pub fn fail(&self, def: ErrorDef, span: SourceSpan<'a>, args: &[&dyn std::fmt::Debug]) {
        // Simple formatting for now.
        // We handle {} placeholders.
        let mut msg = def.msg.to_string();
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
            id: def.id,
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
            if let Some(span) = &diag.span {
                println!("{}: error: {}", span.position_str(), diag.message);
                // Print squiggle?
            } else {
                println!("error: {}", diag.message);
            }
        }
    }
}
