#[cfg(test)]
mod tests {
    use crate::test_library::TestLibrary;
    use crate::source_file::SourceFile;
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    fn good_simple_struct() {
        let source = SourceFile::new("good/fi-0001.test.fidl".to_string(), get_file_content("good/fi-0001.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_primitive_default_value_literal() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = 20;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        // TODO: assert type_decl members length = 1
    }

    #[test]
    #[ignore]
    fn bad_primitive_default_value_no_annotation() {
        let source = SourceFile::new("bad/fi-0050.test.fidl".to_string(), get_file_content("bad/fi-0050.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_primitive_default_value_const_reference() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

const A int32 = 20;

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_missing_default_value_reference_target() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_enum_default_value_enum_member_reference() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyEnum = strict enum : int32 {
    A = 5;
};

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyEnum = MyEnum.A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_primitive_default_value_enum_member_reference() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyEnum = strict enum : int32 {
    A = 5;
};

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = MyEnum.A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_default_value_enum_type() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyEnum = enum : int32 { A = 1; };
type OtherEnum = enum : int32 { A = 1; };

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyEnum = OtherEnum.A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_default_value_primitive_in_enum() {
        let source = SourceFile::new("bad/fi-0103.test.fidl".to_string(), get_file_content("bad/fi-0103.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_enum_default_value_bits_member_reference() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyBits = strict bits : uint32 {
    A = 0x00000001;
};

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyBits = MyBits.A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_primitive_default_value_bits_member_reference() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyBits = strict bits : uint32 {
    A = 0x00000001;
};

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field int64 = MyBits.A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_default_value_bits_type() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyBits = bits : uint32 { A = 0x00000001; };
type OtherBits = bits : uint32 { A = 0x00000001; };

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyBits = OtherBits.A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_default_value_primitive_in_bits() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyBits = enum : int32 { A = 0x00000001; };

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyBits = 1;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_legacy_enum_member_reference() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyEnum = enum : int32 { A = 5; };

type MyStruct = struct {
    @allow_deprecated_struct_defaults
    field MyEnum = A;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_default_value_nullable_string() {
        let source = SourceFile::new("bad/fi-0091.test.fidl".to_string(), get_file_content("bad/fi-0091.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_duplicate_member_name() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyStruct = struct {
    my_struct_member string;
    my_struct_member uint8;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_max_inline_size() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MyStruct = struct {
    arr array<uint8, 65535>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_inline_size_exceeds_64k() {
        let source = SourceFile::new("bad/fi-0111.test.fidl".to_string(), get_file_content("bad/fi-0111.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_mutually_recursive() {
        let source = SourceFile::new("bad/fi-0057-a.test.fidl".to_string(), get_file_content("bad/fi-0057-a.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_self_recursive() {
        let source = SourceFile::new("bad/fi-0057-c.test.fidl".to_string(), get_file_content("bad/fi-0057-c.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_recursive_box() {
        let source = SourceFile::new("good/fi-0057.test.fidl".to_string(), get_file_content("good/fi-0057.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_recursive_through_vector() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MySelf = struct {
    me vector<MySelf>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_recursive_optional_vector() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type MySelf = struct {
    me vector<MySelf>:optional;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_mutually_recursive_with_incoming_leaf() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
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
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_mutually_recursive_with_outoging_leaf() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
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
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_mutually_recursive_intersecting_loops() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
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
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_box_cannot_be_optional() {
        let source = SourceFile::new("bad/fi-0169.test.fidl".to_string(), get_file_content("bad/fi-0169.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_struct_cannot_be_optional() {
        let source = SourceFile::new("bad/fi-0159.test.fidl".to_string(), get_file_content("bad/fi-0159.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_handle_cannot_be_boxed_should_be_optional() {
        let source = SourceFile::new("bad/fi-0171.test.fidl".to_string(), r#"
library test.bad.fi0171;

using zx;

type Foo = resource struct {
    handle_member box<zx.Handle>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_cannot_box_primitive() {
        let source = SourceFile::new("bad/fi-0193.test.fidl".to_string(), get_file_content("bad/fi-0193.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_default_value_references_invalid_const() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Foo = struct {
    @allow_deprecated_struct_defaults
    flag bool = BAR;
};

const BAR bool = "not a bool";
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn cannot_refer_to_int_member() {
        let source = SourceFile::new("bad/fi-0053-a.test.fidl".to_string(), get_file_content("bad/fi-0053-a.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn cannot_refer_to_struct_member() {
        let source = SourceFile::new("bad/fi-0053-b.test.fidl".to_string(), get_file_content("bad/fi-0053-b.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    // Type param tests omitted as they require dynamic evaluation to fully loop through array of names
}
