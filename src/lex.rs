pub fn tokenize(file_contents: &str) -> i32 {
    let mut result = 0;
    let mut lines = 1;
    let mut chars = file_contents.chars().peekable();

    while let Some(&character) = chars.peek() {
        match character {
            '\n' => {
                lines += 1;
                chars.next();
            },
            ' ' | '\r' | '\t' => {
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
                chars.next();
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next();
                        println!("EQUAL_EQUAL == null");
                    } else {
                        println!("EQUAL = null");
                    }
                } else {
                    println!("EQUAL = null");
                }
            },
            '!' => {
                chars.next();
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next();
                        println!("BANG_EQUAL != null");
                    } else {
                        println!("BANG ! null");
                    }
                } else {
                    println!("BANG ! null");
                }
            },
            '<' => {
                chars.next();
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next();
                        println!("LESS_EQUAL <= null");
                    } else {
                        println!("LESS < null");
                    }
                } else {
                    println!("LESS < null");
                }
            },
            '>' => {
                chars.next();
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next();
                        println!("GREATER_EQUAL >= null");
                    } else {
                        println!("GREATER > null");
                    }
                } else {
                    println!("GREATER > null");
                }
            },
            '/' => {
                chars.next();
                if let Some(&next_char) = chars.peek() {
                    if next_char == '/' {
                        chars.next();
                        while let Some(&next_char) = chars.peek() {
                            if next_char == '\n' { // handling a comment
                                break;
                            }
                            chars.next();
                        }
                    } else {
                        println!("SLASH / null");
                    }
                } else {
                    println!("SLASH / null");
                }
            },
            '\"' => {
                chars.next();
                let mut string = String::new();
                let start_line = lines;
                let mut unterminated = true;

                while let Some(&next_char) = chars.peek() {
                    if next_char == '\"' {
                        chars.next();
                        println!("STRING \"{}\" {}", string, string);
                        unterminated = false;
                        break;
                    }
                    if next_char == '\n' {
                        lines += 1;
                    }
                    string.push(next_char);
                    chars.next();
                }

                if unterminated {
                    eprintln!("[line {}] Error: Unterminated string.", start_line);
                    result = 65;
                }
            },
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&next_char) = chars.peek() {
                    let mut is_float = false;
                    if next_char.is_digit(10)  {
                        number.push(next_char);
                        chars.next();
                    } else if next_char == '.'{
                        is_float = true;
                        number.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                println!("NUMBER {}", number);
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
