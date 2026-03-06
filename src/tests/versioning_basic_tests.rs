use super::test_library::TestLibrary;
use crate::diagnostics::Error;

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_library_default() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_library_added_at_head() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=HEAD)
library example;
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_library_added_at_one() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_library_added_and_removed() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1, removed=2)
library example;
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_library_added_and_deprecated_and_removed() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1, deprecated=2, removed=HEAD)
library example;
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_decl_added_at_head() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;

@available(added=HEAD)
type Foo = struct {};
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_decl_added_at_one() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;

@available(added=1)
type Foo = struct {};
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_decl_added_and_removed() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;

@available(added=1, removed=2)
type Foo = struct {};
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_decl_added_and_replaced() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;

@available(added=1, replaced=2)
type Foo = struct {};

@available(added=2)
type Foo = resource struct {};
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_decl_added_and_deprecated_and_removed() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;

@available(added=1, deprecated=2, removed=HEAD)
type Foo = struct {};
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_member_added_at_head() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_member_added_at_one() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_member_added_and_removed() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_member_added_and_replaced() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_member_added_and_deprecated_and_removed() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_decl_strictness_added_and_removed() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_decl_resourceness_added_and_removed() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;

type Foo = resource(added=2, removed=3) struct {};
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_protocol_openness_added_and_removed() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;

closed(added=2, removed=3) open(added=3) protocol Foo {};
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_method_strictness_added_and_removed() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn bad_change_interacting_modifiers() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_change_interacting_modifiers() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_add_resource_modifier_and_handle() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn remove_resource_modifier_and_handle() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn bad_reference_outside_availability() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "bad/fi-0220.test.fidl".to_string(),
        std::fs::read_to_string("fidlc/tests/bad/fi-0220.test.fidl").unwrap(),
    );
    library.add_source(&source0);
    // library.select_versions("test", "GetParam()");
    // library.expect_fail(Error::ErrNameNotFoundInVersionRange);
    let _ = library.compile();
    // library.assert_diagnostics();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_regular_deprecated_references_versioned_deprecated() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "example.fidl".to_string(),
        r#"
@available(added=1)
library example;

@deprecated
const FOO uint32 = BAR;
@available(deprecated=1)
const BAR uint32 = 1;
"#
        .to_string(),
    );
    library.add_source(&source0);
    // library.select_versions("example", "GetParam()");
    let _ = library.compile();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_deprecation_logic_regression1() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_deprecation_logic_regression2() {
    let mut library = TestLibrary::new();
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_multiple_files() {
    let mut library = TestLibrary::new();
    let mut source0 = crate::source_file::SourceFile::new(
        "overview.fidl".to_string(),
        r#"
/// Some doc comment.
@available(added=1)
library example;
"#
        .to_string(),
    );
    library.add_source(&source0);
}

#[test]
#[ignore] // TODO: Versioning logic is not fully implemented
fn good_multiple_libraries() {
    let mut library = TestLibrary::new();
}
