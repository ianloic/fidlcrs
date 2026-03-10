use crate::attribute_schema::ArgType;
use crate::attribute_schema::AttributeArgSchema;
use crate::attribute_schema::AttributeSchema;
use crate::attribute_schema::ConstantValueKind;
use crate::attribute_schema::Kind;
use crate::attribute_schema::Optionality;
use crate::compiler::Compiler;
use crate::diagnostics::Error;
use crate::raw_ast;
use crate::source_span::SourceSpan;
use crate::tests::test_library::{SharedAmongstLibraries, TestLibrary};

#[test]
fn good_placement_of_attributes() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library exampleusing;

@on_dep_struct
type Empty = struct {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_official_attributes() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
@no_doc
library example;

/// For EXAMPLE_CONSTANT
@no_doc
@deprecated("Note")
const EXAMPLE_CONSTANT string = "foo";

/// For ExampleEnum
@deprecated("Reason")
type ExampleEnum = flexible enum {
    A = 1;
    /// For EnumMember
    @unknown
    B = 2;
};

/// For ExampleStruct
@max_bytes("1234")
@max_handles("5678")
type ExampleStruct = resource struct {
  data @generated_name("CustomName") table {
    1: a uint8;
  };
};

/// For ExampleProtocol
@discoverable
@transport("Syscall")
protocol ExampleProtocol {
    /// For ExampleMethod
    @internal
    @selector("Bar")
    ExampleMethod();
};

/// For ExampleService
@foo("ExampleService")
@no_doc
service ExampleService {
    /// For ExampleProtocol
    @foo("ExampleProtocol")
    @no_doc
    p client_end:ExampleProtocol;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_no_attribute_on_using_not_event_doc() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0045-a.test.fidl");
    lib.add_errcat_file("bad/fi-0045-b.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_no_two_same_attribute() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test.dupattributes;

@dup("first")
@dup("second")
protocol A {
    MethodA();
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_no_two_same_attribute_canonical() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0123.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_doc_attribute() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0028-b.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_no_two_same_doc_attribute() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test.dupattributes;

/// first
@doc("second")
protocol A {
    MethodA();
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_no_two_same_attribute_on_library() {
    let mut lib = TestLibrary::new();

    lib.add_source_file(
        "first.fidl",
        r#"
@dup("first")
library fidl.test.dupattributes;
"#,
    );

    lib.add_source_file(
        "second.fidl",
        r#"
@dup("second")
library fidl.test.dupattributes;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn warn_on_close_to_official_attribute() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0145.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_not_too_close_unofficial_attribute() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0145.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]
fn warn_on_close_attribute_with_other_errors() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
@available(platform="foo", added=1)
library fidl.test;

@available(added=1, removed=2)
type Foo = struct {};

// This actually gets added at 1 because we misspelled "available".
@availabe(added=2)
type Foo = resource struct {};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_warnings_as_errors() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@duc("should be doc")
protocol A {
    MethodA();
};
"#,
    );
}

#[test]
fn bad_unknown_argument() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0129.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_empty_transport() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0128.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_unrecognized_transport() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0142.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_channel_transport() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test.transportattributes;

@transport("Channel")
protocol A {
    MethodA();
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_syscall_transport() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test.transportattributes;

@transport("Syscall")
protocol A {
    MethodA();
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_multiple_transports() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test.transportattributes;

@transport("Channel, Syscall")
protocol A {
    MethodA();
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_unknown_invalid_placement_on_union() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@unknown
type U = flexible union {
  1: a int32;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_unknown_invalid_placement_on_union_member() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

type U = flexible union {
  @unknown 1: a int32;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_unknown_invalid_placement_on_bits_member() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

type B = flexible bits : uint32 {
  @unknown A = 0x1;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_unknown_invalid_on_strict_enum_member() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0071.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_incorrect_placement_layout() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
@selector("test") // 1
library fidl.test;

@selector("test") // 2
const MyConst uint32 = 0;

@selector("test") // 3
type MyEnum = enum {
    @selector("test") // 4
    MyMember = 5;
};

@selector("test") // 5
type MyStruct = struct {
    @selector("test") // 6
    MyMember int32;
};

@selector("test") // 7
type MyUnion = union {
    @selector("test") // 8
    1: MyMember int32;
};

@selector("test") // 9
type MyTable = table {
    @selector("test") // 10
    1: MyMember int32;
};

@selector("test") // 11
protocol MyProtocol {
    @selector("test") // no error, this placement is allowed
    MyMethod();
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_single_deprecated_attribute() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0121.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_deprecated_attributes() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@example_deprecated_attribute
type MyStruct = struct {};

@example_deprecated_attribute
protocol MyOtherProtocol {
  MyMethod();
};

@example_deprecated_attribute
protocol MyProtocol {
  MyMethod();
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_constraint_only_three_members_on_struct() {
    fn must_have_three_members(compiler: &Compiler, attr: &raw_ast::Attribute) -> bool {
        let span: SourceSpan = unsafe { std::mem::transmute(attr.element.span().clone()) };
        compiler.reporter.fail(
            Error::ErrInvalidAttributePlacement,
            span,
            &[&"must_have_three_members".to_string()],
        );
        false
    }
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@must_have_three_members
type MyStruct = struct {
    one int64;
    two int64;
    three int64;
    oh_no_four int64;
};
"#,
    );
    let s_must = AttributeSchema::new(Kind::ValidateOnly).constrain(must_have_three_members);
    lib.add_attribute_schema("must_have_three_members", s_must);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_constraint_only_three_members_on_method() {
    fn must_have_three_members(compiler: &Compiler, attr: &raw_ast::Attribute) -> bool {
        let span: SourceSpan = unsafe { std::mem::transmute(attr.element.span().clone()) };
        compiler.reporter.fail(
            Error::ErrInvalidAttributePlacement,
            span,
            &[&"must_have_three_members".to_string()],
        );
        false
    }
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

protocol MyProtocol {
    @must_have_three_members MyMethod();
};
"#,
    );
    let s_must = AttributeSchema::new(Kind::ValidateOnly).constrain(must_have_three_members);
    lib.add_attribute_schema("must_have_three_members", s_must);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_constraint_only_three_members_on_protocol() {
    fn must_have_three_members(compiler: &Compiler, attr: &raw_ast::Attribute) -> bool {
        let span: SourceSpan = unsafe { std::mem::transmute(attr.element.span().clone()) };
        compiler.reporter.fail(
            Error::ErrInvalidAttributePlacement,
            span,
            &[&"must_have_three_members".to_string()],
        );
        false
    }
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@must_have_three_members
protocol MyProtocol {
    MyMethod();
    MySecondMethod();
};
"#,
    );
    let s_must = AttributeSchema::new(Kind::ValidateOnly).constrain(must_have_three_members);
    lib.add_attribute_schema("must_have_three_members", s_must);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_attribute_value() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0132.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_selector_incorrect_placement() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0120-a.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_parameter_attribute_incorrect_placement() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

protocol ExampleProtocol {
    Method(struct { arg exampleusing.Empty; } @on_parameter);
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_attribute_on_top_level_layout() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0023.noformat.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_layout_attribute_placements() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@foo
type Foo = struct {};

protocol MyProtocol {
  MyMethod(@baz struct {
    inner_layout @qux struct {};
  });
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_no_arguments_empty_parens() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0014.noformat.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_multiple_arguments() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(bar="abc", baz="def")
type MyStruct = struct {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_multiple_arguments_with_no_names() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0015.noformat.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_multiple_arguments_some_names_unnamed_string_arg_first() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo("abc", bar="def")
type MyStruct = struct {};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_multiple_arguments_some_names_unnamed_string_arg_second() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(bar="abc", "def")
type MyStruct = struct {};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_multiple_arguments_some_names_unnamed_identifier_arg_first() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo("abc", bar=def)
type MyStruct = struct {};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_multiple_arguments_some_names_unnamed_identifier_arg_second() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(bar="abc", def)
type MyStruct = struct {};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_multiple_arguments_duplicate_names() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0130.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_multiple_arguments_duplicate_canonical_names() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0131.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_single_argument_is_not_named() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo("bar")
type MyStruct = struct {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_single_argument_is_named_without_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(a="bar")
type MyStruct = struct {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_single_schema_argument() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo("bar")
type MyStruct = struct {};
"#,
    );
    let mut s_foo = AttributeSchema::new(Kind::ValidateOnly);
    s_foo = s_foo.add_arg(
        "value",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    lib.add_attribute_schema("foo", s_foo);
    lib.compile().expect("compilation failed");
}

#[test]
fn good_single_schema_argument_with_inferred_name() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo("bar")
type MyStruct = struct {};
"#,
    );
    let mut s_foo = AttributeSchema::new(Kind::ValidateOnly);
    s_foo = s_foo.add_arg(
        "inferrable",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    lib.add_attribute_schema("foo", s_foo);
    lib.compile().expect("compilation failed");
}

#[test]
fn good_single_schema_argument_respect_optionality() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo("bar")
type MyStruct = struct {};

@foo
type MyOtherStruct = struct {};
"#,
    );
    let mut s_foo = AttributeSchema::new(Kind::ValidateOnly);
    s_foo = s_foo.add_arg(
        "value",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Optional,
        ),
    );
    lib.add_attribute_schema("foo", s_foo);
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_single_schema_argument_is_named() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0125.test.fidl");
    let mut s_foo = AttributeSchema::new(Kind::ValidateOnly);
    s_foo = s_foo.add_arg(
        "value",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    lib.add_attribute_schema("foo", s_foo);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_single_schema_argument_is_not_named() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0126.test.fidl");
    let mut s_foo = AttributeSchema::new(Kind::ValidateOnly);
    s_foo = s_foo.add_arg(
        "value",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    lib.add_attribute_schema("foo", s_foo);
    assert!(lib.compile().is_err());
}

#[test]
fn good_multiple_schema_arguments_required_only() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@multiple_args(first="foo", second="bar")
type MyStruct = struct {};

// Order independent.
@multiple_args(second="bar", first="foo")
type MyOtherStruct = struct {};
"#,
    );
    let mut s_multi = AttributeSchema::new(Kind::ValidateOnly);
    s_multi = s_multi.add_arg(
        "first",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    s_multi = s_multi.add_arg(
        "second",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    lib.add_attribute_schema("multiple_args", s_multi);
    lib.compile().expect("compilation failed");
}

#[test]
fn good_multiple_schema_arguments_optional_only() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@multiple_args(first="foo", second="bar")
type MyStruct = struct {};

// Order independent.
@multiple_args(second="bar", first="foo")
type MyStruct2 = struct {};

// Only 1 argument present.
@multiple_args(first="foo")
type MyStruct3 = struct {};
@multiple_args(second="bar")
type MyStruct4 = struct {};

// No arguments at all.
@multiple_args
type MyStruct5 = struct {};
"#,
    );
    let mut s_multi = AttributeSchema::new(Kind::ValidateOnly);
    s_multi = s_multi.add_arg(
        "first",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Optional,
        ),
    );
    s_multi = s_multi.add_arg(
        "second",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Optional,
        ),
    );
    lib.add_attribute_schema("multiple_args", s_multi);
    lib.compile().expect("compilation failed");
}

#[test]
fn good_multiple_schema_arguments_required_and_optional() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@multiple_args(first="foo", second="bar")
type MyStruct = struct {};

// Order independent.
@multiple_args(second="bar", first="foo")
type MyStruct2 = struct {};

// Only 1 argument present.
@multiple_args(first="foo")
type MyStruct3 = struct {};
"#,
    );
    let mut s_multi = AttributeSchema::new(Kind::ValidateOnly);
    s_multi = s_multi.add_arg(
        "first",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    s_multi = s_multi.add_arg(
        "second",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Optional,
        ),
    );
    lib.add_attribute_schema("multiple_args", s_multi);
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_multiple_schema_arguments_required_missing() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0127.test.fidl");
    let mut s_req = AttributeSchema::new(Kind::ValidateOnly);
    s_req = s_req.add_arg(
        "required",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    s_req = s_req.add_arg(
        "optional",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Optional,
        ),
    );
    lib.add_attribute_schema("has_required_arg", s_req);
    assert!(lib.compile().is_err());
}

#[test]
fn good_literal_types_without_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@attr(foo="abc", bar=true, baz=false)
type MyStruct = struct {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_literal_numeric_types_without_schema() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0124.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_referenced_types_without_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

const foo string:3 = "abc";
const bar bool = true;
const baz bool = false;

@attr(foo=foo, bar=bar, baz=baz)
type MyStruct = struct {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_referenced_numeric_types_without_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

const foo int8 = -1;
const bar float32 = -2.3;

@attr(foo=foo, bar=bar)
type MyStruct = struct {};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn good_literal_types_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

@attr(
        string="foo",
        bool=true,
        int8=-1,
        int16=-2,
        int32=-3,
        int64=-4,
        uint8=1,
        uint16=2,
        uint32=3,
        uint64=4,
        usize64=5,
        uintptr64=6,
        uchar=7,
        float32=1.2,
        float64=-3.4)
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    for k in [
        "string",
        "bool",
        "int8",
        "int16",
        "int32",
        "int64",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "usize64",
        "uintptr64",
        "uchar",
        "float32",
        "float64",
    ] {
        let kind = match k {
            "string" => ConstantValueKind::String,
            "bool" => ConstantValueKind::Bool,
            "int8" => ConstantValueKind::Int8,
            "int16" => ConstantValueKind::Int16,
            "int32" => ConstantValueKind::Int32,
            "int64" => ConstantValueKind::Int64,
            "uint8" => ConstantValueKind::Uint8,
            "uint16" => ConstantValueKind::Uint16,
            "uint32" => ConstantValueKind::Uint32,
            "uint64" => ConstantValueKind::Uint64,
            "usize64" => ConstantValueKind::ZxUsize64,
            "uintptr64" => ConstantValueKind::ZxUintptr64,
            "uchar" => ConstantValueKind::ZxUchar,
            "float32" => ConstantValueKind::Float32,
            "float64" => ConstantValueKind::Float64,
            _ => unreachable!(),
        };
        s_attr = s_attr.add_arg(
            k,
            AttributeArgSchema::new(ArgType::Kind(kind), Optionality::Optional),
        );
    }
    lib.add_attribute_schema("attr", s_attr);
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_invalid_literal_string_type_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@attr(true)
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    for k in [
        "string",
        "bool",
        "int8",
        "int16",
        "int32",
        "int64",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "usize64",
        "uintptr64",
        "uchar",
        "float32",
        "float64",
    ] {
        let kind = match k {
            "string" => ConstantValueKind::String,
            "bool" => ConstantValueKind::Bool,
            "int8" => ConstantValueKind::Int8,
            "int16" => ConstantValueKind::Int16,
            "int32" => ConstantValueKind::Int32,
            "int64" => ConstantValueKind::Int64,
            "uint8" => ConstantValueKind::Uint8,
            "uint16" => ConstantValueKind::Uint16,
            "uint32" => ConstantValueKind::Uint32,
            "uint64" => ConstantValueKind::Uint64,
            "usize64" => ConstantValueKind::ZxUsize64,
            "uintptr64" => ConstantValueKind::ZxUintptr64,
            "uchar" => ConstantValueKind::ZxUchar,
            "float32" => ConstantValueKind::Float32,
            "float64" => ConstantValueKind::Float64,
            _ => unreachable!(),
        };
        s_attr = s_attr.add_arg(
            k,
            AttributeArgSchema::new(ArgType::Kind(kind), Optionality::Optional),
        );
    }
    lib.add_attribute_schema("attr", s_attr);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_invalid_literal_bool_type_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@attr("foo")
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    for k in [
        "string",
        "bool",
        "int8",
        "int16",
        "int32",
        "int64",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "usize64",
        "uintptr64",
        "uchar",
        "float32",
        "float64",
    ] {
        let kind = match k {
            "string" => ConstantValueKind::String,
            "bool" => ConstantValueKind::Bool,
            "int8" => ConstantValueKind::Int8,
            "int16" => ConstantValueKind::Int16,
            "int32" => ConstantValueKind::Int32,
            "int64" => ConstantValueKind::Int64,
            "uint8" => ConstantValueKind::Uint8,
            "uint16" => ConstantValueKind::Uint16,
            "uint32" => ConstantValueKind::Uint32,
            "uint64" => ConstantValueKind::Uint64,
            "usize64" => ConstantValueKind::ZxUsize64,
            "uintptr64" => ConstantValueKind::ZxUintptr64,
            "uchar" => ConstantValueKind::ZxUchar,
            "float32" => ConstantValueKind::Float32,
            "float64" => ConstantValueKind::Float64,
            _ => unreachable!(),
        };
        s_attr = s_attr.add_arg(
            k,
            AttributeArgSchema::new(ArgType::Kind(kind), Optionality::Optional),
        );
    }
    lib.add_attribute_schema("attr", s_attr);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_invalid_literal_numeric_type_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@attr(-1)
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    for k in [
        "string",
        "bool",
        "int8",
        "int16",
        "int32",
        "int64",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "usize64",
        "uintptr64",
        "uchar",
        "float32",
        "float64",
    ] {
        let kind = match k {
            "string" => ConstantValueKind::String,
            "bool" => ConstantValueKind::Bool,
            "int8" => ConstantValueKind::Int8,
            "int16" => ConstantValueKind::Int16,
            "int32" => ConstantValueKind::Int32,
            "int64" => ConstantValueKind::Int64,
            "uint8" => ConstantValueKind::Uint8,
            "uint16" => ConstantValueKind::Uint16,
            "uint32" => ConstantValueKind::Uint32,
            "uint64" => ConstantValueKind::Uint64,
            "usize64" => ConstantValueKind::ZxUsize64,
            "uintptr64" => ConstantValueKind::ZxUintptr64,
            "uchar" => ConstantValueKind::ZxUchar,
            "float32" => ConstantValueKind::Float32,
            "float64" => ConstantValueKind::Float64,
            _ => unreachable!(),
        };
        s_attr = s_attr.add_arg(
            k,
            AttributeArgSchema::new(ArgType::Kind(kind), Optionality::Optional),
        );
    }
    lib.add_attribute_schema("attr", s_attr);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_invalid_literal_with_real_schema() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0065-c.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_referenced_types_with_schema() {
    let mut lib = TestLibrary::new();
    lib.enable_flag("zx_c_types");
    lib.add_source_file(
        "example.fidl",
        r#"
library fidl.test;

const string fidl.string = "foo";
const bool fidl.bool = true;
const int8 fidl.int8 = -1;
const int16 fidl.int16 = -2;
const int32 fidl.int32 = -3;
type int64 = enum : fidl.int64 {
    MEMBER = -4;
};
const uint8 fidl.uint8 = 1;
const uint16 fidl.uint16 = 2;
const uint32 fidl.uint32 = 3;
type uint64 = bits : fidl.uint64 {
    MEMBER = 4;
};
const usize64 fidl.usize64 = 5;
const uintptr64 fidl.uintptr64 = 6;
const uchar fidl.uchar = 7;
const float32 fidl.float32 = 1.2;
const float64 fidl.float64 = -3.4;

@attr(
        string=string,
        bool=bool,
        int8=int8,
        int16=int16,
        int32=int32,
        int64=int64.MEMBER,
        uint8=uint8,
        uint16=uint16,
        uint32=uint32,
        uint64=uint64.MEMBER,
        usize64=usize64,
        uintptr64=uintptr64,
        uchar=uchar,
        float32=float32,
        float64=float64)
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    for k in [
        "string",
        "bool",
        "int8",
        "int16",
        "int32",
        "int64",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "usize64",
        "uintptr64",
        "uchar",
        "float32",
        "float64",
    ] {
        let kind = match k {
            "string" => ConstantValueKind::String,
            "bool" => ConstantValueKind::Bool,
            "int8" => ConstantValueKind::Int8,
            "int16" => ConstantValueKind::Int16,
            "int32" => ConstantValueKind::Int32,
            "int64" => ConstantValueKind::Int64,
            "uint8" => ConstantValueKind::Uint8,
            "uint16" => ConstantValueKind::Uint16,
            "uint32" => ConstantValueKind::Uint32,
            "uint64" => ConstantValueKind::Uint64,
            "usize64" => ConstantValueKind::ZxUsize64,
            "uintptr64" => ConstantValueKind::ZxUintptr64,
            "uchar" => ConstantValueKind::ZxUchar,
            "float32" => ConstantValueKind::Float32,
            "float64" => ConstantValueKind::Float64,
            _ => unreachable!(),
        };
        s_attr = s_attr.add_arg(
            k,
            AttributeArgSchema::new(ArgType::Kind(kind), Optionality::Optional),
        );
    }
    lib.add_attribute_schema("attr", s_attr);
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_invalid_referenced_string_type_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

const foo bool = true;

@attr(foo)
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    for k in [
        "string",
        "bool",
        "int8",
        "int16",
        "int32",
        "int64",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "usize64",
        "uintptr64",
        "uchar",
        "float32",
        "float64",
    ] {
        let kind = match k {
            "string" => ConstantValueKind::String,
            "bool" => ConstantValueKind::Bool,
            "int8" => ConstantValueKind::Int8,
            "int16" => ConstantValueKind::Int16,
            "int32" => ConstantValueKind::Int32,
            "int64" => ConstantValueKind::Int64,
            "uint8" => ConstantValueKind::Uint8,
            "uint16" => ConstantValueKind::Uint16,
            "uint32" => ConstantValueKind::Uint32,
            "uint64" => ConstantValueKind::Uint64,
            "usize64" => ConstantValueKind::ZxUsize64,
            "uintptr64" => ConstantValueKind::ZxUintptr64,
            "uchar" => ConstantValueKind::ZxUchar,
            "float32" => ConstantValueKind::Float32,
            "float64" => ConstantValueKind::Float64,
            _ => unreachable!(),
        };
        s_attr = s_attr.add_arg(
            k,
            AttributeArgSchema::new(ArgType::Kind(kind), Optionality::Optional),
        );
    }
    lib.add_attribute_schema("attr", s_attr);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_invalid_referenced_bool_type_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

const foo string:3 = "foo";

@attr(foo)
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    for k in [
        "string",
        "bool",
        "int8",
        "int16",
        "int32",
        "int64",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "usize64",
        "uintptr64",
        "uchar",
        "float32",
        "float64",
    ] {
        let kind = match k {
            "string" => ConstantValueKind::String,
            "bool" => ConstantValueKind::Bool,
            "int8" => ConstantValueKind::Int8,
            "int16" => ConstantValueKind::Int16,
            "int32" => ConstantValueKind::Int32,
            "int64" => ConstantValueKind::Int64,
            "uint8" => ConstantValueKind::Uint8,
            "uint16" => ConstantValueKind::Uint16,
            "uint32" => ConstantValueKind::Uint32,
            "uint64" => ConstantValueKind::Uint64,
            "usize64" => ConstantValueKind::ZxUsize64,
            "uintptr64" => ConstantValueKind::ZxUintptr64,
            "uchar" => ConstantValueKind::ZxUchar,
            "float32" => ConstantValueKind::Float32,
            "float64" => ConstantValueKind::Float64,
            _ => unreachable!(),
        };
        s_attr = s_attr.add_arg(
            k,
            AttributeArgSchema::new(ArgType::Kind(kind), Optionality::Optional),
        );
    }
    lib.add_attribute_schema("attr", s_attr);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_invalid_referenced_numeric_type_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

const foo uint16 = 259;

@attr(foo)
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    for k in [
        "string",
        "bool",
        "int8",
        "int16",
        "int32",
        "int64",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "usize64",
        "uintptr64",
        "uchar",
        "float32",
        "float64",
    ] {
        let kind = match k {
            "string" => ConstantValueKind::String,
            "bool" => ConstantValueKind::Bool,
            "int8" => ConstantValueKind::Int8,
            "int16" => ConstantValueKind::Int16,
            "int32" => ConstantValueKind::Int32,
            "int64" => ConstantValueKind::Int64,
            "uint8" => ConstantValueKind::Uint8,
            "uint16" => ConstantValueKind::Uint16,
            "uint32" => ConstantValueKind::Uint32,
            "uint64" => ConstantValueKind::Uint64,
            "usize64" => ConstantValueKind::ZxUsize64,
            "uintptr64" => ConstantValueKind::ZxUintptr64,
            "uchar" => ConstantValueKind::ZxUchar,
            "float32" => ConstantValueKind::Float32,
            "float64" => ConstantValueKind::Float64,
            _ => unreachable!(),
        };
        s_attr = s_attr.add_arg(
            k,
            AttributeArgSchema::new(ArgType::Kind(kind), Optionality::Optional),
        );
    }
    lib.add_attribute_schema("attr", s_attr);
    assert!(lib.compile().is_err());
}

#[test]
fn good_compile_early_attribute_literal_argument() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@attr(1)
type MyStruct = struct {};
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    s_attr = s_attr.add_anonymous_arg(AttributeArgSchema::new(
        ArgType::Kind(ConstantValueKind::Uint8),
        Optionality::Required,
    ));
    s_attr = s_attr.compile_early();
    lib.add_attribute_schema("attr", s_attr);
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_compile_early_attribute_referenced_argument() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@attr(BAD)
type MyStruct = struct {};

const BAD uint8 = 1;
"#,
    );
    let mut s_attr = AttributeSchema::new(Kind::ValidateOnly);
    s_attr = s_attr.add_anonymous_arg(AttributeArgSchema::new(
        ArgType::Kind(ConstantValueKind::Uint8),
        Optionality::Required,
    ));
    s_attr = s_attr.compile_early();
    lib.add_attribute_schema("attr", s_attr);
    assert!(lib.compile().is_err());
}

#[test]
fn good_anonymous_argument_gets_named_value() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@attr("abc")
type MyStruct = struct {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_single_named_argument_keeps_name() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@attr(foo="abc")
type MyStruct = struct {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_references_nonexistent_const_without_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(nonexistent)
type MyStruct = struct {};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_references_nonexistent_const_with_single_arg_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(nonexistent)
type MyStruct = struct {};
"#,
    );
    let mut s_foo = AttributeSchema::new(Kind::ValidateOnly);
    s_foo = s_foo.add_arg(
        "value",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    lib.add_attribute_schema("foo", s_foo);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_references_nonexistent_const_with_multiple_arg_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(nonexistent)
type MyStruct = struct {};
"#,
    );
    let mut s_foo = AttributeSchema::new(Kind::ValidateOnly);
    s_foo = s_foo.add_arg(
        "value",
        AttributeArgSchema::new(
            ArgType::Kind(ConstantValueKind::String),
            Optionality::Required,
        ),
    );
    lib.add_attribute_schema("foo", s_foo);
    assert!(lib.compile().is_err());
}

#[test]
fn bad_references_invalid_const_without_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(BAD)
type MyStruct = struct {};

const BAD bool = "not a bool";
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_references_invalid_const_with_single_arg_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(BAD)
type MyStruct = struct {};

const BAD bool = "not a bool";
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_references_invalid_const_with_multiple_arg_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(BAD)
type MyStruct = struct {};

const BAD bool = "not a bool";
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_self_reference_without_schema_bool() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(BAR)
const BAR bool = true;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_self_reference_without_schema_string() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(BAR)
const BAR string = "bar";
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_self_reference_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(BAR)
const BAR bool = true;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_mutual_reference_without_schema_bool() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(SECOND)
const FIRST bool = true;
@foo(FIRST)
const SECOND bool = false;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_mutual_reference_without_schema_string() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(SECOND)
const FIRST string = "first";
@foo(FIRST)
const SECOND string = "second";
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_mutual_reference_with_schema() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@foo(SECOND)
const FIRST bool = true;
@foo(FIRST)
const SECOND bool = false;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_library_references_nonexistent_const() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
@foo(nonexistent)
library example;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_library_references_const() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
@foo(BAR)
library example;

const BAR bool = true;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_library_references_external_const() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "dependency.fidl",
        r#"
library dependency;
const BAR bool = true;
"#,
    );
    lib.add_source_file(
        "example.fidl",
        r#"
@foo(dependency.BAR)
library example;
using dependency;
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn good_discoverable_implicit_name() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@discoverable
protocol Foo {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_discoverable_explicit_name() {
    for name in ["example.Foo", "notexample.NotFoo", "not.example.NotFoo"] {
        let source_text = r#"
library example;

@discoverable(name="%1")
protocol Foo {};
"#
        .replace("%1", name);

        let mut lib = TestLibrary::new();
        lib.add_source_file("example.fidl", &(source_text));
        lib.compile()
            .expect(&format!("compilation failed for {}", name));
    }
}

#[test]
fn bad_discoverable_invalid_name() {
    for name in ["", "example/Foo?", "Foo", "not example.Not Foo"] {
        let source_text = r#"
library example;

@discoverable(name="%1")
protocol Foo {};
"#
        .replace("%1", name);

        let mut lib = TestLibrary::new();
        lib.add_source_file("example.fidl", &(source_text));
        assert!(
            lib.compile().is_err(),
            "expected compilation to fail for {}",
            name
        );
    }
}

#[test]
fn bad_discoverable_invalid_name_errcat() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0135.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_discoverable_location_errcat() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0210.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_result_attribute() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@result
type Foo = union {
    1: s string;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_discoverable_location() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@discoverable(client="")
protocol P{};

@discoverable(client="platform", server="external")
protocol Q{};

@discoverable(client="platform,external", server="external,platform")
protocol R{};

@discoverable(client="platform, external", server="external, platform")
protocol S{};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_no_resource() {
    let mut lib = TestLibrary::new();
    lib.enable_flag("no_resource_attribute");
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@no_resource
protocol P{
  compose Q;
  Method(struct { i int32; }) -> ();
};

@no_resource
protocol Q{};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_no_resource_uses_resource() {
    let mut lib = TestLibrary::new();
    lib.enable_flag("no_resource_attribute");
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@no_resource
protocol P{
  compose Q;
  Method(resource struct { i int32; }) -> ();
};

@no_resource
protocol Q{};
"#,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_no_resource_composition() {
    let mut lib = TestLibrary::new();
    lib.enable_flag("no_resource_attribute");
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@no_resource
protocol P{
  compose Q;
  Method( struct { i int32; }) -> ();
};

protocol Q{};
"#,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}

#[test]
fn bad_no_resource_is_experimental() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

@no_resource
protocol P{
  compose Q;
  Method(struct { i int32; }) -> ();
};

@no_resource
protocol Q{};
"#,
    );
    assert!(lib.compile().is_err(), "expected compilation to fail");
}
