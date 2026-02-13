use crate::source_file::{SourceFile, Position};

#[derive(Clone, Copy, Debug)]
pub struct SourceSpan<'a> {
    pub data: &'a str,
    pub source_file: &'a SourceFile,
}

impl<'a> SourceSpan<'a> {
    pub fn new(data: &'a str, source_file: &'a SourceFile) -> Self {
        Self { data, source_file }
    }

    pub fn position(&self) -> Position {
        self.source_file.line_containing(self.data).expect("span data must be part of source file").1
    }

    pub fn position_str(&self) -> String {
        let pos = self.position();
        format!("{}:{}:{}", self.source_file.filename(), pos.line, pos.column)
    }
}

impl PartialEq for SourceSpan<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data.as_ptr() == other.data.as_ptr() && self.data.len() == other.data.len()
    }
}

impl Eq for SourceSpan<'_> {}
