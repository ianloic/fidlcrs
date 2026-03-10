use crate::diagnostics::Error;
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

    lib.add_source(SourceFile::new(
        "dependent.fidl".to_string(),
        r#"
library dependent;
type Bar = struct {
    s int8;
};
"#
        .to_string(),
    ));
    lib.add_source(SourceFile::new(
        "good/fi-0178.test.fidl".to_string(),
        get_file_content("good/fi-0178.test.fidl"),
    ));
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
fn bad_missing_using() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        "library example;
type Foo = struct { dep dependent.Bar; };"
            .to_string(),
    ));
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrNameNotFound);
}

#[test]
fn bad_unknown_using() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0046.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrUnknownLibrary);
}

#[test]
fn bad_using_alias_ref_through_fqn() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "dependent.fidl".to_string(),
        "library dependent;
type Bar = struct { s int8; };"
            .to_string(),
    ));

    lib.add_source_file(
        "example.fidl",
        "library example;
using dependent as the_alias;
type Foo = struct { dep1 dependent.Bar; };",
    );
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == Error::ErrNameNotFound));
}

#[test]
fn bad_duplicate_using_no_alias() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0042-a.test.fidl");
    lib.add_errcat_file("bad/fi-0042-b.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_duplicate_using_first_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "dependent.fidl".to_string(),
        "library dependent;".to_string(),
    ));
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        "library example;
using dependent as alias;
using dependent;"
            .to_string(),
    ));
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_duplicate_using_second_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "dependent.fidl".to_string(),
        "library dependent;".to_string(),
    ));
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        "library example;
using dependent;
using dependent as alias;"
            .to_string(),
    ));
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_duplicate_using_same_library_same_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "dependent.fidl".to_string(),
        "library dependent;".to_string(),
    ));
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        "library example;
using dependent as alias;
using dependent as alias;"
            .to_string(),
    ));
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_duplicate_using_same_library_different_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "dependent.fidl".to_string(),
        "library dependent;".to_string(),
    ));
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        "library example;
using dependent as alias1;
using dependent as alias2;"
            .to_string(),
    ));
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_conflicting_using_library_and_alias() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "dependent1.fidl".to_string(),
        "library dependent1;".to_string(),
    ));
    lib.add_source(SourceFile::new(
        "dependent2.fidl".to_string(),
        "library dependent2;".to_string(),
    ));
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        "library example;
using dependent1;
using dependent2 as dependent1;"
            .to_string(),
    ));
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrConflictingLibraryImportAlias);
}

#[test]
fn bad_conflicting_using_alias_and_library() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0043-a.test.fidl");
    lib.add_errcat_file("bad/fi-0043-b.test.fidl");
    lib.add_errcat_file("bad/fi-0043-c.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrConflictingLibraryImport);
}

#[test]
fn bad_conflicting_using_alias_and_alias() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0044-a.test.fidl");
    lib.add_errcat_file("bad/fi-0044-b.test.fidl");
    lib.add_errcat_file("bad/fi-0044-c.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrConflictingLibraryImportAlias);
}

#[test]
fn bad_unused_using() {
    let mut lib = TestLibrary::new();

    lib.add_source(SourceFile::new(
        "dependent.fidl".to_string(),
        "library dependent;".to_string(),
    ));
    lib.add_errcat_file("bad/fi-0178.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrUnusedImport);
}

#[test]
fn bad_unknown_dependent_library() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0051.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrUnknownDependentLibrary);
}

#[test]
fn bad_library_declaration_name_collision() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0038-a.test.fidl");
    lib.add_errcat_file("bad/fi-0038-b.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrDeclNameConflictsWithLibraryImport);
}

#[test]
fn bad_aliased_library_declaration_name_collision() {
    let dep = SourceFile::new(
        "dep.fidl".to_string(),
        "library dep;\ntype A = struct{};".to_string(),
    );

    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "dep.fidl".to_string(),
        "library dep;
type A = struct{};"
            .to_string(),
    ));
    lib.add_source(SourceFile::new(
        "lib.fidl".to_string(),
        "library lib;
using dep as x;
type x = struct{};
type B = struct{a dep.A;};"
            .to_string(),
    ));
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, Error::ErrDeclNameConflictsWithLibraryImport);
}
