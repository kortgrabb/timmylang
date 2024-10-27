use crate::ast::{Expr, VarType};
use crate::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut expressions = Vec::new();
        while !self.is_at_end() {
            expressions.push(self.statement());
        }
        expressions
    }

    fn statement(&mut self) -> Expr {
        if self.match_token(&[Token::Print]) {
            let expr = self.expression();
            self.consume(&Token::Semicolon, "Expect ';' after print statement.");
            Expr::Print(Box::new(expr))
        } else if self.check_type_declaration() {
            self.var_declaration()
        } else {
            let expr = self.assignment();
            self.consume(&Token::Semicolon, "Expect ';' after statement.");
            expr
        }
    }

    fn var_declaration(&mut self) -> Expr {
        let var_type = if self.match_token(&[Token::Int]) {
            VarType::Int
        } else if self.match_token(&[Token::StringType]) {
            VarType::StringType
        } else {
            panic!("Expected variable type.");
        };

        // Correctly retrieve the identifier without skipping
        let name = if let Token::Identifier(name) = self.peek().clone() {
            self.advance(); // Now consume the identifier
            name
        } else {
            panic!("Expected variable name.");
        };

        self.consume(&Token::Equals, "Expect '=' after variable name.");

        let value = self.expression();

        self.consume(&Token::Semicolon, "Expect ';' after variable declaration.");

        Expr::VarDeclaration {
            var_type,
            name,
            value: Box::new(value),
        }
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.expression();
        if let Expr::Variable(name) = expr {
            if self.match_token(&[Token::Equals]) {
                let value = self.expression();
                Expr::Assignment {
                    name,
                    value: Box::new(value),
                }
            } else {
                Expr::Variable(name)
            }
        } else {
            expr
        }
    }

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&[Token::Star, Token::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            Expr::Binary {
                left: Box::new(Expr::Number(0.0)),
                operator,
                right: Box::new(right),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_number() {
            if let Token::Number(value) = self.previous().clone() {
                return Expr::Number(value);
            }
        }

        if self.match_string() {
            if let Token::String(value) = self.previous().clone() {
                return Expr::String(value);
            }
        }

        if self.match_identifier() {
            if let Token::Identifier(name) = self.previous().clone() {
                return Expr::Variable(name);
            }
        }

        if self.match_token(&[Token::LParen]) {
            let expr = self.expression();
            self.consume(&Token::RParen, "Expect ')' after expression.");
            return expr;
        }

        panic!("Expected expression.");
    }

    fn match_token(&mut self, types: &[Token]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_number(&mut self) -> bool {
        if self.check_number() {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_string(&mut self) -> bool {
        if self.check_string() {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_identifier(&mut self) -> bool {
        if self.check_identifier() {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == token_type
    }

    fn check_type_declaration(&self) -> bool {
        if self.is_at_end() {
            return false;
        }
        matches!(self.peek(), Token::Int | Token::StringType)
    }

    fn check_number(&self) -> bool {
        matches!(self.peek(), Token::Number(_))
    }

    fn check_string(&self) -> bool {
        matches!(self.peek(), Token::String(_))
    }

    fn check_identifier(&self) -> bool {
        matches!(self.peek(), Token::Identifier(_))
    }

    fn consume(&mut self, token_type: &Token, message: &str) {
        if self.check(token_type) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::EOF)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
