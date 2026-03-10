use super::test_library::TestLibrary;
use crate::source_file::SourceFile;

#[test]
#[ignore]
fn bad_unexpected_token() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0007.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_at_end_of_file() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Enum = enum {
    ONE;          // First error
};

type Bits = bits {
    CONSTANT = ;  // Second error
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_at_end_of_decl() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Enum = enum {
    VARIANT = 0;
    MISSING_EQUALS 5;
};

type Union = union {
    1: string_value string;
    2 missing_colon uint16;
};

type Struct = struct {
    value string;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_at_end_of_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type SettingType = enum {
    UNKNOWN = 0;
    TIME_ZONE = 1;
    CONNECTIVITY 2;                    // Error: missing equals
};

type SettingData = union {
    1: string_value string;
    2 time_zone_value ConnectedState;  // Error: missing colon
    /// Unattached doc comment.        // erroneous doc comment is skipped during recovery
};

type LoginOverride = {                 // Error: missing keyword
    NONE = 0;
    AUTH.PROVIDER = 2,                 // Error: '.' in identifier
};

type AccountSettings = table {
    1: mo.de LoginOverride;            // Error: '.' in identifier
    3: setting OtherSetting;
};

type TimeZoneInfo = struct {
    current TimeZone:optional;
    available vector<<TimeZone>;       // Error: extra <
};

type TimeZone = struct {
    id string;
    name string;
    region vector<string>;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_do_not_compile_after_parsing_fails() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

const compound.identifier uint8 = 0;  // Syntax error

type NameCollision = struct {};
type NameCollision = struct {};       // This name collision error will not be
                                      // reported, because if parsing fails
                                      // compilation is skipped
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_to_next_bits_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Bits = bits {
    ONE 0x1;      // First error
    TWO = 0x2;
    FOUR = 0x4    // Second error
    EIGHT = 0x8;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_to_next_enum_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Enum = enum {
    ONE 1;      // First error
    TWO = 2;
    THREE = 3   // Second error
    FOUR = 4;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn bad_recover_to_next_protocol_member() {
    let library = TestLibrary::new();
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recoverable_param_list_parsing() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

protocol Example {
  Method(/// Doc comment
      struct { b bool; }) -> (/// Doc comment
      struct { b bool; });
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recoverable_unmatched_delimiter_in_param_list() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

protocol Example {
  Method() -> (vector<);
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_to_next_service_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

protocol P {};
protocol Q {};
protocol R {};

service Service {
  p P extra_token; // First error
  q Q              // Second error
  r R;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_to_next_struct_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Struct = struct {
    string_value string extra_token; // Error
    uint_value uint8;
    vector_value vector<handle>      // Error
    int_value int32;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_to_next_table_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Table = table {
    1: string_value string              // Error
    2: uint_value uint8;
    3: value_with space vector<handle>; // Error
    4: int_value int32;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_to_next_union_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Union = union {
    1 missing_colon string;     // First error
    3: uint_value uint8;
    4: missing_semicolon string // Second error
    5: int_value int16;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_recover_final_member_missing_semicolon() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

type Struct = struct {
    uint_value uint8;
    foo string // First error
};

// Recovered back to top-level parsing.
type Good = struct {};

extra_token // Second error
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn bad_recover_final_member_missing_name_and_semicolon() {
    let library = TestLibrary::new();
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn bad_constraints_recoverability() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;
type TypeDecl = struct {
    // errors[0]: no constraints specified
    f0 vector<uint16>:;
    // errors[1]: no constraints specified
    f1 vector<uint16>:<>;
    // errors[2]: leading comma
    f2 vector<uint16>:<,16,optional>;
    // errors[3]: trailing comma
    f3 vector<uint16>:<16,optional,>;
    // errors[4]: double comma
    f4 vector<uint16>:<16,,optional>;
    // errors[5]: missing comma, errors[6], errors[7]: consume > and ; trying
    // to get to next member
    f5 vector<uint16>:<16 optional>;
    // errors[8] missing close bracket
    f7 vector<uint16>:<16;
    // errors[10]: invalid constant
    f8 vector<uint16>:1~6,optional;
    // errors[11]: unexpected token
    f9 vector<uint16>:,16,,optional,;
};
"#
        .to_string(),
    ));
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn unexpected_line_break_in_literal() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0002.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn unexpected_control_character() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0184.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn invalid_escape_sequence_in_literal() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0003.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn invalid_hex_digit() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0004.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn unicode_escape_missing_braces() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0185.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn unicode_escape_unterminated() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0186.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn unicode_escape_empty() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0187.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn unicode_escape_too_long() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0188.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
#[ignore]
fn unicode_escape_too_large() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0189.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}

#[test]
fn expected_declaration() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0006.noformat.test.fidl");
    let result = library.compile();
    assert!(result.is_err(), "Expected compilation to fail");
}
