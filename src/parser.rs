pub let literals = [
    "true", "false", "nil"
];

pub fn parse(file_contents: &str) -> i32 {
    let mut chars = file_contents.chars().peekable();

    while let Some(&char = chars.peek()) {
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

                if literals.contains(&identifier) {
                    println!("{}", identifier);
                }
                
            },
        }
    }
}