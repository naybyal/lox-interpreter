use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn tokenize(file_contents: &str) -> i32 {
    let mut result = 0;
    let mut lines = 1; 
    for character in file_contents.chars() {
        match character {
            '\n' => {
                lines += 1;
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
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", lines, character);
                result = 65;
            }
        }
    }

    println!("EOF  null");
    return result;
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
