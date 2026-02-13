use std::env;
use std::fs;
use std::process;
use std::io::Write;

use fidlcrs::compiler::Compiler;
use fidlcrs::lexer::Lexer;
use fidlcrs::parser::Parser;
use fidlcrs::reporter::Reporter;
use fidlcrs::source_file::SourceFile;
use fidlcrs::token::TokenKind;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.fidl> [--json <output.json>]", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let mut json_output = None;

    let mut i = 2;
    while i < args.len() {
        if args[i] == "--json" {
            if i + 1 < args.len() {
                json_output = Some(&args[i+1]);
                i += 2;
            } else {
                eprintln!("Missing argument for --json");
                process::exit(1);
            }
        } else {
            i += 1;
        }
    }

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            process::exit(1);
        }
    };

    let source = SourceFile::new(filename.to_string(), content);
    let reporter = Reporter::new();
    let mut lexer = Lexer::new(&source, &reporter);
    let mut parser = Parser::new(&mut lexer, &reporter);

    // Prime the parser
    parser.consume_token(TokenKind::StartOfFile).expect("Failed to consume StartOfFile");

    // println!("Parsing {}...", filename);
    match parser.parse_file() {
        Some(file) => {
            // println!("Successfully parsed file!");
            let mut compiler = Compiler::new();
            let json_root = compiler.compile(file, &source);

            let json_string = serde_json::to_string_pretty(&json_root).unwrap();

            if let Some(out_path) = json_output {
                let mut f = fs::File::create(out_path).unwrap();
                f.write_all(json_string.as_bytes()).unwrap();
                println!("Wrote JSON to {}", out_path);
            } else {
                println!("{}", json_string);
            }
        }
        None => {
            eprintln!("Failed to parse file.");
            process::exit(1);
        }
    }
}
