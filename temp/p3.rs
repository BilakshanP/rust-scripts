use regex::Regex;

#[derive(Debug, PartialEq)]
enum Function {
    Sin,
    Cos,
    Max,
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Float(String),
    Operator(String),
    Identifier(String),
    Function(Function),
    OpenParen,
    CloseParen,
    Comma,
}

fn tokenize(input: &str) -> Vec<Token> {
    let re_number = Regex::new(r"^(0x[0-9a-fA-F]+|0b[01]+|0o[0-7]+|\d+)").unwrap();
    let re_float = Regex::new(r"^-?(\d*\.\d+|\d+\.\d*)([eE][-+]?\d+)?").unwrap();
    let re_identifier = Regex::new(r"^[a-zA-Z_]\w*").unwrap();
    let re_operator = Regex::new(r"^[+\-*/^&|=!<>]+").unwrap();

    let mut tokens = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let input = &input[i..];

        if input.starts_with('(') {
            tokens.push(Token::OpenParen);
            i += 1;
        } else if input.starts_with(')') {
            tokens.push(Token::CloseParen);
            i += 1;
        } else if input.starts_with(',') {
            tokens.push(Token::Comma);
            i += 1;
        } else if let Some(mat) = re_float.find(input) {
            tokens.push(Token::Float(mat.as_str().to_string()));
            i += mat.end();
        } else if let Some(mat) = re_number.find(input) {
            tokens.push(Token::Number(mat.as_str().to_string()));
            i += mat.end();
        } else if let Some(mat) = re_identifier.find(input) {
            let identifier = mat.as_str();

            match identifier {
                "sin" => tokens.push(Token::Function(Function::Sin)),
                "cos" => tokens.push(Token::Function(Function::Cos)),
                "max" => tokens.push(Token::Function(Function::Max)),
                _ => tokens.push(Token::Identifier(identifier.to_string())),
            }

            i += mat.end();
        } else if let Some(mat) = re_operator.find(input) {
            tokens.push(Token::Operator(mat.as_str().to_string()));
            i += mat.end();
        } else if input.chars().next().unwrap().is_whitespace() {
            i += 1; // Skip whitespace
        } else {
            panic!("Unknown character: {}", input.chars().next().unwrap());
        }
    }

    tokens
}

fn main() {
    let input = "max(3.14, sin(1) + cos(0xABC) * 2.0)";
    let tokens = tokenize(input);
    for token in tokens {
        println!("{:?}", token);
    }
}
