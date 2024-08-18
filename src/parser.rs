pub fn parse(file_contents: &str) -> i32 {
    let result: i32 = 0;
    let mut chars = file_contents.chars().peekable();

    while let Some(&char) = chars.peek() {
        match char {
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&char) = chars.peek() {
                    if char.is_alphanumeric() {
                        identifier.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let literals = ["true", "false", "nil"];
                if literals.contains(&identifier.as_str()) {
                    println!("{}", identifier);
                }
            },
            '0'..='9' => {
                let mut number = String::new();
                let mut is_float = false;
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
                let mut string = String::new();
                let mut unterminated = true;

                while let Some(&next_char) = chars.peek() {
                    if next_char == '\"' {
                        chars.next();
                        println!("{}", string);
                        unterminated = false;
                        break;
                    }
                    
                    string.push(next_char);
                    chars.next();
                }

                if unterminated {
                    eprintln!("Error: Unterminated string.");
                    return 65;
                }
            },
            '(' => {
                chars.next();
                let group = parse_inner(&mut chars);
                if group.is_none() {
                    return 65;
                }
                println!("(group {})", group.unwrap());
            },
            _ => {
                println!("EOF null");
                break;
            }
        }
    }
    result
}

fn parse_inner(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    let mut string = String::new();
    let mut unterminated = true;
    let mut empty_group = true; // Track if the group is empty

    while let Some(&next_char) = chars.peek() {
        match next_char {
            '(' => {
                chars.next();
                if let Some(inner_group) = parse_inner(chars) {
                    string.push_str(&format!("(group {})", inner_group));
                } else {
                    return None;
                }
                empty_group = false; // Not empty since we found inner content
            }
            ')' => {
                chars.next();
                unterminated = false;
                break;
            }
            '\'' | '\"' => {
                chars.next();
                empty_group = false; // Not empty since we found content
            }
            _ => {
                string.push(next_char);
                chars.next();
                empty_group = false; // Not empty since we found content
            }
        }
    }

    if unterminated || empty_group {
        eprintln!("Error: Unmatched or empty parentheses.");
        return None;
    }

    Some(string)
}
