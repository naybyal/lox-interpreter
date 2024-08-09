pub fn parse(file_contents: &str) -> i32 {
    let mut result = 0;
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
                    println!("{identifier}");
                }
            },
            _ => {
                println!("EOF null");
                result = 0;
                break;
            }
        }
        '0'..='9' => {
            let mut number = String::new();
            let mut is_float = String::new();
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

            println!("{number}")
        }
    }
    result
}
