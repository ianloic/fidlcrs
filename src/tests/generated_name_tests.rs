use super::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_inside_struct() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  bar @generated_name("Good") struct {};
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_struct("fidl.test/Foo")
        .expect("struct Foo not found");
    assert_eq!(foo.members[0].type_.identifier().unwrap(), "fidl.test/Good");
}

#[test]
fn good_inside_table() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = table {
  1: bar @generated_name("Good") struct {};
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_table("fidl.test/Foo")
        .expect("table Foo not found");
    assert_eq!(
        foo.members[0].type_.as_ref().unwrap().identifier().unwrap(),
        "fidl.test/Good"
    );
}

#[test]
fn good_inside_union() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = union {
  1: bar @generated_name("Good") struct {};
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_union("fidl.test/Foo")
        .expect("union Foo not found");
    assert_eq!(
        foo.members[0].type_.as_ref().unwrap().identifier().unwrap(),
        "fidl.test/Good"
    );
}

#[test]
fn good_inside_request() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

protocol Foo {
  Bar(@generated_name("Good") struct { x uint32; });
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_protocol("fidl.test/Foo")
        .expect("protocol Foo not found");
    let req = foo.methods[0].maybe_request_payload.as_ref().unwrap();
    assert_eq!(req.identifier().unwrap(), "fidl.test/Good");
}

#[test]
fn good_inside_response() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

protocol Foo {
  strict Bar() -> (@generated_name("Good") struct { x uint32; });
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_protocol("fidl.test/Foo")
        .expect("protocol Foo not found");
    let res = foo.methods[0].maybe_response_payload.as_ref().unwrap();
    assert_eq!(res.identifier().unwrap(), "fidl.test/Good");
}

#[test]
fn good_inside_result_success() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

protocol Foo {
  Bar() -> (@generated_name("Good") struct { x uint32; }) error uint32;
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_protocol("fidl.test/Foo")
        .expect("protocol Foo not found");

    // Result response type should be fidl.test/FooBarResult
    let result_name = foo.methods[0]
        .maybe_response_payload
        .as_ref()
        .unwrap()
        .identifier()
        .unwrap();
    let result_union = root
        .lookup_union(&result_name)
        .expect("result union not found");
    assert_eq!(
        result_union.members[0]
            .type_
            .as_ref()
            .unwrap()
            .identifier()
            .unwrap(),
        "fidl.test/Good"
    );
}

#[test]
fn good_inside_result_error() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

protocol Foo {
  Bar() -> () error @generated_name("Good") enum { A = 1; };
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_protocol("fidl.test/Foo")
        .expect("protocol Foo not found");

    let result_name = foo.methods[0]
        .maybe_response_payload
        .as_ref()
        .unwrap()
        .identifier()
        .unwrap();
    let result_union = root
        .lookup_union(&result_name)
        .expect("result union not found");
    assert_eq!(
        result_union.members[1]
            .type_
            .as_ref()
            .unwrap()
            .identifier()
            .unwrap(),
        "fidl.test/Good"
    );
}

#[test]
fn good_on_bits() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  bar @generated_name("Good") bits {
    A = 1;
  };
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_struct("fidl.test/Foo")
        .expect("struct Foo not found");
    assert_eq!(foo.members[0].type_.identifier().unwrap(), "fidl.test/Good");
}

#[test]
fn good_on_enum() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  bar @generated_name("Good") enum {
    A = 1;
  };
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_struct("fidl.test/Foo")
        .expect("struct Foo not found");
    assert_eq!(foo.members[0].type_.identifier().unwrap(), "fidl.test/Good");
}

#[test]
fn good_on_struct() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  bar @generated_name("Good") struct {
    x uint32;
  };
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_struct("fidl.test/Foo")
        .expect("struct Foo not found");
    assert_eq!(foo.members[0].type_.identifier().unwrap(), "fidl.test/Good");
}

#[test]
fn good_on_table() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  bar @generated_name("Good") table {
    1: x uint32;
  };
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_struct("fidl.test/Foo")
        .expect("struct Foo not found");
    assert_eq!(foo.members[0].type_.identifier().unwrap(), "fidl.test/Good");
}

#[test]
fn good_on_union() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  bar @generated_name("Good") union {
    1: x uint32;
  };
};
        "#,
    );
    let root = library.compile().expect("compilation failed");
    let foo = root
        .lookup_struct("fidl.test/Foo")
        .expect("struct Foo not found");
    assert_eq!(foo.members[0].type_.identifier().unwrap(), "fidl.test/Good");
}

#[test]
fn good_prevents_collision() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  foo @generated_name("Bar") struct {};
};
        "#,
    );
    let result = library.compile();
    assert!(result.is_ok(), "compilation failed");
}

#[test]
#[ignore]
fn bad_on_type_declaration() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

@generated_name("Good")
type Bad = struct {};
        "#,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrInvalidAttributePlacement"
    );
}

#[test]
fn bad_on_top_level_struct() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = @generated_name("Bad") struct {};
        "#,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrAttributeInsideTypeDeclaration"
    );
}

#[test]
fn bad_on_identifier_type() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  bar @generated_name("Bad") Bar;
};

type Bar = struct {};
        "#,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrCannotAttachAttributeToIdentifier"
    );
}

#[test]
#[ignore]
fn bad_on_struct_member() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0120-b.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrInvalidAttributePlacement"
    );
}

#[test]
#[ignore]
fn bad_on_enum_member() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type MetaVars = enum {
  FOO = 1;
  @generated_name("BAD")
  BAR = 2;
};
        "#,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrInvalidAttributePlacement"
    );
}

#[test]
#[ignore]
fn bad_on_service_member() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

protocol Foo {};

service Bar {
  @generated_name("One")
  bar_one client_end:Foo;
};
        "#,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrInvalidAttributePlacement"
    );
}

#[test]
#[ignore]
fn bad_missing_argument() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  bad @generated_name struct {};
};
        "#,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrMissingRequiredAnonymousAttributeArg"
    );
}

#[test]
#[ignore]
fn bad_invalid_identifier() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0146.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrInvalidGeneratedName"
    );
}

#[test]
#[ignore]
fn bad_name_collision() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example0.fidl",
        r#"
library fidl.test;

type Foo = struct {
  foo @generated_name("Baz") struct {};
};

type Baz = struct {};
        "#,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrNameCollision"
    );
}

#[test]
#[ignore]
fn bad_not_string() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0104.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrTypeCannotBeConvertedToType / ErrCouldNotResolveAttributeArg"
    );
}

#[test]
#[ignore]
fn bad_non_literal_argument() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0133.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrAttributeArgRequiresLiteral"
    );
}
