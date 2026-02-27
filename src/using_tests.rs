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
    fn good_using() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_using_alias() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_using_swap_names() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_decl_with_same_name_as_aliased_library() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_missing_using() {
        let mut lib = TestLibrary::new(); // TODO unhandled format
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_unknown_using() {
        let source = SourceFile::new(
            "bad/fi-0046.test.fidl".to_string(),
            get_file_content("bad/fi-0046.test.fidl"),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_using_alias_ref_through_fqn() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_duplicate_using_no_alias() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_duplicate_using_first_alias() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_duplicate_using_second_alias() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_duplicate_using_same_library_same_alias() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_duplicate_using_same_library_different_alias() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_conflicting_using_library_and_alias() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_conflicting_using_alias_and_library() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_conflicting_using_alias_and_alias() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_unused_using() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_unknown_dependent_library() {
        let source = SourceFile::new(
            "bad/fi-0051.test.fidl".to_string(),
            get_file_content("bad/fi-0051.test.fidl"),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_too_many_provided_libraries() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_library_declaration_name_collision() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_aliased_library_declaration_name_collision() {
        // SharedAmongstLibraries logic
        let mut lib = TestLibrary::new(); // TODO 
        assert!(lib.compile().is_err());
    }
}
