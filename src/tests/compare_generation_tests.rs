use crate::cli::Cli;
use crate::cli::run;
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
        public_deps: Vec<&'static str>,
    }

    impl TestCase {
        fn new(filename: &'static str) -> Self {
            Self {
                filename,
                experimental_flags: vec![],
                available: None,
                contains_drivers: false,
                public_deps: vec![],
            }
        }
        fn experimental(mut self, flag: &'static str) -> Self {
            self.experimental_flags.push(flag);
            self
        }
        #[allow(dead_code)]
        fn available(mut self, avail: &'static str) -> Self {
            self.available = Some(avail);
            self
        }
        fn contains_drivers(mut self) -> Self {
            self.contains_drivers = true;
            self
        }
        fn public_deps<I: IntoIterator<Item = &'static str>>(mut self, deps: I) -> Self {
            self.public_deps.extend(deps);
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
        TestCase::new("experimental_zx_c_types.test.fidl").experimental("zx_c_types"),
        TestCase::new("string_arrays.test.fidl").experimental("zx_c_types"),
        TestCase::new("overlay.test.fidl").experimental("zx_c_types"),
        TestCase::new("handles_in_types.test.fidl"),
        TestCase::new("new_type.test.fidl").experimental("allow_new_types"),
        TestCase::new("driver_handle.test.fidl").contains_drivers(),
        TestCase::new("driver_service.test.fidl").contains_drivers(),
        TestCase::new("consts.test.fidl"),
        TestCase::new("versions.test.fidl"),
        TestCase::new("handles.test.fidl")
            .public_deps(vec!["sdk-fidl/fdf/handle.fidl"])
            .contains_drivers(),
        TestCase::new("unknown_interactions.test.fidl").contains_drivers(),
    ];

    let disabled_tests = vec![];

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

        let cli = Cli {
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

        let mut source_managers = vec![vec![vdso1, vdso2, vdso3], vec![main_file]];
        if !tc.public_deps.is_empty() {
            source_managers.insert(0, tc.public_deps.iter().map(|s| s.to_string()).collect());
        }

        if let Err(e) = run(&cli, &source_managers) {
            println!("fidlcrs failed for {}:", file);
            println!("{}", e);
            return false;
        }

        let expected_path = manifest_dir.join(format!("fidlc/goldens/{}.json.golden", name));
        let expected_json_raw = fs::read_to_string(&expected_path).unwrap_or_default();
        let actual_json_raw = fs::read_to_string(&output_json).unwrap_or_default();

        let mut expected_val: serde_json::Value =
            serde_json::from_str(&expected_json_raw).unwrap_or(serde_json::Value::Null);
        let mut actual_val: serde_json::Value =
            serde_json::from_str(&actual_json_raw).unwrap_or(serde_json::Value::Null);

        fn normalize_generated(val: &mut serde_json::Value) {
            match val {
                serde_json::Value::Object(map) => {
                    if map.contains_key("filename") && map["filename"] == "generated" {
                        if let Some(line) = map.get_mut("line") {
                            *line = serde_json::Value::Number(serde_json::Number::from(0));
                        }
                    }
                    for (_, v) in map.iter_mut() {
                        normalize_generated(v);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr.iter_mut() {
                        normalize_generated(v);
                    }
                }
                _ => {}
            }
        }
        normalize_generated(&mut expected_val);
        normalize_generated(&mut actual_val);

        let expected_json = serde_json::to_string_pretty(&expected_val).unwrap();
        let actual_json = serde_json::to_string_pretty(&actual_val).unwrap();

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
