use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_compare_generation() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let goldens_dir = manifest_dir.join("goldens");
    fs::create_dir_all(&goldens_dir).unwrap();

    // Removed cargo build since we call the run function directly
    struct TestCase {
        filename: &'static str,
        experimental_flags: Vec<&'static str>,
        available: Option<&'static str>,
        contains_drivers: bool,
    }

    impl TestCase {
        fn new(filename: &'static str) -> Self {
            Self {
                filename,
                experimental_flags: vec![],
                available: None,
                contains_drivers: false,
            }
        }
        fn experimental(mut self, flag: &'static str) -> Self {
            self.experimental_flags.push(flag);
            self
        }
        fn available(mut self, avail: &'static str) -> Self {
            self.available = Some(avail);
            self
        }
        fn contains_drivers(mut self) -> Self {
            self.contains_drivers = true;
            self
        }
    }

    let active_tests = vec![
        TestCase::new("experimental_maybe_from_alias.test.fidl"),
        TestCase::new("anonymous.test.fidl"),
        TestCase::new("arrays.test.fidl"),
        TestCase::new("byte_and_bytes.test.fidl"),
        TestCase::new("bits.test.fidl"),
        TestCase::new("doc_comments.test.fidl"),
        TestCase::new("empty_struct.test.fidl"),
        TestCase::new("encapsulated_structs.test.fidl"),
        TestCase::new("enum.test.fidl"),
        TestCase::new("error.test.fidl"),
        TestCase::new("escaping.test.fidl"),
        TestCase::new("nullable.test.fidl"),
        TestCase::new("padding.test.fidl"),
        TestCase::new("protocol_request.test.fidl"),
        TestCase::new("protocols.test.fidl"),
        TestCase::new("request_flexible_envelope.test.fidl"),
        TestCase::new("serializable.test.fidl"),
        TestCase::new("service.test.fidl"),
        TestCase::new("struct.test.fidl"),
        TestCase::new("table.test.fidl"),
        TestCase::new("union_sandwich.test.fidl"),
        TestCase::new("union.test.fidl"),
        TestCase::new("vectors.test.fidl"),
        TestCase::new("driver_one_way.test.fidl").contains_drivers(),
        TestCase::new("driver_two_way.test.fidl").contains_drivers(),
        TestCase::new("inheritance.test.fidl"),
        TestCase::new("inheritance_with_recursive_decl.test.fidl"),
        TestCase::new("time.test.fidl"),
        TestCase::new("bits_constants.test.fidl"),
        TestCase::new("constants.test.fidl"),
        TestCase::new("types_in_protocols.test.fidl"),
    ];

    let disabled_tests = vec![
        // TestCase::new("consts.test.fidl"),
        TestCase::new("driver_handle.test.fidl").contains_drivers(),
        TestCase::new("driver_service.test.fidl").contains_drivers(),
        TestCase::new("experimental_zx_c_types.test.fidl").experimental("zx_c_types"),
        TestCase::new("handles_in_types.test.fidl"),
        // TODO(ianloic): Add this back when we support public deps
        // TestCase::new("handles.test.fidl")
        //     .public_dep("//sdk/fidl/fdf")
        //     .contains_drivers(),
        TestCase::new("new_type.test.fidl").experimental("allow_new_types"),
        TestCase::new("overlay.test.fidl").experimental("zx_c_types"),
        TestCase::new("string_arrays.test.fidl").experimental("zx_c_types"),
        TestCase::new("unknown_interactions.test.fidl").contains_drivers(),
        TestCase::new("versions.test.fidl").available("test:HEAD"),
    ];

    let mut failed = false;

    let check_test = |tc: &TestCase, expect_match: bool| -> bool {
        let file = tc.filename;
        let name = file.strip_suffix(".test.fidl").unwrap();

        let output_json = goldens_dir.join(format!("{}.json", name));

        let mut experimental = vec!["output_index_json".to_string()];
        for flag in &tc.experimental_flags {
            experimental.push(flag.to_string());
        }

        let mut available_args = Vec::new();
        if let Some(av) = tc.available {
            available_args.push(av.to_string());
        } else {
            available_args.push("fuchsia:HEAD".to_string());
            available_args.push("test:HEAD".to_string());
        }

        let vdso1 = "vdso-fidl/rights.fidl".to_string();
        let vdso2 = "vdso-fidl/zx_common.fidl".to_string();
        let vdso3 = "vdso-fidl/overview.fidl".to_string();
        let main_file = format!("fidlc/testdata/{}", file);

        let cli = crate::cli::Cli {
            json: Some(output_json.to_string_lossy().to_string()),
            available: available_args,
            experimental,
            files: vec![
                vdso1.clone(),
                vdso2.clone(),
                vdso3.clone(),
                main_file.clone(),
            ],
            format: "text".to_string(),
            ..Default::default()
        };

        let source_managers = vec![vec![vdso1, vdso2, vdso3], vec![main_file]];

        if let Err(e) = crate::cli::run(&cli, &source_managers) {
            println!("fidlcrs failed for {}:", file);
            println!("{}", e);
            return false;
        }

        let expected_path = manifest_dir.join(format!("fidlc/goldens/{}.json.golden", name));
        let expected_json = fs::read_to_string(&expected_path).unwrap_or_default();
        let actual_json = fs::read_to_string(&output_json).unwrap_or_default();

        if expect_match {
            if expected_json != actual_json {
                println!("Golden mismatch for {}", file);
                let diff_output = Command::new("diff")
                    .arg("-u")
                    .arg(&expected_path)
                    .arg(&output_json)
                    .output()
                    .unwrap();
                println!("{}", String::from_utf8_lossy(&diff_output.stdout));
                return false;
            }
        } else {
            if expected_json == actual_json {
                println!(
                    "Error: Disabled test {} matches golden output. It should be moved from disabled_tests to active_tests.",
                    file
                );
                return false;
            }
        }
        true
    };

    for tc in active_tests {
        if !check_test(&tc, true) {
            failed = true;
        }
    }

    for tc in disabled_tests {
        if !check_test(&tc, false) {
            failed = true;
        }
    }

    assert!(
        !failed,
        "One or more golden tests failed. See output above."
    );
}
