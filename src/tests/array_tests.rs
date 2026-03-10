use crate::source_file::SourceFile;
use crate::tests::test_library::TestLibrary;
use std::fs;

fn get_file_content(path: &str) -> String {
    let full_path = format!("fidlc/tests/fidl/{}", path);
    fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
}

#[test]
fn good_nonzero_size_array() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type S = struct {
    arr array<uint8, 1>;
};
"#
        ,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_zero_size_array() {
    let file_content = get_file_content("bad/fi-0161.test.fidl");

    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "bad/fi-0161.test.fidl".to_string(),
        file_content,
    ));
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
"#
        ,
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
"#
        ,
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
"#
        ,
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
"#
        ,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}
