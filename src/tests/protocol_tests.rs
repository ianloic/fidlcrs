use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_valid_empty_protocol() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol Empty {};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_protocol("example/Empty")
        .expect("Empty protocol not found");
    assert_eq!(type_decl.methods.len(), 0);
    assert_eq!(type_decl.openness, crate::flat_ast::Openness::Open);
}

#[test]
fn good_valid_empty_open_protocol() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

open protocol Empty {};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_protocol("example/Empty")
        .expect("Empty protocol not found");
    assert_eq!(type_decl.methods.len(), 0);
    assert_eq!(type_decl.openness, crate::flat_ast::Openness::Open);
}

#[test]
fn good_valid_empty_ajar_protocol() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

ajar protocol Empty {};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_protocol("example/Empty")
        .expect("Empty protocol not found");
    assert_eq!(type_decl.methods.len(), 0);
    assert_eq!(type_decl.openness, crate::flat_ast::Openness::Ajar);
}

#[test]
fn good_valid_empty_closed_protocol() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

closed protocol Empty {};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let type_decl = root
        .lookup_protocol("example/Empty")
        .expect("Empty protocol not found");
    assert_eq!(type_decl.methods.len(), 0);
    assert_eq!(type_decl.openness, crate::flat_ast::Openness::Closed);
}

#[test]

fn bad_empty_strict_protocol() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

strict protocol Empty {};
"#,
    );
    lib.expect_fail(crate::diagnostics::Error::ErrMustHaveOneMember, &[]);
    assert!(lib.check_compile());
}

#[test]

fn bad_empty_flexible_protocol() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

flexible protocol Empty {};
"#,
    );
    lib.expect_fail(crate::diagnostics::Error::ErrMustHaveOneMember, &[]);
    assert!(lib.check_compile());
}

#[test]

fn bad_open_missing_protocol_token() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

open Empty {};
"#,
    );
    lib.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &[]);
    lib.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &[]);
    lib.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &[]);
    lib.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &[]);
    assert!(lib.check_compile());
}

#[test]

fn bad_empty_protocol_member() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol Example {
  ;
};
"#,
    );
    lib.expect_fail(crate::diagnostics::Error::ErrInvalidProtocolMember, &[]);
    assert!(lib.check_compile());
}

#[test]
fn good_valid_protocol_composition() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol A {
    MethodA();
};

protocol B {
    compose A;
    MethodB();
};

protocol C {
    compose A;
    MethodC();
};

protocol D {
    compose B;
    compose C;
    MethodD();
};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let protocol_a = root.lookup_protocol("example/A").expect("A not found");
    assert_eq!(protocol_a.methods.len(), 1);

    let protocol_b = root.lookup_protocol("example/B").expect("B not found");
    assert_eq!(protocol_b.methods.len(), 2);

    let protocol_d = root.lookup_protocol("example/D").expect("D not found");
    assert_eq!(protocol_d.methods.len(), 5);
    assert_eq!(protocol_d.composed_protocols.len(), 2);
}

#[test]
fn good_valid_open_closed_protocol_composition() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

closed protocol Closed {};
ajar protocol Ajar {};
open protocol Open {};

closed protocol ComposeInClosed {
  compose Closed;
};

ajar protocol ComposeInAjar {
  compose Closed;
  compose Ajar;
};

open protocol ComposeInOpen {
  compose Closed;
  compose Ajar;
  compose Open;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let p1 = root.lookup_protocol("example/ComposeInClosed").unwrap();
    assert_eq!(p1.composed_protocols.len(), 1);

    let p2 = root.lookup_protocol("example/ComposeInAjar").unwrap();
    assert_eq!(p2.composed_protocols.len(), 2);

    let p3 = root.lookup_protocol("example/ComposeInOpen").unwrap();
    assert_eq!(p3.composed_protocols.len(), 3);
}

#[test]

fn bad_invalid_compose_open_in_closed() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

open protocol Composed {};

closed protocol Composing {
  compose Composed;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrComposedProtocolTooOpen,
        &[
            "\"closed\"",
            "\"Composing\"",
            "\"open\"",
            "\"example/Composed\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_modifier_strict_on_compose() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol A {};

protocol B {
  strict compose A;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrCannotSpecifyModifier,
        &["\"strict\"", "\"compose\""],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_attach_attributes_to_compose() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol ParentA {
    ParentMethodA();
};

protocol ParentB {
    ParentMethodB();
};

protocol Child {
    @this_is_allowed
    compose ParentA;
    /// This is also allowed.
    compose ParentB;
    ChildMethod();
};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let child = root.lookup_protocol("example/Child").unwrap();
    assert_eq!(child.composed_protocols.len(), 2);
    assert_eq!(child.composed_protocols[0].maybe_attributes.len(), 1);
    assert_eq!(
        child.composed_protocols[0].maybe_attributes[0].name,
        "this_is_allowed"
    );
    assert_eq!(child.composed_protocols[1].maybe_attributes.len(), 1);
    assert_eq!(child.composed_protocols[1].maybe_attributes[0].name, "doc");
}

#[test]

fn bad_duplicate_method_names() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol MyProtocol {
    MyMethod();
    MyMethod();
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrNameCollision,
        &[
            "\"method\"",
            "\"MyMethod\"",
            "\"method\"",
            "\"example.fidl:4:5\"",
        ],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_request_must_be_protocol() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0157.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrMustBeAProtocol,
        &["\"test.bad.fi0157/MyStruct\""],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_request_must_be_parameterized() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0168.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrWrongNumberOfLayoutParameters,
        &["\"client_end/server_end\"", "1", "0"],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_typed_channels() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol MyProtocol {};

type Foo = resource struct {
    a client_end:MyProtocol;
    b client_end:<MyProtocol, optional>;
    c server_end:MyProtocol;
    d server_end:<MyProtocol, optional>;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_partial_typed_channel_constraints() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol MyProtocol {};

alias ClientEnd = client_end:MyProtocol;
alias ServerEnd = server_end:MyProtocol;

type Foo = resource struct {
    a ClientEnd;
    b ClientEnd:optional;
    c ServerEnd;
    d ServerEnd:optional;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_method_absent_payload_struct() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0077-a.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]
fn good_event_absent_payload_struct() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0077-b.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_method_empty_payload_struct() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0077-a.test.fidl");
    lib.expect_fail(crate::diagnostics::Error::ErrEmptyPayloadStructs, &[]);
    lib.expect_fail(crate::diagnostics::Error::ErrEmptyPayloadStructs, &[]);
    assert!(lib.check_compile());
}

#[test]
fn good_method_named_type_request() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type MyStruct = struct{
  a bool;
};

protocol MyProtocol {
    MyMethodOneWay(MyStruct);
    MyMethodTwoWay(MyStruct) -> ();
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_method_named_type_response() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type MyStruct = struct{
  a bool;
};

protocol MyProtocol {
  MyMethod() -> (MyStruct);
    -> OnMyEvent(MyStruct);
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_method_named_type_result_payload() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type MyStruct = struct{
  a bool;
};

protocol MyProtocol {
  MyMethod() -> (MyStruct) error uint32;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_method_table_request() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol MyOtherProtocol {};

type MyTable = resource table {
  1: a client_end:<MyProtocol>;
};

protocol MyProtocol {
  MyMethodOneWay(table {
    1: b bool;
  });
  MyMethodTwoWay(MyTable) -> ();
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_method_union_response() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol MyOtherProtocol {};

type MyUnion = strict resource union {
  1: a client_end:<MyProtocol>;
};

protocol MyProtocol {
  MyMethod() -> (flexible union {
    1: b bool;
  });
  -> OnMyEvent(MyUnion);
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_one_way_error_syntax() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol MyProtocol {
    MyOneWay() error int32;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrUnexpectedTokenOfKind,
        &["\"Identifier\"", "\"Semicolon\""],
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrUnexpectedTokenOfKind,
        &["\"Identifier\"", "\"RightCurly\""],
    );
    lib.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &[]);
    lib.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &[]);
    lib.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &[]);
    lib.expect_fail(crate::diagnostics::Error::ErrExpectedDeclaration, &[]);
    assert!(lib.check_compile());
}

#[test]

fn bad_disallowed_request_type() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0075.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrInvalidMethodPayloadLayoutClass,
        &["\"provided type\""],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_disallowed_response_type() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

protocol MyProtocol {
    MyMethod() -> (uint32);
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrInvalidMethodPayloadLayoutClass,
        &["\"provided type\""],
    );
    assert!(lib.check_compile());
}
