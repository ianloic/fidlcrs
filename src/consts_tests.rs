#[cfg(test)]
mod tests {
    use crate::test_library::TestLibrary;
    use crate::source_file::SourceFile;
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    fn good_literals_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const C_SIMPLE uint32 = 11259375;
const C_HEX_S uint32 = 0xABCDEF;
const C_HEX_L uint32 = 0XABCDEF;
const C_BINARY_S uint32 = 0b101010111100110111101111;
const C_BINARY_L uint32 = 0B101010111100110111101111;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_const_test_bool() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c bool = false;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_const_test_bool_with_string() {
        let file_content = get_file_content("bad/fi-0065-a.test.fidl");
        let source = SourceFile::new("bad/fi-0065-a.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_bool_with_numeric() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c bool = 6;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_const_test_int32() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c int32 = 42;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_const_test_int32_from_other_const() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const b int32 = 42;
const c int32 = b;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_const_test_int32_with_string() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c int32 = "foo";
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_int32_with_bool() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c int32 = true;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_const_test_int64() {
        let file_content = get_file_content("good/fi-0066-b.test.fidl");
        let source = SourceFile::new("good/fi-0066-b.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_const_test_uint64() {
        let file_content = get_file_content("good/fi-0066-a.test.fidl");
        let source = SourceFile::new("good/fi-0066-a.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_const_test_uint64_from_other_uint32() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const a uint32 = 42;
const b uint64 = a;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_const_test_uint64_negative() {
        let file_content = get_file_content("bad/fi-0066.test.fidl");
        let source = SourceFile::new("bad/fi-0066.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_uint64_overflow() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const a uint64 = 18446744073709551616;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_const_test_float32() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const b float32 = 1.61803;
const c float32 = -36.46216;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_const_test_float32_high_limit() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const hi float32 = 3.402823e38;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_const_test_float32_low_limit() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const lo float32 = -3.40282e38;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_const_test_float32_high_limit() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const hi float32 = 3.41e38;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_float32_low_limit() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const b float32 = -3.41e38;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_const_test_string() {
        let file_content = get_file_content("good/fi-0002.test.fidl");
        let source = SourceFile::new("good/fi-0002.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_const_test_string_from_other_const() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c string:4 = "four";
const d string:5 = c;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_const_test_string_with_numeric() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c string = 4;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_string_with_bool() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c string = true;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_string_with_string_too_long() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c string:4 = "hello";
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_const_test_using() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

alias foo = int32;
const c foo = 2;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_const_test_using_with_inconvertible_value() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

alias foo = int32;
const c foo = "nope";
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_nullable_string() {
        let file_content = get_file_content("bad/fi-0059.test.fidl");
        let source = SourceFile::new("bad/fi-0059.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_array() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c array<int32,2> = -1;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_vector() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const c vector<int32>:2 = -1;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_test_handle_of_thread() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type ObjType = enum : uint32 {
    NONE = 0;
    THREAD = 2;
};

resource_definition handle : uint32 {
    properties {
        subtype ObjType;
    };
};

const c handle:THREAD = -1;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_const_enum_member_reference() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyEnum = strict enum : int32 {
    A = 5;
};
const c int32 = MyEnum.A;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_const_bits_member_reference() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyBits = strict bits : uint32 {
    A = 0x00000001;
};
const c uint32 = MyBits.A;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_enum_typed_const_enum_member_reference() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyEnum = strict enum : int32 {
    A = 5;
};
const c MyEnum = MyEnum.A;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_bits_typed_const_bits_member_reference() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyBits = strict bits : uint32 {
    A = 0x00000001;
};
const c MyBits = MyBits.A;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_bits_typed_const_zero() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyBits = strict bits : uint32 {
    A = 0x00000001;
};
const c MyBits = 0;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_const_different_enum_member_reference() {
        let file_content = get_file_content("bad/fi-0064.test.fidl");
        let source = SourceFile::new("bad/fi-0064.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_different_bits_member_reference() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyBits = bits : uint32 { VALUE = 0x00000001; };
type OtherBits = bits : uint32 { VALUE = 0x00000004; };
const c MyBits = OtherBits.VALUE;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_assign_primitive_to_enum() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyEnum = enum : int32 { VALUE = 1; };
const c MyEnum = 5;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_assign_primitive_to_bits() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyBits = bits : uint32 { VALUE = 0x00000001; };
const c MyBits = 5;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_max_bound_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const S string:MAX = "";

type Example = struct {
    s string:MAX;
    v vector<bool>:MAX;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_max_bound_test_convert_to_unbounded() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const A string:MAX = "foo";
const B string = A;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_max_bound_test_convert_from_unbounded() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const A string = "foo";
const B string:MAX = A;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_max_bound_test_assign_to_const() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const FOO uint32 = MAX;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_max_bound_test_library_qualified() {
        let dependency = SourceFile::new(
            "dependency.fidl".to_string(),
            r#"library dependency;

type Example = struct {};
"#
            .to_string(),
        );
        let main = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

using dependency;

type Example = struct { s string:dependency.MAX; };
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&dependency); // Or however multi-file or multi-lib is supported (might fail)
        lib.add_source(&main);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_parameterize_primitive() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const u uint8<string> = 0;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_name_collision() {
        let file_content = get_file_content("bad/fi-0034.test.fidl");
        let source = SourceFile::new("bad/fi-0034.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_fix_name_collision_rename() {
        let file_content = get_file_content("good/fi-0034-b.test.fidl");
        let source = SourceFile::new("good/fi-0034-b.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_fix_name_collision_remove() {
        let file_content = get_file_content("good/fi-0034-a.test.fidl");
        let source = SourceFile::new("good/fi-0034-a.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_multi_file_const_reference() {
        let s1 = SourceFile::new(
            "first.fidl".to_string(),
            r#"library example;

type Protein = struct {
    amino_acids vector<uint64>:SMALL_SIZE;
};
"#
            .to_string(),
        );
        let s2 = SourceFile::new(
            "second.fidl".to_string(),
            r#"library example;

const SMALL_SIZE uint32 = 4;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&s1);
        lib.add_source(&s2);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_unknown_enum_member_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type EnumType = enum : int32 {
    A = 0x00000001;
    B = 0x80;
    C = 0x2;
};

const dee EnumType = EnumType.D;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_unknown_bits_member_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type BitsType = bits {
    A = 2;
    B = 4;
    C = 8;
};

const dee BitsType = BitsType.D;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_or_operator_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyBits = strict bits : uint8 {
    A = 0x00000001;
    B = 0x00000002;
    C = 0x00000004;
    D = 0x00000008;
};
const bitsValue MyBits = MyBits.A | MyBits.B | MyBits.D;
const Result uint16 = MyBits.A | MyBits.B | MyBits.D;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_or_operator_different_types_test() {
        let file_content = get_file_content("bad/fi-0065-b.test.fidl");
        let source = SourceFile::new("bad/fi-0065-b.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_or_operator_different_types_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

const one uint8 = 0x0001;
const two_fifty_six uint16 = 0x0100;
const two_fifty_seven uint16 = one | two_fifty_six;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_or_operator_non_primitive_types_test() {
        let file_content = get_file_content("bad/fi-0061.test.fidl");
        let source = SourceFile::new("bad/fi-0061.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_or_operator_parentheses_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyBits = strict bits : uint8 {
    A = 0x00000001;
    B = 0x00000002;
    C = 0x00000004;
    D = 0x00000008;
};
const three MyBits = MyBits.A | MyBits.B;
const seven MyBits = three | MyBits.C;
const fifteen MyBits = (three | seven) | MyBits.D;
const bitsValue MyBits = MyBits.A | ( ( (MyBits.A | MyBits.B) | MyBits.D) | MyBits.C);
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_identifier_const_mismatched_types_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type OneEnum = enum {
    A = 1;
};
type AnotherEnum = enum {
    B = 1;
};
const a OneEnum = OneEnum.A;
const b AnotherEnum = a;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_enum_bits_const_mismatched_types_test() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type OneEnum = enum {
    A = 1;
};
type AnotherEnum = enum {
    B = 1;
};
const a OneEnum = AnotherEnum.B;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_const_references_invalid_const() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const A string = Z;
const Z string = 1;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_declaration() {
        let file_content = get_file_content("good/fi-0006.test.fidl");
        let source = SourceFile::new("good/fi-0006.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_integer_convert_wider() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X uint16 = 23;
const WIDE uint32 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_integer_convert_narrower() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X uint16 = 255;
const NARROW uint8 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_integer_convert_to_signed() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X uint16 = 23;
const SIGNED int16 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_integer_convert_to_unsigned() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X int16 = 23;
const UNSIGNED uint16 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_integer_convert_narrower_out_of_range() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X uint16 = 256;
const NARROW uint8 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_integer_convert_to_signed_out_of_range() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X uint16 = 32768; // 2^15
const SIGNED int16 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_integer_convert_to_unsigned_negative() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X int16 = -1;
const UNSIGNED uint16 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_convert_float_wider() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X float32 = 23;
const WIDE float64 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_convert_float_narrower() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X float64 = 3.4028234663852886e38; // max float32
const NARROW float32 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_convert_float_narrower_out_of_range_positive() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X float64 = 3.402823466385289e38; // just above max float32
const NARROW float32 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_convert_float_narrower_out_of_range_negative() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X float64 = -3.402823466385289e38; // just below min float32
const NARROW float32 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_convert_integer_to_float() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X uint16 = 1;
const FLOAT float32 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_convert_float_to_integer() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;
const X float32 = 1;
const INTEGER uint16 = X;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }
}
