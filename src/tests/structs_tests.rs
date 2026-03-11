use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_simple_struct() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0001.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]
fn good_primitive_default_value_literal() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = 20;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let struct_decl = root
        .lookup_struct("example/MyStruct")
        .expect("struct not found");
    assert_eq!(struct_decl.members.len(), 1);
}

#[test]
fn bad_primitive_default_value_no_annotation() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0050.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_primitive_default_value_const_reference() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

const A int32 = 20;

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = A;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_missing_default_value_reference_target() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = A;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn good_enum_default_value_enum_member_reference() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyEnum = strict enum : int32 {
    A = 5;
};

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyEnum = MyEnum.A;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_primitive_default_value_enum_member_reference() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyEnum = strict enum : int32 {
    A = 5;
};

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = MyEnum.A;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_default_value_enum_type() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyEnum = enum : int32 { A = 1; };
type OtherEnum = enum : int32 { A = 1; };

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyEnum = OtherEnum.A;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_default_value_primitive_in_enum() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0103.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_enum_default_value_bits_member_reference() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyBits = strict bits : uint32 {
    A = 0x00000001;
};

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyBits = MyBits.A;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_primitive_default_value_bits_member_reference() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyBits = strict bits : uint32 {
    A = 0x00000001;
};

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = MyBits.A;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_default_value_bits_type() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyBits = bits : uint32 { A = 0x00000001; };
type OtherBits = bits : uint32 { A = 0x00000001; };

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyBits = OtherBits.A;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_default_value_primitive_in_bits() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyBits = enum : int32 { A = 0x00000001; };

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyBits = 1;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_legacy_enum_member_reference() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyEnum = enum : int32 { A = 5; };

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyEnum = A;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_default_value_nullable_string() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0091.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_duplicate_member_name() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = struct {
    my_struct_member string;
    my_struct_member uint8;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn good_max_inline_size() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MyStruct = struct {
    arr array<uint8, 65535>;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_inline_size_exceeds_64k() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0111.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_mutually_recursive() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0057-a.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_self_recursive() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0057-c.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn good_recursive_box() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0057.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_recursive_through_vector() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MySelf = struct {
    me vector<MySelf>;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn good_recursive_optional_vector() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type MySelf = struct {
    me vector<MySelf>:optional;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn bad_mutually_recursive_with_incoming_leaf() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Yin = struct {
  yang Yang;
};

type Yang = struct {
  yin Yin;
};

type Leaf = struct {
  yin Yin;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_mutually_recursive_with_outoging_leaf() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Yin = struct {
  yang Yang;
};

type Yang = struct {
  yin Yin;
  leaf Leaf;
};

type Leaf = struct {
  x int32;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_mutually_recursive_intersecting_loops() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Yin = struct {
  intersection Intersection;
};

type Yang = struct {
  intersection Intersection;
};

type Intersection = struct {
  yin Yin;
  yang Yang;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_box_cannot_be_optional() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0169.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_struct_cannot_be_optional() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0159.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_handle_cannot_be_boxed_should_be_optional() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "bad/fi-0171.test.fidl",
        r#"
library test.bad.fi0171;

using zx;

type Foo = resource struct {
    handle_member box<zx.Handle>;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_cannot_box_primitive() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0193.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_default_value_references_invalid_const() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type Foo = struct {
    @allow_deprecated_struct_defaults
    flag bool = BAR;
};

const BAR bool = "not a bool";
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn cannot_refer_to_int_member() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0053-a.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn cannot_refer_to_struct_member() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0053-b.test.fidl");
    assert!(lib.compile().is_err());
}

// Type param tests omitted as they require dynamic evaluation to fully loop through array of names

#[test]
fn bad_type_cannot_be_boxed_should_be_optional() {
    let boxed_names = vec!["Endpoint", "server_end:Endpoint", "client_end:Endpoint"];
    for boxed_name in boxed_names {
        let mut lib = TestLibrary::new();
        lib.add_source_file(
            "example.fidl",
            &(format!(
                r#"
library example;

protocol Endpoint {{}};

type MyStruct = struct {{
    foo box<{}>;
}};
"#,
                boxed_name
            )),
        );
        assert!(lib.compile().is_err());
    }
}

#[test]
fn bad_type_cannot_be_boxed_nor_optional() {
    let boxed_names = vec!["int32", "uint32", "bool"];
    for boxed_name in boxed_names {
        let mut lib = TestLibrary::new();
        lib.add_source_file(
            "example.fidl",
            &(format!(
                r#"
library example;

type MyStruct = struct {{
    foo box<{}>;
}};
"#,
                boxed_name
            )),
        );
        assert!(lib.compile().is_err());
    }
}
