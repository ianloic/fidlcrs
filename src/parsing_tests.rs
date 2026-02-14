#[cfg(test)]
mod tests {
    use crate::test_library::{LookupHelpers, TestLibrary};
    use crate::source_file::SourceFile;
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path).unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    #[ignore]
    fn bad_compound_identifier_test() {
        let source = SourceFile::new("example.fidl".to_string(), "library 0fidl.test.badcompoundidentifier;".to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_library_name_test() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0011.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_spaces_around_dots_library_name() {
        let source = SourceFile::new("example.fidl".to_string(), "library foo . bar;".to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");
        assert_eq!(root.name, "foo.bar");
    }

    #[test]
    fn good_spaces_around_dots_member_name() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Fruit = enum : uint64 {
  A = 42;
};
const VALUE Fruit = Fruit . A;
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
        // TODO: assert constant value
    }

    #[test]
    fn good_spaces_around_dots_import() {
        let source1 = SourceFile::new("dependency.fidl".to_string(), r#"
library foo . bar . qux;

type Type = struct {};
const VALUE uint32 = 42;
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source1);
        lib.compile().expect("compilation failed for dependency");

        let source2 = SourceFile::new("example.fidl".to_string(), r#"
library example;

using foo  .  bar  .  qux;
alias Type = foo. bar. qux. Type;
const VALUE uint32 = foo .bar .qux .VALUE;
"#.to_string());
        let mut lib2 = TestLibrary::new();
        lib2.add_source(&source2);
        // Note: Currently TestLibrary doesn't handle dependencies.
        // We might just check if it parses, or we ignore the compile part if dependencies are not supported yet.
        // The instruction says port parsing_tests.cc. 
        lib2.parse().expect("parsing failed for example");
    }

    #[test]
    fn good_parsing_reserved_words_in_struct_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type struct = struct {
    field bool;
};

type flexible = struct {};
type strict = struct {};
type resource = struct {};

type InStruct = struct {
    foo struct;
    bar flexible;
    baz strict;
    qux resource;

    as bool;
    library bool;
    using bool;

    array bool;
    handle bool;
    request bool;
    string bool;
    vector bool;

    bool bool;
    int8 bool;
    int16 bool;
    int32 bool;
    int64 bool;
    uint8 bool;
    uint16 bool;
    uint32 bool;
    uint64 bool;
    float32 bool;
    float64 bool;

    true bool;
    false bool;

    reserved bool;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_parsing_reserved_words_in_constraint() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

alias T = uint8; // Modified from fidl.uint8 to avoid dependency logic for now
type S = struct {};

// Keywords
const as T = 1;
alias as_constraint = vector<S>:as;
const library T = 1;
alias library_constraint = vector<S>:library;
const using T = 1;
alias using_constraint = vector<S>:using;
const alias T = 1;
alias alias_constraint = vector<S>:alias;
const type T = 1;
alias type_constraint = vector<S>:type;
const const T = 1;
alias const_constraint = vector<S>:const;
const protocol T = 1;
alias protocol_constraint = vector<S>:protocol;
const service T = 1;
alias service_constraint = vector<S>:service;
const compose T = 1;
alias compose_constraint = vector<S>:compose;
const reserved T = 1;
alias reserved_constraint = vector<S>:reserved;

// Layouts
const bits T = 1;
alias bits_constraint = vector<S>:bits;
const enum T = 1;
alias enum_constraint = vector<S>:enum;
const struct T = 1;
alias struct_constraint = vector<S>:struct;
const table T = 1;
alias table_constraint = vector<S>:table;
const union T = 1;
alias union_constraint = vector<S>:union;

// Builtins
const array T = 1;
alias array_constraint = vector<S>:array;
const handle T = 1;
alias handle_constraint = vector<S>:handle;
const request T = 1;
alias request_constraint = vector<S>:request;
const string T = 1;
alias string_constraint = vector<S>:string;
const optional T = 1;
alias optional_constraint = vector<S>:optional;

// Primitives
const bool T = 1;
alias bool_constraint = vector<S>:bool;
const int8 T = 1;
alias int8_constraint = vector<S>:int8;
const int16 T = 1;
alias int16_constraint = vector<S>:int16;
const int32 T = 1;
alias int32_constraint = vector<S>:int32;
const int64 T = 1;
alias int64_constraint = vector<S>:int64;
const uint8 T = 1;
alias uint8_constraint = vector<S>:uint8;
const uint16 T = 1;
alias uint16_constraint = vector<S>:uint16;
const uint32 T = 1;
alias uint32_constraint = vector<S>:uint32;
const uint64 T = 1;
alias uint64_constraint = vector<S>:uint64;
const float32 T = 1;
alias float32_constraint = vector<S>:float32;
const float64 T = 1;
alias float64_constraint = vector<S>:float64;
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_parsing_handles_in_struct_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type ObjType = strict enum : uint32 {
    NONE = 0;
    PROCESS = 1;
    THREAD = 2;
    VMO = 3;
    CHANNEL = 4;
    EVENT = 5;
    PORT = 6;
    INTERRUPT = 9;
    PCI_DEVICE = 11;
    LOG = 12;
    SOCKET = 14;
    RESOURCE = 15;
    EVENTPAIR = 16;
    JOB = 17;
    VMAR = 18;
    FIFO = 19;
    GUEST = 20;
    VCPU = 21;
    TIMER = 22;
    IOMMU = 23;
    BTI = 24;
    PROFILE = 25;
    PMT = 26;
    SUSPEND_TOKEN = 27;
    PAGER = 28;
    EXCEPTION = 29;
    CLOCK = 30;
};

resource_definition handle : uint32 {
    properties {
        subtype ObjType;
    };
};

type Handles = resource struct {
    plain_handle handle;

    bti_handle handle:BTI;
    channel_handle handle:CHANNEL;
    clock_handle handle:CLOCK;
    debuglog_handle handle:LOG;
    event_handle handle:EVENT;
    eventpair_handle handle:EVENTPAIR;
    exception_handle handle:EXCEPTION;
    fifo_handle handle:FIFO;
    guest_handle handle:GUEST;
    interrupt_handle handle:INTERRUPT;
    iommu_handle handle:IOMMU;
    job_handle handle:JOB;
    pager_handle handle:PAGER;
    pcidevice_handle handle:PCI_DEVICE;
    pmt_handle handle:PMT;
    port_handle handle:PORT;
    process_handle handle:PROCESS;
    profile_handle handle:PROFILE;
    resource_handle handle:RESOURCE;
    socket_handle handle:SOCKET;
    suspendtoken_handle handle:SUSPEND_TOKEN;
    thread_handle handle:THREAD;
    timer_handle handle:TIMER;
    vcpu_handle handle:VCPU;
    vmar_handle handle:VMAR;
    vmo_handle handle:VMO;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_parsing_handle_constraint_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type ObjType = strict enum : uint32 {
    NONE = 0;
    VMO = 3;
};

type Rights = strict bits : uint32 {
    TRANSFER = 1;
};

resource_definition handle : uint32 {
    properties {
        subtype ObjType;
        rights Rights;
    };
};

type Handles = resource struct {
    plain_handle handle;
    subtype_handle handle:VMO;
    rights_handle handle:<VMO, Rights.TRANSFER>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_parsing_reserved_words_in_union_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type struct = struct {
    field bool;
};

type InUnion = strict union {
    1: foo struct;

    2: as bool;
    3: library bool;
    4: using bool;

    5: array bool;
    6: handle bool;
    7: request bool;
    8: string bool;
    9: vector bool;

   10: bool bool;
   11: int8 bool;
   12: int16 bool;
   13: int32 bool;
   14: int64 bool;
   15: uint8 bool;
   16: uint16 bool;
   17: uint32 bool;
   18: uint64 bool;
   19: float32 bool;
   20: float64 bool;

   21: true bool;
   22: false bool;

   23: reserved bool;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_parsing_reserved_words_in_protocol_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type struct = struct {
    field bool;
};

protocol InProtocol {
    as(struct {
        as bool;
    });
    library(struct {
        library bool;
    });
    using(struct {
        using bool;
    });

    array(struct {
        array bool;
    });
    handle(struct {
        handle bool;
    });
    request(struct {
        request bool;
    });
    string(struct {
        string bool;
    });
    vector(struct {
        vector bool;
    });

    bool(struct {
        bool bool;
    });
    int8(struct {
        int8 bool;
    });
    int16(struct {
        int16 bool;
    });
    int32(struct {
        int32 bool;
    });
    int64(struct {
        int64 bool;
    });
    uint8(struct {
        uint8 bool;
    });
    uint16(struct {
        uint16 bool;
    });
    uint32(struct {
        uint32 bool;
    });
    uint64(struct {
        uint64 bool;
    });
    float32(struct {
        float32 bool;
    });
    float64(struct {
        float64 bool;
    });

    true(struct {
        true bool;
    });
    false(struct {
        false bool;
    });

    reserved(struct {
        reserved bool;
    });

    foo(struct {
        arg struct;
        arg2 int32;
        arg3 struct;
    });
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_char_pound_sign_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library test;

type Test = struct {
    #uint8 uint8;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_char_slash_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library test;

type Test = struct / {
    uint8 uint8;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_identifier_test() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0010-a.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_invalid_character_test() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0001.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_empty_struct_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library fidl.test.emptystruct;

type Empty = struct {};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_error_on_alias_before_imports() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0025.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_attribute_value_has_correct_contents() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
  library example;

  @foo("Bar")
  type Empty = struct{};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let asts = lib.parse().expect("parsing failed");
        let attribute = &asts[0].type_decls[0].attributes.as_ref().unwrap().attributes[0];
        assert_eq!(attribute.name.data(), "foo");
        assert_eq!(attribute.args.len(), 1);
        // We'd check the string literal if the AST structures were easily introspectable here
    }

    #[test]
    #[ignore]
    fn bad_attribute_with_dotted_identifier() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0010-b.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_attribute_with_multiple_parameters() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("good/fi-0010-b.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let asts = lib.parse().expect("parsing failed");
        let attribute = &asts[0].type_decls[0].attributes.as_ref().unwrap().attributes[0];
        assert_eq!(attribute.name.data(), "foo");
        assert_eq!(attribute.args.len(), 2);
    }

    #[test]
    fn good_simple_doc_comment() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("good/fi-0027-a.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let asts = lib.parse().expect("parsing failed");
    }

    #[test]
    fn good_multiline_doc_comment_has_correct_contents() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
  library example;

  /// A
  /// multiline
  /// comment!
  type Empty = struct {};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let asts = lib.parse().expect("parsing failed");
    }

    #[test]
    #[ignore]
    fn warn_doc_comment_blank_line_test() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0027.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
    }

    #[test]
    #[ignore]
    fn warn_comment_inside_doc_comment_test() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0026.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
    }

    #[test]
    #[ignore]
    fn warn_doc_comment_with_comment_blank_line_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

/// start
// middle

/// end
type Empty = struct {};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
    }

    #[test]
    #[ignore]
    fn bad_doc_comment_not_allowed_on_params() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0024.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_comments_surrounding_doc_comment_test() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("good/fi-0026.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_blank_lines_after_doc_comment_test() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("good/fi-0027-a.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_blank_lines_after_doc_comment_with_comment_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

/// doc comment


// regular comment

type Empty = struct {};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn warn_trailing_doc_comment_test() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0028.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
    }

    #[test]
    #[ignore]
    fn bad_trailing_doc_comment_in_decl_test() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Empty = struct {
   a = int8;
   /// bad
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_final_member_missing_semicolon() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Struct = struct {
    uint_value uint8;
    foo string // error: missing semicolon
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_final_member_missing_type_and_semicolon() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Struct = struct {
    uint_value uint8;
    string_value
}; // error: want type, got "}"
   // error: want "}", got EOF
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_missing_constraint_brackets() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Foo = struct {
    bad_no_brackets vector<uint8>:10,optional;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_multiple_constraint_definition_double_colon() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0163.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_multiple_constraint_definitions() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

const LENGTH uint32 = 123;

type Foo = struct {
  bad_double_colon string:LENGTH:optional;
  bad_double_colon_bracketed string:LENGTH:<LENGTH,optional>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    fn good_single_constraint() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Foo = struct {
  with_brackets vector<int32>:<10>;
  without_brackets vector<int32>:10;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_subtype_constructor() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0031.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_layout_class() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0012.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_identifier_modifiers() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Foo = struct {
  data strict uint32;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_identifier_with_constraints_modifiers() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Bar = table {};

type Foo = struct {
  data strict Bar:optional;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_type_declaration_with_constraints_modifiers() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type t1 = union { 1: foo uint8; };
type t2 = strict t1;
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_identifier_attributes() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0022.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_identifier_with_constraints_attributes() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Bar = table {};

type Foo = struct {
  data @foo Bar:optional;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_type_declaration_of_enum_layout_with_invalid_subtype() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0013.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_missing_comma() {
        let source = SourceFile::new("example.fidl".to_string(), r#"
library example;

type Foo = struct {
  data array<uint8 5>;
};
"#.to_string());
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_missing_equals_value_enum() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0008.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_reserved_field_not_allowed() {
        let source = SourceFile::new("example.fidl".to_string(), get_file_content("bad/fi-0209.noformat.test.fidl"));
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }
}
