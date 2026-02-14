#[cfg(test)]
mod tests {
    use crate::source_file::SourceFile;
    use crate::test_library::TestLibrary;
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    fn good_nonzero_size_array() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type S = struct {
    arr array<uint8, 1>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_zero_size_array() {
        let file_content = get_file_content("bad/fi-0161.test.fidl");
        let source = SourceFile::new("bad/fi-0161.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_no_size_array() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type S = struct {
    arr array<uint8>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_non_parameterized_array() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type S = struct {
    arr array;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_optional_array() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type S = struct {
    arr array<uint8, 10>:optional;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_multiple_constraints_on_array() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type S = struct {
    arr array<uint8, 10>:<1, 2, 3>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }
}
