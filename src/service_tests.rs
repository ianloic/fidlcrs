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
    fn good_empty_service() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

service SomeService {};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let service = root
            .lookup_service("example/SomeService")
            .expect("service not found");
        assert_eq!(service.members.len(), 0);
    }

    #[test]
    fn good_service() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

protocol SomeProtocol1 {};
protocol SomeProtocol2 {};

service SomeService {
    some_protocol_first_first client_end:SomeProtocol1;
    some_protocol_first_second client_end:SomeProtocol1;
    some_protocol_second client_end:SomeProtocol2;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        let root = lib.compile().expect("compilation failed");

        let service = root
            .lookup_service("example/SomeService")
            .expect("service not found");
        assert_eq!(service.members.len(), 3);
        assert_eq!(service.members[0].name, "some_protocol_first_first");
        // TODO: check more properties of members when available in rust lib
    }

    #[test]
    #[ignore]
    fn bad_cannot_have_conflicting_members() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

protocol MyProtocol {};

service MyService {
    my_service_member client_end:MyProtocol;
    my_service_member client_end:MyProtocol;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_no_nullable_protocol_members() {
        let source = SourceFile::new(
            "bad/fi-0088.test.fidl".to_string(),
            get_file_content("bad/fi-0088.test.fidl"),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_only_protocol_members() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

type NotAProtocol = struct {};

service SomeService {
    not_a_protocol NotAProtocol;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_no_server_ends() {
        let source = SourceFile::new(
            "bad/fi-0112.test.fidl".to_string(),
            get_file_content("bad/fi-0112.test.fidl"),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_cannot_use_services_in_decls() {
        let source = SourceFile::new(
            "example.fidl".to_string(),
            r#"
library example;

service SomeService {};

type CannotUseService = struct {
    svc SomeService;
};
"#
            .to_string(),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }

    #[test]
    #[ignore]
    fn bad_cannot_use_more_than_one_protocol_transport_kind() {
        let source = SourceFile::new(
            "bad/fi-0113.test.fidl".to_string(),
            get_file_content("bad/fi-0113.test.fidl"),
        );
        let mut lib = TestLibrary::new();
        lib.add_source(&source);
        assert!(lib.compile().is_err());
    }
}
