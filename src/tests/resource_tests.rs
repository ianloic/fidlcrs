
    use crate::source_file::SourceFile;
    use crate::tests::test_library::TestLibrary;
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]

    fn good_valid_without_rights() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type MyEnum = strict enum : uint32 {
    NONE = 0;
};

resource_definition SomeResource : uint32 {
    properties {
        subtype MyEnum;
    };
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let _root = match lib.compile() {
            Ok(root) => root,
            Err(e) => {
                lib.reporter().print_reports();
                panic!("compilation failed: {}", e);
            }
        };
        // We will need to check ExperimentalResourceDeclaration once it is implemented.
    }

    #[test]

    fn good_valid_with_rights() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type MyEnum = strict enum : uint32 {
    NONE = 0;
};

resource_definition SomeResource : uint32 {
    properties {
        subtype MyEnum;
        rights uint32;
    };
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let _root = match lib.compile() {
            Ok(root) => root,
            Err(e) => {
                lib.reporter().print_reports();
                panic!("compilation failed: {}", e);
            }
        };
    }

    #[test]

    fn good_aliased_base_type_without_rights() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type MyEnum = strict enum : uint32 {
    NONE = 0;
};

alias via = uint32;

resource_definition SomeResource : via {
    properties {
        subtype MyEnum;
    };
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let _root = match lib.compile() {
            Ok(root) => root,
            Err(e) => {
                lib.reporter().print_reports();
                panic!("compilation failed: {}", e);
            }
        };
    }

    #[test]

    fn good_aliased_base_type_with_rights() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type MyEnum = strict enum : uint32 {
    NONE = 0;
};

alias via = uint32;

resource_definition SomeResource : via {
    properties {
        subtype MyEnum;
        rights via;
    };
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let _root = match lib.compile() {
            Ok(root) => root,
            Err(e) => {
                lib.reporter().print_reports();
                panic!("compilation failed: {}", e);
            }
        };
    }

    #[test]

    fn bad_empty() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

resource_definition SomeResource : uint32 {
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]

    fn bad_no_properties() {
        let file_content = get_file_content("bad/fi-0029.noformat.test.fidl");
        let source = SourceFile::new("bad/fi-0029.noformat.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]

    fn bad_duplicate_property() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

resource_definition MyResource : uint32 {
    properties {
        subtype flexible enum : uint32 {};
        rights uint32;
        rights uint32;
    };
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]

    fn bad_not_uint32() {
        let file_content = get_file_content("bad/fi-0172.test.fidl");
        let source = SourceFile::new("bad/fi-0172.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]

    fn bad_missing_subtype_property_test() {
        let file_content = get_file_content("bad/fi-0173.test.fidl");
        let source = SourceFile::new("bad/fi-0173.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]

    fn bad_subtype_not_enum() {
        let file_content = get_file_content("bad/fi-0175.test.fidl");
        let source = SourceFile::new("bad/fi-0175.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]

    fn bad_subtype_not_identifier() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

resource_definition handle : uint32 {
    properties {
        subtype uint32;
    };
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]

    fn bad_non_bits_rights() {
        let file_content = get_file_content("bad/fi-0177.test.fidl");
        let source = SourceFile::new("bad/fi-0177.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]

    fn bad_include_cycle() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

resource_definition handle : uint32 {
    properties {
        subtype handle;
    };
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

