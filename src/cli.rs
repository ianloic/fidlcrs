use std::fs;
use std::process;

pub struct Arguments {
    count: usize,
    args: Vec<String>,
    index: usize,
    response_file_args: Option<Vec<String>>,
    response_file_index: usize,
}

impl Arguments {
    pub fn new(args: Vec<String>) -> Self {
        let count = args.len();
        Self {
            count,
            args,
            index: 0,
            response_file_args: None,
            response_file_index: 0,
        }
    }

    pub fn claim(&mut self) -> String {
        if let Some(ref response_args) = self.response_file_args {
            if self.response_file_index < response_args.len() {
                let arg = response_args[self.response_file_index].clone();
                self.response_file_index += 1;
                return arg;
            } else {
                self.response_file_args = None;
            }
        }

        if self.index >= self.count {
            eprintln!("Missing part of an argument");
            process::exit(1);
        }

        let arg = self.args[self.index].clone();
        self.index += 1;

        if arg.starts_with('@') && arg.len() > 1 {
            let filename = &arg[1..];
            let content = match fs::read_to_string(filename) {
                Ok(c) => c,
                Err(_e) => {
                    eprintln!("Could not open file: {}", filename);
                    process::exit(1);
                }
            };
            let parsed_args = content.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
            self.response_file_args = Some(parsed_args);
            self.response_file_index = 0;
            return self.claim();
        }

        arg
    }

    pub fn remaining(&self) -> bool {
        if let Some(ref response_args) = self.response_file_args {
            if self.response_file_index < response_args.len() {
                return true;
            }
        }
        self.index < self.count
    }
}
