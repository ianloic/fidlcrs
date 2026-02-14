#[cfg(test)]
mod tests {
    use crate::source_file::SourceFile;
    use crate::test_library::{LookupHelpers, TestLibrary};
    use std::fs;

    fn get_file_content(path: &str) -> String {
        let full_path = format!("fidlc/tests/fidl/{}", path);
        fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Failed to read file {}", full_path))
    }

    #[test]
    fn good_valid_empty_protocol() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol Empty {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let type_decl = root
            .lookup_protocol("example/Empty")
            .expect("Empty protocol not found");
        assert_eq!(type_decl.methods.len(), 0);
        assert_eq!(type_decl.openness, "open");
    }

    #[test]
    fn good_valid_empty_open_protocol() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

open protocol Empty {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let type_decl = root
            .lookup_protocol("example/Empty")
            .expect("Empty protocol not found");
        assert_eq!(type_decl.methods.len(), 0);
        assert_eq!(type_decl.openness, "open");
    }

    #[test]
    fn good_valid_empty_ajar_protocol() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

ajar protocol Empty {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let type_decl = root
            .lookup_protocol("example/Empty")
            .expect("Empty protocol not found");
        assert_eq!(type_decl.methods.len(), 0);
        assert_eq!(type_decl.openness, "ajar");
    }

    #[test]
    fn good_valid_empty_closed_protocol() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

closed protocol Empty {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let type_decl = root
            .lookup_protocol("example/Empty")
            .expect("Empty protocol not found");
        assert_eq!(type_decl.methods.len(), 0);
        assert_eq!(type_decl.openness, "closed");
    }

    #[test]
    #[ignore]
    fn bad_empty_strict_protocol() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

strict protocol Empty {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_empty_flexible_protocol() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

flexible protocol Empty {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_open_missing_protocol_token() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

open Empty {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_empty_protocol_member() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol Example {
  ;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    fn good_valid_protocol_composition() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol A {
    MethodA();
};

protocol B {
    compose A;
    MethodB();
};

protocol C {
    compose A;
    MethodC();
};

protocol D {
    compose B;
    compose C;
    MethodD();
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let protocol_a = root.lookup_protocol("example/A").expect("A not found");
        assert_eq!(protocol_a.methods.len(), 1);

        let protocol_b = root.lookup_protocol("example/B").expect("B not found");
        assert_eq!(protocol_b.methods.len(), 1); // Not all methods computed yet in fidlcrs maybe

        let protocol_d = root.lookup_protocol("example/D").expect("D not found");
        assert_eq!(protocol_d.methods.len(), 1);
        assert_eq!(protocol_d.composed_protocols.len(), 2);
    }

    #[test]
    fn good_valid_open_closed_protocol_composition() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

closed protocol Closed {};
ajar protocol Ajar {};
open protocol Open {};

closed protocol ComposeInClosed {
  compose Closed;
};

ajar protocol ComposeInAjar {
  compose Closed;
  compose Ajar;
};

open protocol ComposeInOpen {
  compose Closed;
  compose Ajar;
  compose Open;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let p1 = root.lookup_protocol("example/ComposeInClosed").unwrap();
        assert_eq!(p1.composed_protocols.len(), 1);

        let p2 = root.lookup_protocol("example/ComposeInAjar").unwrap();
        assert_eq!(p2.composed_protocols.len(), 2);

        let p3 = root.lookup_protocol("example/ComposeInOpen").unwrap();
        assert_eq!(p3.composed_protocols.len(), 3);
    }

    #[test]
    #[ignore]
    fn bad_invalid_compose_open_in_closed() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

open protocol Composed {};

closed protocol Composing {
  compose Composed;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_modifier_strict_on_compose() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol A {};

protocol B {
  strict compose A;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    fn good_attach_attributes_to_compose() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol ParentA {
    ParentMethodA();
};

protocol ParentB {
    ParentMethodB();
};

protocol Child {
    @this_is_allowed
    compose ParentA;
    /// This is also allowed.
    compose ParentB;
    ChildMethod();
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let child = root.lookup_protocol("example/Child").unwrap();
        assert_eq!(child.composed_protocols.len(), 2);
        assert_eq!(child.composed_protocols[0].maybe_attributes.len(), 1);
        assert_eq!(
            child.composed_protocols[0].maybe_attributes[0].name,
            "this_is_allowed"
        );
        assert_eq!(child.composed_protocols[1].maybe_attributes.len(), 1);
        assert_eq!(child.composed_protocols[1].maybe_attributes[0].name, "doc");
    }

    #[test]
    #[ignore]
    fn bad_duplicate_method_names() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol MyProtocol {
    MyMethod();
    MyMethod();
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_request_must_be_protocol() {
        let file_content = get_file_content("bad/fi-0157.test.fidl");
        let source = SourceFile::new("bad/fi-0157.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_request_must_be_parameterized() {
        let file_content = get_file_content("bad/fi-0168.test.fidl");
        let source = SourceFile::new("bad/fi-0168.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    fn good_typed_channels() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol MyProtocol {};

type Foo = resource struct {
    a client_end:MyProtocol;
    b client_end:<MyProtocol, optional>;
    c server_end:MyProtocol;
    d server_end:<MyProtocol, optional>;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_partial_typed_channel_constraints() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol MyProtocol {};

alias ClientEnd = client_end:MyProtocol;
alias ServerEnd = server_end:MyProtocol;

type Foo = resource struct {
    a ClientEnd;
    b ClientEnd:optional;
    c ServerEnd;
    d ServerEnd:optional;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_method_absent_payload_struct() {
        let file_content = get_file_content("good/fi-0077-a.test.fidl");
        let source = SourceFile::new("good/fi-0077-a.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_event_absent_payload_struct() {
        let file_content = get_file_content("good/fi-0077-b.test.fidl");
        let source = SourceFile::new("good/fi-0077-b.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_method_empty_payload_struct() {
        let file_content = get_file_content("bad/fi-0077-a.test.fidl");
        let source = SourceFile::new("bad/fi-0077-a.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    fn good_method_named_type_request() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyStruct = struct{
  a bool;
};

protocol MyProtocol {
    MyMethodOneWay(MyStruct);
    MyMethodTwoWay(MyStruct) -> ();
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_method_named_type_response() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyStruct = struct{
  a bool;
};

protocol MyProtocol {
  MyMethod() -> (MyStruct);
    -> OnMyEvent(MyStruct);
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_method_named_type_result_payload() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

type MyStruct = struct{
  a bool;
};

protocol MyProtocol {
  MyMethod() -> (MyStruct) error uint32;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_method_table_request() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol MyOtherProtocol {};

type MyTable = resource table {
  1: a client_end:<MyProtocol>;
};

protocol MyProtocol {
  MyMethodOneWay(table {
    1: b bool;
  });
  MyMethodTwoWay(MyTable) -> ();
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    fn good_method_union_response() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol MyOtherProtocol {};

type MyUnion = strict resource union {
  1: a client_end:<MyProtocol>;
};

protocol MyProtocol {
  MyMethod() -> (flexible union {
    1: b bool;
  });
  -> OnMyEvent(MyUnion);
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        lib.compile().expect("compilation failed");
    }

    #[test]
    #[ignore]
    fn bad_one_way_error_syntax() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol MyProtocol {
    MyOneWay() error int32;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err(), "expected compilation to fail");
    }

    #[test]
    #[ignore]
    fn bad_disallowed_request_type() {
        let file_content = get_file_content("bad/fi-0075.test.fidl");
        let source = SourceFile::new("bad/fi-0075.test.fidl".to_string(), file_content);
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_disallowed_response_type() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"library example;

protocol MyProtocol {
    MyMethod() -> (uint32);
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }
}
