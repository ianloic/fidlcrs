
    use crate::source_file::SourceFile;
    use crate::tests::test_library::{LookupHelpers, TestLibrary};
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/{}", path);
        fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    #[ignore]
    fn bad_duplicate_alias() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_int16;
};

alias alias_of_int16 = int16;
alias alias_of_int16 = int16;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_alias_of_struct() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
type TypeDecl = struct {
    field1 uint16;
    field2 uint16;
};
alias AliasOfDecl = TypeDecl;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let type_decl = root.lookup_struct("example/TypeDecl").unwrap();
        assert_eq!(type_decl.members.len(), 2);
        assert!(root.lookup_alias("example/AliasOfDecl").is_some());
    }

    #[test]
    #[ignore]
    fn good_primitive() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_int16;
};

alias alias_of_int16 = int16;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let msg = root.lookup_struct("example/Message").unwrap();
        assert_eq!(msg.members.len(), 1);

        let type_ = &msg.members[0].type_;
        assert_eq!(type_.kind(), crate::json_generator::TypeKind::Primitive);
        // We cannot check `resolved_params` easily as it relies on internal compiler state,
        // but checking the resulting JSON is sufficient for now.
    }

    #[test]
    #[ignore]
    fn good_primitive_alias_before_use() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

alias alias_of_int16 = int16;

type Message = struct {
    f alias_of_int16;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let msg = root.lookup_struct("example/Message").unwrap();
        assert_eq!(msg.members.len(), 1);

        let type_ = &msg.members[0].type_;
        assert_eq!(type_.kind(), crate::json_generator::TypeKind::Primitive);
    }

    #[test]
    #[ignore]
    fn bad_self_referential_alias() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

alias uint32 = uint32;

type Message = struct {
    f uint32;
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
    fn bad_no_optional_on_primitive() {
        let file_content = get_file_content("fidl/bad/fi-0156.test.fidl");
        let source = SourceFile::new("bad/fi-0156.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_multiple_constraints_on_primitive() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test.optionals;

type Bad = struct {
    opt_num int64:<1, 2, 3>;
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
    fn bad_invalid_size_constraint_type() {
        let file_content = get_file_content("fidl/bad/fi-0101-a.test.fidl");
        let source = SourceFile::new("bad/fi-0101-a.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_invalid_size_constraint_is_not_value() {
        let file_content = get_file_content("fidl/bad/fi-0101-b.test.fidl");
        let source = SourceFile::new("bad/fi-0101-b.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_no_optional_on_aliased_primitive() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library test.optionals;

alias alias = int64;

type Bad = struct {
    opt_num alias:optional;
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
    fn good_vector_parameterized_on_decl() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_vector_of_string;
};

alias alias_of_vector_of_string = vector<string>;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let msg = root.lookup_struct("example/Message").unwrap();
        assert_eq!(msg.members.len(), 1);

        let type_ = &msg.members[0].type_;
        assert_eq!(type_.kind(), crate::json_generator::TypeKind::Vector);
        assert_eq!(type_.element_count(), None); // default is max size
    }

    #[test]
    #[ignore]
    fn bad_vector_parameterized_on_use() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_vector<uint8>;
};

alias alias_of_vector = vector;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_vector_bounded_on_decl() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_vector_max_8<string>;
};

alias alias_of_vector_max_8 = vector:8;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn good_vector_bounded_on_use() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_vector_of_string:8;
};

alias alias_of_vector_of_string = vector<string>;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let msg = root.lookup_struct("example/Message").unwrap();
        assert_eq!(msg.members.len(), 1);

        let type_ = &msg.members[0].type_;
        assert_eq!(type_.kind(), crate::json_generator::TypeKind::Vector);
        assert_eq!(type_.element_count(), Some(8));
    }

    #[test]
    fn good_unbounded_vector_bound_twice() {
        let file_content = get_file_content("fidl/good/fi-0158.test.fidl");
        let source = SourceFile::new("good/fi-0158.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn good_vector_nullable_on_decl() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_vector_of_string_nullable;
};

alias alias_of_vector_of_string_nullable = vector<string>:optional;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let msg = root.lookup_struct("example/Message").unwrap();
        assert_eq!(msg.members.len(), 1);

        let type_ = &msg.members[0].type_;
        assert_eq!(type_.kind(), crate::json_generator::TypeKind::Vector);
        assert_eq!(type_.nullable(), Some(true));
    }

    #[test]
    #[ignore]
    fn good_vector_nullable_on_use() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_vector_of_string:optional;
};

alias alias_of_vector_of_string = vector<string>;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let msg = root.lookup_struct("example/Message").unwrap();
        assert_eq!(msg.members.len(), 1);

        let type_ = &msg.members[0].type_;
        assert_eq!(type_.kind(), crate::json_generator::TypeKind::Vector);
        assert_eq!(type_.nullable(), Some(true));
    }

    #[test]
    #[ignore]
    fn bad_cannot_parameterize_twice() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type Message = struct {
    f alias_of_vector_of_string<string>;
};

alias alias_of_vector_of_string = vector<string>;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_cannot_bound_twice() {
        let file_content = get_file_content("fidl/bad/fi-0158.test.fidl");
        let source = SourceFile::new("bad/fi-0158.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_cannot_null_twice() {
        let file_content = get_file_content("fidl/bad/fi-0160.test.fidl");
        let source = SourceFile::new("bad/fi-0160.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_multi_file_alias_reference() {
        let first_source = SourceFile::new(
            "first.fidl".to_string(),
            r#"library example;

type Protein = struct {
  amino_acids AminoAcids;
};"#
            .to_string(),
        );
        let second_source = SourceFile::new(
            "second.fidl".to_string(),
            r#"library example;

alias AminoAcids = vector<uint64>:32;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&first_source);
        lib.add_source(&second_source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_multi_file_nullable_alias_reference() {
        let first_source = SourceFile::new(
            "first.fidl".to_string(),
            r#"library example;

type Protein = struct {
    amino_acids AminoAcids:optional;
};"#
            .to_string(),
        );
        let second_source = SourceFile::new(
            "second.fidl".to_string(),
            r#"library example;

alias AminoAcids = vector<uint64>:32;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&first_source);
        lib.add_source(&second_source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_recursive_alias() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

alias TheAlias = TheStruct;

type TheStruct = struct {
    many_mini_me vector<TheAlias>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn bad_compound_identifier() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

alias foo.bar.baz = uint8;
"#
            .to_string(),
        );
        // We use TestLibrary::parse because this fails at the parsing stage in C++ implementation as well
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_using_library() {
        let dependency_source = SourceFile::new(
            "dependent.fidl".to_string(),
            r#"library dependent;

type Bar = struct {
    s int8;
};
"#
            .to_string(),
        );
        let mut dependency_lib = TestLibrary::new();
        dependency_lib.add_source(&dependency_source);
        dependency_lib.compile().expect("compilation failed");

        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

using dependent;

alias Bar2 = dependent.Bar;
"#
            .to_string(),
        );

        let mut lib = TestLibrary::new();
        lib.add_source(&dependency_source); // We need to provide the parsed AST of dependencies
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

