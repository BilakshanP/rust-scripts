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

#[derive(Debug)]
enum Int {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug)]
enum Float {
    F32,
    F64,
}

#[derive(Debug)]
enum Num {
    Integer(Int),
    Float(Float),
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug)]
enum Func {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Exp,
    Ln,
    Log2,
    Log10,
    Sqrt,
    Cbrt,
    Floor,
    Ceil,
    Round,
    Trunc,
    Fract,
    Abs,
    Signum,
    Recip,
    ToDeg,
    ToRad,
    Max,
    Min,
    Hypot,
    Atan2,
    Powf,
    Log,
    Log2f,
    Log10f,
    Expf,
    Sqrtf,
    Cbrtf,
    Floorf,
    Ceilf,
    Roundf,
    Truncf,
    Fractf,
    Absf,
    Signumf,
    Recipf,
    ToDegf,
    ToRadf,
    Maxf,
    Minf,
    Hypotf,
    Atan2f,
}

#[derive(Debug)]
enum Tokens {
    Number(Num),
    Variable(String),
    Operator(Op),
    Function(Func),
    LeftParen,
    RightParen,
    Comma,
    Null,
}

fn tokenize(input: &str) -> Vec<Tokens> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = iter.peek() {
                    match c {
                        '0'..='9' | '.' => {
                            num.push(c);
                            iter.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Tokens::Number(Num::Float(Float::F64)));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut func = String::new();
                while let Some(&c) = iter.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' => {
                            func.push(c);
                            iter.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Tokens::Function(Func::Sin));
            }
            '+' => {
                tokens.push(Tokens::Operator(Op::Add));
                iter.next();
            }
            '-' => {
                tokens.push(Tokens::Operator(Op::Sub));
                iter.next();
            }
            '*' => {
                tokens.push(Tokens::Operator(Op::Mul));
                iter.next();
            }
            '/' => {
                tokens.push(Tokens::Operator(Op::Div));
                iter.next();
            }
            '%' => {
                tokens.push(Tokens::Operator(Op::Mod));
                iter.next();
            }
            '^' => {
                tokens.push(Tokens::Operator(Op::Pow));
                iter.next();
            }
            '&' => {
                tokens.push(Tokens::Operator(Op::And));
                iter.next();
            }
            '|' => {
                tokens.push(Tokens::Operator(Op::Or));
                iter.next();
            }
            '^' => {
                tokens.push(Tokens::Operator(Op::Xor));
                iter.next();
            }
            '!' => {
                tokens.push(Tokens::Operator(Op::Not));
                iter.next();
            }
            '<' => {
                tokens.push(Tokens::Operator(Op::Lt));
                iter.next();
            }
            '>' => {
                tokens.push(Tokens::Operator(Op::Gt));
                iter.next();
            }
            '=' => {
                tokens.push(Tokens::Operator(Op::Eq));
                iter.next();
            }
            '(' => {
                tokens.push(Tokens::LeftParen);
                iter.next();
            }
            ')' => {
                tokens.push(Tokens::RightParen);
                iter.next();
            }
            ',' => {
                tokens.push(Tokens::Comma);
                iter.next();
            }
            _ => {
                iter.next();
            }
        }
    }

    tokens
}

fn main() {
    let input = "sin(2.0) + 3.0 * 4.0";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}
