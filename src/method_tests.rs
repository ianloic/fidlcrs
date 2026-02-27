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
    fn good_valid_compose_method() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_strict_compose_method() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_flexible_compose_method() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn disabled__good_valid_strict_method() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn disabled__good_valid_flexible_two_way_method() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_normal_method() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_strict_normal_method() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_flexible_normal_method() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_event() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_strict_event() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_flexible_event() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_valid_strictness_modifiers() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_invalid_strictness_flexible_event_in_closed() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_invalid_strictness_flexible_one_way_method_in_closed() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0116.test.fidl".to_string(),
            get_file_content("bad/fi-0116.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_invalid_strictness_flexible_two_way_method_in_closed() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_invalid_strictness_flexible_two_way_method_in_ajar() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0115.test.fidl".to_string(),
            get_file_content("bad/fi-0115.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_invalid_openness_modifier_on_method() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_valid_empty_payloads() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_invalid_empty_struct_payload_strict_no_error() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_empty_struct_payload_flexible_no_error() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_empty_struct_payload_strict_error() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_empty_struct_payload_flexible_error() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_absent_payload_flexible_no_error() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_absent_payload_strict_error() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_absent_payload_flexible_error() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_flexible_no_error_response_union() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_flexible_error_response_union() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }
}
