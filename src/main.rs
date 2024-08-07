use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                panic!("Scanner not implemented");
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }

            for character in file_contents {
                match character {
                    ' ' | '\t' | '\n' => {
                        // Ignore whitespace
                    },
                    // 'a'..='z' | 'A'..='Z' => {
                    //     println!("IDENTIFIER  {}", character);
                    // },
                    // '0'..='9' => {
                    //     println!("NUMBER  {}", character);
                    // },
                    '+' | '-' | '*' | '/'  | '=' => {
                        writeln!("OPERATOR  {}", character).unwrap();
                    },
                    '(' => {
                        writeln!("LEFT_PAREN {} null", character).unwrap();
                    },
                    ')' => {
                        writeln!("RIGHT_PAREN {} null", character).unwrap();
                    },
                    _ => {
                        writeln!(io::stderr(), "Unknown character: {}", character).unwrap();
                        return;
                    }
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
