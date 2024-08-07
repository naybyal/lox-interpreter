use std::env;
use std::fs;
use std::io::{self, Write};

fn tokenize(file_contents: &str) {
    let mut lines = 0;
    for character in file_contents.chars() {
        match character {
            '\n' => {
                lines++;
            },
            '*' => {
                println!("STAR {} null", character);
            },
            '+' => {
                println!("PLUS {} null", character);
            },
            ',' => {
                println!("COMMA {} null", character);
            },
            '-' => {
                println!("MINUS {} null", character);
            },
            '.' => {
                println!("DOT {} null", character);
            },
            '/' => {
                println!("SLASH {} null", character);
            },
            ';' => {
                println!("SEMICOLON {} null", character);
            },
            '(' => {
                println!("LEFT_PAREN {} null", character);
            },
            ')' => {
                println!("RIGHT_PAREN {} null", character);
            },
            '{' => {
                println!("LEFT_BRACE {} null", character);
            },
            '}' => {
                println!("RIGHT_BRACE {} null", character);
            },
            '$' => {
                println!("[Line {}] Error: Unexpected character: $")
            },
            '#' => {
                println!("[Line {}] Error: Unexpected character: #")
            },
            _ => {
                writeln!(io::stderr(), "Unknown character: {}", character).unwrap();
                return;
            }
        }
    }
}

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
            // if !file_contents.is_empty() {
            //     panic!("Scanner not implemented");
            // } else {
            //     println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            // }
            tokenize(&file_contents);
            println!("EOF  null");
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
