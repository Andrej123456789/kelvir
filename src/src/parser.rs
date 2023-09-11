use std::rc::Rc;

use super::{nodes::*, tokens::*};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    current_token: Token,
}

impl Parser {
    fn raise_error(&mut self) {

    }

    fn advance(&mut self) {
        if self.pos >= self.tokens.len() {
            self.current_token = Token{ _type: TokenType::NULL, value: "".to_string() };
        } else {
            let mut dummy_token = Token {
                _type: TokenType::NULL,
                value: "".to_string(),
            };
            std::mem::swap(&mut self.tokens[self.pos], &mut dummy_token);
            self.current_token = dummy_token;
            self.pos += 1;
        }
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens: tokens,
            pos: 0,
            current_token: Token{ _type: TokenType::NULL, value: "".to_string() },
        };

        parser.advance();
        return parser;
    }

    /* -------------------------------- */

    fn expr(&mut self) -> ALL_VARIANT {
        let mut result: ALL_VARIANT = self.term();

        let types: [String; 2] = ["PLUS".to_string(), "MINUS".to_string()];
        while !self.current_token.__repr__().contains("NULL") && types.contains(&self.current_token.__repr__()) {
            if self.current_token.__repr__() == "PLUS" {
                self.advance();
                result = ALL_VARIANT::Add(
                    Rc::new(
                        AddNode::new(result, self.term())
                    )
                );
            } else if self.current_token.__repr__() == "MINUS" {
                result = ALL_VARIANT::Sub(
                    Rc::new(
                        SubtractNode::new(result, self.term())
                    )
                );
            }
        }

        return result;
    }

    fn term(&mut self) -> ALL_VARIANT {
        let mut result: ALL_VARIANT = self.factor();

        let types: [String; 2] = ["MULTIPLY".to_string(), "DIVIDE".to_string()];
        while !self.current_token.__repr__().contains("NULL") && types.contains(&self.current_token.__repr__()) {
            if self.current_token.__repr__() == "MULTIPLY" {
                self.advance();
                result = ALL_VARIANT::Mul(
                    Rc::new(
                        MultiplyNode::new(result, self.term())
                    )
                );
            } else if self.current_token.__repr__() == "DIVIDE" {
                result = ALL_VARIANT::Div(
                    Rc::new(
                        DivideNode::new(result, self.term())
                    )
                );
            }
        }

        return result;
    }
    
    fn factor(&mut self) -> ALL_VARIANT {
        let token = self.current_token.clone();
        let mut result: ALL_VARIANT = ALL_VARIANT::EmptyNode();

        if token.__repr__().contains("LPAREN") {
            self.advance();

            result = self.expr();

            if !token.__repr__().contains("RPAREN") {
                self.raise_error();
            }

            self.advance();
            return result;
        } else if token.__repr__().contains("NUMBER") {
            self.advance();
            return ALL_VARIANT::Number(
                Rc::new(
                    NumberNode::new(token.value.parse::<f64>().unwrap())
                )
            );
        } else if token.__repr__().contains("PLUS") {
            self.advance();
            return ALL_VARIANT::Plus(
                Rc::new(
                    PlusNode::new(self.factor())
                )
            );
        } else if token.__repr__().contains("MINUS") {
            self.advance();
            return ALL_VARIANT::Minus(
                Rc::new(
                    MinusNode::new(self.factor())
                )
            );
        }

        return result;
    }

    pub fn parse(&mut self) -> ALL_VARIANT {
        if self.current_token.__repr__().contains("NULL") { /* cannot compare TokenType type yet */
            return ALL_VARIANT::EmptyNode();
        }

        let result: ALL_VARIANT = self.expr();
        return result;
    }
}
