use std::ops::Range;

#[derive(Debug)]
pub struct SourceFile {
    filename: String,
    data: String,
    lines: Vec<Range<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl SourceFile {
    pub fn new(filename: String, data: String) -> Self {
        let mut lines = Vec::new();
        let mut start = 0;
        for (i, c) in data.char_indices() {
            if c == '\n' || c == '\0' {
                lines.push(start..i);
                start = i + 1;
            }
        }
        // Include the last line if the file does not end in a newline
        // Note: The C++ implementation includes {start, size}
        // If data ends with \n, start will be len.
        // If data is "abc", start is 0, loop doesn't hit \n.
        // We need to handle the loop correctly.
        // Re-implementing logic:

        lines.clear();
        start = 0;
        let mut len = 0;
        for (i, b) in data.as_bytes().iter().enumerate() {
            if *b == b'\n' || *b == 0 {
                lines.push(start..(start + len));
                start = i + 1;
                len = 0;
            } else {
                len += 1;
            }
        }

        if len > 0 {
            lines.push(start..(start + len));
        }

        // Use simpler logic: split by \n, but keep track of indices.
        // Actually the C++ logic handles \0 as newline too?
        // "if (*it == '\n' || *it == '\0')"

        Self {
            filename,
            data,
            lines,
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn line_containing(&self, view: &str) -> Option<(&str, Position)> {
        // Verify view is part of data
        let self_start = self.data.as_ptr() as usize;
        let view_start = view.as_ptr() as usize;
        let self_end = self_start + self.data.len();
        let view_end = view_start + view.len();

        if view_end > self_end || view_start < self_start {
            // Handle EOF case (view is empty and at the end)
            if view.is_empty() && view_start == self_end {
                if self.data.is_empty() {
                    return Some(("", Position { line: 1, column: 1 }));
                }
                let last_line = self.lines.last()?;
                let line_str = &self.data[last_line.clone()];
                // Position is line number (1-based), column (1-based)
                return Some((
                    line_str,
                    Position {
                        line: self.lines.len(),
                        column: last_line.len() + 1,
                    },
                ));
            }
            return None;
        }

        // Find the line containing the view
        // Is upper_bound of lines.
        // lines are sorted by start index.
        // We want the last line that starts <= view_start.

        // Relative offset
        let offset = view_start - self_start;

        // Binary search
        let _line_idx = match self.lines.binary_search_by(|range| {
            if range.start > offset {
                std::cmp::Ordering::Greater
            } else if range.end < offset && range.end <= offset {
                // Wait, line range is start..end (exclusive of newline)
                // If offset is within range, it's Equal.
                // If offset is at newline, it might be tricky.
                // C++ uses std::upper_bound looking for first line starting *after* the token?
                // No, "looking for the first line ... to start at or before".
                // Abstractly: find max i such that lines[i].start <= offset.
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }) {
            Ok(idx) => idx,
            Err(idx) => idx - 1, // idx is where it would be inserted (first greater), so idx-1 is last smaller or equal
        };

        // Need to be careful with Err(0)

        // Let's use simple search for now or fix binary search logic.
        // We want index i such that lines[i].start <= offset < lines[i+1].start

        let idx = self.lines.partition_point(|range| range.start <= offset);
        // partition_point returns the first index where predicate is false.
        // So lines[idx] has start > offset.
        // So lines[idx-1] has start <= offset.

        if idx == 0 {
            // Should not happen if data contains the view and lines are populated correctly.
            // Unless partial view implies something else.
            return None;
        }
        let line_idx = idx - 1;
        let line_range = &self.lines[line_idx];

        let line_str = &self.data[line_range.clone()];
        let column = offset - line_range.start + 1;

        Some((
            line_str,
            Position {
                line: line_idx + 1,
                column,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines() {
        let sf = SourceFile::new("test".to_string(), "abc\ndef".to_string());
        assert_eq!(sf.lines.len(), 2);
        assert_eq!(&sf.data[sf.lines[0].clone()], "abc");
        assert_eq!(&sf.data[sf.lines[1].clone()], "def");
    }
}
