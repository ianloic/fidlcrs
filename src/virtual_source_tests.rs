#[cfg(test)]
mod tests {
    use crate::source_file::SourceFile;
    use crate::source_span::SourceSpan;

    #[test]
    fn add_line() {
        let file = SourceFile::new("imaginary-test-file".to_string(), "one\ntwo\nthree".to_string());

        let data = file.data();
        let one = SourceSpan::new(&data[0..3], &file);
        let two = SourceSpan::new(&data[4..7], &file);
        let three = SourceSpan::new(&data[8..13], &file);

        assert_eq!(one.data, "one");
        assert_eq!(two.data, "two");
        assert_eq!(three.data, "three");
    }

    #[test]
    fn line_containing() {
        let file = SourceFile::new("imaginary-test-file".to_string(), "one\ntwo\nthree".to_string());

        let data = file.data();
        let two = SourceSpan::new(&data[4..7], &file);

        let (line_str, pos) = file.line_containing(two.data).expect("Must find line");
        
        assert_eq!(line_str, "two");
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 1);
    }
}
