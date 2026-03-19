use super::test_library::TestLibrary;
use crate::diagnostics::Error;

#[test]

fn bad_multiple_library_declarations_agree() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "first.fidl",
        r#"
@available(added=1)
library example;
"#,
    );

    library.add_source_file(
        "second.fidl",
        r#"
@available(added=1)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrDuplicateAttribute(..)));
}

#[test]

fn bad_multiple_library_declarations_disagree() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "first.fidl",
        r#"
@available(added=1)
library example;
"#,
    );

    library.add_source_file(
        "second.fidl",
        r#"
@available(added=2)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrDuplicateAttribute(..)));
}

#[test]

fn bad_multiple_library_declarations_head() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "first.fidl",
        r#"
@available(added=HEAD)
library example;
"#,
    );

    library.add_source_file(
        "second.fidl",
        r#"
@available(added=HEAD)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrDuplicateAttribute(..)));
}

#[test]

fn good_all_arguments_on_library() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(platform="notexample", added=1, deprecated=2, removed=3, note="use xyz instead")
library example;
"#,
    );
    library.select_version("notexample", "1");
    library.compile().expect("compilation failed");
}

#[test]

fn good_all_arguments_on_decl() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, deprecated=2, removed=3, note="use xyz instead")
type Foo = struct {};
"#,
    );
    library.select_version("example", "1");
    library.compile().expect("compilation failed");
}

#[test]

fn good_all_arguments_on_member() {
    let _library = TestLibrary::new();
}

#[test]

fn good_all_arguments_on_decl_modifier() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = resource(added=1, removed=2) struct {};
"#,
    );
    library.select_version("example", "1");
    library.compile().expect("compilation failed");
}

#[test]

fn good_all_arguments_on_member_modifier() {
    let _library = TestLibrary::new();
}

#[test]

fn good_attribute_on_everything() {
    let _library = TestLibrary::new();
}

#[test]

fn bad_anonymous_layout_top_level() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

type Foo = @available(added=2) struct {};
"#,
    );
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrAttributeInsideTypeDeclaration
    ));
}

#[test]

fn bad_anonymous_layout_in_member() {
    let _library = TestLibrary::new();
}

#[test]

fn bad_invalid_version_zero() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=0)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrInvalidVersion(..)));
}

#[test]

fn good_version_min_normal() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    library.compile().expect("compilation failed");
}

#[test]

fn good_version_max_normal() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=0x7fffffff) // 2^31-1
library example;
"#,
    );
    library.select_version("example", "HEAD");
    library.compile().expect("compilation failed");
}

#[test]

fn bad_invalid_version_above_max_normal() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=0x80000000) // 2^31
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrInvalidVersion(..)));
}

#[test]

fn bad_invalid_version_unknown_reserved() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=0x8abc1234)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrInvalidVersion(..)));
}

#[test]

fn good_version_next_name() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=NEXT)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    library.compile().expect("compilation failed");
}

#[test]

fn good_version_next_number() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=0xFFD00000)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    library.compile().expect("compilation failed");
}

#[test]

fn good_version_head_name() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=HEAD)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    library.compile().expect("compilation failed");
}

#[test]

fn good_version_head_number() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=0xFFE00000)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    library.compile().expect("compilation failed");
}

#[test]

fn bad_invalid_version_legacy_name() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=LEGACY)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrInvalidVersion(..)));
}

#[test]

fn bad_invalid_version_legacy_number() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=0xFFF00000)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrInvalidVersion(..)));
}

#[test]

fn bad_invalid_version_negative() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=-1)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrInvalidVersion(..)));
}

#[test]

fn bad_invalid_version_overflow_uint32() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=0x100000000) // 2^32
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrInvalidVersion(..)));
}

#[test]

fn bad_no_arguments() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0147.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0147.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrAvailableMissingArguments));
}

#[test]

fn bad_library_missing_added_only_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0150-a.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0150-a.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrLibraryAvailabilityMissingAdded
    ));
}

#[test]

fn bad_library_missing_added_only_platform() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0150-b.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0150-b.test.fidl").unwrap()),
    );
    library.select_version("foo", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrLibraryAvailabilityMissingAdded
    ));
}

#[test]

fn bad_library_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0204.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0204.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrLibraryReplaced));
}

#[test]

fn bad_library_renamed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1, removed=2, renamed="foo")
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrCannotBeRenamed(..)));
}

#[test]

fn bad_decl_renamed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0211.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0211.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrCannotBeRenamed(..)));
}

#[test]

fn bad_compose_renamed() {
    let _library = TestLibrary::new();
}

#[test]

fn good_note_with_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, removed=2, note="use xyz instead")
type Foo = struct {};
"#,
    );
    library.select_version("example", "1");
    library.compile().expect("compilation failed");
}

#[test]

fn good_note_with_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, replaced=2, note="use xyz instead")
type Foo = struct {};

@available(added=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", "1");
    library.compile().expect("compilation failed");
}

#[test]

fn bad_note_without_deprecation_removed_or_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0148.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0148.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrNoteWithoutDeprecationOrRemoval
    ));
}

#[test]

fn bad_renamed_without_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0212.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0212.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrRenamedWithoutReplacedOrRemoved
    ));
}

#[test]

fn bad_renamed_to_same_name() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0213.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0213.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrRenamedToSameName(..)));
}

#[test]

fn bad_removed_and_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0203.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0203.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrRemovedAndReplaced));
}

#[test]

fn bad_platform_not_on_library() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(platform="bad")
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(errors[0].def, Error::ErrPlatformNotOnLibrary));
}

#[test]

fn bad_invalid_argument_on_modifier() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0218.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0218.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidModifierAvailableArgument(..)
    ));
}

#[test]

fn bad_strictness_two_way_method_without_error() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0219.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0219.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrCannotChangeMethodStrictness
    ));
}

#[test]

fn bad_use_in_unversioned_library() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0151.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0151.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrMissingLibraryAvailability
    ));
}

#[test]

fn bad_use_in_unversioned_library_reported_once_per_attribute() {
    let _library = TestLibrary::new();
}

#[test]

fn bad_added_equals_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0154-a.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0154-a.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}

#[test]

fn bad_added_equals_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=2, replaced=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}

#[test]

fn bad_added_greater_than_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, removed=1)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}

#[test]

fn bad_added_greater_than_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=3, replaced=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}

#[test]

fn good_added_equals_deprecated() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1, deprecated=1)
library example;
"#,
    );
    library.select_version("example", "1");
    library.compile().expect("compilation failed");
}

#[test]

fn bad_added_greater_than_deprecated() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=2, deprecated=1)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}

#[test]

fn bad_deprecated_equals_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "bad/fi-0154-b.test.fidl",
        &(std::fs::read_to_string("fidlc/tests/fidl/bad/fi-0154-b.test.fidl").unwrap()),
    );
    library.select_version("test", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}

#[test]

fn bad_deprecated_equals_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, deprecated=2, replaced=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}

#[test]

fn bad_deprecated_greater_than_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1, deprecated=3, removed=2)
library example;
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}

#[test]

fn bad_deprecated_greater_than_replaced() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
@available(added=1)
library example;

@available(added=1, deprecated=3, replaced=2)
type Foo = struct {};
"#,
    );
    library.select_version("example", "HEAD");
    assert!(library.compile().is_err());
    let errors = library.reporter().diagnostics();
    assert!(matches!(
        errors[0].def,
        Error::ErrInvalidAvailabilityOrder(..)
    ));
}
