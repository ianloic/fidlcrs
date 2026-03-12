use clap::{ArgAction, Parser as ClapParser};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::compiler::Compiler;
use crate::experimental_flags::ExperimentalFlags;
use crate::json_generator::JsonRoot;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::raw_ast;
use crate::reporter::Reporter;
use crate::source_file::SourceFile;
use crate::token::TokenKind;
use crate::versioning_types::{Platform, Version, VersionSelection};

#[derive(ClapParser, Debug, Default)]
#[command(name = "fidlc", about = "The FIDL compiler", disable_help_flag = true)]
pub struct Cli {
    #[arg(long, value_name = "JSON_PATH")]
    pub json: Option<String>,

    #[arg(long, value_name = "PLATFORM:VERSION[,VERSION]...")]
    pub available: Vec<String>,

    #[arg(long, value_name = "PLATFORM[:VERSION]")]
    pub versioned: Option<String>,

    #[arg(long, value_name = "LIBRARY_NAME")]
    pub name: Option<String>,

    #[arg(long, value_name = "FLAG_NAME")]
    pub experimental: Vec<String>,

    #[arg(long)]
    pub werror: bool,

    #[arg(long, value_name = "[text|json]", default_value = "text", value_parser(["text", "json"]))]
    pub format: String,

    #[arg(long)]
    pub json_schema: bool,

    #[arg(long, value_name = "DEPFILE_PATH")]
    pub depfile: Option<String>,

    #[arg(
        long,
        value_name = "FIDL_FILE...",
        num_args = 1..,
        action = ArgAction::Append,
    )]
    pub files: Vec<String>,

    #[arg(long, action = ArgAction::Help, help = "Print help (see more with '--help')")]
    pub help: Option<bool>,
}

pub fn run(cli: &Cli, source_managers: &[Vec<String>]) -> Result<(), String> {
    if cli.json_schema {
        println!("{}", include_str!("../fidlc/schema.json"));
        return Ok(());
    }

    let json_path = &cli.json;
    let _warnings_as_errors = cli.werror;
    let _format = &cli.format;
    let _expected_library_name = &cli.name;
    let mut _expected_platform: Option<String> = None;
    let mut _expected_version_added: Option<String> = None;
    let mut version_selection = VersionSelection::new();
    let dep_file_path = &cli.depfile;

    if let Some(ref arg) = cli.versioned {
        let parts: Vec<&str> = arg.splitn(2, ':').collect();
        _expected_platform = Some(parts[0].to_string());
        if parts.len() > 1 {
            _expected_version_added = Some(parts[1].to_string());
        }
    }

    for arg in &cli.available {
        let parts: Vec<&str> = arg.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid syntax for --available: {}", arg));
        }
        if let Some(platform) = Platform::parse(parts[0]) {
            let mut versions = BTreeSet::new();
            for v_str in parts[1].split(',') {
                if let Some(v) = Version::parse(v_str) {
                    versions.insert(v);
                } else {
                    return Err(format!("Invalid version in --available: {}", v_str));
                }
            }
            if !version_selection.insert(platform, versions) {
                return Err(format!("Duplicate platform in --available: {}", parts[0]));
            }
        } else {
            return Err(format!("Invalid platform in --available: {}", parts[0]));
        }
    }

    if source_managers.is_empty() {
        return Err("No files provided".to_string());
    }

    let mut sm = source_managers.to_vec();
    let main_filenames = sm.pop().unwrap();
    let mut dep_filenames = Vec::new();
    for group in sm {
        dep_filenames.extend(group);
    }

    if main_filenames.is_empty() {
        return Err("No files provided".to_string());
    }

    let mut filenames = dep_filenames.clone();
    filenames.extend(main_filenames.clone());

    let mut source_files = Vec::new();
    for filename in &filenames {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("Error reading file {}: {}", filename, e));
            }
        };
        source_files.push(SourceFile::new(filename.to_string(), content));
    }

    let mut reporter = Reporter::new();
    reporter.warnings_as_errors = _warnings_as_errors;
    let mut files = Vec::new();

    for source in &source_files {
        let mut lexer = Lexer::new(source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser
            .consume_token(TokenKind::StartOfFile)
            .expect("Failed to consume StartOfFile");

        match parser.parse_file() {
            Some(file) => files.push(file),
            None => {
                return Err(format!("Failed to parse file: {}", source.filename()));
            }
        }
    }

    let mut compiler = Compiler::new(&reporter);
    compiler.version_selection = version_selection;
    let mut flags = ExperimentalFlags::new();
    for f in &cli.experimental {
        if let Ok(flag) = f.parse() {
            flags.enable_flag(flag);
        } else {
            return Err(format!("Unknown experimental flag: {}", f));
        }
    }
    compiler.experimental_flags = flags;
    let source_refs: Vec<&SourceFile> = source_files.iter().collect();
    let (dep_files, main_files) = files.split_at(dep_filenames.len());
    let json_root = match compiler.compile(main_files, dep_files, &source_refs) {
        Ok(root) => {
            reporter.print_reports();

            let mut provided_libraries = std::collections::BTreeSet::new();
            for file in dep_files {
                if let Some(decl) = &file.library_decl {
                    provided_libraries.insert(decl.path.to_string());
                }
            }

            let mut reachability: std::collections::HashMap<
                String,
                std::collections::BTreeSet<String>,
            > = std::collections::HashMap::new();
            for file in files.iter() {
                if let Some(decl) = &file.library_decl {
                    let lib_name = decl.path.to_string();
                    let entry = reachability.entry(lib_name).or_default();
                    for using_decl in &file.using_decls {
                        entry.insert(using_decl.using_path.to_string());
                    }
                }
            }

            let mut used_libraries = std::collections::BTreeSet::new();
            let mut worklist = std::collections::VecDeque::new();
            if let Some(main_decl) = main_files.first().and_then(|f| f.library_decl.as_ref()) {
                worklist.push_back(main_decl.path.to_string());
            }

            while let Some(lib) = worklist.pop_front() {
                if used_libraries.insert(lib.clone()) {
                    let _is_root = lib.split('.').count() == 1
                        && lib.split('.').next().is_some_and(|c| c == "fuchsia");

                    let mut res_path = Vec::new();
                    for component in lib.split('.') {
                        res_path.push(component);
                    }
                    if let Some(deps) = reachability.get(&lib) {
                        for dep in deps {
                            worklist.push_back(dep.clone());
                        }
                    }
                }
            }

            let mut unused_libraries = Vec::new();
            for provided in &provided_libraries {
                if provided != "zx" && !used_libraries.contains(provided) {
                    unused_libraries.push(provided.clone());
                }
            }

            if !unused_libraries.is_empty() {
                return Err(format!(
                    "Unused libraries provided via --files: {}",
                    unused_libraries.join(", ")
                ));
            }

            root
        }
        Err(e) => {
            reporter.print_reports();
            return Err(format!("Compilation failed: {}\n", e));
        }
    };

    if let Some(expected_name) = _expected_library_name
        && compiler.library_name.as_string() != *expected_name
    {
        return Err(format!(
            "Library name '{}' does not match the expected name '{}'\n",
            compiler.library_name, expected_name
        ));
    }

    if let Some(expected_platform) = _expected_platform {
        if !compiler.is_versioned_library() {
            return Err(format!(
                "Library '{}' is unversioned, but expected to be versioned under platform '{}'\n",
                compiler.library_name, expected_platform
            ));
        }
        let actual_platform = compiler.library_name.versioning_platform().to_string();
        if actual_platform != expected_platform {
            return Err(format!(
                "Library platform '{}' does not match the expected platform '{}'\n",
                actual_platform, expected_platform
            ));
        }
        if let Some(expected_version) = _expected_version_added {
            let mut found_added: Option<Version> = None;
            if let Some(decl) = &compiler.library_decl
                && let Some(attrs) = &decl.attributes
            {
                for attr in &attrs.attributes {
                    if attr.name.data() == "available" {
                        for arg in &attr.args {
                            let arg_name = arg.name.as_ref().map(|n| n.data()).unwrap_or("value");
                            if arg_name == "added" || arg_name == "value" {
                                let val_str = match &arg.value {
                                    raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                                    raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                                    _ => "".to_string(),
                                };
                                found_added = Version::parse(&val_str);
                            }
                        }
                    }
                }
            }

            match found_added {
                Some(v) => {
                    let expected_v = Version::parse(&expected_version).unwrap_or(Version::HEAD);
                    if v != expected_v {
                        return Err(format!(
                            "Library is added at version {:?}, but expected version {:?}\n",
                            v, expected_v
                        ));
                    }
                }
                None => {
                    return Err(format!(
                        "Library does not specify an added version, but expected version {}\n",
                        expected_version
                    ));
                }
            }
        }
    }

    let serialized_root = JsonRoot::from(&json_root);
    let json_string = serde_json::to_string_pretty(&serialized_root).unwrap();

    if let Some(out_path) = json_path {
        if let Some(p) = Path::new(&out_path).parent() {
            fs::create_dir_all(p).unwrap_or(());
        }
        let mut f = match fs::File::create(out_path) {
            Ok(f) => f,
            Err(_e) => {
                return Err(format!("Could not open file: {}\n", out_path));
            }
        };
        f.write_all(json_string.as_bytes()).unwrap();
        f.write_all(b"\n").unwrap();
    }

    if let Some(dep_path) = dep_file_path {
        let mut f = fs::File::create(dep_path).unwrap();
        if let Some(jp) = json_path {
            let input_files = filenames.join(" ");
            writeln!(f, "{} : {}", jp, input_files).unwrap();
        }
    }

    Ok(())
}
