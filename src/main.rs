use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn tokenize(file_contents: &str) -> i32 {
    let mut result = 0;
    let mut lines = 1; 
    let mut chars = file_contents.chars().peekable();

    while let Some(&character) = chars.peek() {
        match character {
            '\n' => {
                lines += 1;
                chars.next();
            },
            '*' => {
                println!("STAR {} null", character);
                chars.next();
            },
            '+' => {
                println!("PLUS {} null", character);
                chars.next();
            },
            ',' => {
                println!("COMMA {} null", character);
                chars.next();
            },
            '-' => {
                println!("MINUS {} null", character);
                chars.next();
            },
            '.' => {
                println!("DOT {} null", character);
                chars.next();
            },
            '/' => {
                println!("SLASH {} null", character);
                chars.next();
            },
            ';' => {
                println!("SEMICOLON {} null", character);
                chars.next();
            },
            '(' => {
                println!("LEFT_PAREN {} null", character);
                chars.next();
            },
            ')' => {
                println!("RIGHT_PAREN {} null", character);
                chars.next();
            },
            '{' => {
                println!("LEFT_BRACE {} null", character);
                chars.next();
            },
            '}' => {
                println!("RIGHT_BRACE {} null", character);
                chars.next();
            },
            '=' => {
                chars.next(); // consume '='
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next(); // consume second '='
                        println!("EQUAL_EQUAL == null");
                    } else {
                        println!("EQUAL = null");
                    }
                } else {
                    println!("EQUAL = null");
                }
            },
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", lines, character);
                result = 65;
                chars.next();
            }
        }
    }

    println!("EOF  null");
    result
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

            if !file_contents.is_empty() {
                let result = tokenize(&file_contents);
                exit(result);
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
