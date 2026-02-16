#[cfg(test)]
mod tests {
    use crate::test_library::{TestLibrary, LookupHelpers};
    use crate::source_file::SourceFile;

    #[test]
    fn good_empty_struct() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Empty = struct {};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let empty = root.lookup_struct("example/Empty").expect("Empty not found");
        assert_eq!(empty.type_shape_v2.inline_size, 1, "inline_size mismatch for empty");
        assert_eq!(empty.type_shape_v2.alignment, 1, "alignment mismatch for empty");
        assert_eq!(empty.type_shape_v2.depth, 0, "depth mismatch for empty");
        assert_eq!(empty.type_shape_v2.max_handles, 0, "max_handles mismatch for empty");
        assert_eq!(empty.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for empty");
        assert_eq!(empty.type_shape_v2.has_padding, false, "has_padding mismatch for empty");
        assert_eq!(empty.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for empty");
    }

    #[test]
    fn good_empty_struct_within_another_struct() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Empty = struct {};

// Size = 1 byte for |bool a|
//      + 1 byte for |Empty b|
//      + 2 bytes for |int16 c|
//      + 1 bytes for |Empty d|
//      + 3 bytes padding
//      + 4 bytes for |int32 e|
//      + 2 bytes for |int16 f|
//      + 1 byte for |Empty g|
//      + 1 byte for |Empty h|
//      = 16 bytes
//
// Alignment = 4 bytes stemming from largest member (int32).
//
type EmptyWithOtherThings = struct {
    a bool;
    // no padding
    b Empty;
    // no padding
    c int16;
    // no padding
    d Empty;
    // 3 bytes padding
    e int32;
    // no padding
    f int16;
    // no padding
    g Empty;
    // no padding
    h Empty;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let empty_with_other_things = root.lookup_struct("example/EmptyWithOtherThings").expect("EmptyWithOtherThings not found");
        assert_eq!(empty_with_other_things.type_shape_v2.inline_size, 16, "inline_size mismatch for empty_with_other_things");
        assert_eq!(empty_with_other_things.type_shape_v2.alignment, 4, "alignment mismatch for empty_with_other_things");
        assert_eq!(empty_with_other_things.type_shape_v2.depth, 0, "depth mismatch for empty_with_other_things");
        assert_eq!(empty_with_other_things.type_shape_v2.max_handles, 0, "max_handles mismatch for empty_with_other_things");
        assert_eq!(empty_with_other_things.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for empty_with_other_things");
        assert_eq!(empty_with_other_things.type_shape_v2.has_padding, true, "has_padding mismatch for empty_with_other_things");
        assert_eq!(empty_with_other_things.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for empty_with_other_things");
        let member_empty_with_other_things_0 = empty_with_other_things.members[0].clone();
        assert_eq!(member_empty_with_other_things_0.field_shape_v2.offset, 0, "offset mismatch for empty_with_other_things member 0");
        assert_eq!(member_empty_with_other_things_0.field_shape_v2.padding, 0, "padding mismatch for empty_with_other_things member 0");
        let member_empty_with_other_things_1 = empty_with_other_things.members[1].clone();
        assert_eq!(member_empty_with_other_things_1.field_shape_v2.offset, 1, "offset mismatch for empty_with_other_things member 1");
        assert_eq!(member_empty_with_other_things_1.field_shape_v2.padding, 0, "padding mismatch for empty_with_other_things member 1");
        let member_empty_with_other_things_2 = empty_with_other_things.members[2].clone();
        assert_eq!(member_empty_with_other_things_2.field_shape_v2.offset, 2, "offset mismatch for empty_with_other_things member 2");
        assert_eq!(member_empty_with_other_things_2.field_shape_v2.padding, 0, "padding mismatch for empty_with_other_things member 2");
        let member_empty_with_other_things_3 = empty_with_other_things.members[3].clone();
        assert_eq!(member_empty_with_other_things_3.field_shape_v2.offset, 4, "offset mismatch for empty_with_other_things member 3");
        assert_eq!(member_empty_with_other_things_3.field_shape_v2.padding, 3, "padding mismatch for empty_with_other_things member 3");
        let member_empty_with_other_things_4 = empty_with_other_things.members[4].clone();
        assert_eq!(member_empty_with_other_things_4.field_shape_v2.offset, 8, "offset mismatch for empty_with_other_things member 4");
        assert_eq!(member_empty_with_other_things_4.field_shape_v2.padding, 0, "padding mismatch for empty_with_other_things member 4");
        let member_empty_with_other_things_5 = empty_with_other_things.members[5].clone();
        assert_eq!(member_empty_with_other_things_5.field_shape_v2.offset, 12, "offset mismatch for empty_with_other_things member 5");
        assert_eq!(member_empty_with_other_things_5.field_shape_v2.padding, 0, "padding mismatch for empty_with_other_things member 5");
        let member_empty_with_other_things_6 = empty_with_other_things.members[6].clone();
        assert_eq!(member_empty_with_other_things_6.field_shape_v2.offset, 14, "offset mismatch for empty_with_other_things member 6");
        assert_eq!(member_empty_with_other_things_6.field_shape_v2.padding, 0, "padding mismatch for empty_with_other_things member 6");
        let member_empty_with_other_things_7 = empty_with_other_things.members[7].clone();
        assert_eq!(member_empty_with_other_things_7.field_shape_v2.offset, 15, "offset mismatch for empty_with_other_things member 7");
        assert_eq!(member_empty_with_other_things_7.field_shape_v2.padding, 0, "padding mismatch for empty_with_other_things member 7");
    }

    #[test]
    fn good_simple_new_types() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type BoolAndU32 = struct {
    b bool;
    u uint32;
};
type NewBoolAndU32 = BoolAndU32;

type BitsImplicit = strict bits {
    VALUE = 1;
};
type NewBitsImplicit = BitsImplicit;


type TableWithBoolAndU32 = table {
    1: b bool;
    2: u uint32;
};
type NewTableWithBoolAndU32 = TableWithBoolAndU32;

type BoolAndU64 = struct {
    b bool;
    u uint64;
};
type UnionOfThings = strict union {
    1: ob bool;
    2: bu BoolAndU64;
};
type NewUnionOfThings = UnionOfThings;
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let new_bool_and_u32_struct = root.lookup_alias("example/NewBoolAndU32").expect("NewBoolAndU32 not found");
        let new_bits_implicit = root.lookup_alias("example/NewBitsImplicit").expect("NewBitsImplicit not found");
        let new_bool_and_u32_table = root.lookup_alias("example/NewTableWithBoolAndU32").expect("NewTableWithBoolAndU32 not found");
        let new_union = root.lookup_alias("example/NewUnionOfThings").expect("NewUnionOfThings not found");
        assert_eq!(new_bool_and_u32_struct.type_.type_shape_v2.inline_size, 8, "inline_size mismatch for new_bool_and_u32_struct");
        assert_eq!(new_bool_and_u32_struct.type_.type_shape_v2.alignment, 4, "alignment mismatch for new_bool_and_u32_struct");
        assert_eq!(new_bool_and_u32_struct.type_.type_shape_v2.depth, 0, "depth mismatch for new_bool_and_u32_struct");
        assert_eq!(new_bool_and_u32_struct.type_.type_shape_v2.max_handles, 0, "max_handles mismatch for new_bool_and_u32_struct");
        assert_eq!(new_bool_and_u32_struct.type_.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for new_bool_and_u32_struct");
        assert_eq!(new_bool_and_u32_struct.type_.type_shape_v2.has_padding, true, "has_padding mismatch for new_bool_and_u32_struct");
        assert_eq!(new_bool_and_u32_struct.type_.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for new_bool_and_u32_struct");
        assert_eq!(new_bits_implicit.type_.type_shape_v2.inline_size, 4, "inline_size mismatch for new_bits_implicit");
        assert_eq!(new_bits_implicit.type_.type_shape_v2.alignment, 4, "alignment mismatch for new_bits_implicit");
        assert_eq!(new_bits_implicit.type_.type_shape_v2.depth, 0, "depth mismatch for new_bits_implicit");
        assert_eq!(new_bits_implicit.type_.type_shape_v2.max_handles, 0, "max_handles mismatch for new_bits_implicit");
        assert_eq!(new_bits_implicit.type_.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for new_bits_implicit");
        assert_eq!(new_bits_implicit.type_.type_shape_v2.has_padding, false, "has_padding mismatch for new_bits_implicit");
        assert_eq!(new_bits_implicit.type_.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for new_bits_implicit");
        assert_eq!(new_bool_and_u32_table.type_.type_shape_v2.inline_size, 16, "inline_size mismatch for new_bool_and_u32_table");
        assert_eq!(new_bool_and_u32_table.type_.type_shape_v2.alignment, 8, "alignment mismatch for new_bool_and_u32_table");
        assert_eq!(new_bool_and_u32_table.type_.type_shape_v2.depth, 2, "depth mismatch for new_bool_and_u32_table");
        assert_eq!(new_bool_and_u32_table.type_.type_shape_v2.max_handles, 0, "max_handles mismatch for new_bool_and_u32_table");
        assert_eq!(new_bool_and_u32_table.type_.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for new_bool_and_u32_table");
        assert_eq!(new_bool_and_u32_table.type_.type_shape_v2.has_padding, true, "has_padding mismatch for new_bool_and_u32_table");
        assert_eq!(new_bool_and_u32_table.type_.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for new_bool_and_u32_table");
        assert_eq!(new_union.type_.type_shape_v2.inline_size, 16, "inline_size mismatch for new_union");
        assert_eq!(new_union.type_.type_shape_v2.alignment, 8, "alignment mismatch for new_union");
        assert_eq!(new_union.type_.type_shape_v2.depth, 1, "depth mismatch for new_union");
        assert_eq!(new_union.type_.type_shape_v2.max_handles, 0, "max_handles mismatch for new_union");
        assert_eq!(new_union.type_.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for new_union");
        assert_eq!(new_union.type_.type_shape_v2.has_padding, true, "has_padding mismatch for new_union");
        assert_eq!(new_union.type_.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for new_union");
    }

    #[test]
    fn good_simple_structs() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type OneBool = struct {
    b bool;
};

type TwoBools = struct {
    a bool;
    b bool;
};

type BoolAndU32 = struct {
    b bool;
    u uint32;
};

type BoolAndU64 = struct {
    b bool;
    u uint64;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let one_bool = root.lookup_struct("example/OneBool").expect("OneBool not found");
        let two_bools = root.lookup_struct("example/TwoBools").expect("TwoBools not found");
        let bool_and_u32 = root.lookup_struct("example/BoolAndU32").expect("BoolAndU32 not found");
        let bool_and_u64 = root.lookup_struct("example/BoolAndU64").expect("BoolAndU64 not found");
        assert_eq!(one_bool.type_shape_v2.inline_size, 1, "inline_size mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.alignment, 1, "alignment mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.depth, 0, "depth mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_handles, 0, "max_handles mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_padding, false, "has_padding mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for one_bool");
        assert_eq!(two_bools.type_shape_v2.inline_size, 2, "inline_size mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.alignment, 1, "alignment mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.depth, 0, "depth mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.max_handles, 0, "max_handles mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.has_padding, false, "has_padding mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for two_bools");
        assert_eq!(bool_and_u32.type_shape_v2.inline_size, 8, "inline_size mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.alignment, 4, "alignment mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.depth, 0, "depth mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.max_handles, 0, "max_handles mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.has_padding, true, "has_padding mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for bool_and_u32");
        assert_eq!(bool_and_u64.type_shape_v2.inline_size, 16, "inline_size mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.alignment, 8, "alignment mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.depth, 0, "depth mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.max_handles, 0, "max_handles mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.has_padding, true, "has_padding mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for bool_and_u64");
        let member_one_bool_0 = one_bool.members[0].clone();
        assert_eq!(member_one_bool_0.field_shape_v2.offset, 0, "offset mismatch for one_bool member 0");
        assert_eq!(member_one_bool_0.field_shape_v2.padding, 0, "padding mismatch for one_bool member 0");
        let member_two_bools_0 = two_bools.members[0].clone();
        assert_eq!(member_two_bools_0.field_shape_v2.offset, 0, "offset mismatch for two_bools member 0");
        assert_eq!(member_two_bools_0.field_shape_v2.padding, 0, "padding mismatch for two_bools member 0");
        let member_two_bools_1 = two_bools.members[1].clone();
        assert_eq!(member_two_bools_1.field_shape_v2.offset, 1, "offset mismatch for two_bools member 1");
        assert_eq!(member_two_bools_1.field_shape_v2.padding, 0, "padding mismatch for two_bools member 1");
        let member_bool_and_u32_0 = bool_and_u32.members[0].clone();
        assert_eq!(member_bool_and_u32_0.field_shape_v2.offset, 0, "offset mismatch for bool_and_u32 member 0");
        assert_eq!(member_bool_and_u32_0.field_shape_v2.padding, 3, "padding mismatch for bool_and_u32 member 0");
        let member_bool_and_u32_1 = bool_and_u32.members[1].clone();
        assert_eq!(member_bool_and_u32_1.field_shape_v2.offset, 4, "offset mismatch for bool_and_u32 member 1");
        assert_eq!(member_bool_and_u32_1.field_shape_v2.padding, 0, "padding mismatch for bool_and_u32 member 1");
        let member_bool_and_u64_0 = bool_and_u64.members[0].clone();
        assert_eq!(member_bool_and_u64_0.field_shape_v2.offset, 0, "offset mismatch for bool_and_u64 member 0");
        assert_eq!(member_bool_and_u64_0.field_shape_v2.padding, 7, "padding mismatch for bool_and_u64 member 0");
        let member_bool_and_u64_1 = bool_and_u64.members[1].clone();
        assert_eq!(member_bool_and_u64_1.field_shape_v2.offset, 8, "offset mismatch for bool_and_u64 member 1");
        assert_eq!(member_bool_and_u64_1.field_shape_v2.padding, 0, "padding mismatch for bool_and_u64 member 1");
    }

    #[test]
    fn good_simple_structs_with_handles() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type OneHandle = resource struct {
  h zx.Handle;
};

type TwoHandles = resource struct {
  h1 zx.Handle:CHANNEL;
  h2 zx.Handle:PORT;
};

type ThreeHandlesOneOptional = resource struct {
  h1 zx.Handle:CHANNEL;
  h2 zx.Handle:PORT;
  opt_h3 zx.Handle:<VMO, optional>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let one_handle = root.lookup_struct("example/OneHandle").expect("OneHandle not found");
        let two_handles = root.lookup_struct("example/TwoHandles").expect("TwoHandles not found");
        let three_handles_one_optional = root.lookup_struct("example/ThreeHandlesOneOptional").expect("ThreeHandlesOneOptional not found");
        assert_eq!(one_handle.type_shape_v2.inline_size, 4, "inline_size mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.alignment, 4, "alignment mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.depth, 0, "depth mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.max_handles, 1, "max_handles mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.has_padding, false, "has_padding mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for one_handle");
        assert_eq!(two_handles.type_shape_v2.inline_size, 8, "inline_size mismatch for two_handles");
        assert_eq!(two_handles.type_shape_v2.alignment, 4, "alignment mismatch for two_handles");
        assert_eq!(two_handles.type_shape_v2.depth, 0, "depth mismatch for two_handles");
        assert_eq!(two_handles.type_shape_v2.max_handles, 2, "max_handles mismatch for two_handles");
        assert_eq!(two_handles.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for two_handles");
        assert_eq!(two_handles.type_shape_v2.has_padding, false, "has_padding mismatch for two_handles");
        assert_eq!(two_handles.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for two_handles");
        assert_eq!(three_handles_one_optional.type_shape_v2.inline_size, 12, "inline_size mismatch for three_handles_one_optional");
        assert_eq!(three_handles_one_optional.type_shape_v2.alignment, 4, "alignment mismatch for three_handles_one_optional");
        assert_eq!(three_handles_one_optional.type_shape_v2.depth, 0, "depth mismatch for three_handles_one_optional");
        assert_eq!(three_handles_one_optional.type_shape_v2.max_handles, 3, "max_handles mismatch for three_handles_one_optional");
        assert_eq!(three_handles_one_optional.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for three_handles_one_optional");
        assert_eq!(three_handles_one_optional.type_shape_v2.has_padding, false, "has_padding mismatch for three_handles_one_optional");
        assert_eq!(three_handles_one_optional.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for three_handles_one_optional");
        let member_one_handle_0 = one_handle.members[0].clone();
        assert_eq!(member_one_handle_0.field_shape_v2.offset, 0, "offset mismatch for one_handle member 0");
        assert_eq!(member_one_handle_0.field_shape_v2.padding, 0, "padding mismatch for one_handle member 0");
        let member_two_handles_0 = two_handles.members[0].clone();
        assert_eq!(member_two_handles_0.field_shape_v2.offset, 0, "offset mismatch for two_handles member 0");
        assert_eq!(member_two_handles_0.field_shape_v2.padding, 0, "padding mismatch for two_handles member 0");
        let member_two_handles_1 = two_handles.members[1].clone();
        assert_eq!(member_two_handles_1.field_shape_v2.offset, 4, "offset mismatch for two_handles member 1");
        assert_eq!(member_two_handles_1.field_shape_v2.padding, 0, "padding mismatch for two_handles member 1");
        let member_three_handles_one_optional_0 = three_handles_one_optional.members[0].clone();
        assert_eq!(member_three_handles_one_optional_0.field_shape_v2.offset, 0, "offset mismatch for three_handles_one_optional member 0");
        assert_eq!(member_three_handles_one_optional_0.field_shape_v2.padding, 0, "padding mismatch for three_handles_one_optional member 0");
        let member_three_handles_one_optional_1 = three_handles_one_optional.members[1].clone();
        assert_eq!(member_three_handles_one_optional_1.field_shape_v2.offset, 4, "offset mismatch for three_handles_one_optional member 1");
        assert_eq!(member_three_handles_one_optional_1.field_shape_v2.padding, 0, "padding mismatch for three_handles_one_optional member 1");
        let member_three_handles_one_optional_2 = three_handles_one_optional.members[2].clone();
        assert_eq!(member_three_handles_one_optional_2.field_shape_v2.offset, 8, "offset mismatch for three_handles_one_optional member 2");
        assert_eq!(member_three_handles_one_optional_2.field_shape_v2.padding, 0, "padding mismatch for three_handles_one_optional member 2");
    }

    #[test]
    fn good_bits() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Bits16 = strict bits : uint16 {
    VALUE = 1;
};

type BitsImplicit = strict bits {
    VALUE = 1;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let bits16 = root.lookup_bits("example/Bits16").expect("Bits16 not found");
        let bits_implicit = root.lookup_bits("example/BitsImplicit").expect("BitsImplicit not found");
        assert_eq!(bits16.type_.type_shape_v2.inline_size, 2, "inline_size mismatch for bits16");
        assert_eq!(bits16.type_.type_shape_v2.alignment, 2, "alignment mismatch for bits16");
        assert_eq!(bits16.type_.type_shape_v2.depth, 0, "depth mismatch for bits16");
        assert_eq!(bits16.type_.type_shape_v2.max_handles, 0, "max_handles mismatch for bits16");
        assert_eq!(bits16.type_.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for bits16");
        assert_eq!(bits16.type_.type_shape_v2.has_padding, false, "has_padding mismatch for bits16");
        assert_eq!(bits16.type_.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for bits16");
        assert_eq!(bits_implicit.type_.type_shape_v2.inline_size, 4, "inline_size mismatch for bits_implicit");
        assert_eq!(bits_implicit.type_.type_shape_v2.alignment, 4, "alignment mismatch for bits_implicit");
        assert_eq!(bits_implicit.type_.type_shape_v2.depth, 0, "depth mismatch for bits_implicit");
        assert_eq!(bits_implicit.type_.type_shape_v2.max_handles, 0, "max_handles mismatch for bits_implicit");
        assert_eq!(bits_implicit.type_.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for bits_implicit");
        assert_eq!(bits_implicit.type_.type_shape_v2.has_padding, false, "has_padding mismatch for bits_implicit");
        assert_eq!(bits_implicit.type_.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for bits_implicit");
    }

    #[test]
    fn good_simple_tables() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type TableWithNoMembers = table {};

type TableWithOneBool = table {
    1: b bool;
};

type TableWithTwoBools = table {
    1: a bool;
    2: b bool;
};

type TableWithBoolAndU32 = table {
    1: b bool;
    2: u uint32;
};

type TableWithBoolAndU64 = table {
    1: b bool;
    2: u uint64;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let no_members = root.lookup_table("example/TableWithNoMembers").expect("TableWithNoMembers not found");
        let one_bool = root.lookup_table("example/TableWithOneBool").expect("TableWithOneBool not found");
        let two_bools = root.lookup_table("example/TableWithTwoBools").expect("TableWithTwoBools not found");
        let bool_and_u32 = root.lookup_table("example/TableWithBoolAndU32").expect("TableWithBoolAndU32 not found");
        let bool_and_u64 = root.lookup_table("example/TableWithBoolAndU64").expect("TableWithBoolAndU64 not found");
        assert_eq!(no_members.type_shape_v2.inline_size, 16, "inline_size mismatch for no_members");
        assert_eq!(no_members.type_shape_v2.alignment, 8, "alignment mismatch for no_members");
        assert_eq!(no_members.type_shape_v2.depth, 1, "depth mismatch for no_members");
        assert_eq!(no_members.type_shape_v2.max_handles, 0, "max_handles mismatch for no_members");
        assert_eq!(no_members.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for no_members");
        assert_eq!(no_members.type_shape_v2.has_padding, false, "has_padding mismatch for no_members");
        assert_eq!(no_members.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for no_members");
        assert_eq!(one_bool.type_shape_v2.inline_size, 16, "inline_size mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.alignment, 8, "alignment mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.depth, 2, "depth mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_handles, 0, "max_handles mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_padding, true, "has_padding mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for one_bool");
        assert_eq!(two_bools.type_shape_v2.inline_size, 16, "inline_size mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.alignment, 8, "alignment mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.depth, 2, "depth mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.max_handles, 0, "max_handles mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.has_padding, true, "has_padding mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for two_bools");
        assert_eq!(bool_and_u32.type_shape_v2.inline_size, 16, "inline_size mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.alignment, 8, "alignment mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.depth, 2, "depth mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.max_handles, 0, "max_handles mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.has_padding, true, "has_padding mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for bool_and_u32");
        assert_eq!(bool_and_u64.type_shape_v2.inline_size, 16, "inline_size mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.alignment, 8, "alignment mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.depth, 2, "depth mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.max_handles, 0, "max_handles mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.has_padding, true, "has_padding mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for bool_and_u64");
    }

    #[test]
    fn good_tables_with_ordinal_gaps() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type GapInMiddle = table {
    1: b bool;
    3: b2 bool;
};

type GapAtStart = table {
    3: b bool;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let some_reserved = root.lookup_table("example/GapInMiddle").expect("GapInMiddle not found");
        let last_non_reserved = root.lookup_table("example/GapAtStart").expect("GapAtStart not found");
        assert_eq!(some_reserved.type_shape_v2.inline_size, 16, "inline_size mismatch for some_reserved");
        assert_eq!(some_reserved.type_shape_v2.alignment, 8, "alignment mismatch for some_reserved");
        assert_eq!(some_reserved.type_shape_v2.depth, 2, "depth mismatch for some_reserved");
        assert_eq!(some_reserved.type_shape_v2.max_handles, 0, "max_handles mismatch for some_reserved");
        assert_eq!(some_reserved.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for some_reserved");
        assert_eq!(some_reserved.type_shape_v2.has_padding, true, "has_padding mismatch for some_reserved");
        assert_eq!(some_reserved.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for some_reserved");
        assert_eq!(last_non_reserved.type_shape_v2.inline_size, 16, "inline_size mismatch for last_non_reserved");
        assert_eq!(last_non_reserved.type_shape_v2.alignment, 8, "alignment mismatch for last_non_reserved");
        assert_eq!(last_non_reserved.type_shape_v2.depth, 2, "depth mismatch for last_non_reserved");
        assert_eq!(last_non_reserved.type_shape_v2.max_handles, 0, "max_handles mismatch for last_non_reserved");
        assert_eq!(last_non_reserved.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for last_non_reserved");
        assert_eq!(last_non_reserved.type_shape_v2.has_padding, true, "has_padding mismatch for last_non_reserved");
        assert_eq!(last_non_reserved.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for last_non_reserved");
    }

    #[test]
    fn good_simple_tables_with_handles() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type TableWithOneHandle = resource table {
  1: h zx.Handle;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let one_handle = root.lookup_table("example/TableWithOneHandle").expect("TableWithOneHandle not found");
        assert_eq!(one_handle.type_shape_v2.inline_size, 16, "inline_size mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.alignment, 8, "alignment mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.depth, 2, "depth mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.max_handles, 1, "max_handles mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.has_padding, false, "has_padding mismatch for one_handle");
        assert_eq!(one_handle.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for one_handle");
    }

    #[test]
    fn good_optional_structs() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type OneBool = struct {
    b bool;
};

type OptionalOneBool = struct {
    s box<OneBool>;
};

type TwoBools = struct {
    a bool;
    b bool;
};

type OptionalTwoBools = struct {
    s box<TwoBools>;
};

type BoolAndU32 = struct {
    b bool;
    u uint32;
};

type OptionalBoolAndU32 = struct {
    s box<BoolAndU32>;
};

type BoolAndU64 = struct {
    b bool;
    u uint64;
};

type OptionalBoolAndU64 = struct {
    s box<BoolAndU64>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let one_bool = root.lookup_struct("example/OptionalOneBool").expect("OptionalOneBool not found");
        let two_bools = root.lookup_struct("example/OptionalTwoBools").expect("OptionalTwoBools not found");
        let bool_and_u32 = root.lookup_struct("example/OptionalBoolAndU32").expect("OptionalBoolAndU32 not found");
        let bool_and_u64 = root.lookup_struct("example/OptionalBoolAndU64").expect("OptionalBoolAndU64 not found");
        assert_eq!(one_bool.type_shape_v2.inline_size, 8, "inline_size mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.alignment, 8, "alignment mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.depth, 1, "depth mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_handles, 0, "max_handles mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_padding, true, "has_padding mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for one_bool");
        assert_eq!(two_bools.type_shape_v2.inline_size, 8, "inline_size mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.alignment, 8, "alignment mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.depth, 1, "depth mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.max_handles, 0, "max_handles mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.has_padding, true, "has_padding mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for two_bools");
        assert_eq!(bool_and_u32.type_shape_v2.inline_size, 8, "inline_size mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.alignment, 8, "alignment mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.depth, 1, "depth mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.max_handles, 0, "max_handles mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.has_padding, true, "has_padding mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for bool_and_u32");
        assert_eq!(bool_and_u64.type_shape_v2.inline_size, 8, "inline_size mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.alignment, 8, "alignment mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.depth, 1, "depth mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.max_handles, 0, "max_handles mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.has_padding, true, "has_padding mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for bool_and_u64");
    }

    #[test]
    fn good_optional_tables() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type OneBool = struct {
    b bool;
};

type TableWithOptionalOneBool = table {
    1: s OneBool;
};

type TableWithOneBool = table {
    1: b bool;
};

type TableWithOptionalTableWithOneBool = table {
    1: s TableWithOneBool;
};

type TwoBools = struct {
    a bool;
    b bool;
};

type TableWithOptionalTwoBools = table {
    1: s TwoBools;
};

type TableWithTwoBools = table {
    1: a bool;
    2: b bool;
};

type TableWithOptionalTableWithTwoBools = table {
    1: s TableWithTwoBools;
};

type BoolAndU32 = struct {
    b bool;
    u uint32;
};

type TableWithOptionalBoolAndU32 = table {
    1: s BoolAndU32;
};

type TableWithBoolAndU32 = table {
    1: b bool;
    2: u uint32;
};

type TableWithOptionalTableWithBoolAndU32 = table {
    1: s TableWithBoolAndU32;
};

type BoolAndU64 = struct {
    b bool;
    u uint64;
};

type TableWithOptionalBoolAndU64 = table {
    1: s BoolAndU64;
};

type TableWithBoolAndU64 = table {
    1: b bool;
    2: u uint64;
};

type TableWithOptionalTableWithBoolAndU64 = table {
    1: s TableWithBoolAndU64;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let one_bool = root.lookup_table("example/TableWithOptionalOneBool").expect("TableWithOptionalOneBool not found");
        let table_with_one_bool = root.lookup_table("example/TableWithOptionalTableWithOneBool").expect("TableWithOptionalTableWithOneBool not found");
        let two_bools = root.lookup_table("example/TableWithOptionalTwoBools").expect("TableWithOptionalTwoBools not found");
        let table_with_two_bools = root.lookup_table("example/TableWithOptionalTableWithTwoBools").expect("TableWithOptionalTableWithTwoBools not found");
        let bool_and_u32 = root.lookup_table("example/TableWithOptionalBoolAndU32").expect("TableWithOptionalBoolAndU32 not found");
        let table_with_bool_and_u32 = root.lookup_table("example/TableWithOptionalTableWithBoolAndU32").expect("TableWithOptionalTableWithBoolAndU32 not found");
        let bool_and_u64 = root.lookup_table("example/TableWithOptionalBoolAndU64").expect("TableWithOptionalBoolAndU64 not found");
        let table_with_bool_and_u64 = root.lookup_table("example/TableWithOptionalTableWithBoolAndU64").expect("TableWithOptionalTableWithBoolAndU64 not found");
        assert_eq!(one_bool.type_shape_v2.inline_size, 16, "inline_size mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.alignment, 8, "alignment mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.depth, 2, "depth mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_handles, 0, "max_handles mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_padding, true, "has_padding mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for one_bool");
        assert_eq!(table_with_one_bool.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_one_bool");
        assert_eq!(table_with_one_bool.type_shape_v2.alignment, 8, "alignment mismatch for table_with_one_bool");
        assert_eq!(table_with_one_bool.type_shape_v2.depth, 4, "depth mismatch for table_with_one_bool");
        assert_eq!(table_with_one_bool.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_one_bool");
        assert_eq!(table_with_one_bool.type_shape_v2.max_out_of_line, 32, "max_out_of_line mismatch for table_with_one_bool");
        assert_eq!(table_with_one_bool.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_one_bool");
        assert_eq!(table_with_one_bool.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_one_bool");
        assert_eq!(two_bools.type_shape_v2.inline_size, 16, "inline_size mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.alignment, 8, "alignment mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.depth, 2, "depth mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.max_handles, 0, "max_handles mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.has_padding, true, "has_padding mismatch for two_bools");
        assert_eq!(two_bools.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for two_bools");
        assert_eq!(table_with_two_bools.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_two_bools");
        assert_eq!(table_with_two_bools.type_shape_v2.alignment, 8, "alignment mismatch for table_with_two_bools");
        assert_eq!(table_with_two_bools.type_shape_v2.depth, 4, "depth mismatch for table_with_two_bools");
        assert_eq!(table_with_two_bools.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_two_bools");
        assert_eq!(table_with_two_bools.type_shape_v2.max_out_of_line, 40, "max_out_of_line mismatch for table_with_two_bools");
        assert_eq!(table_with_two_bools.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_two_bools");
        assert_eq!(table_with_two_bools.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_two_bools");
        assert_eq!(bool_and_u32.type_shape_v2.inline_size, 16, "inline_size mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.alignment, 8, "alignment mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.depth, 2, "depth mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.max_handles, 0, "max_handles mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.has_padding, true, "has_padding mismatch for bool_and_u32");
        assert_eq!(bool_and_u32.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for bool_and_u32");
        assert_eq!(table_with_bool_and_u32.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_bool_and_u32");
        assert_eq!(table_with_bool_and_u32.type_shape_v2.alignment, 8, "alignment mismatch for table_with_bool_and_u32");
        assert_eq!(table_with_bool_and_u32.type_shape_v2.depth, 4, "depth mismatch for table_with_bool_and_u32");
        assert_eq!(table_with_bool_and_u32.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_bool_and_u32");
        assert_eq!(table_with_bool_and_u32.type_shape_v2.max_out_of_line, 40, "max_out_of_line mismatch for table_with_bool_and_u32");
        assert_eq!(table_with_bool_and_u32.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_bool_and_u32");
        assert_eq!(table_with_bool_and_u32.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_bool_and_u32");
        assert_eq!(bool_and_u64.type_shape_v2.inline_size, 16, "inline_size mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.alignment, 8, "alignment mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.depth, 2, "depth mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.max_handles, 0, "max_handles mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.has_padding, true, "has_padding mismatch for bool_and_u64");
        assert_eq!(bool_and_u64.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for bool_and_u64");
        assert_eq!(table_with_bool_and_u64.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_bool_and_u64");
        assert_eq!(table_with_bool_and_u64.type_shape_v2.alignment, 8, "alignment mismatch for table_with_bool_and_u64");
        assert_eq!(table_with_bool_and_u64.type_shape_v2.depth, 4, "depth mismatch for table_with_bool_and_u64");
        assert_eq!(table_with_bool_and_u64.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_bool_and_u64");
        assert_eq!(table_with_bool_and_u64.type_shape_v2.max_out_of_line, 48, "max_out_of_line mismatch for table_with_bool_and_u64");
        assert_eq!(table_with_bool_and_u64.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_bool_and_u64");
        assert_eq!(table_with_bool_and_u64.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_bool_and_u64");
    }

    #[test]
    fn good_unions() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type BoolAndU64 = struct {
    b bool;
    u uint64;
};

type UnionOfThings = strict union {
    1: ob bool;
    2: bu BoolAndU64;
};

type Bool = struct {
    b bool;
};

type OptBool = struct {
    opt_b box<Bool>;
};

type UnionWithOutOfLine = strict union {
    1: opt_bool OptBool;
};

type OptionalUnion = struct {
    u UnionOfThings:optional;
};

type TableWithOptionalUnion = table {
    1: u UnionOfThings;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let union_with_out_of_line = root.lookup_union("example/UnionWithOutOfLine").expect("UnionWithOutOfLine not found");
        let a_union = root.lookup_union("example/UnionOfThings").expect("UnionOfThings not found");
        let optional_union = root.lookup_struct("example/OptionalUnion").expect("OptionalUnion not found");
        let table_with_optional_union = root.lookup_table("example/TableWithOptionalUnion").expect("TableWithOptionalUnion not found");
        assert_eq!(union_with_out_of_line.type_shape_v2.inline_size, 16, "inline_size mismatch for union_with_out_of_line");
        assert_eq!(union_with_out_of_line.type_shape_v2.alignment, 8, "alignment mismatch for union_with_out_of_line");
        assert_eq!(union_with_out_of_line.type_shape_v2.depth, 2, "depth mismatch for union_with_out_of_line");
        assert_eq!(union_with_out_of_line.type_shape_v2.max_handles, 0, "max_handles mismatch for union_with_out_of_line");
        assert_eq!(union_with_out_of_line.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for union_with_out_of_line");
        assert_eq!(union_with_out_of_line.type_shape_v2.has_padding, true, "has_padding mismatch for union_with_out_of_line");
        assert_eq!(union_with_out_of_line.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for union_with_out_of_line");
        assert_eq!(a_union.type_shape_v2.inline_size, 16, "inline_size mismatch for a_union");
        assert_eq!(a_union.type_shape_v2.alignment, 8, "alignment mismatch for a_union");
        assert_eq!(a_union.type_shape_v2.depth, 1, "depth mismatch for a_union");
        assert_eq!(a_union.type_shape_v2.max_handles, 0, "max_handles mismatch for a_union");
        assert_eq!(a_union.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for a_union");
        assert_eq!(a_union.type_shape_v2.has_padding, true, "has_padding mismatch for a_union");
        assert_eq!(a_union.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for a_union");
        assert_eq!(optional_union.type_shape_v2.inline_size, 16, "inline_size mismatch for optional_union");
        assert_eq!(optional_union.type_shape_v2.alignment, 8, "alignment mismatch for optional_union");
        assert_eq!(optional_union.type_shape_v2.depth, 1, "depth mismatch for optional_union");
        assert_eq!(optional_union.type_shape_v2.max_handles, 0, "max_handles mismatch for optional_union");
        assert_eq!(optional_union.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for optional_union");
        assert_eq!(optional_union.type_shape_v2.has_padding, true, "has_padding mismatch for optional_union");
        assert_eq!(optional_union.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for optional_union");
        assert_eq!(table_with_optional_union.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_optional_union");
        assert_eq!(table_with_optional_union.type_shape_v2.alignment, 8, "alignment mismatch for table_with_optional_union");
        assert_eq!(table_with_optional_union.type_shape_v2.depth, 3, "depth mismatch for table_with_optional_union");
        assert_eq!(table_with_optional_union.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_optional_union");
        assert_eq!(table_with_optional_union.type_shape_v2.max_out_of_line, 40, "max_out_of_line mismatch for table_with_optional_union");
        assert_eq!(table_with_optional_union.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_optional_union");
        assert_eq!(table_with_optional_union.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_optional_union");
    }

    #[test]
    fn good_unions_with_handles() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type OneHandleUnion = strict resource union {
  1: one_handle zx.Handle;
  2: one_bool bool;
  3: one_int uint32;
};

type ManyHandleUnion = strict resource union {
  1: one_handle zx.Handle;
  2: handle_array array<zx.Handle, 8>;
  3: handle_vector vector<zx.Handle>:8;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let one_handle_union = root.lookup_union("example/OneHandleUnion").expect("OneHandleUnion not found");
        let many_handle_union = root.lookup_union("example/ManyHandleUnion").expect("ManyHandleUnion not found");
        assert_eq!(one_handle_union.type_shape_v2.inline_size, 16, "inline_size mismatch for one_handle_union");
        assert_eq!(one_handle_union.type_shape_v2.alignment, 8, "alignment mismatch for one_handle_union");
        assert_eq!(one_handle_union.type_shape_v2.depth, 1, "depth mismatch for one_handle_union");
        assert_eq!(one_handle_union.type_shape_v2.max_handles, 1, "max_handles mismatch for one_handle_union");
        assert_eq!(one_handle_union.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for one_handle_union");
        assert_eq!(one_handle_union.type_shape_v2.has_padding, true, "has_padding mismatch for one_handle_union");
        assert_eq!(one_handle_union.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for one_handle_union");
        assert_eq!(many_handle_union.type_shape_v2.inline_size, 16, "inline_size mismatch for many_handle_union");
        assert_eq!(many_handle_union.type_shape_v2.alignment, 8, "alignment mismatch for many_handle_union");
        assert_eq!(many_handle_union.type_shape_v2.depth, 2, "depth mismatch for many_handle_union");
        assert_eq!(many_handle_union.type_shape_v2.max_handles, 8, "max_handles mismatch for many_handle_union");
        assert_eq!(many_handle_union.type_shape_v2.max_out_of_line, 48, "max_out_of_line mismatch for many_handle_union");
        assert_eq!(many_handle_union.type_shape_v2.has_padding, true, "has_padding mismatch for many_handle_union");
        assert_eq!(many_handle_union.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for many_handle_union");
    }

    #[test]
    fn good_overlays() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type BoolOrStringOrU64 = strict overlay {
    1: b bool;
    2: s string:255;
    3: u uint64;
};

type BoolOverlay = strict overlay {
    1: b bool;
};

type U64BoolStruct = struct {
    u uint64;
    b bool;
};

type BoolOverlayStruct = struct {
    bo BoolOverlay;
};

type BoolOverlayAndUint8Struct = struct {
    bo BoolOverlay;
    x uint8;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let bool_or_string_or_u64 = root.lookup_union("example/BoolOrStringOrU64").expect("BoolOrStringOrU64 not found");
        let bool_overlay = root.lookup_union("example/BoolOverlay").expect("BoolOverlay not found");
        let u64_bool_struct = root.lookup_struct("example/U64BoolStruct").expect("U64BoolStruct not found");
        let bool_overlay_struct = root.lookup_struct("example/BoolOverlayStruct").expect("BoolOverlayStruct not found");
        let bool_overlay_and_uint8_struct = root.lookup_struct("example/BoolOverlayAndUint8Struct").expect("BoolOverlayAndUint8Struct not found");
        assert_eq!(u64_bool_struct.type_shape_v2.inline_size, 16, "inline_size mismatch for u64_bool_struct");
        assert_eq!(u64_bool_struct.type_shape_v2.alignment, 8, "alignment mismatch for u64_bool_struct");
        assert_eq!(u64_bool_struct.type_shape_v2.depth, 0, "depth mismatch for u64_bool_struct");
        assert_eq!(u64_bool_struct.type_shape_v2.max_handles, 0, "max_handles mismatch for u64_bool_struct");
        assert_eq!(u64_bool_struct.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for u64_bool_struct");
        assert_eq!(u64_bool_struct.type_shape_v2.has_padding, true, "has_padding mismatch for u64_bool_struct");
        assert_eq!(u64_bool_struct.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for u64_bool_struct");
        let member_bool_overlay_struct_0 = bool_overlay_struct.members[0].clone();
        assert_eq!(member_bool_overlay_struct_0.field_shape_v2.offset, 0, "offset mismatch for bool_overlay_struct member 0");
        assert_eq!(member_bool_overlay_struct_0.field_shape_v2.padding, 0, "padding mismatch for bool_overlay_struct member 0");
        let member_bool_overlay_and_uint8_struct_0 = bool_overlay_and_uint8_struct.members[0].clone();
        assert_eq!(member_bool_overlay_and_uint8_struct_0.field_shape_v2.offset, 0, "offset mismatch for bool_overlay_and_uint8_struct member 0");
        assert_eq!(member_bool_overlay_and_uint8_struct_0.field_shape_v2.padding, 0, "padding mismatch for bool_overlay_and_uint8_struct member 0");
        let member_bool_overlay_and_uint8_struct_1 = bool_overlay_and_uint8_struct.members[1].clone();
        assert_eq!(member_bool_overlay_and_uint8_struct_1.field_shape_v2.offset, 16, "offset mismatch for bool_overlay_and_uint8_struct member 1");
        assert_eq!(member_bool_overlay_and_uint8_struct_1.field_shape_v2.padding, 7, "padding mismatch for bool_overlay_and_uint8_struct member 1");
    }

    #[test]
    fn good_vectors() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type PaddedVector = struct {
    pv vector<int32>:3;
};

type NoPaddingVector = struct {
    npv vector<uint64>:3;
};

type UnboundedVector = struct {
    uv vector<int32>;
};

type UnboundedVectors = struct {
    uv1 vector<int32>;
    uv2 vector<int32>;
};

type TableWithPaddedVector = table {
    1: pv vector<int32>:3;
};

type TableWithUnboundedVector = table {
    1: uv vector<int32>;
};

type TableWithUnboundedVectors = table {
    1: uv1 vector<int32>;
    2: uv2 vector<int32>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let padded_vector = root.lookup_struct("example/PaddedVector").expect("PaddedVector not found");
        let no_padding_vector = root.lookup_struct("example/NoPaddingVector").expect("NoPaddingVector not found");
        let unbounded_vector = root.lookup_struct("example/UnboundedVector").expect("UnboundedVector not found");
        let unbounded_vectors = root.lookup_struct("example/UnboundedVectors").expect("UnboundedVectors not found");
        let table_with_padded_vector = root.lookup_table("example/TableWithPaddedVector").expect("TableWithPaddedVector not found");
        let table_with_unbounded_vector = root.lookup_table("example/TableWithUnboundedVector").expect("TableWithUnboundedVector not found");
        let table_with_unbounded_vectors = root.lookup_table("example/TableWithUnboundedVectors").expect("TableWithUnboundedVectors not found");
        assert_eq!(padded_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for padded_vector");
        assert_eq!(padded_vector.type_shape_v2.alignment, 8, "alignment mismatch for padded_vector");
        assert_eq!(padded_vector.type_shape_v2.depth, 1, "depth mismatch for padded_vector");
        assert_eq!(padded_vector.type_shape_v2.max_handles, 0, "max_handles mismatch for padded_vector");
        assert_eq!(padded_vector.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for padded_vector");
        assert_eq!(padded_vector.type_shape_v2.has_padding, true, "has_padding mismatch for padded_vector");
        assert_eq!(padded_vector.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for padded_vector");
        assert_eq!(no_padding_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for no_padding_vector");
        assert_eq!(no_padding_vector.type_shape_v2.alignment, 8, "alignment mismatch for no_padding_vector");
        assert_eq!(no_padding_vector.type_shape_v2.depth, 1, "depth mismatch for no_padding_vector");
        assert_eq!(no_padding_vector.type_shape_v2.max_handles, 0, "max_handles mismatch for no_padding_vector");
        assert_eq!(no_padding_vector.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for no_padding_vector");
        assert_eq!(no_padding_vector.type_shape_v2.has_padding, false, "has_padding mismatch for no_padding_vector");
        assert_eq!(no_padding_vector.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for no_padding_vector");
        assert_eq!(unbounded_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for unbounded_vector");
        assert_eq!(unbounded_vector.type_shape_v2.alignment, 8, "alignment mismatch for unbounded_vector");
        assert_eq!(unbounded_vector.type_shape_v2.depth, 1, "depth mismatch for unbounded_vector");
        assert_eq!(unbounded_vector.type_shape_v2.max_handles, 0, "max_handles mismatch for unbounded_vector");
        assert_eq!(unbounded_vector.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for unbounded_vector");
        assert_eq!(unbounded_vector.type_shape_v2.has_padding, true, "has_padding mismatch for unbounded_vector");
        assert_eq!(unbounded_vector.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for unbounded_vector");
        assert_eq!(unbounded_vectors.type_shape_v2.inline_size, 32, "inline_size mismatch for unbounded_vectors");
        assert_eq!(unbounded_vectors.type_shape_v2.alignment, 8, "alignment mismatch for unbounded_vectors");
        assert_eq!(unbounded_vectors.type_shape_v2.depth, 1, "depth mismatch for unbounded_vectors");
        assert_eq!(unbounded_vectors.type_shape_v2.max_handles, 0, "max_handles mismatch for unbounded_vectors");
        assert_eq!(unbounded_vectors.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for unbounded_vectors");
        assert_eq!(unbounded_vectors.type_shape_v2.has_padding, true, "has_padding mismatch for unbounded_vectors");
        assert_eq!(unbounded_vectors.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for unbounded_vectors");
        assert_eq!(table_with_padded_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_padded_vector");
        assert_eq!(table_with_padded_vector.type_shape_v2.alignment, 8, "alignment mismatch for table_with_padded_vector");
        assert_eq!(table_with_padded_vector.type_shape_v2.depth, 3, "depth mismatch for table_with_padded_vector");
        assert_eq!(table_with_padded_vector.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_padded_vector");
        assert_eq!(table_with_padded_vector.type_shape_v2.max_out_of_line, 40, "max_out_of_line mismatch for table_with_padded_vector");
        assert_eq!(table_with_padded_vector.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_padded_vector");
        assert_eq!(table_with_padded_vector.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_padded_vector");
        assert_eq!(table_with_unbounded_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_unbounded_vector");
        assert_eq!(table_with_unbounded_vector.type_shape_v2.alignment, 8, "alignment mismatch for table_with_unbounded_vector");
        assert_eq!(table_with_unbounded_vector.type_shape_v2.depth, 3, "depth mismatch for table_with_unbounded_vector");
        assert_eq!(table_with_unbounded_vector.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_unbounded_vector");
        assert_eq!(table_with_unbounded_vector.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for table_with_unbounded_vector");
        assert_eq!(table_with_unbounded_vector.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_unbounded_vector");
        assert_eq!(table_with_unbounded_vector.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_unbounded_vector");
        assert_eq!(table_with_unbounded_vectors.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_unbounded_vectors");
        assert_eq!(table_with_unbounded_vectors.type_shape_v2.alignment, 8, "alignment mismatch for table_with_unbounded_vectors");
        assert_eq!(table_with_unbounded_vectors.type_shape_v2.depth, 3, "depth mismatch for table_with_unbounded_vectors");
        assert_eq!(table_with_unbounded_vectors.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_unbounded_vectors");
        assert_eq!(table_with_unbounded_vectors.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for table_with_unbounded_vectors");
        assert_eq!(table_with_unbounded_vectors.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_unbounded_vectors");
        assert_eq!(table_with_unbounded_vectors.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_unbounded_vectors");
    }

    #[test]
    fn good_vectors_with_handles() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type HandleVector = resource struct {
  hv vector<zx.Handle>:8;
};

type HandleNullableVector = resource struct {
  hv vector<zx.Handle>:<8, optional>;
};

type TableWithHandleVector = resource table {
  1: hv vector<zx.Handle>:8;
};

type UnboundedHandleVector = resource struct {
  hv vector<zx.Handle>;
};

type TableWithUnboundedHandleVector = resource table {
  1: hv vector<zx.Handle>;
};

type OneHandle = resource struct {
  h zx.Handle;
};

type HandleStructVector = resource struct {
  sv vector<OneHandle>:8;
};

type TableWithOneHandle = resource table {
  1: h zx.Handle;
};

type HandleTableVector = resource struct {
  sv vector<TableWithOneHandle>:8;
};

type TableWithHandleStructVector = resource table {
  1: sv vector<OneHandle>:8;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let handle_vector = root.lookup_struct("example/HandleVector").expect("HandleVector not found");
        let handle_nullable_vector = root.lookup_struct("example/HandleNullableVector").expect("HandleNullableVector not found");
        let unbounded_handle_vector = root.lookup_struct("example/UnboundedHandleVector").expect("UnboundedHandleVector not found");
        let table_with_unbounded_handle_vector = root.lookup_table("example/TableWithUnboundedHandleVector").expect("TableWithUnboundedHandleVector not found");
        let handle_struct_vector = root.lookup_struct("example/HandleStructVector").expect("HandleStructVector not found");
        let handle_table_vector = root.lookup_struct("example/HandleTableVector").expect("HandleTableVector not found");
        let table_with_handle_struct_vector = root.lookup_table("example/TableWithHandleStructVector").expect("TableWithHandleStructVector not found");
        assert_eq!(handle_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for handle_vector");
        assert_eq!(handle_vector.type_shape_v2.alignment, 8, "alignment mismatch for handle_vector");
        assert_eq!(handle_vector.type_shape_v2.depth, 1, "depth mismatch for handle_vector");
        assert_eq!(handle_vector.type_shape_v2.max_handles, 8, "max_handles mismatch for handle_vector");
        assert_eq!(handle_vector.type_shape_v2.max_out_of_line, 32, "max_out_of_line mismatch for handle_vector");
        assert_eq!(handle_vector.type_shape_v2.has_padding, true, "has_padding mismatch for handle_vector");
        assert_eq!(handle_vector.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for handle_vector");
        assert_eq!(handle_nullable_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for handle_nullable_vector");
        assert_eq!(handle_nullable_vector.type_shape_v2.alignment, 8, "alignment mismatch for handle_nullable_vector");
        assert_eq!(handle_nullable_vector.type_shape_v2.depth, 1, "depth mismatch for handle_nullable_vector");
        assert_eq!(handle_nullable_vector.type_shape_v2.max_handles, 8, "max_handles mismatch for handle_nullable_vector");
        assert_eq!(handle_nullable_vector.type_shape_v2.max_out_of_line, 32, "max_out_of_line mismatch for handle_nullable_vector");
        assert_eq!(handle_nullable_vector.type_shape_v2.has_padding, true, "has_padding mismatch for handle_nullable_vector");
        assert_eq!(handle_nullable_vector.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for handle_nullable_vector");
        assert_eq!(unbounded_handle_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for unbounded_handle_vector");
        assert_eq!(unbounded_handle_vector.type_shape_v2.alignment, 8, "alignment mismatch for unbounded_handle_vector");
        assert_eq!(unbounded_handle_vector.type_shape_v2.depth, 1, "depth mismatch for unbounded_handle_vector");
        assert_eq!(unbounded_handle_vector.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for unbounded_handle_vector");
        assert_eq!(unbounded_handle_vector.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for unbounded_handle_vector");
        assert_eq!(unbounded_handle_vector.type_shape_v2.has_padding, true, "has_padding mismatch for unbounded_handle_vector");
        assert_eq!(unbounded_handle_vector.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for unbounded_handle_vector");
        assert_eq!(table_with_unbounded_handle_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_unbounded_handle_vector");
        assert_eq!(table_with_unbounded_handle_vector.type_shape_v2.alignment, 8, "alignment mismatch for table_with_unbounded_handle_vector");
        assert_eq!(table_with_unbounded_handle_vector.type_shape_v2.depth, 3, "depth mismatch for table_with_unbounded_handle_vector");
        assert_eq!(table_with_unbounded_handle_vector.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for table_with_unbounded_handle_vector");
        assert_eq!(table_with_unbounded_handle_vector.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for table_with_unbounded_handle_vector");
        assert_eq!(table_with_unbounded_handle_vector.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_unbounded_handle_vector");
        assert_eq!(table_with_unbounded_handle_vector.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_unbounded_handle_vector");
        assert_eq!(handle_struct_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for handle_struct_vector");
        assert_eq!(handle_struct_vector.type_shape_v2.alignment, 8, "alignment mismatch for handle_struct_vector");
        assert_eq!(handle_struct_vector.type_shape_v2.depth, 1, "depth mismatch for handle_struct_vector");
        assert_eq!(handle_struct_vector.type_shape_v2.max_handles, 8, "max_handles mismatch for handle_struct_vector");
        assert_eq!(handle_struct_vector.type_shape_v2.max_out_of_line, 32, "max_out_of_line mismatch for handle_struct_vector");
        assert_eq!(handle_struct_vector.type_shape_v2.has_padding, true, "has_padding mismatch for handle_struct_vector");
        assert_eq!(handle_struct_vector.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for handle_struct_vector");
        assert_eq!(handle_table_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for handle_table_vector");
        assert_eq!(handle_table_vector.type_shape_v2.alignment, 8, "alignment mismatch for handle_table_vector");
        assert_eq!(handle_table_vector.type_shape_v2.depth, 3, "depth mismatch for handle_table_vector");
        assert_eq!(handle_table_vector.type_shape_v2.max_handles, 8, "max_handles mismatch for handle_table_vector");
        assert_eq!(handle_table_vector.type_shape_v2.max_out_of_line, 192, "max_out_of_line mismatch for handle_table_vector");
        assert_eq!(handle_table_vector.type_shape_v2.has_padding, false, "has_padding mismatch for handle_table_vector");
        assert_eq!(handle_table_vector.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for handle_table_vector");
        assert_eq!(table_with_handle_struct_vector.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_handle_struct_vector");
        assert_eq!(table_with_handle_struct_vector.type_shape_v2.alignment, 8, "alignment mismatch for table_with_handle_struct_vector");
        assert_eq!(table_with_handle_struct_vector.type_shape_v2.depth, 3, "depth mismatch for table_with_handle_struct_vector");
        assert_eq!(table_with_handle_struct_vector.type_shape_v2.max_handles, 8, "max_handles mismatch for table_with_handle_struct_vector");
        assert_eq!(table_with_handle_struct_vector.type_shape_v2.max_out_of_line, 56, "max_out_of_line mismatch for table_with_handle_struct_vector");
        assert_eq!(table_with_handle_struct_vector.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_handle_struct_vector");
        assert_eq!(table_with_handle_struct_vector.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_handle_struct_vector");
    }

    #[test]
    fn good_strings() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type ShortString = struct {
    s string:5;
};

type UnboundedString = struct {
    s string;
};

type TableWithShortString = table {
    1: s string:5;
};

type TableWithUnboundedString = table {
    1: s string;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let short_string = root.lookup_struct("example/ShortString").expect("ShortString not found");
        let unbounded_string = root.lookup_struct("example/UnboundedString").expect("UnboundedString not found");
        let table_with_short_string = root.lookup_table("example/TableWithShortString").expect("TableWithShortString not found");
        let table_with_unbounded_string = root.lookup_table("example/TableWithUnboundedString").expect("TableWithUnboundedString not found");
        assert_eq!(short_string.type_shape_v2.inline_size, 16, "inline_size mismatch for short_string");
        assert_eq!(short_string.type_shape_v2.alignment, 8, "alignment mismatch for short_string");
        assert_eq!(short_string.type_shape_v2.depth, 1, "depth mismatch for short_string");
        assert_eq!(short_string.type_shape_v2.max_handles, 0, "max_handles mismatch for short_string");
        assert_eq!(short_string.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for short_string");
        assert_eq!(short_string.type_shape_v2.has_padding, true, "has_padding mismatch for short_string");
        assert_eq!(short_string.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for short_string");
        assert_eq!(unbounded_string.type_shape_v2.inline_size, 16, "inline_size mismatch for unbounded_string");
        assert_eq!(unbounded_string.type_shape_v2.alignment, 8, "alignment mismatch for unbounded_string");
        assert_eq!(unbounded_string.type_shape_v2.depth, 1, "depth mismatch for unbounded_string");
        assert_eq!(unbounded_string.type_shape_v2.max_handles, 0, "max_handles mismatch for unbounded_string");
        assert_eq!(unbounded_string.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for unbounded_string");
        assert_eq!(unbounded_string.type_shape_v2.has_padding, true, "has_padding mismatch for unbounded_string");
        assert_eq!(unbounded_string.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for unbounded_string");
        assert_eq!(table_with_short_string.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_short_string");
        assert_eq!(table_with_short_string.type_shape_v2.alignment, 8, "alignment mismatch for table_with_short_string");
        assert_eq!(table_with_short_string.type_shape_v2.depth, 3, "depth mismatch for table_with_short_string");
        assert_eq!(table_with_short_string.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_short_string");
        assert_eq!(table_with_short_string.type_shape_v2.max_out_of_line, 32, "max_out_of_line mismatch for table_with_short_string");
        assert_eq!(table_with_short_string.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_short_string");
        assert_eq!(table_with_short_string.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_short_string");
        assert_eq!(table_with_unbounded_string.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_unbounded_string");
        assert_eq!(table_with_unbounded_string.type_shape_v2.alignment, 8, "alignment mismatch for table_with_unbounded_string");
        assert_eq!(table_with_unbounded_string.type_shape_v2.depth, 3, "depth mismatch for table_with_unbounded_string");
        assert_eq!(table_with_unbounded_string.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_unbounded_string");
        assert_eq!(table_with_unbounded_string.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for table_with_unbounded_string");
        assert_eq!(table_with_unbounded_string.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_unbounded_string");
        assert_eq!(table_with_unbounded_string.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_unbounded_string");
    }

    #[test]
    fn good_string_arrays() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type StringArray = struct {
    s string_array<5>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let string_array = root.lookup_struct("example/StringArray").expect("StringArray not found");
        assert_eq!(string_array.type_shape_v2.inline_size, 5, "inline_size mismatch for string_array");
        assert_eq!(string_array.type_shape_v2.alignment, 1, "alignment mismatch for string_array");
        assert_eq!(string_array.type_shape_v2.depth, 0, "depth mismatch for string_array");
        assert_eq!(string_array.type_shape_v2.max_handles, 0, "max_handles mismatch for string_array");
        assert_eq!(string_array.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for string_array");
        assert_eq!(string_array.type_shape_v2.has_padding, false, "has_padding mismatch for string_array");
        assert_eq!(string_array.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for string_array");
    }

    #[test]
    fn good_arrays() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type AnArray = struct {
    a array<int64, 5>;
};

type TableWithAnArray = table {
    1: a array<int64, 5>;
};

type TableWithAnInt32ArrayWithPadding = table {
    1: a array<int32, 3>;
};

type TableWithAnInt32ArrayNoPadding = table {
    1: a array<int32, 4>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let an_array = root.lookup_struct("example/AnArray").expect("AnArray not found");
        let table_with_an_array = root.lookup_table("example/TableWithAnArray").expect("TableWithAnArray not found");
        let table_with_an_int32_array_with_padding = root.lookup_table("example/TableWithAnInt32ArrayWithPadding").expect("TableWithAnInt32ArrayWithPadding not found");
        let table_with_an_int32_array_no_padding = root.lookup_table("example/TableWithAnInt32ArrayNoPadding").expect("TableWithAnInt32ArrayNoPadding not found");
        assert_eq!(an_array.type_shape_v2.inline_size, 40, "inline_size mismatch for an_array");
        assert_eq!(an_array.type_shape_v2.alignment, 8, "alignment mismatch for an_array");
        assert_eq!(an_array.type_shape_v2.depth, 0, "depth mismatch for an_array");
        assert_eq!(an_array.type_shape_v2.max_handles, 0, "max_handles mismatch for an_array");
        assert_eq!(an_array.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for an_array");
        assert_eq!(an_array.type_shape_v2.has_padding, false, "has_padding mismatch for an_array");
        assert_eq!(an_array.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for an_array");
        assert_eq!(table_with_an_array.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_an_array");
        assert_eq!(table_with_an_array.type_shape_v2.alignment, 8, "alignment mismatch for table_with_an_array");
        assert_eq!(table_with_an_array.type_shape_v2.depth, 2, "depth mismatch for table_with_an_array");
        assert_eq!(table_with_an_array.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_an_array");
        assert_eq!(table_with_an_array.type_shape_v2.max_out_of_line, 48, "max_out_of_line mismatch for table_with_an_array");
        assert_eq!(table_with_an_array.type_shape_v2.has_padding, false, "has_padding mismatch for table_with_an_array");
        assert_eq!(table_with_an_array.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_an_array");
        assert_eq!(table_with_an_int32_array_with_padding.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_an_int32_array_with_padding");
        assert_eq!(table_with_an_int32_array_with_padding.type_shape_v2.alignment, 8, "alignment mismatch for table_with_an_int32_array_with_padding");
        assert_eq!(table_with_an_int32_array_with_padding.type_shape_v2.depth, 2, "depth mismatch for table_with_an_int32_array_with_padding");
        assert_eq!(table_with_an_int32_array_with_padding.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_an_int32_array_with_padding");
        assert_eq!(table_with_an_int32_array_with_padding.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for table_with_an_int32_array_with_padding");
        assert_eq!(table_with_an_int32_array_with_padding.type_shape_v2.has_padding, true, "has_padding mismatch for table_with_an_int32_array_with_padding");
        assert_eq!(table_with_an_int32_array_with_padding.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_an_int32_array_with_padding");
        assert_eq!(table_with_an_int32_array_no_padding.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_an_int32_array_no_padding");
        assert_eq!(table_with_an_int32_array_no_padding.type_shape_v2.alignment, 8, "alignment mismatch for table_with_an_int32_array_no_padding");
        assert_eq!(table_with_an_int32_array_no_padding.type_shape_v2.depth, 2, "depth mismatch for table_with_an_int32_array_no_padding");
        assert_eq!(table_with_an_int32_array_no_padding.type_shape_v2.max_handles, 0, "max_handles mismatch for table_with_an_int32_array_no_padding");
        assert_eq!(table_with_an_int32_array_no_padding.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for table_with_an_int32_array_no_padding");
        assert_eq!(table_with_an_int32_array_no_padding.type_shape_v2.has_padding, false, "has_padding mismatch for table_with_an_int32_array_no_padding");
        assert_eq!(table_with_an_int32_array_no_padding.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_an_int32_array_no_padding");
    }

    #[test]
    fn good_arrays_with_handles() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type HandleArray = resource struct {
  h1 array<zx.Handle, 8>;
};

type TableWithHandleArray = resource table {
  1: ha array<zx.Handle, 8>;
};

type NullableHandleArray = resource struct {
  ha array<zx.Handle:optional, 8>;
};

type TableWithNullableHandleArray = resource table {
  1: ha array<zx.Handle:optional, 8>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let handle_array = root.lookup_struct("example/HandleArray").expect("HandleArray not found");
        let table_with_handle_array = root.lookup_table("example/TableWithHandleArray").expect("TableWithHandleArray not found");
        let nullable_handle_array = root.lookup_struct("example/NullableHandleArray").expect("NullableHandleArray not found");
        let table_with_nullable_handle_array = root.lookup_table("example/TableWithNullableHandleArray").expect("TableWithNullableHandleArray not found");
        assert_eq!(handle_array.type_shape_v2.inline_size, 32, "inline_size mismatch for handle_array");
        assert_eq!(handle_array.type_shape_v2.alignment, 4, "alignment mismatch for handle_array");
        assert_eq!(handle_array.type_shape_v2.depth, 0, "depth mismatch for handle_array");
        assert_eq!(handle_array.type_shape_v2.max_handles, 8, "max_handles mismatch for handle_array");
        assert_eq!(handle_array.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for handle_array");
        assert_eq!(handle_array.type_shape_v2.has_padding, false, "has_padding mismatch for handle_array");
        assert_eq!(handle_array.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for handle_array");
        assert_eq!(table_with_handle_array.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_handle_array");
        assert_eq!(table_with_handle_array.type_shape_v2.alignment, 8, "alignment mismatch for table_with_handle_array");
        assert_eq!(table_with_handle_array.type_shape_v2.depth, 2, "depth mismatch for table_with_handle_array");
        assert_eq!(table_with_handle_array.type_shape_v2.max_handles, 8, "max_handles mismatch for table_with_handle_array");
        assert_eq!(table_with_handle_array.type_shape_v2.max_out_of_line, 40, "max_out_of_line mismatch for table_with_handle_array");
        assert_eq!(table_with_handle_array.type_shape_v2.has_padding, false, "has_padding mismatch for table_with_handle_array");
        assert_eq!(table_with_handle_array.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_handle_array");
        assert_eq!(nullable_handle_array.type_shape_v2.inline_size, 32, "inline_size mismatch for nullable_handle_array");
        assert_eq!(nullable_handle_array.type_shape_v2.alignment, 4, "alignment mismatch for nullable_handle_array");
        assert_eq!(nullable_handle_array.type_shape_v2.depth, 0, "depth mismatch for nullable_handle_array");
        assert_eq!(nullable_handle_array.type_shape_v2.max_handles, 8, "max_handles mismatch for nullable_handle_array");
        assert_eq!(nullable_handle_array.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for nullable_handle_array");
        assert_eq!(nullable_handle_array.type_shape_v2.has_padding, false, "has_padding mismatch for nullable_handle_array");
        assert_eq!(nullable_handle_array.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for nullable_handle_array");
        assert_eq!(table_with_nullable_handle_array.type_shape_v2.inline_size, 16, "inline_size mismatch for table_with_nullable_handle_array");
        assert_eq!(table_with_nullable_handle_array.type_shape_v2.alignment, 8, "alignment mismatch for table_with_nullable_handle_array");
        assert_eq!(table_with_nullable_handle_array.type_shape_v2.depth, 2, "depth mismatch for table_with_nullable_handle_array");
        assert_eq!(table_with_nullable_handle_array.type_shape_v2.max_handles, 8, "max_handles mismatch for table_with_nullable_handle_array");
        assert_eq!(table_with_nullable_handle_array.type_shape_v2.max_out_of_line, 40, "max_out_of_line mismatch for table_with_nullable_handle_array");
        assert_eq!(table_with_nullable_handle_array.type_shape_v2.has_padding, false, "has_padding mismatch for table_with_nullable_handle_array");
        assert_eq!(table_with_nullable_handle_array.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_with_nullable_handle_array");
    }

    #[test]
    fn good_flexible_unions() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type UnionWithOneBool = flexible union {
    1: b bool;
};

type StructWithOptionalUnionWithOneBool = struct {
    opt_union_with_bool UnionWithOneBool:optional;
};

type UnionWithBoundedOutOfLineObject = flexible union {
    // smaller than |v| below, so will not be selected for max-out-of-line
    // calculation.
    1: b bool;

    // 1. vector<int32>:5 = 8 bytes for vector element count
    //                    + 8 bytes for data pointer
    //                    + 24 bytes out-of-line (20 bytes contents +
    //                                            4 bytes for 8-byte alignment)
    //                    = 40 bytes total
    // 1. vector<vector<int32>:5>:6 = vector of up to six of vector<int32>:5
    //                              = 8 bytes for vector element count
    //                              + 8 bytes for data pointer
    //                              + 240 bytes out-of-line (40 bytes contents * 6)
    //                              = 256 bytes total
    2: v vector<vector<int32>:5>:6;
};

type UnionWithUnboundedOutOfLineObject = flexible union {
    1: s string;
};

type UnionWithoutPayloadPadding = flexible union {
    1: a array<uint64, 7>;
};

type PaddingCheck = flexible union {
    1: three array<uint8, 3>;
    2: five array<uint8, 5>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let one_bool = root.lookup_union("example/UnionWithOneBool").expect("UnionWithOneBool not found");
        let opt_one_bool = root.lookup_struct("example/StructWithOptionalUnionWithOneBool").expect("StructWithOptionalUnionWithOneBool not found");
        let xu = root.lookup_union("example/UnionWithBoundedOutOfLineObject").expect("UnionWithBoundedOutOfLineObject not found");
        let unbounded = root.lookup_union("example/UnionWithUnboundedOutOfLineObject").expect("UnionWithUnboundedOutOfLineObject not found");
        let xu_no_payload_padding = root.lookup_union("example/UnionWithoutPayloadPadding").expect("UnionWithoutPayloadPadding not found");
        let padding_check = root.lookup_union("example/PaddingCheck").expect("PaddingCheck not found");
        assert_eq!(one_bool.type_shape_v2.inline_size, 16, "inline_size mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.alignment, 8, "alignment mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.depth, 1, "depth mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_handles, 0, "max_handles mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_padding, true, "has_padding mismatch for one_bool");
        assert_eq!(one_bool.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for one_bool");
        assert_eq!(opt_one_bool.type_shape_v2.inline_size, 16, "inline_size mismatch for opt_one_bool");
        assert_eq!(opt_one_bool.type_shape_v2.alignment, 8, "alignment mismatch for opt_one_bool");
        assert_eq!(opt_one_bool.type_shape_v2.depth, 1, "depth mismatch for opt_one_bool");
        assert_eq!(opt_one_bool.type_shape_v2.max_handles, 0, "max_handles mismatch for opt_one_bool");
        assert_eq!(opt_one_bool.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for opt_one_bool");
        assert_eq!(opt_one_bool.type_shape_v2.has_padding, true, "has_padding mismatch for opt_one_bool");
        assert_eq!(opt_one_bool.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for opt_one_bool");
        assert_eq!(xu.type_shape_v2.inline_size, 16, "inline_size mismatch for xu");
        assert_eq!(xu.type_shape_v2.alignment, 8, "alignment mismatch for xu");
        assert_eq!(xu.type_shape_v2.depth, 3, "depth mismatch for xu");
        assert_eq!(xu.type_shape_v2.max_handles, 0, "max_handles mismatch for xu");
        assert_eq!(xu.type_shape_v2.max_out_of_line, 256, "max_out_of_line mismatch for xu");
        assert_eq!(xu.type_shape_v2.has_padding, true, "has_padding mismatch for xu");
        assert_eq!(xu.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for xu");
        assert_eq!(unbounded.type_shape_v2.inline_size, 16, "inline_size mismatch for unbounded");
        assert_eq!(unbounded.type_shape_v2.alignment, 8, "alignment mismatch for unbounded");
        assert_eq!(unbounded.type_shape_v2.depth, 2, "depth mismatch for unbounded");
        assert_eq!(unbounded.type_shape_v2.max_handles, 0, "max_handles mismatch for unbounded");
        assert_eq!(unbounded.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for unbounded");
        assert_eq!(unbounded.type_shape_v2.has_padding, true, "has_padding mismatch for unbounded");
        assert_eq!(unbounded.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for unbounded");
        assert_eq!(xu_no_payload_padding.type_shape_v2.inline_size, 16, "inline_size mismatch for xu_no_payload_padding");
        assert_eq!(xu_no_payload_padding.type_shape_v2.alignment, 8, "alignment mismatch for xu_no_payload_padding");
        assert_eq!(xu_no_payload_padding.type_shape_v2.depth, 1, "depth mismatch for xu_no_payload_padding");
        assert_eq!(xu_no_payload_padding.type_shape_v2.max_handles, 0, "max_handles mismatch for xu_no_payload_padding");
        assert_eq!(xu_no_payload_padding.type_shape_v2.max_out_of_line, 56, "max_out_of_line mismatch for xu_no_payload_padding");
        assert_eq!(xu_no_payload_padding.type_shape_v2.has_padding, false, "has_padding mismatch for xu_no_payload_padding");
        assert_eq!(xu_no_payload_padding.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for xu_no_payload_padding");
        assert_eq!(padding_check.type_shape_v2.inline_size, 16, "inline_size mismatch for padding_check");
        assert_eq!(padding_check.type_shape_v2.alignment, 8, "alignment mismatch for padding_check");
        assert_eq!(padding_check.type_shape_v2.depth, 1, "depth mismatch for padding_check");
        assert_eq!(padding_check.type_shape_v2.max_handles, 0, "max_handles mismatch for padding_check");
        assert_eq!(padding_check.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for padding_check");
        assert_eq!(padding_check.type_shape_v2.has_padding, true, "has_padding mismatch for padding_check");
        assert_eq!(padding_check.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for padding_check");
    }

    #[test]
    fn good_envelope_strictness() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type StrictLeafUnion = strict union {
    1: a int64;
};

type FlexibleLeafUnion = flexible union {
    1: a int64;
};

type FlexibleUnionOfStrictUnion = flexible union {
    1: xu StrictLeafUnion;
};

type FlexibleUnionOfFlexibleUnion = flexible union {
    1: xu FlexibleLeafUnion;
};

type StrictUnionOfStrictUnion = strict union {
    1: xu StrictLeafUnion;
};

type StrictUnionOfFlexibleUnion = strict union {
    1: xu FlexibleLeafUnion;
};

type FlexibleLeafTable = table {};

type StrictUnionOfFlexibleTable = strict union {
    1: ft FlexibleLeafTable;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let strict_union = root.lookup_union("example/StrictLeafUnion").expect("StrictLeafUnion not found");
        let flexible_union = root.lookup_union("example/FlexibleLeafUnion").expect("FlexibleLeafUnion not found");
        let flexible_of_strict = root.lookup_union("example/FlexibleUnionOfStrictUnion").expect("FlexibleUnionOfStrictUnion not found");
        let flexible_of_flexible = root.lookup_union("example/FlexibleUnionOfFlexibleUnion").expect("FlexibleUnionOfFlexibleUnion not found");
        let strict_of_strict = root.lookup_union("example/StrictUnionOfStrictUnion").expect("StrictUnionOfStrictUnion not found");
        let strict_of_flexible = root.lookup_union("example/StrictUnionOfFlexibleUnion").expect("StrictUnionOfFlexibleUnion not found");
        let flexible_table = root.lookup_table("example/FlexibleLeafTable").expect("FlexibleLeafTable not found");
        let strict_union_of_flexible_table = root.lookup_union("example/StrictUnionOfFlexibleTable").expect("StrictUnionOfFlexibleTable not found");
        assert_eq!(strict_union.type_shape_v2.inline_size, 16, "inline_size mismatch for strict_union");
        assert_eq!(strict_union.type_shape_v2.alignment, 8, "alignment mismatch for strict_union");
        assert_eq!(strict_union.type_shape_v2.depth, 1, "depth mismatch for strict_union");
        assert_eq!(strict_union.type_shape_v2.max_handles, 0, "max_handles mismatch for strict_union");
        assert_eq!(strict_union.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for strict_union");
        assert_eq!(strict_union.type_shape_v2.has_padding, false, "has_padding mismatch for strict_union");
        assert_eq!(strict_union.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for strict_union");
        assert_eq!(flexible_union.type_shape_v2.inline_size, 16, "inline_size mismatch for flexible_union");
        assert_eq!(flexible_union.type_shape_v2.alignment, 8, "alignment mismatch for flexible_union");
        assert_eq!(flexible_union.type_shape_v2.depth, 1, "depth mismatch for flexible_union");
        assert_eq!(flexible_union.type_shape_v2.max_handles, 0, "max_handles mismatch for flexible_union");
        assert_eq!(flexible_union.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for flexible_union");
        assert_eq!(flexible_union.type_shape_v2.has_padding, false, "has_padding mismatch for flexible_union");
        assert_eq!(flexible_union.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for flexible_union");
        assert_eq!(flexible_of_strict.type_shape_v2.inline_size, 16, "inline_size mismatch for flexible_of_strict");
        assert_eq!(flexible_of_strict.type_shape_v2.alignment, 8, "alignment mismatch for flexible_of_strict");
        assert_eq!(flexible_of_strict.type_shape_v2.depth, 2, "depth mismatch for flexible_of_strict");
        assert_eq!(flexible_of_strict.type_shape_v2.max_handles, 0, "max_handles mismatch for flexible_of_strict");
        assert_eq!(flexible_of_strict.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for flexible_of_strict");
        assert_eq!(flexible_of_strict.type_shape_v2.has_padding, false, "has_padding mismatch for flexible_of_strict");
        assert_eq!(flexible_of_strict.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for flexible_of_strict");
        assert_eq!(flexible_of_flexible.type_shape_v2.inline_size, 16, "inline_size mismatch for flexible_of_flexible");
        assert_eq!(flexible_of_flexible.type_shape_v2.alignment, 8, "alignment mismatch for flexible_of_flexible");
        assert_eq!(flexible_of_flexible.type_shape_v2.depth, 2, "depth mismatch for flexible_of_flexible");
        assert_eq!(flexible_of_flexible.type_shape_v2.max_handles, 0, "max_handles mismatch for flexible_of_flexible");
        assert_eq!(flexible_of_flexible.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for flexible_of_flexible");
        assert_eq!(flexible_of_flexible.type_shape_v2.has_padding, false, "has_padding mismatch for flexible_of_flexible");
        assert_eq!(flexible_of_flexible.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for flexible_of_flexible");
        assert_eq!(strict_of_strict.type_shape_v2.inline_size, 16, "inline_size mismatch for strict_of_strict");
        assert_eq!(strict_of_strict.type_shape_v2.alignment, 8, "alignment mismatch for strict_of_strict");
        assert_eq!(strict_of_strict.type_shape_v2.depth, 2, "depth mismatch for strict_of_strict");
        assert_eq!(strict_of_strict.type_shape_v2.max_handles, 0, "max_handles mismatch for strict_of_strict");
        assert_eq!(strict_of_strict.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for strict_of_strict");
        assert_eq!(strict_of_strict.type_shape_v2.has_padding, false, "has_padding mismatch for strict_of_strict");
        assert_eq!(strict_of_strict.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for strict_of_strict");
        assert_eq!(strict_of_flexible.type_shape_v2.inline_size, 16, "inline_size mismatch for strict_of_flexible");
        assert_eq!(strict_of_flexible.type_shape_v2.alignment, 8, "alignment mismatch for strict_of_flexible");
        assert_eq!(strict_of_flexible.type_shape_v2.depth, 2, "depth mismatch for strict_of_flexible");
        assert_eq!(strict_of_flexible.type_shape_v2.max_handles, 0, "max_handles mismatch for strict_of_flexible");
        assert_eq!(strict_of_flexible.type_shape_v2.max_out_of_line, 24, "max_out_of_line mismatch for strict_of_flexible");
        assert_eq!(strict_of_flexible.type_shape_v2.has_padding, false, "has_padding mismatch for strict_of_flexible");
        assert_eq!(strict_of_flexible.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for strict_of_flexible");
        assert_eq!(flexible_table.type_shape_v2.inline_size, 16, "inline_size mismatch for flexible_table");
        assert_eq!(flexible_table.type_shape_v2.alignment, 8, "alignment mismatch for flexible_table");
        assert_eq!(flexible_table.type_shape_v2.depth, 1, "depth mismatch for flexible_table");
        assert_eq!(flexible_table.type_shape_v2.max_handles, 0, "max_handles mismatch for flexible_table");
        assert_eq!(flexible_table.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for flexible_table");
        assert_eq!(flexible_table.type_shape_v2.has_padding, false, "has_padding mismatch for flexible_table");
        assert_eq!(flexible_table.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for flexible_table");
        assert_eq!(strict_union_of_flexible_table.type_shape_v2.inline_size, 16, "inline_size mismatch for strict_union_of_flexible_table");
        assert_eq!(strict_union_of_flexible_table.type_shape_v2.alignment, 8, "alignment mismatch for strict_union_of_flexible_table");
        assert_eq!(strict_union_of_flexible_table.type_shape_v2.depth, 2, "depth mismatch for strict_union_of_flexible_table");
        assert_eq!(strict_union_of_flexible_table.type_shape_v2.max_handles, 0, "max_handles mismatch for strict_union_of_flexible_table");
        assert_eq!(strict_union_of_flexible_table.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for strict_union_of_flexible_table");
        assert_eq!(strict_union_of_flexible_table.type_shape_v2.has_padding, false, "has_padding mismatch for strict_union_of_flexible_table");
        assert_eq!(strict_union_of_flexible_table.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for strict_union_of_flexible_table");
    }

    #[test]
    fn good_protocols_and_request_of_protocols() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

protocol SomeProtocol {};

type UsingSomeProtocol = resource struct {
    value client_end:SomeProtocol;
};

type UsingOptSomeProtocol = resource struct {
    value client_end:<SomeProtocol, optional>;
};

type UsingRequestSomeProtocol = resource struct {
    value server_end:SomeProtocol;
};

type UsingOptRequestSomeProtocol = resource struct {
    value server_end:<SomeProtocol, optional>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let using_some_protocol = root.lookup_struct("example/UsingSomeProtocol").expect("UsingSomeProtocol not found");
        let using_opt_some_protocol = root.lookup_struct("example/UsingOptSomeProtocol").expect("UsingOptSomeProtocol not found");
        let using_request_some_protocol = root.lookup_struct("example/UsingRequestSomeProtocol").expect("UsingRequestSomeProtocol not found");
        let using_opt_request_some_protocol = root.lookup_struct("example/UsingOptRequestSomeProtocol").expect("UsingOptRequestSomeProtocol not found");
        assert_eq!(using_some_protocol.type_shape_v2.inline_size, 4, "inline_size mismatch for using_some_protocol");
        assert_eq!(using_some_protocol.type_shape_v2.alignment, 4, "alignment mismatch for using_some_protocol");
        assert_eq!(using_some_protocol.type_shape_v2.depth, 0, "depth mismatch for using_some_protocol");
        assert_eq!(using_some_protocol.type_shape_v2.max_handles, 1, "max_handles mismatch for using_some_protocol");
        assert_eq!(using_some_protocol.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for using_some_protocol");
        assert_eq!(using_some_protocol.type_shape_v2.has_padding, false, "has_padding mismatch for using_some_protocol");
        assert_eq!(using_some_protocol.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for using_some_protocol");
        assert_eq!(using_opt_some_protocol.type_shape_v2.inline_size, 4, "inline_size mismatch for using_opt_some_protocol");
        assert_eq!(using_opt_some_protocol.type_shape_v2.alignment, 4, "alignment mismatch for using_opt_some_protocol");
        assert_eq!(using_opt_some_protocol.type_shape_v2.depth, 0, "depth mismatch for using_opt_some_protocol");
        assert_eq!(using_opt_some_protocol.type_shape_v2.max_handles, 1, "max_handles mismatch for using_opt_some_protocol");
        assert_eq!(using_opt_some_protocol.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for using_opt_some_protocol");
        assert_eq!(using_opt_some_protocol.type_shape_v2.has_padding, false, "has_padding mismatch for using_opt_some_protocol");
        assert_eq!(using_opt_some_protocol.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for using_opt_some_protocol");
        assert_eq!(using_request_some_protocol.type_shape_v2.inline_size, 4, "inline_size mismatch for using_request_some_protocol");
        assert_eq!(using_request_some_protocol.type_shape_v2.alignment, 4, "alignment mismatch for using_request_some_protocol");
        assert_eq!(using_request_some_protocol.type_shape_v2.depth, 0, "depth mismatch for using_request_some_protocol");
        assert_eq!(using_request_some_protocol.type_shape_v2.max_handles, 1, "max_handles mismatch for using_request_some_protocol");
        assert_eq!(using_request_some_protocol.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for using_request_some_protocol");
        assert_eq!(using_request_some_protocol.type_shape_v2.has_padding, false, "has_padding mismatch for using_request_some_protocol");
        assert_eq!(using_request_some_protocol.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for using_request_some_protocol");
        assert_eq!(using_opt_request_some_protocol.type_shape_v2.inline_size, 4, "inline_size mismatch for using_opt_request_some_protocol");
        assert_eq!(using_opt_request_some_protocol.type_shape_v2.alignment, 4, "alignment mismatch for using_opt_request_some_protocol");
        assert_eq!(using_opt_request_some_protocol.type_shape_v2.depth, 0, "depth mismatch for using_opt_request_some_protocol");
        assert_eq!(using_opt_request_some_protocol.type_shape_v2.max_handles, 1, "max_handles mismatch for using_opt_request_some_protocol");
        assert_eq!(using_opt_request_some_protocol.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for using_opt_request_some_protocol");
        assert_eq!(using_opt_request_some_protocol.type_shape_v2.has_padding, false, "has_padding mismatch for using_opt_request_some_protocol");
        assert_eq!(using_opt_request_some_protocol.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for using_opt_request_some_protocol");
    }

    #[test]
    fn good_simple_request() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

protocol Test {
    Method(struct { a int16; b int16; });
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let protocol = root.lookup_protocol("example/Test").expect("Test not found");
    }

    #[test]
    fn good_simple_response() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

protocol Test {
    strict Method() -> (struct { a int16; b int16; });
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let protocol = root.lookup_protocol("example/Test").expect("Test not found");
    }

    #[test]
    fn good_recursive_request() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type WebMessage = resource struct {
    message_port_req server_end:MessagePort;
};

protocol MessagePort {
    PostMessage(resource struct {
        message WebMessage;
    }) -> (struct {
        success bool;
    });
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let web_message = root.lookup_struct("example/WebMessage").expect("WebMessage not found");
        let message_port = root.lookup_protocol("example/MessagePort").expect("MessagePort not found");
        assert_eq!(web_message.type_shape_v2.inline_size, 4, "inline_size mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.alignment, 4, "alignment mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.depth, 0, "depth mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.max_handles, 1, "max_handles mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.has_padding, false, "has_padding mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for web_message");
        let member_web_message_0 = web_message.members[0].clone();
        assert_eq!(member_web_message_0.field_shape_v2.offset, 0, "offset mismatch for web_message member 0");
        assert_eq!(member_web_message_0.field_shape_v2.padding, 0, "padding mismatch for web_message member 0");
    }

    #[test]
    fn good_recursive_opt_request() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type WebMessage = resource struct {
    opt_message_port_req server_end:<MessagePort, optional>;
};

protocol MessagePort {
    PostMessage(resource struct {
        message WebMessage;
    }) -> (struct {
        success bool;
    });
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let web_message = root.lookup_struct("example/WebMessage").expect("WebMessage not found");
        let message_port = root.lookup_protocol("example/MessagePort").expect("MessagePort not found");
        assert_eq!(web_message.type_shape_v2.inline_size, 4, "inline_size mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.alignment, 4, "alignment mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.depth, 0, "depth mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.max_handles, 1, "max_handles mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.has_padding, false, "has_padding mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for web_message");
    }

    #[test]
    fn good_recursive_protocol() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type WebMessage = resource struct {
    message_port client_end:MessagePort;
};

protocol MessagePort {
    PostMessage(resource struct {
        message WebMessage;
    }) -> (struct {
        success bool;
    });
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let web_message = root.lookup_struct("example/WebMessage").expect("WebMessage not found");
        let message_port = root.lookup_protocol("example/MessagePort").expect("MessagePort not found");
        assert_eq!(web_message.type_shape_v2.inline_size, 4, "inline_size mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.alignment, 4, "alignment mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.depth, 0, "depth mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.max_handles, 1, "max_handles mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.has_padding, false, "has_padding mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for web_message");
    }

    #[test]
    fn good_recursive_opt_protocol() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type WebMessage = resource struct {
    opt_message_port client_end:<MessagePort, optional>;
};

protocol MessagePort {
    PostMessage(resource struct {
        message WebMessage;
    }) -> (struct {
        success bool;
    });
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let web_message = root.lookup_struct("example/WebMessage").expect("WebMessage not found");
        let message_port = root.lookup_protocol("example/MessagePort").expect("MessagePort not found");
        assert_eq!(web_message.type_shape_v2.inline_size, 4, "inline_size mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.alignment, 4, "alignment mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.depth, 0, "depth mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.max_handles, 1, "max_handles mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.has_padding, false, "has_padding mismatch for web_message");
        assert_eq!(web_message.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for web_message");
    }

    #[test]
    fn good_recursive_struct() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type TheStruct = struct {
    opt_one_more box<TheStruct>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let the_struct = root.lookup_struct("example/TheStruct").expect("TheStruct not found");
        assert_eq!(the_struct.type_shape_v2.inline_size, 8, "inline_size mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.alignment, 8, "alignment mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.depth, u32::MAX, "depth mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.max_handles, 0, "max_handles mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.has_padding, false, "has_padding mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for the_struct");
        let member_the_struct_0 = the_struct.members[0].clone();
        assert_eq!(member_the_struct_0.field_shape_v2.offset, 0, "offset mismatch for the_struct member 0");
        assert_eq!(member_the_struct_0.field_shape_v2.padding, 0, "padding mismatch for the_struct member 0");
    }

    #[test]
    fn good_recursive_struct_with_handles() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type TheStruct = resource struct {
  some_handle zx.Handle:VMO;
  opt_one_more box<TheStruct>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let the_struct = root.lookup_struct("example/TheStruct").expect("TheStruct not found");
        assert_eq!(the_struct.type_shape_v2.inline_size, 16, "inline_size mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.alignment, 8, "alignment mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.depth, u32::MAX, "depth mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.has_padding, true, "has_padding mismatch for the_struct");
        assert_eq!(the_struct.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for the_struct");
        let member_the_struct_0 = the_struct.members[0].clone();
        assert_eq!(member_the_struct_0.field_shape_v2.offset, 0, "offset mismatch for the_struct member 0");
        assert_eq!(member_the_struct_0.field_shape_v2.padding, 4, "padding mismatch for the_struct member 0");
        let member_the_struct_1 = the_struct.members[1].clone();
        assert_eq!(member_the_struct_1.field_shape_v2.offset, 8, "offset mismatch for the_struct member 1");
        assert_eq!(member_the_struct_1.field_shape_v2.padding, 0, "padding mismatch for the_struct member 1");
    }

    #[test]
    fn good_co_recursive_struct() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type A = struct {
    foo box<B>;
};

type B = struct {
    bar box<A>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let struct_a = root.lookup_struct("example/A").expect("A not found");
        let struct_b = root.lookup_struct("example/B").expect("B not found");
        assert_eq!(struct_a.type_shape_v2.inline_size, 8, "inline_size mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.alignment, 8, "alignment mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.depth, u32::MAX, "depth mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.max_handles, 0, "max_handles mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.has_padding, false, "has_padding mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for struct_a");
        assert_eq!(struct_b.type_shape_v2.inline_size, 8, "inline_size mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.alignment, 8, "alignment mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.depth, u32::MAX, "depth mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.max_handles, 0, "max_handles mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.has_padding, false, "has_padding mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for struct_b");
    }

    #[test]
    fn good_co_recursive_struct_with_handles() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type A = resource struct {
    a zx.Handle;
    foo box<B>;
};

type B = resource struct {
    b zx.Handle;
    bar box<A>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let struct_a = root.lookup_struct("example/A").expect("A not found");
        let struct_b = root.lookup_struct("example/B").expect("B not found");
        assert_eq!(struct_a.type_shape_v2.inline_size, 16, "inline_size mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.alignment, 8, "alignment mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.depth, u32::MAX, "depth mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.has_padding, true, "has_padding mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for struct_a");
        assert_eq!(struct_b.type_shape_v2.inline_size, 16, "inline_size mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.alignment, 8, "alignment mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.depth, u32::MAX, "depth mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.has_padding, true, "has_padding mismatch for struct_b");
        assert_eq!(struct_b.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for struct_b");
    }

    #[test]
    fn good_co_recursive_struct2() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Foo = struct {
    b Bar;
};

type Bar = struct {
    f box<Foo>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let struct_foo = root.lookup_struct("example/Foo").expect("Foo not found");
        let struct_bar = root.lookup_struct("example/Bar").expect("Bar not found");
        assert_eq!(struct_foo.type_shape_v2.inline_size, 8, "inline_size mismatch for struct_foo");
        assert_eq!(struct_foo.type_shape_v2.alignment, 8, "alignment mismatch for struct_foo");
        assert_eq!(struct_foo.type_shape_v2.depth, u32::MAX, "depth mismatch for struct_foo");
        assert_eq!(struct_foo.type_shape_v2.max_handles, 0, "max_handles mismatch for struct_foo");
        assert_eq!(struct_foo.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for struct_foo");
        assert_eq!(struct_foo.type_shape_v2.has_padding, false, "has_padding mismatch for struct_foo");
        assert_eq!(struct_foo.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for struct_foo");
        assert_eq!(struct_bar.type_shape_v2.inline_size, 8, "inline_size mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.alignment, 8, "alignment mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.depth, u32::MAX, "depth mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.max_handles, 0, "max_handles mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.has_padding, false, "has_padding mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for struct_bar");
    }

    #[test]
    fn good_recursive_union_and_struct() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Foo = union {
    1: bar struct { f Foo:optional; };
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let union_foo = root.lookup_union("example/Foo").expect("Foo not found");
        let struct_bar = root.lookup_struct("example/Bar").expect("Bar not found");
        println!("UNION FOO: {:#?}", union_foo);
        println!("STRUCT BAR: {:#?}", struct_bar);
        assert_eq!(struct_bar.type_shape_v2.inline_size, 16, "inline_size mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.alignment, 8, "alignment mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.depth, u32::MAX, "depth mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.max_handles, 0, "max_handles mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.has_padding, false, "has_padding mismatch for struct_bar");
        assert_eq!(struct_bar.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for struct_bar");
    }

    #[test]
    fn good_recursive_union_and_vector() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Foo = union {
    1: bar vector<Foo:optional>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let union_foo = root.lookup_union("example/Foo").expect("Foo not found");
        assert_eq!(union_foo.type_shape_v2.inline_size, 16, "inline_size mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.alignment, 8, "alignment mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.depth, u32::MAX, "depth mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.max_handles, 0, "max_handles mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.has_padding, false, "has_padding mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for union_foo");
    }

    #[test]
    fn good_recursion_handles_bounded() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type Foo = resource union {
    1: bar array<Foo:optional, 1>;
    2: h zx.Handle;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let union_foo = root.lookup_union("example/Foo").expect("Foo not found");
        assert_eq!(union_foo.type_shape_v2.inline_size, 16, "inline_size mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.alignment, 8, "alignment mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.depth, u32::MAX, "depth mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.has_padding, false, "has_padding mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for union_foo");
    }

    #[test]
    fn good_recursion_handles_unbounded_branching() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type Foo = resource union {
    1: bar array<Foo:optional, 2>;
    2: h zx.Handle;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let union_foo = root.lookup_union("example/Foo").expect("Foo not found");
        assert_eq!(union_foo.type_shape_v2.inline_size, 16, "inline_size mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.alignment, 8, "alignment mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.depth, u32::MAX, "depth mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.has_padding, false, "has_padding mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for union_foo");
    }

    #[test]
    fn good_recursion_handles_unbounded_in_cycle() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type Foo = resource union {
    1: bar resource table {
        1: baz resource struct { f Foo:optional; };
        2: h zx.Handle;
    };
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let union_foo = root.lookup_union("example/Foo").expect("Foo not found");
        let table_bar = root.lookup_table("example/Bar").expect("Bar not found");
        let struct_baz = root.lookup_struct("example/Baz").expect("Baz not found");
        assert_eq!(union_foo.type_shape_v2.inline_size, 16, "inline_size mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.alignment, 8, "alignment mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.depth, u32::MAX, "depth mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.has_padding, false, "has_padding mismatch for union_foo");
        assert_eq!(union_foo.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for union_foo");
        assert_eq!(table_bar.type_shape_v2.inline_size, 16, "inline_size mismatch for table_bar");
        assert_eq!(table_bar.type_shape_v2.alignment, 8, "alignment mismatch for table_bar");
        assert_eq!(table_bar.type_shape_v2.depth, u32::MAX, "depth mismatch for table_bar");
        assert_eq!(table_bar.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for table_bar");
        assert_eq!(table_bar.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for table_bar");
        assert_eq!(table_bar.type_shape_v2.has_padding, false, "has_padding mismatch for table_bar");
        assert_eq!(table_bar.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for table_bar");
        assert_eq!(struct_baz.type_shape_v2.inline_size, 16, "inline_size mismatch for struct_baz");
        assert_eq!(struct_baz.type_shape_v2.alignment, 8, "alignment mismatch for struct_baz");
        assert_eq!(struct_baz.type_shape_v2.depth, u32::MAX, "depth mismatch for struct_baz");
        assert_eq!(struct_baz.type_shape_v2.max_handles, u32::MAX, "max_handles mismatch for struct_baz");
        assert_eq!(struct_baz.type_shape_v2.max_out_of_line, u32::MAX, "max_out_of_line mismatch for struct_baz");
        assert_eq!(struct_baz.type_shape_v2.has_padding, false, "has_padding mismatch for struct_baz");
        assert_eq!(struct_baz.type_shape_v2.has_flexible_envelope, true, "has_flexible_envelope mismatch for struct_baz");
    }

    #[test]
    fn good_struct_two_deep() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type DiffEntry = resource struct {
    key vector<uint8>:256;

    base box<Value>;
    left box<Value>;
    right box<Value>;
};

type Value = resource struct {
    value box<Buffer>;
    priority Priority;
};

type Buffer = resource struct {
    vmo zx.Handle:VMO;
    size uint64;
};

type Priority = enum {
    EAGER = 0;
    LAZY = 1;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let buffer = root.lookup_struct("example/Buffer").expect("Buffer not found");
        let value = root.lookup_struct("example/Value").expect("Value not found");
        let diff_entry = root.lookup_struct("example/DiffEntry").expect("DiffEntry not found");
        assert_eq!(buffer.type_shape_v2.inline_size, 16, "inline_size mismatch for buffer");
        assert_eq!(buffer.type_shape_v2.alignment, 8, "alignment mismatch for buffer");
        assert_eq!(buffer.type_shape_v2.depth, 0, "depth mismatch for buffer");
        assert_eq!(buffer.type_shape_v2.max_handles, 1, "max_handles mismatch for buffer");
        assert_eq!(buffer.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for buffer");
        assert_eq!(buffer.type_shape_v2.has_padding, true, "has_padding mismatch for buffer");
        assert_eq!(buffer.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for buffer");
        assert_eq!(value.type_shape_v2.inline_size, 16, "inline_size mismatch for value");
        assert_eq!(value.type_shape_v2.alignment, 8, "alignment mismatch for value");
        assert_eq!(value.type_shape_v2.depth, 1, "depth mismatch for value");
        assert_eq!(value.type_shape_v2.max_handles, 1, "max_handles mismatch for value");
        assert_eq!(value.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for value");
        assert_eq!(value.type_shape_v2.has_padding, true, "has_padding mismatch for value");
        assert_eq!(value.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for value");
        assert_eq!(diff_entry.type_shape_v2.inline_size, 40, "inline_size mismatch for diff_entry");
        assert_eq!(diff_entry.type_shape_v2.alignment, 8, "alignment mismatch for diff_entry");
        assert_eq!(diff_entry.type_shape_v2.depth, 2, "depth mismatch for diff_entry");
        assert_eq!(diff_entry.type_shape_v2.max_handles, 3, "max_handles mismatch for diff_entry");
        assert_eq!(diff_entry.type_shape_v2.max_out_of_line, 352, "max_out_of_line mismatch for diff_entry");
        assert_eq!(diff_entry.type_shape_v2.has_padding, true, "has_padding mismatch for diff_entry");
        assert_eq!(diff_entry.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for diff_entry");
    }

    #[test]
    fn good_union_size8_alignment4_sandwich() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type UnionSize8Alignment4 = strict union {
    1: variant uint32;
};

type Sandwich = struct {
    before uint32;
    union UnionSize8Alignment4;
    after uint32;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let sandwich = root.lookup_struct("example/Sandwich").expect("Sandwich not found");
        assert_eq!(sandwich.type_shape_v2.inline_size, 32, "inline_size mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.alignment, 8, "alignment mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.depth, 1, "depth mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.max_handles, 0, "max_handles mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.has_padding, true, "has_padding mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for sandwich");
        let member_sandwich_0 = sandwich.members[0].clone();
        assert_eq!(member_sandwich_0.field_shape_v2.offset, 0, "offset mismatch for sandwich member 0");
        assert_eq!(member_sandwich_0.field_shape_v2.padding, 4, "padding mismatch for sandwich member 0");
        let member_sandwich_1 = sandwich.members[1].clone();
        assert_eq!(member_sandwich_1.field_shape_v2.offset, 8, "offset mismatch for sandwich member 1");
        assert_eq!(member_sandwich_1.field_shape_v2.padding, 0, "padding mismatch for sandwich member 1");
        let member_sandwich_2 = sandwich.members[2].clone();
        assert_eq!(member_sandwich_2.field_shape_v2.offset, 24, "offset mismatch for sandwich member 2");
        assert_eq!(member_sandwich_2.field_shape_v2.padding, 4, "padding mismatch for sandwich member 2");
    }

    #[test]
    fn good_union_size12_alignment4_sandwich() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type UnionSize12Alignment4 = strict union {
    1: variant array<uint8, 6>;
};

type Sandwich = struct {
    before uint32;
    union UnionSize12Alignment4;
    after int32;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let sandwich = root.lookup_struct("example/Sandwich").expect("Sandwich not found");
        assert_eq!(sandwich.type_shape_v2.inline_size, 32, "inline_size mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.alignment, 8, "alignment mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.depth, 1, "depth mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.max_handles, 0, "max_handles mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.max_out_of_line, 8, "max_out_of_line mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.has_padding, true, "has_padding mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for sandwich");
        let member_sandwich_0 = sandwich.members[0].clone();
        assert_eq!(member_sandwich_0.field_shape_v2.offset, 0, "offset mismatch for sandwich member 0");
        assert_eq!(member_sandwich_0.field_shape_v2.padding, 4, "padding mismatch for sandwich member 0");
        let member_sandwich_1 = sandwich.members[1].clone();
        assert_eq!(member_sandwich_1.field_shape_v2.offset, 8, "offset mismatch for sandwich member 1");
        assert_eq!(member_sandwich_1.field_shape_v2.padding, 0, "padding mismatch for sandwich member 1");
        let member_sandwich_2 = sandwich.members[2].clone();
        assert_eq!(member_sandwich_2.field_shape_v2.offset, 24, "offset mismatch for sandwich member 2");
        assert_eq!(member_sandwich_2.field_shape_v2.padding, 4, "padding mismatch for sandwich member 2");
    }

    #[test]
    fn good_union_size24_alignment8_sandwich() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type StructSize16Alignment8 = struct {
    f1 uint64;
    f2 uint64;
};

type UnionSize24Alignment8 = strict union {
    1: variant StructSize16Alignment8;
};

type Sandwich = struct {
    before uint32;
    union UnionSize24Alignment8;
    after uint32;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let sandwich = root.lookup_struct("example/Sandwich").expect("Sandwich not found");
        assert_eq!(sandwich.type_shape_v2.inline_size, 32, "inline_size mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.alignment, 8, "alignment mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.depth, 1, "depth mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.max_handles, 0, "max_handles mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.max_out_of_line, 16, "max_out_of_line mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.has_padding, true, "has_padding mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for sandwich");
        let member_sandwich_0 = sandwich.members[0].clone();
        assert_eq!(member_sandwich_0.field_shape_v2.offset, 0, "offset mismatch for sandwich member 0");
        assert_eq!(member_sandwich_0.field_shape_v2.padding, 4, "padding mismatch for sandwich member 0");
        let member_sandwich_1 = sandwich.members[1].clone();
        assert_eq!(member_sandwich_1.field_shape_v2.offset, 8, "offset mismatch for sandwich member 1");
        assert_eq!(member_sandwich_1.field_shape_v2.padding, 0, "padding mismatch for sandwich member 1");
        let member_sandwich_2 = sandwich.members[2].clone();
        assert_eq!(member_sandwich_2.field_shape_v2.offset, 24, "offset mismatch for sandwich member 2");
        assert_eq!(member_sandwich_2.field_shape_v2.padding, 4, "padding mismatch for sandwich member 2");
    }

    #[test]
    fn good_union_size36_alignment4_sandwich() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type UnionSize36Alignment4 = strict union {
    1: variant array<uint8, 32>;
};

type Sandwich = struct {
    before uint32;
    union UnionSize36Alignment4;
    after uint32;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        let sandwich = root.lookup_struct("example/Sandwich").expect("Sandwich not found");
        assert_eq!(sandwich.type_shape_v2.inline_size, 32, "inline_size mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.alignment, 8, "alignment mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.depth, 1, "depth mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.max_handles, 0, "max_handles mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.max_out_of_line, 32, "max_out_of_line mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.has_padding, true, "has_padding mismatch for sandwich");
        assert_eq!(sandwich.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for sandwich");
        let member_sandwich_0 = sandwich.members[0].clone();
        assert_eq!(member_sandwich_0.field_shape_v2.offset, 0, "offset mismatch for sandwich member 0");
        assert_eq!(member_sandwich_0.field_shape_v2.padding, 4, "padding mismatch for sandwich member 0");
        let member_sandwich_1 = sandwich.members[1].clone();
        assert_eq!(member_sandwich_1.field_shape_v2.offset, 8, "offset mismatch for sandwich member 1");
        assert_eq!(member_sandwich_1.field_shape_v2.padding, 0, "padding mismatch for sandwich member 1");
        let member_sandwich_2 = sandwich.members[2].clone();
        assert_eq!(member_sandwich_2.field_shape_v2.offset, 24, "offset mismatch for sandwich member 2");
        assert_eq!(member_sandwich_2.field_shape_v2.padding, 4, "padding mismatch for sandwich member 2");
    }

    #[test]
    fn good_zero_size_vector() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
using zx;

type A = resource struct {
    zero_size vector<zx.Handle>:0;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let dummy_zx = SourceFile::new("zx.fidl".to_string(), "library zx; type Handle = resource struct {}; type ObjType = strict enum: uint32 { NONE = 0; CHANNEL = 4; PORT = 6; VMO = 3; };".to_string());
        lib.add_source(&dummy_zx);
        let root = lib.compile().expect("compilation failed");
        let struct_a = root.lookup_struct("example/A").expect("A not found");
        assert_eq!(struct_a.type_shape_v2.inline_size, 16, "inline_size mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.alignment, 8, "alignment mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.depth, 1, "depth mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.max_handles, 0, "max_handles mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.max_out_of_line, 0, "max_out_of_line mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.has_padding, true, "has_padding mismatch for struct_a");
        assert_eq!(struct_a.type_shape_v2.has_flexible_envelope, false, "has_flexible_envelope mismatch for struct_a");
    }

    #[test]
    #[ignore] // TODO: diagnostics checks not mostly implemented
    fn bad_integer_overflow_struct() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
type Foo = struct {
    f1 array<uint8, 2147483648>; // 2^31
    f2 array<uint8, 2147483648>; // 2^31
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore] // TODO: diagnostics checks not mostly implemented
    fn bad_inline_size_exceeds_limit() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;
type Foo = struct {
    big array<uint8, 65536>; // 2^16
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

}
