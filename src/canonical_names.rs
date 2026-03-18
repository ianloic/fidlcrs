use std::collections::HashMap;
use crate::compiler::MemberKind;
use crate::source_span::SourceSpan;

pub fn canonicalize(name: &str) -> String {
    let mut canonical = String::new();
    let chars: Vec<char> = name.chars().collect();
    let mut prev = '_';
    for i in 0..chars.len() {
        let c = chars[i];
        if c == '_' {
            if prev != '_' {
                canonical.push('_');
            }
        } else if ((prev.is_ascii_lowercase() || prev.is_ascii_digit()) && c.is_ascii_uppercase())
            || (prev != '_'
                && c.is_ascii_uppercase()
                && i + 1 < chars.len()
                && chars[i + 1].is_ascii_lowercase())
        {
            canonical.push('_');
            canonical.push(c.to_ascii_lowercase());
        } else {
            canonical.push(c.to_ascii_lowercase());
        }
        prev = c;
    }
    canonical
}

pub struct CanonicalNameEntry<'src> {
    pub raw: String,
    pub kind: MemberKind,
    pub site: SourceSpan<'src>,
    pub is_versioned: bool,
}

pub struct CanonicalNames<'src> {
    pub names: HashMap<String, CanonicalNameEntry<'src>>,
}

impl<'src> CanonicalNames<'src> {
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        raw_name: String,
        kind: MemberKind,
        span: SourceSpan<'src>,
        is_versioned: bool,
    ) -> Result<(), (bool, String, MemberKind, SourceSpan<'src>)> {
        let canonical = canonicalize(&raw_name);
        if let Some(prev) = self.names.get(&canonical) {
            if is_versioned && prev.is_versioned {
                Ok(())
            } else {
                Err((prev.raw == raw_name, prev.raw.clone(), prev.kind, prev.site))
            }
        } else {
            self.names.insert(
                canonical,
                CanonicalNameEntry {
                    raw: raw_name,
                    kind,
                    site: span,
                    is_versioned,
                },
            );
            Ok(())
        }
    }
}
