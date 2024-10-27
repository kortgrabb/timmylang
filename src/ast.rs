use crate::tokens::Token;

#[derive(Debug)]
pub enum Expr {
    // Variable Declaration with Type
    VarDeclaration {
        var_type: VarType,
        name: String,
        value: Box<Expr>,
    },
    // Literals
    Number(f64),
    String(String),
    // Binary Operations
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    // Variable Usage
    Variable(String),
    // Assignment (without type)
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    // Print Statement
    Print(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
    Int,
    StringType,
}
