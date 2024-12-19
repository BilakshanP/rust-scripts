use std::collections::HashMap;

// Represents different types of numbers.
#[derive(Debug, Clone)]
pub enum Number {
    Binary(u64),
    Octal(u64),
    Decimal(u64),
    Hexadecimal(u64),
    Float(f64),
}

// Represents an algebraic variable.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Func {
    Sin,
    Cos,
    Sqrt,
}

// Represents a function with a name and a list of arguments.
#[derive(Debug, Clone)]
pub struct Function {
    pub func: Func,
    pub args: Vec<Expr>,
}

// Represents different operations.
#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Factorial,  // New operation
    UnaryMinus, // New operation
}

// The core of the AST: an algebraic expression.
#[derive(Debug, Clone)]
pub enum Expr {
    Number(Number),
    Variable(Variable),
    Function(Function),
    Operation {
        op: Operation,
        left: Option<Box<Expr>>,  // Option to allow for unary operations
        right: Option<Box<Expr>>, // Option for binary operations
    },
}

// Factorial helper function
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

// Function to evaluate the AST.
impl Expr {
    pub fn eval(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        match self {
            Expr::Number(num) => Ok(match num {
                Number::Binary(val) => *val as f64,
                Number::Octal(val) => *val as f64,
                Number::Decimal(val) => *val as f64,
                Number::Hexadecimal(val) => *val as f64,
                Number::Float(val) => *val,
            }),
            Expr::Variable(var) => variables
                .get(&var.name)
                .cloned()
                .ok_or_else(|| format!("Unknown variable: {}", var.name)),
            Expr::Function(func) => {
                let args: Result<Vec<f64>, _> =
                    func.args.iter().map(|arg| arg.eval(variables)).collect();
                match func.func {
                    Func::Sin => Ok(args?
                        .first()
                        .map(|v| v.sin())
                        .ok_or("sin requires one argument")?),
                    Func::Cos => Ok(args?
                        .first()
                        .map(|v| v.cos())
                        .ok_or("cos requires one argument")?),
                    Func::Sqrt => Ok(args?
                        .first()
                        .map(|v| v.sqrt())
                        .ok_or("sqrt requires one argument")?),
                }
            }
            Expr::Operation { op, left, right } => match op {
                Operation::Add => {
                    let left_val = left.as_ref().unwrap().eval(variables)?;
                    let right_val = right.as_ref().unwrap().eval(variables)?;
                    Ok(left_val + right_val)
                }
                Operation::Subtract => {
                    let left_val = left.as_ref().unwrap().eval(variables)?;
                    let right_val = right.as_ref().unwrap().eval(variables)?;
                    Ok(left_val - right_val)
                }
                Operation::Multiply => {
                    let left_val = left.as_ref().unwrap().eval(variables)?;
                    let right_val = right.as_ref().unwrap().eval(variables)?;
                    Ok(left_val * right_val)
                }
                Operation::Divide => {
                    let left_val = left.as_ref().unwrap().eval(variables)?;
                    let right_val = right.as_ref().unwrap().eval(variables)?;
                    Ok(left_val / right_val)
                }
                Operation::Power => {
                    let left_val = left.as_ref().unwrap().eval(variables)?;
                    let right_val = right.as_ref().unwrap().eval(variables)?;
                    Ok(left_val.powf(right_val))
                }
                Operation::Factorial => {
                    let val = left.as_ref().unwrap().eval(variables)?;
                    if val < 0.0 || val.fract() != 0.0 {
                        Err("Factorial is not defined for negative or non-integer values".into())
                    } else {
                        Ok(factorial(val as u64) as f64)
                    }
                }
                Operation::UnaryMinus => {
                    let val = left.as_ref().unwrap().eval(variables)?;
                    Ok(-val)
                }
            },
        }
    }
}

fn main() {
    // Example: -3.5 + 2 * (x!) where x = 4
    // let expr = Expr::Operation {
    //     op: Operation::Add,
    //     left: Some(Box::new(Expr::Operation {
    //         op: Operation::UnaryMinus,
    //         left: Some(Box::new(Expr::Number(Number::Float(3.5)))),
    //         right: None,
    //     })),
    //     right: Some(Box::new(Expr::Operation {
    //         op: Operation::Multiply,
    //         left: Some(Box::new(Expr::Number(Number::Decimal(2)))),
    //         right: Some(Box::new(Expr::Operation {
    //             op: Operation::Factorial,
    //             left: Some(Box::new(Expr::Variable(Variable { name: "x".into() }))),
    //             right: None,
    //         })),
    //     })),
    // };

    let expr = Expr::Operation {
        op: Operation::Factorial,
        left: Some(Box::new(Expr::Number(Number::Decimal(10)))),
        right: None,
    };

    // Define the value for variable 'x'.
    let mut variables = HashMap::new();
    variables.insert("x".into(), 4.0);

    println!("Expression: {:#?}", expr);

    // Evaluate the expression.
    match expr.eval(&variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
