use std::io;

enum WordSuffix {
    Pending(String),
    Empty,
}

fn main() {
    let mut in_str: String = String::new();

    io::stdin().read_line(&mut in_str).expect("Failed to read line!");

    println!("{}", convert_to_pig_latin(&in_str));
}

fn convert_to_pig_latin(s: &str) -> String {
    let mut pig_latin_string: String = String::new();
    let mut to_add: WordSuffix = WordSuffix::Empty;
    for c in s.chars() {
        match &mut to_add {
            WordSuffix::Empty => {
                if !c.is_whitespace() && c.is_alphabetic() {
                    let is_vowel = matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u');
                    let mut to_add_builder = String::from('-');
                    if is_vowel {
                        to_add_builder.push_str("hay");
                        to_add = WordSuffix::Pending(to_add_builder);
                    } else {
                        to_add_builder.push(c);
                        to_add_builder.push_str("ay");
                        to_add = WordSuffix::Pending(to_add_builder);
                        continue;
                    }
                }
                //add only if not consonant
                pig_latin_string.push(c);
            }
            WordSuffix::Pending(str_to_add) => {
                if c.is_whitespace() {
                    pig_latin_string.push_str(str_to_add);
                    to_add = WordSuffix::Empty;
                }
                pig_latin_string.push(c);
            }
        }
    }
    if let WordSuffix::Pending(str_to_add) = to_add {
        pig_latin_string.push_str(&str_to_add);
    }
    pig_latin_string
}