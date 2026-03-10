use crate::flat_ast;
use crate::source_file::SourceFile;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_error() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;

protocol Example {
    strict Method() -> (struct {
        foo string;
    }) error int32;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
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
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;

protocol Example {
    Method() -> (struct {
        foo string;
    }) error uint32;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
}

#[test]
fn good_error_empty_struct_as_success() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;

protocol MyProtocol {
  strict MyMethod() -> () error uint32;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
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
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
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
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
}

#[test]
fn good_error_enum_after() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
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
"#
        .to_string(),
    );
    library.add_source(&source0);
    library.compile().expect("compilation failed");
}

#[test]
fn bad_error_unknown_identifier() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0052.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
fn bad_error_wrong_primitive() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0141.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
fn bad_error_missing_type() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    Method() -> (flub int32) error;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
fn bad_error_not_a_type() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    Method() -> (flub int32) error "hello";
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
fn bad_error_no_response() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    Method() -> error int32;
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
fn bad_error_unexpected_end_of_file() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
type ForgotTheSemicolon = table {}
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
fn bad_incorrect_identifier() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0009.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
fn bad_error_empty_file() {
    let mut library = TestLibrary::new();
    let source0 = SourceFile::new("example0.fidl".to_string(), "".to_string());
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}

#[test]
fn experimental_allow_arbitrary_error_types() {
    let mut library = TestLibrary::new();
    library.enable_flag("allow_arbitrary_error_types");
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    Method() -> () error table {};
};
"#
        .to_string(),
    );
    library.add_source(&source0);
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
    let source0 = SourceFile::new(
        "example0.fidl".to_string(),
        r#"

library example;
protocol Example {
    @transitional
    Method();
};
"#
        .to_string(),
    );
    library.add_source(&source0);
    let result = library.compile();
    assert!(result.is_err(), "expected compilation to fail");
}
