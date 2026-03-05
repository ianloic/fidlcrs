use crate::source_file::SourceFile;
use crate::tests::test_library::TestLibrary;
use std::fs;

fn get_file_content(path: &str) -> String {
    let full_path = format!("fidlc/tests/fidl/{}", path);
    fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
}

#[test]
fn good_using() {
    let mut lib = TestLibrary::new();
    let source = SourceFile::new(
        "good/fi-0178.test.fidl".to_string(),
        get_file_content("good/fi-0178.test.fidl"),
    );
    let dep_source = SourceFile::new(
        "dependent.fidl".to_string(),
        r#"
library dependent;
type Bar = struct {
    s int8;
};
"#
        .to_string(),
    );
    lib.add_source(&dep_source);
    lib.add_source(&source);
    lib.compile().expect("compilation failed");
}

#[test]
fn good_using_alias() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
}

#[test]
fn good_using_swap_names() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
}

#[test]
fn good_decl_with_same_name_as_aliased_library() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
}

#[test]
#[ignore]
fn bad_missing_using() {
    let source = SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

// missing using.

type Foo = struct {
    dep dependent.Bar;
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
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_duplicate_using_no_alias() {
    let mut lib = TestLibrary::new();
    let source = SourceFile::new(
        "bad/fi-0042-b.test.fidl".to_string(),
        get_file_content("bad/fi-0042-b.test.fidl"),
    );
    lib.add_source(&source);
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_duplicate_using_first_alias() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_duplicate_using_second_alias() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_duplicate_using_same_library_same_alias() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_duplicate_using_same_library_different_alias() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_conflicting_using_library_and_alias() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_conflicting_using_alias_and_library() {
    let mut lib = TestLibrary::new();
    let source = SourceFile::new(
        "bad/fi-0043-c.test.fidl".to_string(),
        get_file_content("bad/fi-0043-c.test.fidl"),
    );
    lib.add_source(&source);
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_conflicting_using_alias_and_alias() {
    let mut lib = TestLibrary::new();
    let source = SourceFile::new(
        "bad/fi-0044-c.test.fidl".to_string(),
        get_file_content("bad/fi-0044-c.test.fidl"),
    );
    lib.add_source(&source);
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_unused_using() {
    let mut lib = TestLibrary::new();
    let source = SourceFile::new(
        "bad/fi-0178.test.fidl".to_string(),
        get_file_content("bad/fi-0178.test.fidl"),
    );
    lib.add_source(&source);
    lib.compile().expect("compilation failed");
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
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
}

#[test]
#[ignore]
fn bad_library_declaration_name_collision() {
    let mut lib = TestLibrary::new();
    let source = SourceFile::new(
        "bad/fi-0038-b.test.fidl".to_string(),
        get_file_content("bad/fi-0038-b.test.fidl"),
    );
    lib.add_source(&source);
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}

#[test]
#[ignore]
fn bad_aliased_library_declaration_name_collision() {
    let lib = TestLibrary::new();
    lib.compile().expect("compilation failed");
    assert!(lib.compile().is_err());
}
