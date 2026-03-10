use super::test_library::{LookupHelpers, TestLibrary};
use crate::source_file::SourceFile;

#[test]
#[ignore]
fn bad_ordinal_cannot_be_zero() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Special {
    ThisOneHashesToZero() -> (struct { i int64; });
};
"#
        ,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrGeneratedZeroValueOrdinal"
    );
}

#[test]
fn bad_clashing_ordinal_values() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

protocol Special {
    ClashOne(struct { s string; b bool; }) -> (struct { i int32; });
    ClashTwo(struct { s string; }) -> (resource struct { r zx.Handle:CHANNEL; });
};
"#
        ,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrDuplicateMethodOrdinal"
    );
}

#[test]
fn bad_clashing_ordinal_values_with_attribute() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

protocol Special {
    @selector("ClashOne")
    foo(struct { s string; b bool; }) -> (struct { i int32; });
    @selector("ClashTwo")
    bar(struct { s string; }) -> (resource struct { r zx.Handle:CHANNEL; });
};
"#
        ,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrDuplicateMethodOrdinal"
    );
}

#[test]
#[ignore]
fn bad_clashing_ordinal_bad_selector() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0081.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrDuplicateMethodOrdinal"
    );
}

#[test]
#[ignore]
fn good_attribute_resolves_clashes() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

protocol Special {
    @selector("SomethingElse")
    ClashOne(struct { s string; b bool; }) -> (struct { i int32; });
    ClashTwo(struct { s string; }) -> (resource struct { r zx.Handle:CHANNEL; });
};
"#
        ,
    );
    let _root = library.compile().expect("compilation failed");
}

#[test]
fn good_ordinal_value_is_sha256() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library a.b.c;

protocol protocol {
    selector(struct {
        s string;
        b bool;
    }) -> (struct {
        i int32;
    });
};
"#
        ,
    );
    let root = library.compile().expect("compilation failed");

    let proto = root
        .lookup_protocol("a.b.c/protocol")
        .expect("Protocol not found");
    assert_eq!(proto.methods[0].ordinal, 4257738365359108720);
}

#[test]
fn good_selector_with_full_path() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library not.important;

protocol at {
    @selector("a.b.c/protocol.selector")
    all();
};
"#
        ,
    );
    let root = library.compile().expect("compilation failed");

    let _proto = root
        .lookup_protocol("not.important/at")
        .expect("Protocol not found");
    // TODO: Need actual expected ordinal hash check here, just compiling for now!
}

#[test]
#[ignore]
fn bad_selector_value_wrong_format() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0082.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrInvalidSelectorValue"
    );
}

#[test]
#[ignore]
fn bad_selector_value_not_string() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library not.important;

protocol at {
    // should be a string
    @selector(true)
    all();
};
"#
        ,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrTypeCannotBeConvertedToType"
    );
}

#[test]
fn good_selector_value_references_const() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library not.important;

protocol at {
    @selector(SEL)
    all();
};

const SEL string = "a.b.c/protocol.selector";
"#
        ,
    );
    let _root = library.compile().expect("compilation failed");
}

#[test]
#[ignore]
fn bad_selector_value_references_nonexistent() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library not.important;

protocol at {
    @selector(nonexistent)
    all();
};
"#
        ,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrNameNotFound"
    );
}

#[test]
fn good_ordinal_value_is_first64_bits_of_sha256() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library a.b.c;

protocol protocol {
    s0();
    s1();
    s2();
    s3();
    s4();
    s5();
    s6();
    s7();
    s8();
    s9();
    s10();
    s11();
    s12();
    s13();
    s14();
    s15();
    s16();
    s17();
    s18();
    s19();
    s20();
    s21();
    s22();
    s23();
    s24();
    s25();
    s26();
    s27();
    s28();
    s29();
    s30();
    s31();
};
"#
        ,
    );
    let root = library.compile().expect("compilation failed");

    let proto = root
        .lookup_protocol("a.b.c/protocol")
        .expect("Protocol not found");
    assert_eq!(proto.methods[0].ordinal, 0x3b1625372e15f1ae);
    assert_eq!(proto.methods[1].ordinal, 0x4199e504fa71b5a4);
    assert_eq!(proto.methods[2].ordinal, 0x247ca8a890628135);
    assert_eq!(proto.methods[3].ordinal, 0x64f7c02cfffb7846);
    assert_eq!(proto.methods[4].ordinal, 0x20d3f06c598f0cc3);
    assert_eq!(proto.methods[5].ordinal, 0x1ce13806085dac7a);
    assert_eq!(proto.methods[6].ordinal, 0x09e1d4b200770def);
    assert_eq!(proto.methods[7].ordinal, 0x53df65d26411d8ee);
    assert_eq!(proto.methods[8].ordinal, 0x690c3617405590c7);
    assert_eq!(proto.methods[9].ordinal, 0x4ff9ef5fb170f550);
    assert_eq!(proto.methods[10].ordinal, 0x1542d4c21d8a6c00);
    assert_eq!(proto.methods[11].ordinal, 0x564e9e47f7418e0f);
    assert_eq!(proto.methods[12].ordinal, 0x29681e66f3506231);
    assert_eq!(proto.methods[13].ordinal, 0x5ee63b26268f7760);
    assert_eq!(proto.methods[14].ordinal, 0x256950edf00aac63);
    assert_eq!(proto.methods[15].ordinal, 0x6b21c0ff1aa02896);
    assert_eq!(proto.methods[16].ordinal, 0x5a54f3dca00089e9);
    assert_eq!(proto.methods[17].ordinal, 0x772476706fa4be0e);
    assert_eq!(proto.methods[18].ordinal, 0x294e338bf71a773b);
    assert_eq!(proto.methods[19].ordinal, 0x5a6aa228cfb68d16);
    assert_eq!(proto.methods[20].ordinal, 0x55a09c6b033f3f98);
    assert_eq!(proto.methods[21].ordinal, 0x1192d5b856d22cd8);
    assert_eq!(proto.methods[22].ordinal, 0x2e68bdea28f9ce7b);
    assert_eq!(proto.methods[23].ordinal, 0x4c8ebf26900e4451);
    assert_eq!(proto.methods[24].ordinal, 0x3df0dbe9378c4fd3);
    assert_eq!(proto.methods[25].ordinal, 0x087268657bb0cad1);
    assert_eq!(proto.methods[26].ordinal, 0x0aee6ad161a90ae1);
    assert_eq!(proto.methods[27].ordinal, 0x44e6f2282baf727a);
    assert_eq!(proto.methods[28].ordinal, 0x3e8984f57ab5830d);
    assert_eq!(proto.methods[29].ordinal, 0x696f9f73a5cabd21);
    assert_eq!(proto.methods[30].ordinal, 0x327d7b0d2389e054);
    assert_eq!(proto.methods[31].ordinal, 0x54fd307bb5bfab2d);
}

#[test]
#[ignore]
fn good_hack_to_rename_fuchsia_io_to_fuchsia_io_one_no_selector() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0083.test.fidl");
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrFuchsiaIoExplicitOrdinals"
    );
}

#[test]
fn good_hack_to_rename_fuchsia_io_to_fuchsia_io_one_has_selector() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library fuchsia.io;

protocol SomeProtocol {
    @selector("fuchsia.io1/Node.Open")
    SomeMethod();
};
"#
        ,
    );
    let _root = library.compile().expect("compilation failed");
}

#[test]
fn wrong_composed_method_does_not_get_generated_ordinal() {
    let mut library = TestLibrary::new();

    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol Node {
    SomeMethod(struct { id Id; });
};

protocol Directory {
    compose Node;
    Unlink();
};

protocol DirectoryAdmin {
    compose Directory;
};
"#
        ,
    );
    let result = library.compile();
    assert!(
        result.is_err(),
        "Expected compilation to fail with ErrNameNotFound"
    );
}
