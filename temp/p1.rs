#![allow(unused, unreachable_code)]

mod macros;

module!(
    base,
    hash,
    enc_dec,
    functions,
    data_structures,
    algorithms,
    mathematica
);
fn main() {
    let input = "sin(23) = 0x8 // This is a comment";
    let tokens = tokenize(input);

    for token in tokens {
        println!("{}", token);
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();

    let is_number = |c: char| c.is_ascii_digit() || c == '.' || c == '-';
    let is_alphabetic = |c: char| c.is_alphabetic();

    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if is_number(c) {
            // Check if we should start a new token
            if !token.is_empty() && !is_number(token.chars().next().unwrap()) {
                tokens.push(token.clone());
                token.clear();
            }

            // Collect the entire number (including dots for floats)
            while let Some(&c) = chars.peek() {
                if is_number(c) {
                    token.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
        } else if is_alphabetic(c) {
            // Check if we should start a new token
            if !token.is_empty() && !is_alphabetic(token.chars().next().unwrap()) {
                tokens.push(token.clone());
                token.clear();
            }

            // Collect the entire word
            while let Some(&c) = chars.peek() {
                if is_alphabetic(c) {
                    token.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
        } else if c.is_whitespace() {
            // Push the current token before skipping whitespace
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }

            // Skip all whitespace characters
            chars.next();
        } else {
            // Handle punctuation or other characters
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }

            tokens.push(c.to_string());
            chars.next();
        }
    }

    // Push the last token if any
    if !token.is_empty() {
        tokens.push(token);
    }

    tokens
}
