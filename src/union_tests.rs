#[cfg(test)]
mod tests {
    use crate::source_file::SourceFile;
    use crate::test_library::{LookupHelpers, TestLibrary};
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    fn good_keywords_as_field_names() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type struct = struct {
    field bool;
};

type Foo = strict union {
    1: union int64;
    2: library bool;
    3: uint32 uint32;
    4: member struct;
    5: reserved bool;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        for decl in &root.declaration_order {
            println!("Decl in order: {}", decl);
        }
        for decl in &root.union_declarations {
            println!("Found union: {}", decl.name);
        }
        for decl in &root.struct_declarations {
            println!("Found struct: {}", decl.name);
        }
        let type_decl = root.lookup_union("test/Foo").unwrap_or_else(|| {
            panic!("Foo union not found");
        });
        assert_eq!(type_decl.members.len(), 5);
    }

    #[test]
    fn good_recursive_union() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type Value = strict union {
    1: bool_value bool;
    2: list_value vector<Value:optional>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_mutually_recursive() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type Foo = strict union {
    1: bar Bar;
};

type Bar = struct {
    foo Foo:optional;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_flexible_union() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type Foo = flexible union {
    1: bar string;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_strict_union() {
        let file_content = get_file_content("good/fi-0018.test.fidl");
        let source = SourceFile::new("good/fi-0018.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_must_have_explicit_ordinals() {
        let file_content = get_file_content("bad/fi-0016-b.noformat.test.fidl");
        let source = SourceFile::new("bad/fi-0016-b.noformat.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_explicit_ordinals() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type Foo = strict union {
    1: foo int64;
    2: bar vector<uint32>:10;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let fidl_union = root.lookup_union("test/Foo").expect("Foo union not found");
        assert_eq!(fidl_union.members.len(), 2);
        assert_eq!(fidl_union.members[0].ordinal, 1);
        assert_eq!(fidl_union.members[1].ordinal, 2);
    }

    #[test]
    fn good_ordinal_gaps() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type Foo = strict union {
    2: foo int64;
    4: bar vector<uint32>:10;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let fidl_union = root.lookup_union("test/Foo").expect("Foo union not found");
        assert_eq!(fidl_union.members.len(), 2);
        assert_eq!(fidl_union.members[0].ordinal, 2);
        assert_eq!(fidl_union.members[1].ordinal, 4);
    }

    #[test]
    fn good_ordinals_out_of_order() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type Foo = strict union {
    5: foo int64;
    2: bar vector<uint32>:10;
    4: baz uint32;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let fidl_union = root.lookup_union("test/Foo").expect("Foo union not found");
        assert_eq!(fidl_union.members.len(), 3);
        assert_eq!(fidl_union.members[0].ordinal, 2);
        assert_eq!(fidl_union.members[1].ordinal, 4);
        assert_eq!(fidl_union.members[2].ordinal, 5);
    }

    #[test]
    #[ignore]
    fn bad_ordinal_out_of_bounds_negative() {
        let file_content = get_file_content("bad/fi-0017-b.noformat.test.fidl");
        let source = SourceFile::new("bad/fi-0017-b.noformat.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_ordinal_out_of_bounds_large() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type Foo = union {
  4294967296: foo string;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_ordinals_must_be_unique() {
        let file_content = get_file_content("bad/fi-0097.test.fidl");
        let source = SourceFile::new("bad/fi-0097.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_member_names_must_be_unique() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type MyUnion = strict union {
    1: my_variant string;
    2: my_variant int32;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_cannot_start_at_zero() {
        let file_content = get_file_content("bad/fi-0018.noformat.test.fidl");
        let source = SourceFile::new("bad/fi-0018.noformat.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_default_not_allowed() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test;

type Foo = strict union {
    1: t int64 = 1;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_ordinal_gap_start() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Example = strict union {
    2: two int64;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_ordinal_gap_middle() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Example = strict union {
    1: one int64;
    3: three int64;
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
    fn bad_no_nullable_members() {
        let file_content = get_file_content("bad/fi-0049.test.fidl");
        let source = SourceFile::new("bad/fi-0049.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_no_directly_recursive_unions() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Value = strict union {
  1: value Value;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_empty_flexible_union() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Foo = flexible union {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let fidl_union = root
            .lookup_union("example/Foo")
            .expect("Foo union not found");
        assert_eq!(fidl_union.members.len(), 0);
    }

    #[test]
    #[ignore]
    fn bad_empty_strict_union() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Value = strict union {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn good_error_syntax_explicit_ordinals() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
open protocol Example {
    flexible Method() -> () error int32;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let error_union = root
            .lookup_union("example/Example_Method_Result")
            .expect("Result union not found");
        assert_eq!(error_union.members.len(), 3); // success type, enum err type, framework err
        assert_eq!(error_union.members[0].ordinal, 1);
        assert_eq!(error_union.members[1].ordinal, 2);
        assert_eq!(error_union.members[2].ordinal, 3);
    }

    #[test]
    #[ignore]
    fn bad_no_selector() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Foo = strict union {
  @selector("v2") 1: v string;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }
}
