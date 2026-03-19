use crate::tests::test_library::{LookupHelpers, TestLibrary};

use crate::diagnostics::Error;
use crate::flat_ast::Openness;
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
    assert_eq!(type_decl.openness, Openness::Open);
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
    assert_eq!(type_decl.openness, Openness::Open);
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
    assert_eq!(type_decl.openness, Openness::Ajar);
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
    assert_eq!(type_decl.openness, Openness::Closed);
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
    lib.expect_fail(Error::ErrMustHaveOneMember);
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
    lib.expect_fail(Error::ErrMustHaveOneMember);
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

    lib.expect_fail(Error::ErrExpectedDeclaration(r#"Empty"#.into()));
    lib.expect_fail(Error::ErrExpectedDeclaration(r#"}"#.into()));
    lib.expect_fail(Error::ErrExpectedDeclaration(r#"{"#.into()));
    lib.expect_fail(Error::ErrExpectedDeclaration(r#";"#.into()));

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
    lib.expect_fail(Error::ErrInvalidProtocolMember);
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
    lib.expect_fail(Error::ErrComposedProtocolTooOpen(
        r#"closed"#.into(),
        r#"Composing"#.into(),
        r#"open"#.into(),
        r#"example/Composed"#.into(),
    ));
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
    lib.expect_fail(Error::ErrCannotSpecifyModifier(
        r#"strict"#.into(),
        r#"compose"#.into(),
    ));
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
    lib.expect_fail(Error::ErrNameCollision(
        r#"method"#.into(),
        r#"MyMethod"#.into(),
        r#"method"#.into(),
        r#"example.fidl:4:5"#.into(),
    ));
    assert!(lib.check_compile());
}

#[test]

fn bad_request_must_be_protocol() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0157.test.fidl");
    lib.expect_fail(Error::ErrMustBeAProtocol(
        r#"test.bad.fi0157/MyStruct"#.into(),
    ));
    assert!(lib.check_compile());
}

#[test]

fn bad_request_must_be_parameterized() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0168.test.fidl");
    lib.expect_fail(Error::ErrWrongNumberOfLayoutParameters(
        r#"client_end/server_end"#.into(),
        1,
        0,
    ));
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

    lib.expect_fail(Error::ErrEmptyPayloadStructs(r#"MyMethod"#.into()));

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
    lib.expect_fail(Error::ErrUnexpectedTokenOfKind(
        r#"Identifier"#.into(),
        r#"RightCurly"#.into(),
    ));
    lib.expect_fail(Error::ErrUnexpectedTokenOfKind(
        r#"Identifier"#.into(),
        r#"Semicolon"#.into(),
    ));
    lib.expect_fail(Error::ErrExpectedDeclaration(r#";"#.into()));
    lib.expect_fail(Error::ErrExpectedDeclaration(r#"int32"#.into()));
    lib.expect_fail(Error::ErrExpectedDeclaration(r#"}"#.into()));

    assert!(lib.check_compile());
}

#[test]

fn bad_disallowed_request_type() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0075.test.fidl");
    lib.expect_fail(Error::ErrInvalidMethodPayloadLayoutClass(
        r#"provided type"#.into(),
    ));
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
    lib.expect_fail(Error::ErrInvalidMethodPayloadLayoutClass(
        r#"provided type"#.into(),
    ));
    assert!(lib.check_compile());
}

#[test]
#[ignore]
fn bad_ajar_missing_protocol_token() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

ajar Empty {};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_closed_missing_protocol_token() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

closed Empty {};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_invalid_compose_ajar_in_closed() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

ajar protocol Composed {};

closed protocol Composing {
  compose Composed;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_invalid_compose_open_in_ajar() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0114.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_flexible_on_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol A {};

protocol B {
  flexible compose A;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_strict_on_invalid_member() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Example {
  strict;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_modifier_flexible_on_invalid_member() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Example {
  flexible;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_colon_not_supported() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Parent {};
protocol Child : Parent {};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_doc_comment_outside_attributelist() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol WellDocumented {
    Method();
    /// Misplaced doc comment
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_cannot_compose_yourself() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Narcisse {
    compose Narcisse;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_cannot_mutually_compose() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0057-b.test.fidl");

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_cannot_compose_same_protocol_twice() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Parent {
    Method();
};

protocol Child {
    compose Parent;
    compose Parent;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_cannot_compose_missing_protocol() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Child {
    compose MissingParent;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_cannot_compose_non_protocol() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0073.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_cannot_use_ordinals_in_protocol_declaration() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol NoMoreOrdinals {
    42: NiceTry();
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_empty_named_item() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0020.noformat.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_no_other_pragma_than_compose() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Wrong {
    not_compose Something;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_duplicate_method_names_from_immediate_composition() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyChildProtocol {
    MyMethod();
};

protocol MyProtocol {
    compose MyChildProtocol;
    MyMethod();
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_duplicate_method_names_from_multiple_composition() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol A {
    Method();
};

protocol B {
    Method();
};

protocol C {
    compose A;
    compose B;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_duplicate_method_names_from_nested_composition() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

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
    MethodA();
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_composed_protocols_have_clashing_ordinals() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library methodhasher;

protocol SpecialComposed {
   ClashOne();
};

protocol Special {
    compose SpecialComposed;
    ClashTwo();
};
"#,
    );

    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_request_must_be_protocol_with_optional() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = struct {};
alias ServerEnd = server_end:<MyStruct, optional>;
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_request_must_contain_protocol() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
    MyMethod(resource struct { server server_end; });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_request_cannot_have_size() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol P {};
type S = struct {
    p server_end:<P,0>;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_duplicate_parameter_name() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol P {
  MethodWithDuplicateParams(struct {foo uint8; foo uint8; });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_parameterized_typed_channel() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {};

type Foo = resource struct {
  foo client_end<MyProtocol>;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_too_many_constraints_typed_channel() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {};

type Foo = resource struct {
  foo client_end:<MyProtocol, optional, 1, 2>;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_struct_layout_default_member() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
  MyMethod(struct {
    @allow_deprecated_struct_defaults
    foo uint8 = 1;
  });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_event_empty_payload_struct() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0077-b.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_enum_layout() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0074.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_absent_response_with_error() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
    MyMethod() . () error uint32;
};
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_empty_struct_response_with_error() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
    MyMethod() . (struct {}) error uint32;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_named_alias() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = struct {
  a bool;
};

alias MyStructAlias = MyStruct;
alias MyAliasAlias = MyStructAlias;

protocol MyProtocol {
    MyMethod(MyStructAlias) . (MyAliasAlias);
};
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_named_empty_payload_struct() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = struct{};

protocol MyProtocol {
    MyMethod(MyStruct) . (MyStruct);
};
"#,
    );
    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_named_default_value_struct() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0084.test.fidl");
    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_named_invalid_handle() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type ObjType = strict enum : uint32 {
    NONE = 0;
    VMO = 3;
};

type Rights = strict bits : uint32 {
    TRANSFER = 1;
};

resource_definition Handle : uint32 {
    properties {
        subtype ObjType;
        rights Rights;
    };
};

protocol MyProtocol {
    MyMethod(Handle);
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_named_invalid_alias() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type ObjType = strict enum : uint32 {
    NONE = 0;
    VMO = 3;
};

type Rights = strict bits : uint32 {
    TRANSFER = 1;
};

resource_definition Handle : uint32 {
    properties {
        subtype ObjType;
        rights Rights;
    };
};

alias MyPrimAlias = bool;
alias MyHandleAlias = Handle;
alias MyVectorAlias = vector<MyPrimAlias>;
alias MyAliasAlias = MyVectorAlias:optional;

protocol MyProtocol {
    strict MyMethod(MyPrimAlias) . (MyHandleAlias);
    strict MyOtherMethod(MyVectorAlias) . (MyAliasAlias);
};
"#,
    );

    // expect_fail
    // expect_fail
    // expect_fail
    // TODO(https://fxbug.dev/42175844): Should be "vector<bool>:optional".
    // expect_fail

    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_method_named_invalid_kind() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyOtherProtocol {
  MyOtherMethod();
};

service MyService {
  my_other_protocol client_end:MyOtherProtocol;
};

protocol MyProtocol {
    MyMethod(MyOtherProtocol) . (MyService);
};
"#,
    );
    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_table_response() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyOtherProtocol {};

type MyTable = resource table {
  1: a client_end:<MyProtocol>;
};

protocol MyProtocol {
  MyMethod() . (table {
    1: b bool;
  });
  . OnMyEvent(MyTable);
};
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_table_result_payload() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyOtherProtocol {};

type MyTable = resource table {
  1: a client_end:<MyProtocol>;
};

protocol MyProtocol {
  MyMethod() . (MyTable) error uint32;
  MyAnonResponseMethod() . (table {
    1: b bool;
  }) error uint32;
};
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_union_request() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyOtherProtocol {};

type MyUnion = strict resource union {
  1: a client_end:<MyProtocol>;
};

protocol MyProtocol {
  MyMethodOneWay(flexible union {
    1: b bool;
  });
  MyMethodTwoWay(MyUnion) . ();
};
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_method_union_result_payload() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyOtherProtocol {};

type MyUnion = strict resource union {
  1: a client_end:<MyProtocol>;
};

protocol MyProtocol {
  MyMethod() . (MyUnion) error uint32;
  MyAnonResponseMethod() . (flexible union {
    1: b bool;
  }) error uint32;
};
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_event_error_syntax() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
    . OnMyEvent() error int32;
};
"#,
    );
    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_invalid_request_type() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
    MyMethod(box);
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_invalid_response_type() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
    MyMethod() . (box);
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_disallowed_success_type() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
    MyMethod() . (uint32) error uint32;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_invalid_success_type() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {
    MyMethod() . (box) error uint32;
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}
