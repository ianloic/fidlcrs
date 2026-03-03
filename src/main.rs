use clap::{ArgAction, CommandFactory, FromArgMatches, Parser as ClapParser};
use std::env;
use std::fs;
use std::io::Write;
use std::process;

use fidlcrs::compiler::Compiler;
use fidlcrs::lexer::Lexer;
use fidlcrs::parser::Parser;
use fidlcrs::reporter::Reporter;
use fidlcrs::source_file::SourceFile;
use fidlcrs::token::TokenKind;
use fidlcrs::versioning_types::{Platform, Version, VersionSelection};
use std::collections::BTreeSet;

#[derive(ClapParser, Debug)]
#[command(name = "fidlc", about = "The FIDL compiler", disable_help_flag = true)]
struct Cli {
    #[arg(long, value_name = "JSON_PATH")]
    json: Option<String>,

    #[arg(long, value_name = "PLATFORM:VERSION[,VERSION]...")]
    available: Vec<String>,

    #[arg(long, value_name = "PLATFORM[:VERSION]")]
    versioned: Option<String>,

    #[arg(long, value_name = "LIBRARY_NAME")]
    name: Option<String>,

    #[arg(long, value_name = "FLAG_NAME")]
    experimental: Vec<String>,

    #[arg(long)]
    werror: bool,

    #[arg(long, value_name = "[text|json]", default_value = "text", value_parser(["text", "json"]))]
    format: String,

    #[arg(long)]
    json_schema: bool,

    #[arg(long, value_name = "DEPFILE_PATH")]
    depfile: Option<String>,

    #[arg(
        long,
        value_name = "FIDL_FILE...",
        num_args = 1..,
        action = ArgAction::Append,
    )]
    files: Vec<String>,

    #[arg(long, action = ArgAction::Help, help = "Print help (see more with '--help')")]
    help: Option<bool>,
}

fn fail_with_usage(msg: &str) -> ! {
    eprintln!("{}", msg);
    let mut cmd = Cli::command();
    cmd.print_help().unwrap();
    process::exit(1);
}

fn fail(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let mut expanded_args = Vec::new();

    for arg in raw_args {
        if arg.starts_with('@') && arg.len() > 1 {
            let filename = &arg[1..];
            let content = match fs::read_to_string(filename) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Could not open file: {}: {}", filename, e);
                    process::exit(1);
                }
            };
            let parsed_args = content
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            expanded_args.extend(parsed_args);
        } else {
            expanded_args.push(arg);
        }
    }

    let cmd = Cli::command();
    let matches = cmd.get_matches_from(expanded_args);
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    if cli.json_schema {
        println!("{}", include_str!("../fidlc/schema.json"));
        process::exit(0);
    }

    let json_path = cli.json;
    let _warnings_as_errors = cli.werror;
    let _format = cli.format;
    let _expected_library_name = cli.name;
    let mut _expected_platform: Option<String> = None;
    let mut _expected_version_added: Option<String> = None;
    let mut version_selection = VersionSelection::new();
    let dep_file_path = cli.depfile;

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
            fail_with_usage(&format!("Invalid syntax for --available: {}", arg));
        }
        if let Some(platform) = Platform::parse(parts[0]) {
            let mut versions = BTreeSet::new();
            for v_str in parts[1].split(',') {
                if let Some(v) = Version::parse(v_str) {
                    versions.insert(v);
                } else {
                    fail_with_usage(&format!("Invalid version in --available: {}", v_str));
                }
            }
            if !version_selection.insert(platform, versions) {
                fail_with_usage(&format!("Duplicate platform in --available: {}", parts[0]));
            }
        } else {
            fail_with_usage(&format!("Invalid platform in --available: {}", parts[0]));
        }
    }

    let mut source_managers: Vec<Vec<String>> = Vec::new();

    if let Some(files_vals) = matches.get_many::<String>("files") {
        let vals: Vec<&String> = files_vals.collect();
        let indices: Vec<usize> = matches.indices_of("files").unwrap_or_default().collect();

        let mut current_chunk: Vec<String> = Vec::new();
        let mut last_index: Option<usize> = None;

        for (val, idx) in vals.into_iter().zip(indices.into_iter()) {
            if let Some(last) = last_index {
                // If the gap is > 1, it might be a new group... UNLESS we're seeing an interleaved argument.
                // However, contiguous files parsed as a chunk will have contiguous indices.
                if idx > last + 1 {
                    source_managers.push(current_chunk);
                    current_chunk = Vec::new();
                }
            }
            current_chunk.push(val.clone());
            last_index = Some(idx);
        }
        if !current_chunk.is_empty() {
            source_managers.push(current_chunk);
        }
    } else {
        fail_with_usage("No files provided");
    }

    // Now, run the fidlcrs compiler. We'll only use the last `source_managers` chunk
    // as the main files, and for now we'll only process the first file in it, like the old code did.
    // The previous implementation was:
    // let source = SourceFile::new(filename, content);
    // parser.parse_file(); ... JSON output.

    if source_managers.is_empty() {
        fail_with_usage("No files provided");
    }

    let main_filenames = source_managers.pop().unwrap();
    let mut dep_filenames = Vec::new();
    for group in source_managers {
        dep_filenames.extend(group);
    }

    if main_filenames.is_empty() {
        fail_with_usage("No files provided");
    }

    let mut filenames = dep_filenames.clone();
    filenames.extend(main_filenames.clone());

    let mut source_files = Vec::new();
    for filename in &filenames {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file {}: {}", filename, e);
                process::exit(1);
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
                eprintln!("Failed to parse file: {}", source.filename());
                process::exit(1);
            }
        }
    }

    let mut compiler = Compiler::new(&reporter);
    compiler.version_selection = version_selection;
    compiler.experimental_flags = cli.experimental;
    let source_refs: Vec<&SourceFile> = source_files.iter().collect();
    let (dep_files, main_files) = files.split_at(dep_filenames.len());
    let json_root = match compiler.compile(main_files, dep_files, &source_refs) {
        Ok(root) => {
            reporter.print_reports();
            root
        }
        Err(e) => {
            reporter.print_reports();
            fail(&format!("Compilation failed: {}\n", e));
        }
    };

    if let Some(expected_name) = _expected_library_name {
        if compiler.library_name != expected_name {
            fail(&format!(
                "Library name '{}' does not match the expected name '{}'\n",
                compiler.library_name, expected_name
            ));
        }
    }

    if let Some(expected_platform) = _expected_platform {
        if !compiler.is_versioned_library() {
            fail(&format!(
                "Library '{}' is unversioned, but expected to be versioned under platform '{}'\n",
                compiler.library_name, expected_platform
            ));
        }
        let actual_platform = compiler
            .library_name
            .split('.')
            .next()
            .unwrap_or(&compiler.library_name)
            .to_string();
        if actual_platform != expected_platform {
            fail(&format!(
                "Library platform '{}' does not match the expected platform '{}'\n",
                actual_platform, expected_platform
            ));
        }
        if let Some(expected_version) = _expected_version_added {
            let mut found_added: Option<Version> = None;
            if let Some(decl) = &compiler.library_decl {
                if let Some(attrs) = &decl.attributes {
                    for attr in &attrs.attributes {
                        if attr.name.data() == "available" {
                            for arg in &attr.args {
                                let arg_name = arg.name.as_ref().map(|n| n.data()).unwrap_or("value");
                                if arg_name == "added" || arg_name == "value" {
                                    let val_str = match &arg.value {
                                        fidlcrs::raw_ast::Constant::Literal(lit) => lit.literal.value.clone(),
                                        fidlcrs::raw_ast::Constant::Identifier(id) => id.identifier.to_string(),
                                        _ => "".to_string(),
                                    };
                                    found_added = Version::parse(&val_str);
                                }
                            }
                        }
                    }
                }
            }

            match found_added {
                Some(v) => {
                    let expected_v = Version::parse(&expected_version).unwrap_or(Version::HEAD);
                    if v != expected_v {
                        fail(&format!(
                            "Library is added at version {:?}, but expected version {:?}\n",
                            v, expected_v
                        ));
                    }
                }
                None => {
                    fail(&format!(
                        "Library does not specify an added version, but expected version {}\n",
                        expected_version
                    ));
                }
            }
        }
    }

    let json_string = serde_json::to_string_pretty(&json_root).unwrap();

    if let Some(ref out_path) = json_path {
        if let Some(p) = std::path::Path::new(&out_path).parent() {
            fs::create_dir_all(p).unwrap_or(());
        }
        let mut f = match fs::File::create(out_path) {
            Ok(f) => f,
            Err(_e) => {
                fail(&format!("Could not open file: {}\n", out_path));
            }
        };
        f.write_all(json_string.as_bytes()).unwrap();
        f.write_all(b"\n").unwrap();
    }

    if let Some(dep_path) = dep_file_path {
        let mut f = fs::File::create(&dep_path).unwrap();
        if let Some(ref jp) = json_path {
            let input_files = filenames.join(" ");
            writeln!(f, "{} : {}", jp, input_files).unwrap();
        }
    }
}
