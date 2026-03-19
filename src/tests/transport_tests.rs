#![allow(unused_mut, unused_variables, unused_imports, dead_code)]
use crate::diagnostics::Error;
use crate::experimental_flags::ExperimentalFlag;
use crate::tests::test_library::{LookupHelpers, TestLibrary};

#[test]
#[ignore]
fn good_channel_transport_with_channel_transport_end() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0167.test.fidl");

    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_driver_transport_with_driver_transport_end() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

@transport("Driver")
protocol P {
  M(resource struct{
     c client_end:P;
  }) . (resource struct{
     s server_end:P;
  });
};
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_driver_transport_with_channel_transport_end() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

protocol ChannelProtocol {};

@transport("Driver")
protocol P {
  M(resource struct{
     c client_end:ChannelProtocol;
  }) . (resource struct{
     s server_end:ChannelProtocol;
  });
};
"#,
    );
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_driver_transport_with_zircon_handle() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

@transport("Driver")
protocol P {
  M() . (resource struct{
     h zx.handle;
  });
};
"#,
    );
    library.use_library_zx();
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_syscall_transport_with_zircon_handle() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

@transport("Syscall")
protocol P {
  M() . (resource struct{
     h zx.handle;
  });
};
"#,
    );
    library.use_library_zx();
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_banjo_transport_with_zircon_handle() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

using zx;

@transport("Banjo")
protocol P {
  M() . (resource struct{
     h zx.handle;
  });
};
"#,
    );
    library.use_library_zx();
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn good_driver_transport_with_driver_handle() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

using fdf;

@transport("Driver")
protocol P {
  M() . (resource struct{
     h fdf.handle;
  });
};
"#,
    );
    library.use_library_fdf();
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_channel_transport_with_driver_handle() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

using fdf;

protocol P {
  M() . (resource struct{
     h fdf.handle;
  });
};
"#,
    );
    library.use_library_fdf();
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_channel_transport_with_driver_client_end_request() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

@transport("Driver")
protocol DriverProtocol {};

protocol P {
  M(resource struct{
     c array<vector<box<resource struct{s client_end:DriverProtocol;}>>, 3>;
  });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_channel_transport_with_driver_server_end_response() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

@transport("Driver")
protocol DriverProtocol {};

protocol P {
  M() . (resource table{
     1: s resource union{
       1: s server_end:DriverProtocol;
     };
  });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_banjo_transport_with_driver_client_end_request() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

@transport("Driver")
protocol DriverProtocol {};

@transport("Banjo")
protocol P {
  M(resource struct{
     s client_end:DriverProtocol;
  });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_driver_transport_with_banjo_client_end_request() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

@transport("Banjo")
protocol BanjoProtocol {};

@transport("Driver")
protocol P {
  M(resource struct{
     s client_end:BanjoProtocol;
  });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_syscall_transport_with_driver_client_end_request() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0118.test.fidl");
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_syscall_transport_with_syscall_client_end_request() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

@transport("Syscall")
protocol SyscallProtocol {};

@transport("Syscall")
protocol P {
  M(resource struct{
     s client_end:SyscallProtocol;
  });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_custom_handle_in_zircon_channel() {
    let mut library = TestLibrary::new();
    library.add_source_file(
        "example.fidl",
        r#"
library example;

type ObjType = strict enum : uint32 {
  NONE = 0;
};
type Rights = strict enum : uint32 {
  SAME_RIGHTS = 0;
};

resource_definition handle : uint32 {
    properties {
        subtype ObjType;
        rights Rights;
    };
};

protocol P {
  M(resource struct{
     h handle;
  });
};
"#,
    );
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_driver_handle_in_zircon_channel() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0117.test.fidl");
    library.use_library_fdf();
    // expect_fail
    assert!(library.check_compile());
}

#[test]
#[ignore]
fn bad_cannot_reassign_transport() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("bad/fi-0167.test.fidl");

    // expect_fail
    // expect_fail
    assert!(library.check_compile());
}
