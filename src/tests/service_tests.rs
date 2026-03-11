use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
fn good_empty_service() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

service SomeService {};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let service = root
        .lookup_service("example/SomeService")
        .expect("service not found");
    assert_eq!(service.members.len(), 0);
}

#[test]
fn good_service() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

protocol SomeProtocol1 {};
protocol SomeProtocol2 {};

service SomeService {
    some_protocol_first_first client_end:SomeProtocol1;
    some_protocol_first_second client_end:SomeProtocol1;
    some_protocol_second client_end:SomeProtocol2;
};
"#,
    );
    let root = lib.compile().expect("compilation failed");

    let service = root
        .lookup_service("example/SomeService")
        .expect("service not found");
    assert_eq!(service.members.len(), 3);
    assert_eq!(service.members[0].name, "some_protocol_first_first");
    // TODO: check more properties of members when available in rust lib
}

#[test]
fn bad_cannot_have_conflicting_members() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

protocol MyProtocol {};

service MyService {
    my_service_member client_end:MyProtocol;
    my_service_member client_end:MyProtocol;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_no_nullable_protocol_members() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0088.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_only_protocol_members() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

type NotAProtocol = struct {};

service SomeService {
    not_a_protocol NotAProtocol;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_no_server_ends() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0112.test.fidl");
    assert!(lib.compile().is_err());
}

#[test]
fn bad_cannot_use_services_in_decls() {
    let mut lib = TestLibrary::new();
    lib.add_source_file(
        "example.fidl",
        r#"
library example;

service SomeService {};

type CannotUseService = struct {
    svc SomeService;
};
"#,
    );
    assert!(lib.compile().is_err());
}

#[test]
fn bad_cannot_use_more_than_one_protocol_transport_kind() {
    let mut lib = TestLibrary::new();
    lib.add_errcat_file("bad/fi-0113.test.fidl");
    assert!(lib.compile().is_err());
}
