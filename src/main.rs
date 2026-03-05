use clap::{CommandFactory, FromArgMatches};
use std::env;
use std::fs;
use std::process;

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

    let cmd = fidlcrs::cli::Cli::command();
    let matches = cmd.get_matches_from(expanded_args);
    let cli = fidlcrs::cli::Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

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
        eprintln!("No files provided");
        let mut help_cmd = fidlcrs::cli::Cli::command();
        help_cmd.print_help().unwrap();
        process::exit(1);
    }

    if let Err(e) = fidlcrs::cli::run(&cli, &source_managers) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
