#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Type keywords
    Int,
    StringType, // 'String' is a reserved keyword in Rust, so using 'StringType'

    // Literals
    Number(f64),
    String(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equals,

    // Identifiers and keywords
    Identifier(String),
    Print,

    // Punctuation
    LParen,
    RParen,
    Semicolon,

    // End of file/input
    EOF,
}
