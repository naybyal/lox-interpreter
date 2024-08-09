pub fn parse(file_contents: &str) -> i32 {
    let mut result: i32 = 0;
    let mut chars: std::iter::Peekable<std::str::Chars> = file_contents.chars().peekable();

    while let Some(&char) = chars.peek() {
        match char {
            'a'..='z' | 'A'..='Z' => {
                let mut identifier: String = String::new();
                while let Some(&char) = chars.peek() {
                    if char.is_alphanumeric() {
                        identifier.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let literals: [&str; 3] = ["true", "false", "nil"];
                if literals.contains(&identifier.as_str()) {
                    println!("{identifier}");
                }
            },
            '0'..='9' => {
                let mut number: String = String::new();
                let mut is_float: bool = false;
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_digit(10) {
                        number.push(next_char);
                        chars.next();
                    } else if next_char == '.' && !is_float {
                        is_float = true;
                        number.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if is_float {
                    if number.ends_with('0') {
                        println!("{}", number);
                    } else if number.ends_with('.') {
                        number.pop();
                        println!("{}.0", number);
                    } else {
                        let parsed_float: f64 = number.parse().unwrap();
                        println!("{}", parsed_float);
                    }
                } else {
                    println!("{}.0", number);
                }
            },
            '\"' => {
                chars.next();
                let mut string: String = String::new();
                let mut unterminated: bool = true;

                while let Some(&next_char) = chars.peek() {
                    if next_char == '\"' {
                        chars.next();
                        println!("{string}");
                        unterminated = false;
                        break;
                    }
                    
                    string.push(next_char);
                    chars.next();
                }

                if unterminated {
                    result = 65;
                }
            },
            '(' => {
                chars.next();
                let mut string: String = String::new();
                let mut unterminated: bool = true;

                while let Some(&next_char) = chars.peek() {
                    if next_char == ')' {
                        chars.next();
                        println!("(group {string})");
                        unterminated = false;
                        break;
                    }
                    
                    string.push(next_char);
                    chars.next();
                }

                if unterminated {
                    eprintln!("Error: Unterminated string.");
                    result = 65;
                }
            },
            _ => {
                println!("EOF null");
                result = 0;
                break;
            }
        }
        
    }
    result
}
