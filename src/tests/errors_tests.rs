use crate::flat_ast;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_error() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;

protocol Example {
    strict Method() -> (struct {
        foo string;
    }) error int32;
};
"#,
    );
    let root = library.compile().expect("compilation failed");
    let decl = root
        .lookup_protocol("example/Example")
        .expect("protocol not found");
    assert_eq!(decl.methods.len(), 1);
    let method = &decl.methods[0];
    assert!(method.has_error);
    let err_type = method
        .maybe_response_err_type
        .as_ref()
        .expect("error type not found");
    assert!(matches!(err_type, flat_ast::Type::Primitive(_)));
}

#[test]
fn good_error_unsigned() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;

protocol Example {
    Method() -> (struct {
        foo string;
    }) error uint32;
};
"#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_error_empty_struct_as_success() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;

protocol MyProtocol {
  strict MyMethod() -> () error uint32;
};
"#,
    );
    let root = library.compile().expect("compilation failed");
    let decl = root
        .lookup_protocol("example/MyProtocol")
        .expect("protocol not found");
    let method = &decl.methods[0];
    assert!(method.has_error);
    assert!(method.maybe_response_err_type.is_some());
}

#[test]
fn good_error_enum() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;

type ErrorType = enum : int32 {
    GOOD = 1;
    BAD = 2;
    UGLY = 3;
};

protocol Example {
    Method() -> (struct {
        foo string;
    }) error ErrorType;
};
"#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn good_error_enum_after() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;

protocol Example {
    Method() -> (struct {
        foo string;
    }) error ErrorType;
};

type ErrorType = enum : int32 {
    GOOD = 1;
    BAD = 2;
    UGLY = 3;
};
"#,
    );
    library.compile().expect("compilation failed");
}

#[test]
fn bad_error_unknown_identifier() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0052.test.fidl");
    library.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &["\"ParsingError\"", "\"test.bad.fi0052\""],
    );
    library.expect_fail(crate::diagnostics::Error::ErrInvalidErrorType, &[]);
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrNameNotFound"
    );
}

#[test]
fn bad_error_wrong_primitive() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0141.test.fidl");
    library.expect_fail(crate::diagnostics::Error::ErrInvalidErrorType, &[]);
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrInvalidErrorType"
    );
}

#[test]
fn bad_error_missing_type() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;
protocol Example {
    Method() -> (flub int32) error;
};
"#,
    );
    library.expect_fail(crate::diagnostics::Error::ErrUnexpectedToken, &[]);
    library.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &["\"flub\"", "\"example\""],
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrInvalidMethodPayloadLayoutClass,
        &["\"provided type\""],
    );
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrUnexpectedToken"
    );
}

#[test]
fn bad_error_not_a_type() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;
protocol Example {
    Method() -> (flub int32) error "hello";
};
"#,
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrNameNotFound,
        &["\"flub\"", "\"example\""],
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrInvalidMethodPayloadLayoutClass,
        &["\"provided type\""],
    );
    library.expect_fail(crate::diagnostics::Error::ErrExpectedType, &[]);
    library.expect_fail(crate::diagnostics::Error::ErrInvalidErrorType, &[]);
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrExpectedType"
    );
}

#[test]
fn bad_error_no_response() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;
protocol Example {
    Method() -> error int32;
};
"#,
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrUnexpectedTokenOfKind,
        &["\"Identifier\"", "\"LeftParen\""],
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrUnexpectedTokenOfKind,
        &["\"Identifier\"", "\"RightCurly\""],
    );
    library.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &["{}"]);
    library.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &["{}"]);
    library.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &["{}"]);
    library.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &["{}"]);
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrUnexpectedTokenOfKind"
    );
}

#[test]
fn bad_error_unexpected_end_of_file() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;
type ForgotTheSemicolon = table {}
"#,
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrUnexpectedTokenOfKind,
        &["\"EndOfFile\"", "\"Semicolon\""],
    );
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrUnexpectedTokenOfKind"
    );
}

#[test]
fn bad_incorrect_identifier() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0009.noformat.test.fidl");
    library.expect_fail(
        crate::diagnostics::Error::ErrUnexpectedIdentifier,
        &["\"using\"", "\"library\""],
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrUnknownLibrary,
        &["\"test.bad.fi0009\""],
    );
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrUnexpectedIdentifier"
    );
}

#[test]
fn bad_error_empty_file() {
    let mut library = TestLibrary::new();

    library.add_source_file("example0.fidl", "");
    library.expect_fail(
        crate::diagnostics::Error::ErrUnexpectedIdentifier,
        &["\"end of file\"", "\"library\""],
    );
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrUnexpectedIdentifier"
    );
}

#[test]
fn experimental_allow_arbitrary_error_types() {
    let mut library = TestLibrary::new();
    library.enable_flag("allow_arbitrary_error_types");

    library.add_source_file(
        "example0.fidl",
        r#"

library example;
protocol Example {
    Method() -> () error table {};
};
"#,
    );
    let root = library.compile().expect("compilation failed");

    let decl = root
        .lookup_protocol("example/Example")
        .expect("protocol not found");
    assert_eq!(decl.methods.len(), 1);
    let method = &decl.methods[0];
    assert!(method.has_error);
    let err_type = method
        .maybe_response_err_type
        .as_ref()
        .expect("error type not found");
    assert!(matches!(err_type, flat_ast::Type::Identifier(_)));
}

#[test]
fn transitional_removed() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"

library example;
protocol Example {
    @transitional
    Method();
};
"#,
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrDeprecatedAttribute,
        &["\"transitional\""],
    );
    library.expect_fail(
        crate::diagnostics::Error::ErrDeprecatedAttribute,
        &["\"transitional\""],
    );
    assert!(
        library.check_compile(),
        "expected compilation to fail with ErrDeprecatedAttribute"
    );
}
