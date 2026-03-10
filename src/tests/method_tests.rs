use crate::source_file::SourceFile;
use crate::tests::test_library::TestLibrary;

#[test]

fn good_valid_compose_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol HasComposeMethod1 {
    compose();
};

open protocol HasComposeMethod2 {
    compose() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]

fn good_valid_strict_compose_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol HasComposeMethod1 {
    strict compose();
};

open protocol HasComposeMethod2 {
    strict compose() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]

fn good_valid_flexible_compose_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol HasComposeMethod1 {
    flexible compose();
};

open protocol HasComposeMethod2 {
    flexible compose() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]

fn disabled_good_valid_strict_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol HasStrictMethod1 {
    strict();
};

open protocol HasStrictMethod2 {
    strict() -> ();
};

open protocol HasStrictMethod3 {
    strict strict();
};

open protocol HasStrictMethod4 {
    strict strict() -> ();
};

open protocol HasStrictMethod5 {
    flexible strict();
};

open protocol HasStrictMethod6 {
    flexible strict() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]

fn disabled_good_valid_flexible_two_way_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol HasFlexibleTwoWayMethod1 {
    flexible();
};

open protocol HasFlexibleTwoWayMethod2 {
    flexible() -> ();
};

open protocol HasFlexibleTwoWayMethod3 {
    strict flexible();
};

open protocol HasFlexibleTwoWayMethod4 {
    strict flexible() -> ();
};

open protocol HasFlexibleTwoWayMethod5 {
    flexible flexible();
};

open protocol HasFlexibleTwoWayMethod6 {
    flexible flexible() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_valid_normal_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol HasNormalMethod1 {
    MyMethod();
};

open protocol HasNormalMethod2 {
    MyMethod() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_valid_strict_normal_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol HasNormalMethod1 {
    strict MyMethod();
};

open protocol HasNormalMethod2 {
    strict MyMethod() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_valid_flexible_normal_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol HasNormalMethod1 {
    flexible MyMethod();
};

open protocol HasNormalMethod2 {
    flexible MyMethod() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_valid_event() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

protocol HasEvent {
    -> MyEvent();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_valid_strict_event() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

protocol HasEvent {
    strict -> MyMethod();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_valid_flexible_event() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

protocol HasEvent {
    flexible -> MyMethod();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_valid_strictness_modifiers() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

closed protocol Closed {
  strict StrictOneWay();
  strict StrictTwoWay() -> ();
  strict -> StrictEvent();
};

ajar protocol Ajar {
  strict StrictOneWay();
  flexible FlexibleOneWay();

  strict StrictTwoWay() -> ();

  strict -> StrictEvent();
  flexible -> FlexibleEvent();
};

open protocol Open {
  strict StrictOneWay();
  flexible FlexibleOneWay();

  strict StrictTwoWay() -> ();
  flexible FlexibleTwoWay() -> ();

  strict -> StrictEvent();
  flexible -> FlexibleEvent();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_invalid_strictness_flexible_event_in_closed() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

closed protocol Closed {
  flexible -> Event();
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err());
}

#[test]

fn bad_invalid_strictness_flexible_one_way_method_in_closed() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0116.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]

fn bad_invalid_strictness_flexible_two_way_method_in_closed() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

closed protocol Closed {
  flexible Method() -> ();
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err());
}

#[test]

fn bad_invalid_strictness_flexible_two_way_method_in_ajar() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0115.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]

fn bad_invalid_openness_modifier_on_method() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

protocol BadMethod {
    open Method();
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err());
}

#[test]
fn good_valid_empty_payloads() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Test {
  strict MethodA() -> ();
  flexible MethodB() -> ();
  strict MethodC() -> () error int32;
  flexible MethodD() -> () error int32;
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]

fn bad_invalid_empty_struct_payload_strict_no_error() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Test {
  strict Method() -> (struct {});
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err());
}

#[test]

fn bad_empty_struct_payload_flexible_no_error() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Test {
  flexible Method() -> (struct {});
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err());
}

#[test]

fn bad_empty_struct_payload_strict_error() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Test {
  strict Method() -> (struct {}) error int32;
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err());
}

#[test]

fn bad_empty_struct_payload_flexible_error() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Test {
  flexible Method() -> (struct {}) error int32;
};
"#
        .to_string(),
    ));
    assert!(lib.compile().is_err());
}

#[test]
fn good_absent_payload_flexible_no_error() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Test {
  flexible Method() -> ();
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_absent_payload_strict_error() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Test {
  strict Method() -> () error int32;
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_absent_payload_flexible_error() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Test {
  flexible Method() -> () error int32;
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_flexible_no_error_response_union() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Example {
    flexible Method() -> (struct {
        foo string;
    });
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}

#[test]
fn good_flexible_error_response_union() {
    let mut lib = TestLibrary::new();
    lib.add_source(SourceFile::new(
        "example.fidl".to_string(),
        r#"
library example;

open protocol Example {
    flexible Method() -> (struct {
        foo string;
    }) error uint32;
};
"#
        .to_string(),
    ));
    lib.compile().expect("compilation failed");
}
