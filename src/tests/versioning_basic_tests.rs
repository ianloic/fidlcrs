use super::test_library::TestLibrary;
use crate::source_file::SourceFile;

#[test]

fn good_library_default() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_library_added_at_head() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=HEAD)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_library_added_at_one() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_library_added_and_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1, removed=2)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_library_added_and_deprecated_and_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1, deprecated=2, removed=HEAD)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_decl_added_at_head() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=HEAD)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_decl_added_at_one() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_decl_added_and_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, removed=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_decl_added_and_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, replaced=2)
type Foo = struct {};

@available(added=2)
type Foo = resource struct {};
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_decl_added_and_deprecated_and_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, deprecated=2, removed=HEAD)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_member_added_at_head() {
    let _library = TestLibrary::new();
}

#[test]

fn good_member_added_at_one() {
    let _library = TestLibrary::new();
}

#[test]

fn good_member_added_and_removed() {
    let _library = TestLibrary::new();
}

#[test]

fn good_member_added_and_replaced() {
    let _library = TestLibrary::new();
}

#[test]

fn good_member_added_and_deprecated_and_removed() {
    let _library = TestLibrary::new();
}

#[test]

fn good_decl_strictness_added_and_removed() {
    let _library = TestLibrary::new();
}

#[test]

fn good_decl_resourceness_added_and_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = resource(added=2, removed=3) struct {};
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_protocol_openness_added_and_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

closed(added=2, removed=3) open(added=3) protocol Foo {};
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_method_strictness_added_and_removed() {
    let _library = TestLibrary::new();
}

#[test]

fn bad_change_interacting_modifiers() {
    let _library = TestLibrary::new();
}

#[test]

fn good_change_interacting_modifiers() {
    let _library = TestLibrary::new();
}

#[test]

fn good_add_resource_modifier_and_handle() {
    let _library = TestLibrary::new();
}

#[test]

fn remove_resource_modifier_and_handle() {
    let _library = TestLibrary::new();
}

#[test]

fn bad_reference_outside_availability() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "bad/fi-0220.test.fidl".to_string(),
        std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0220.test.fidl").unwrap(),
    ));
    library.select_version("test", "1");
    // library.expect_fail(Error::ErrNameNotFoundInVersionRange);
    assert!(library.compile().is_err());
}

#[test]

fn good_regular_deprecated_references_versioned_deprecated() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@deprecated
const FOO uint32 = BAR;
@available(deprecated=1)
const BAR uint32 = 1;
"#,
    );
    library.select_version("example", "HEAD");
    let _ = library.compile();
}

#[test]

fn good_deprecation_logic_regression1() {
    let _library = TestLibrary::new();
}

#[test]

fn good_deprecation_logic_regression2() {
    let _library = TestLibrary::new();
}

#[test]

fn good_multiple_files() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "overview.fidl",
        r#"
/// Some doc comment.
@available(added=1)
library example;
"#,
    );
}

#[test]

fn good_multiple_libraries() {
    let _library = TestLibrary::new();
}
