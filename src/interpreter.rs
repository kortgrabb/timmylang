use crate::ast::{Expr, VarType};
use crate::tokens::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum VarInfo {
    Int,
    StringType,
}

pub struct Interpreter {
    pub environment: HashMap<String, (VarInfo, Value)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, expressions: Vec<Expr>) {
        for expr in expressions {
            self.evaluate(expr);
        }
    }

    fn evaluate(&mut self, expr: Expr) -> Option<Value> {
        match expr {
            Expr::VarDeclaration { var_type, name, value } => {
                if self.environment.contains_key(&name) {
                    panic!("Variable '{}' is already defined.", name);
                }

                let val = self.evaluate(*value).unwrap();

                // Type checking
                match (&var_type, &val) {
                    (VarType::Int, Value::Number(_)) => {},
                    (VarType::StringType, Value::String(_)) => {},
                    (VarType::Int, Value::String(_)) => {
                        panic!("Type mismatch: Expected 'int' for variable '{}', got 'string'.", name);
                    },
                    (VarType::StringType, Value::Number(_)) => {
                        panic!("Type mismatch: Expected 'string' for variable '{}', got 'int'.", name);
                    },
                }

                // Store variable with its type and value
                let var_info = match var_type {
                    VarType::Int => VarInfo::Int,
                    VarType::StringType => VarInfo::StringType,
                };

                self.environment.insert(name, (var_info, val));
                None
            },
            Expr::Number(value) => Some(Value::Number(value)),
            Expr::String(value) => Some(Value::String(value)),
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate(*left)?;
                let right_val = self.evaluate(*right)?;
                match operator {
                    Token::Plus => self.evaluate_add(left_val, right_val),
                    Token::Minus => self.evaluate_subtract(left_val, right_val),
                    Token::Star => self.evaluate_multiply(left_val, right_val),
                    Token::Slash => self.evaluate_divide(left_val, right_val),
                    _ => {
                        panic!("Unknown binary operator.");
                    }
                }
            }
            Expr::Variable(name) => {
                if let Some((_, value)) = self.environment.get(&name) {
                    Some(value.clone())
                } else {
                    panic!("Undefined variable '{}'", name);
                }
            }
            Expr::Assignment { name, value } => {
                let val = self.evaluate(*value).unwrap();
                if let Some((var_type, _)) = self.environment.get(&name) {
                    // Type checking
                    match (var_type, &val) {
                        (VarInfo::Int, Value::Number(_)) => {},
                        (VarInfo::StringType, Value::String(_)) => {},
                        (VarInfo::Int, Value::String(_)) => {
                            panic!("Type mismatch: Cannot assign 'string' to 'int' variable '{}'.", name);
                        },
                        (VarInfo::StringType, Value::Number(_)) => {
                            panic!("Type mismatch: Cannot assign 'int' to 'string' variable '{}'.", name);
                        },
                    }
                    self.environment.insert(name.clone(), (var_type.clone(), val));
                    None
                } else {
                    panic!("Undefined variable '{}'", name);
                }
            }
            Expr::Print(expr) => {
                let val = self.evaluate(*expr).unwrap();
                match val {
                    Value::Number(n) => println!("{}", n),
                    Value::String(s) => println!("{}", s),
                }
                None
            }
        }
    }

    fn evaluate_add(&self, left: Value, right: Value) -> Option<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Some(Value::String(a + &b)),
            (Value::String(a), Value::Number(b)) => Some(Value::String(a + &b.to_string())),
            (Value::Number(a), Value::String(b)) => Some(Value::String(a.to_string() + &b)),
        }
    }

    fn evaluate_subtract(&self, left: Value, right: Value) -> Option<Value> {
        if let (Value::Number(a), Value::Number(b)) = (left, right) {
            Some(Value::Number(a - b))
        } else {
            panic!("Unsupported operands for '-' operator.");
        }
    }

    fn evaluate_multiply(&self, left: Value, right: Value) -> Option<Value> {
        if let (Value::Number(a), Value::Number(b)) = (left, right) {
            Some(Value::Number(a * b))
        } else {
            panic!("Unsupported operands for '*' operator.");
        }
    }

    fn evaluate_divide(&self, left: Value, right: Value) -> Option<Value> {
        if let (Value::Number(a), Value::Number(b)) = (left, right) {
            Some(Value::Number(a / b))
        } else {
            panic!("Unsupported operands for '/' operator.");
        }
    }
}
