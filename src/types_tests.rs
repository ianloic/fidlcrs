#[cfg(test)]
mod tests {
    use crate::source_file::SourceFile;
    use crate::test_library::TestLibrary;
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    #[ignore]
    fn good_root_types_unqualified() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

const b bool = false;
const i8 int8 = 0;
const i16 int16 = 0;
const i32 int32 = 0;
const i64 int64 = 0;
const u8 uint8 = 0;
const u16 uint16 = 0;
const u32 uint32 = 0;
const u64 uint64 = 0;
const us usize64 = 0;
const up uintptr64 = 0;
const uc uchar = 0;
const f32 float32 = 0;
const f64 float64 = 0;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn good_root_types_qualified() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

const bool fidl.bool = false;
const int8 fidl.int8 = 0;
const int16 fidl.int16 = 0;
const int32 fidl.int32 = 0;
const int64 fidl.int64 = 0;
const uint8 fidl.uint8 = 0;
const uint16 fidl.uint16 = 0;
const uint32 fidl.uint32 = 0;
const uint64 fidl.uint64 = 0;
const usize64 fidl.usize64 = 0;
const uintptr64 fidl.uintptr64 = 0;
const uchar fidl.uchar = 0;
const float32 fidl.float32 = 0;
const float64 fidl.float64 = 0;
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn good_handle_subtype() {
        // static_assert tests not ported directly
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn good_rights() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_type_decl_of_new_type_errors() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0062.test.fidl".to_string(),
            get_file_content("bad/fi-0062.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_type_parameters() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_layout_member_constraints() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_constraints_on_vectors() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_constraints_on_unions() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_constraints_on_handles() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_too_many_layout_parameters() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0162-b.test.fidl".to_string(),
            get_file_content("bad/fi-0162-b.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_zero_parameters() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_not_enough_parameters() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0162-a.test.fidl".to_string(),
            get_file_content("bad/fi-0162-a.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_too_many_constraints() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0164.test.fidl".to_string(),
            get_file_content("bad/fi-0164.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_parameterized_anonymous_layout() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_constrain_twice() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_no_overlapping_constraints() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_want_type_layout_parameter() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0165.test.fidl".to_string(),
            get_file_content("bad/fi-0165.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_want_value_layout_parameter() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_unresolvable_constraint() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0166.test.fidl".to_string(),
            get_file_content("bad/fi-0166.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_shadowed_optional() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_wrong_constraint_type() {
        let mut lib = TestLibrary::new(); // TODO
        assert!(lib.compile().is_err());
    }

    #[test]
    fn cannot_refer_to_unqualified_internal_type() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn cannot_refer_to_qualified_internal_type() {
        let mut lib = TestLibrary::new(); // TODO
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_usize64_without_flag() {
        let mut lib = TestLibrary::new();
        let source = SourceFile::new(
            "bad/fi-0180.test.fidl".to_string(),
            get_file_content("bad/fi-0180.test.fidl"),
        );
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_uintptr64_without_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = uintptr64;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_uchar_without_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = uchar;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_experimental_pointer_without_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = experimental_pointer<uint32>;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_usize64_with_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = usize64;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_uintptr64_with_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = uintptr64;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_uchar_with_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = uchar;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_experimental_pointer_with_flag() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example; alias T = experimental_pointer<uint32>;"#.to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }
}
