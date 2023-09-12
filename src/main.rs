use std::io::Write;

use crate::src::{nodes::ALL_VARIANT};

mod src {
    pub mod tokens;
    pub mod lexer;
    pub mod nodes;
    pub mod parser;
    pub mod values;
    pub mod interpreter;
}

#[cfg(test)]
mod tests;

fn main() {
    loop {
        print!("kelvir > ");
        std::io::stdout().flush().unwrap();

        let mut user_input = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut user_input).expect("Failed to retrieve user input!");

        if user_input.trim() == "exit".trim() {
            std::process::exit(0);
        }

        let mut lexer = src::lexer::Lexer::new(user_input.trim().to_string());
        let tokens = lexer.generate_tokens();

        let mut parser = src::parser::Parser::new(tokens);
        let tree = parser.parse();

        match tree {
            ALL_VARIANT::EmptyNode() => {
                continue;
            }

            _ => {}
        };

        let interpreter = src::interpreter::Interpreter{};
        let value = interpreter.visit(tree);

        println!("{}", value.__repr__());
    }
}
