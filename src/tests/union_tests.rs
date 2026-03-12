use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_keywords_as_field_names() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
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
"#,
    );
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
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type Value = strict union {
    1: bool_value bool;
    2: list_value vector<Value:optional>;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_mutually_recursive() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type Foo = strict union {
    1: bar Bar;
};

type Bar = struct {
    foo Foo:optional;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_flexible_union() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type Foo = flexible union {
    1: bar string;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_strict_union() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0018.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_must_have_explicit_ordinals() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0016-b.noformat.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrMissingOrdinalBeforeMember));
}

#[test]
fn good_explicit_ordinals() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type Foo = strict union {
    1: foo int64;
    2: bar vector<uint32>:10;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let fidl_union = root.lookup_union("test/Foo").expect("Foo union not found");
    assert_eq!(fidl_union.members.len(), 2);
    assert_eq!(fidl_union.members[0].ordinal, 1);
    assert_eq!(fidl_union.members[1].ordinal, 2);
}

#[test]
fn good_ordinal_gaps() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type Foo = strict union {
    2: foo int64;
    4: bar vector<uint32>:10;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let fidl_union = root.lookup_union("test/Foo").expect("Foo union not found");
    assert_eq!(fidl_union.members.len(), 2);
    assert_eq!(fidl_union.members[0].ordinal, 2);
    assert_eq!(fidl_union.members[1].ordinal, 4);
}

#[test]
fn good_ordinals_out_of_order() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type Foo = strict union {
    5: foo int64;
    2: bar vector<uint32>:10;
    4: baz uint32;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let fidl_union = root.lookup_union("test/Foo").expect("Foo union not found");
    assert_eq!(fidl_union.members.len(), 3);
    assert_eq!(fidl_union.members[0].ordinal, 2);
    assert_eq!(fidl_union.members[1].ordinal, 4);
    assert_eq!(fidl_union.members[2].ordinal, 5);
}

#[test]

fn bad_ordinal_out_of_bounds_negative() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0017-b.noformat.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrOrdinalOutOfBound));
}

#[test]

fn bad_ordinal_out_of_bounds_large() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type Foo = union {
  4294967296: foo string;
};
"#,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrOrdinalOutOfBound));
}

#[test]

fn bad_ordinals_must_be_unique() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0097.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrDuplicateUnionMemberOrdinal));
}

#[test]

fn bad_member_names_must_be_unique() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type MyUnion = strict union {
    1: my_variant string;
    2: my_variant int32;
};
"#,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrNameCollision));
}

#[test]

fn bad_cannot_start_at_zero() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0018.noformat.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrOrdinalsMustStartAtOne));
}

#[test]

fn bad_default_not_allowed() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library test;

type Foo = strict union {
    1: t int64 = 1;
};
"#,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrUnexpectedToken));
}

#[test]
fn good_ordinal_gap_start() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Example = strict union {
    2: two int64;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_ordinal_gap_middle() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Example = strict union {
    1: one int64;
    3: three int64;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_no_nullable_members() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0049.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrOptionalUnionMember));
}

#[test]

fn bad_no_directly_recursive_unions() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Value = strict union {
  1: value Value;
};
"#,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrIncludeCycle));
}

#[test]
fn good_empty_flexible_union() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Foo = flexible union {};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let fidl_union = root
        .lookup_union("example/Foo")
        .expect("Foo union not found");
    assert_eq!(fidl_union.members.len(), 0);
}

#[test]

fn bad_empty_strict_union() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Value = strict union {};
"#,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrMustHaveOneMember));
}

#[test]

fn good_error_syntax_explicit_ordinals() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;
open protocol Example {
    flexible Method() -> () error int32;
};
"#,
    );
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

fn bad_no_selector() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Foo = strict union {
  @selector("v2") 1: v string;
};
"#,
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrInvalidAttributePlacement));
}
