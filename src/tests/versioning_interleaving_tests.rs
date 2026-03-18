#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
#[ignore]
fn same_library() {
    if false {
        let fidl = r#"
@available(added=1)
library example;

${source_available}
const SOURCE bool = TARGET;

${target_available}
const TARGET bool = false;
"#;

        let mut library = TestLibrary::new();
        library.add_errcat_file(fidl);
        library.select_version("example", "HEAD");
    }
}

#[test]
#[ignore]
fn decl_to_decl_external() {
    let example_fidl = r#"
@available(added=1)
library platform.example;

using platform.dependency;

${source_available}
const SOURCE bool = platform.dependency.tARGET;
"#;
    let dependency_fidl = r#"
@available(added=1)
library platform.dependency;

${target_available}
const TARGET bool = false;
"#;
    if false {}
}

#[test]
#[ignore]
fn library_to_library_external() {
    let example_fidl = r#"
${source_available}
library platform.example;

using platform.dependency;

const SOURCE bool = platform.dependency.tARGET;
"#;
    let dependency_fidl = r#"
${target_available}
library platform.dependency;

const TARGET bool = false;
"#;
    if false {}
}

#[test]
#[ignore]
fn library_to_decl_external() {
    let example_fidl = r#"
${source_available}
library platform.example;

using platform.dependency;

const SOURCE bool = platform.dependency.tARGET;
"#;
    let dependency_fidl = r#"
@available(added=1)
library platform.dependency;

${target_available}
const TARGET bool = false;
"#;
    if false {}
}

#[test]
#[ignore]
fn decl_to_library_external() {
    let example_fidl = r#"
@available(added=1)
library platform.example;

using platform.dependency;

${source_available}
const SOURCE bool = platform.dependency.tARGET;
"#;
    let dependency_fidl = r#"
${target_available}
library platform.dependency;

const TARGET bool = false;
"#;
    if false {}
}

#[test]
#[ignore]
fn error0055() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0055.test.fidl");
    library.select_version("test", "HEAD");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn error0056() {
    // SharedAmongstLibraries shared;

    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("bad/fi-0056-a.test.fidl");
    assert!(dependency.check_compile());
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0056-b.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}
