use crate::tokens::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_char(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(current) = self.next_char() {
            match current {
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance(); // Skip whitespace
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }
                '=' => {
                    tokens.push(Token::Equals);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                }
                ';' => { // Handle semicolons
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                '"' => { // Handle string literals
                    tokens.push(self.string());
                }
                '0'..='9' | '.' => {
                    tokens.push(self.number());
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.identifier();
                    match identifier.as_str() {
                        "print" => tokens.push(Token::Print),
                        "int" => tokens.push(Token::Int),
                        "string" => tokens.push(Token::StringType),
                        _ => tokens.push(Token::Identifier(identifier)),
                    }
                }
                _ => {
                    panic!("Unexpected character: {}", current);
                }
            }
        }

        tokens.push(Token::EOF);
        // println!("{:?}", tokens);
        tokens
    }

    fn number(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.next_char() {
            if c.is_digit(10) || c == '.' {
                self.advance();
            } else {
                break;
            }
        }
        let num_str: String = self.input[start..self.position].iter().collect();
        Token::Number(num_str.parse().unwrap())
    }

    fn string(&mut self) -> Token {
        self.advance(); // Skip the opening quote
        let start = self.position;
        while let Some(c) = self.next_char() {
            if c == '"' {
                break;
            }
            self.advance();
        }
        let string_content: String = self.input[start..self.position].iter().collect();
        self.advance(); // Skip the closing quote
        Token::String(string_content)
    }

    fn identifier(&mut self) -> String {
        let start = self.position;
        while let Some(c) = self.next_char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].iter().collect()
    }
}
