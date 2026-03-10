use crate::tests::test_library::TestLibrary;

#[test]
fn good_nonzero_size_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8, 1>;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_zero_size_array() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0161.test.fidl");
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_no_size_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8>;
};
"#,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_non_parameterized_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array;
};
"#,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_optional_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8, 10>:optional;
};
"#,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_multiple_constraints_on_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8, 10>:<1, 2, 3>;
};
"#,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}
