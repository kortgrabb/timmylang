mod ast;
mod interpreter;
mod lexer;
mod parser;
mod tokens;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();

    if args.len() > 1 {
        // File mode
        let filename = &args[1];
        let contents = fs::read_to_string(filename)
            .expect(&format!("Could not read file {}", filename));
        println!("Executing program from file: {}", filename);

        // Lexical Analysis
        let mut lexer = Lexer::new(&contents);
        let tokens = lexer.tokenize();

        // Parsing
        let mut parser = Parser::new(tokens);
        let expressions = parser.parse();

        // Evaluation
        interpreter.interpret(expressions);
    } else {
        // REPL mode
        println!("Simple Rust Interpreter");
        println!("Type 'exit' to quit.");

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read line.");
                continue;
            }

            let input = input.trim();
            if input == "exit" {
                break;
            }

            if input.is_empty() {
                continue;
            }

            // Lexical Analysis
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();

            // Parsing
            let mut parser = Parser::new(tokens);
            let expressions = parser.parse();

            // Evaluation
            interpreter.interpret(expressions);
        }
    }
}
