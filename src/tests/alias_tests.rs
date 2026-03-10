use crate::flat_ast::TypeKind;
use crate::tests::test_library::{LookupHelpers, SharedAmongstLibraries, TestLibrary};

#[test]
#[ignore]
fn bad_duplicate_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_int16;
};

alias alias_of_int16 = int16;
alias alias_of_int16 = int16;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn good_alias_of_struct() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;
type TypeDecl = struct {
    field1 uint16;
    field2 uint16;
};
alias AliasOfDecl = TypeDecl;
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let type_decl = root.lookup_struct("example/TypeDecl").unwrap();
    assert_eq!(type_decl.members.len(), 2);
    assert!(root.lookup_alias("example/AliasOfDecl").is_some());
}

#[test]

fn good_primitive() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_int16;
};

alias alias_of_int16 = int16;
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let msg = root.lookup_struct("example/Message").unwrap();
    assert_eq!(msg.members.len(), 1);

    let type_ = &msg.members[0].type_;
    assert_eq!(type_.kind(), TypeKind::Primitive);
    // We cannot check `resolved_params` easily as it relies on internal compiler state,
    // but checking the resulting JSON is sufficient for now.
}

#[test]

fn good_primitive_alias_before_use() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

alias alias_of_int16 = int16;

type Message = struct {
    f alias_of_int16;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let msg = root.lookup_struct("example/Message").unwrap();
    assert_eq!(msg.members.len(), 1);

    let type_ = &msg.members[0].type_;
    assert_eq!(type_.kind(), TypeKind::Primitive);
}

#[test]
#[ignore]
fn bad_self_referential_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

alias uint32 = uint32;

type Message = struct {
    f uint32;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]

fn bad_no_optional_on_primitive() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0156.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_multiple_constraints_on_primitive() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test.optionals;

type Bad = struct {
    opt_num int64:<1, 2, 3>;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]

fn bad_invalid_size_constraint_type() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0101-a.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]

fn bad_invalid_size_constraint_is_not_value() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0101-b.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_no_optional_on_aliased_primitive() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test.optionals;

alias alias = int64;

type Bad = struct {
    opt_num alias:optional;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]

fn good_vector_parameterized_on_decl() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_vector_of_string;
};

alias alias_of_vector_of_string = vector<string>;
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let msg = root.lookup_struct("example/Message").unwrap();
    assert_eq!(msg.members.len(), 1);

    let type_ = &msg.members[0].type_;
    assert_eq!(type_.kind(), TypeKind::Vector);
    assert_eq!(type_.element_count(), None); // default is max size
}

#[test]

fn bad_vector_parameterized_on_use() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_vector<uint8>;
};

alias alias_of_vector = vector;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]

fn bad_vector_bounded_on_decl() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_vector_max_8<string>;
};

alias alias_of_vector_max_8 = vector:8;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn good_vector_bounded_on_use() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_vector_of_string:8;
};

alias alias_of_vector_of_string = vector<string>;
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let msg = root.lookup_struct("example/Message").unwrap();
    assert_eq!(msg.members.len(), 1);

    let type_ = &msg.members[0].type_;
    assert_eq!(type_.kind(), TypeKind::Vector);
    assert_eq!(type_.element_count(), Some(8));
}

#[test]
fn good_unbounded_vector_bound_twice() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0158.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]

fn good_vector_nullable_on_decl() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_vector_of_string_nullable;
};

alias alias_of_vector_of_string_nullable = vector<string>:optional;
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let msg = root.lookup_struct("example/Message").unwrap();
    assert_eq!(msg.members.len(), 1);

    let type_ = &msg.members[0].type_;
    assert_eq!(type_.kind(), TypeKind::Vector);
    assert_eq!(type_.nullable(), true);
}

#[test]

fn good_vector_nullable_on_use() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_vector_of_string:optional;
};

alias alias_of_vector_of_string = vector<string>;
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let msg = root.lookup_struct("example/Message").unwrap();
    assert_eq!(msg.members.len(), 1);

    let type_ = &msg.members[0].type_;
    assert_eq!(type_.kind(), TypeKind::Vector);
    assert_eq!(type_.nullable(), true);
}

#[test]

fn bad_cannot_parameterize_twice() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Message = struct {
    f alias_of_vector_of_string<string>;
};

alias alias_of_vector_of_string = vector<string>;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]

fn bad_cannot_bound_twice() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0158.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_cannot_null_twice() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0160.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_multi_file_alias_reference() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "first.fidl",
        r#"library example;

type Protein = struct {
  amino_acids AminoAcids;
};"#,
    );
    lib.add_source_file(
        "second.fidl",
        r#"library example;

alias AminoAcids = vector<uint64>:32;
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_multi_file_nullable_alias_reference() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "first.fidl",
        r#"library example;

type Protein = struct {
    amino_acids AminoAcids:optional;
};"#,
    );
    lib.add_source_file(
        "second.fidl",
        r#"library example;

alias AminoAcids = vector<uint64>:32;
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_recursive_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

alias TheAlias = TheStruct;

type TheStruct = struct {
    many_mini_me vector<TheAlias>;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_compound_identifier() {
    // We use TestLibrary::parse because this fails at the parsing stage in C++ implementation as well
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

alias foo.bar.baz = uint8;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn good_using_library() {
    let mut dependency_lib = TestLibrary::new();
    dependency_lib.add_source_file(
        "dependent.fidl",
        r#"library dependent;

type Bar = struct {
    s int8;
};
"#,
    );
    dependency_lib.compile().expect("compilation failed");

    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "dependent.fidl",
        r#"library dependent;

type Bar = struct {
    s int8;
};
"#,
    ); // We need to provide the parsed AST of dependencies
    lib.add_source_file(
        "example.fidl",
        r#"library example;

using dependent;

alias Bar2 = dependent.Bar;
"#,
    );
    lib.compile().expect("compilation failed");
}
