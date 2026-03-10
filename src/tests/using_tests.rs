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
fn bad_missing_using() {
    let source = crate::source_file::SourceFile::new("example.fidl".to_string(), "library example;\ntype Foo = struct { dep dependent.Bar; };".to_string());
    let mut lib = TestLibrary::new();
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrNameNotFound);
}

#[test]
fn bad_unknown_using() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0046.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrUnknownLibrary);
}

#[test]
fn bad_using_alias_ref_through_fqn() {
    let dep = crate::source_file::SourceFile::new("dependent.fidl".to_string(), "library dependent;\ntype Bar = struct { s int8; };".to_string());
    let mut lib = TestLibrary::new();
    lib.add_source(&dep);
    let source = crate::source_file::SourceFile::new("example.fidl".to_string(), "library example;\nusing dependent as the_alias;\ntype Foo = struct { dep1 dependent.Bar; };".to_string());
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert!(errors.iter().any(|e| e.def == crate::diagnostics::Error::ErrNameNotFound));
}

#[test]
fn bad_duplicate_using_no_alias() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0042-a.test.fidl");
    lib.add_errcat_file("bad/fi-0042-b.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_duplicate_using_first_alias() {
    let dep = crate::source_file::SourceFile::new("dependent.fidl".to_string(), "library dependent;".to_string());
    let source = crate::source_file::SourceFile::new("example.fidl".to_string(), "library example;\nusing dependent as alias;\nusing dependent;".to_string());
    let mut lib = TestLibrary::new();
    lib.add_source(&dep);
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_duplicate_using_second_alias() {
    let dep = crate::source_file::SourceFile::new("dependent.fidl".to_string(), "library dependent;".to_string());
    let source = crate::source_file::SourceFile::new("example.fidl".to_string(), "library example;\nusing dependent;\nusing dependent as alias;".to_string());
    let mut lib = TestLibrary::new();
    lib.add_source(&dep);
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_duplicate_using_same_library_same_alias() {
    let dep = crate::source_file::SourceFile::new("dependent.fidl".to_string(), "library dependent;".to_string());
    let source = crate::source_file::SourceFile::new("example.fidl".to_string(), "library example;\nusing dependent as alias;\nusing dependent as alias;".to_string());
    let mut lib = TestLibrary::new();
    lib.add_source(&dep);
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_duplicate_using_same_library_different_alias() {
    let dep = crate::source_file::SourceFile::new("dependent.fidl".to_string(), "library dependent;".to_string());
    let source = crate::source_file::SourceFile::new("example.fidl".to_string(), "library example;\nusing dependent as alias1;\nusing dependent as alias2;".to_string());
    let mut lib = TestLibrary::new();
    lib.add_source(&dep);
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrDuplicateLibraryImport);
}

#[test]
fn bad_conflicting_using_library_and_alias() {
    let dep1 = crate::source_file::SourceFile::new("dependent1.fidl".to_string(), "library dependent1;".to_string());
    let dep2 = crate::source_file::SourceFile::new("dependent2.fidl".to_string(), "library dependent2;".to_string());
    let source = crate::source_file::SourceFile::new("example.fidl".to_string(), "library example;\nusing dependent1;\nusing dependent2 as dependent1;".to_string());
    let mut lib = TestLibrary::new();
    lib.add_source(&dep1);
    lib.add_source(&dep2);
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrConflictingLibraryImportAlias);
}

#[test]
fn bad_conflicting_using_alias_and_library() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0043-a.test.fidl");
    lib.add_errcat_file("bad/fi-0043-b.test.fidl");
    lib.add_errcat_file("bad/fi-0043-c.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrConflictingLibraryImport);
}

#[test]
fn bad_conflicting_using_alias_and_alias() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0044-a.test.fidl");
    lib.add_errcat_file("bad/fi-0044-b.test.fidl");
    lib.add_errcat_file("bad/fi-0044-c.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrConflictingLibraryImportAlias);
}

#[test]
fn bad_unused_using() {
    let mut lib = TestLibrary::new();
    let dep = crate::source_file::SourceFile::new("dependent.fidl".to_string(), "library dependent;".to_string());
    lib.add_source(&dep);
    lib.add_errcat_file("bad/fi-0178.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrUnusedImport);
}

#[test]
fn bad_unknown_dependent_library() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0051.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrUnknownDependentLibrary);
}

#[test]
fn bad_library_declaration_name_collision() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0038-a.test.fidl");
    lib.add_errcat_file("bad/fi-0038-b.test.fidl");
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrDeclNameConflictsWithLibraryImport);
}

#[test]
fn bad_aliased_library_declaration_name_collision() {
    let dep = crate::source_file::SourceFile::new("dep.fidl".to_string(), "library dep;\ntype A = struct{};".to_string());
    let source = crate::source_file::SourceFile::new("lib.fidl".to_string(), "library lib;\nusing dep as x;\ntype x = struct{};\ntype B = struct{a dep.A;};".to_string());
    let mut lib = TestLibrary::new();
    lib.add_source(&dep);
    lib.add_source(&source);
    assert!(lib.compile().is_err());
    let errors = lib.reporter().diagnostics();
    assert_eq!(errors[0].def, crate::diagnostics::Error::ErrDeclNameConflictsWithLibraryImport);
}
