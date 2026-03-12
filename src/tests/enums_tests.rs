use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_enum_test_simple() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Fruit = enum : uint64 {
    ORANGE = 1;
    APPLE = 2;
    BANANA = 3;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let type_decl = root
        .lookup_enum("example/Fruit")
        .expect("Fruit enum not found");
    assert_eq!(type_decl.members.len(), 3);
    assert_eq!(type_decl.type_, "uint64");
}

#[test]
fn good_enum_default_uint32() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Fruit = enum {
    ORANGE = 1;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");
    let type_decl = root
        .lookup_enum("example/Fruit")
        .expect("Fruit enum not found");
    assert_eq!(type_decl.type_, "uint32");
}

#[test]

fn bad_enum_test_with_non_unique_values() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0107.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrDuplicateMemberValue,
        &[r#""enum""#, r#""APPLE""#, r#""ORANGE""#, r#""ORANGE""#],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_enum_test_with_non_unique_values_out_of_line() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Fruit = enum {
    ORANGE = FOUR;
    APPLE = TWO_SQUARED;
};

const FOUR uint32 = 4;
const TWO_SQUARED uint32 = 4;
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrDuplicateMemberValue,
        &[r#""enum""#, r#""APPLE""#, r#""ORANGE""#, r#""ORANGE""#],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_enum_test_unsigned_with_negative_member() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Fruit = enum : uint64 {
    ORANGE = 1;
    APPLE = -2;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrConstantOverflowsType,
        &[r#""-2""#, r#""uint64""#],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_enum_test_inferred_unsigned_with_negative_member() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Fruit = enum {
    ORANGE = 1;
    APPLE = -2;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrConstantOverflowsType,
        &[r#""-2""#, r#""uint32""#],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_enum_test_member_overflow() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Fruit = enum : uint8 {
    ORANGE = 1;
    APPLE = 256;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrConstantOverflowsType,
        &[r#""256""#, r#""uint8""#],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_enum_test_float_type() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0070.test.fidl");
    lib.expect_fail(
        crate::diagnostics::Error::ErrEnumTypeMustBeIntegralPrimitive,
        &[r#""float64""#],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_enum_test_duplicate_member() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Fruit = flexible enum {
    ORANGE = 1;
    APPLE = 2;
    ORANGE = 3;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrNameCollision,
        &[
            r#""member""#,
            r#""ORANGE""#,
            r#""member""#,
            r#""example.fidl:4:5""#,
        ],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_enum_test_no_members_allowed_when_defaults_to_flexible() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type E = enum {};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]
fn good_enum_test_no_members_allowed_when_flexible() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0019-a.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]
fn good_enum_test_strict_with_members() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0019-b.test.fidl");
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_enum_test_no_members_when_strict() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0019.test.fidl");
    lib.expect_fail(crate::diagnostics::Error::ErrMustHaveOneMember, &[]);
    assert!(lib.check_compile());
}

#[test]
fn good_enum_test_keyword_names() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type Fruit = enum : uint64 {
    library = 1;
    enum = 2;
    uint64 = 3;
};
"#,
    );
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_enum_shant_be_nullable() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type NotNullable = enum {
    MEMBER = 1;
};

type Struct = struct {
    not_nullable NotNullable:optional;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrCannotBeOptional,
        &[r#""NotNullable""#],
    );
    assert!(lib.check_compile());
}

#[test]

fn bad_enum_multiple_constraints() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"library example;

type NotNullable = enum {
    MEMBER = 1;
};

type Struct = struct {
    not_nullable NotNullable:<1, 2, 3>;
};
"#,
    );
    lib.expect_fail(
        crate::diagnostics::Error::ErrTooManyConstraints,
        &[r#""NotNullable""#, "0", "3"],
    );
    assert!(lib.check_compile());
}

#[test]
fn good_simple_enum() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("good/fi-0008.test.fidl");
    lib.compile().expect("compilation failed");
}
