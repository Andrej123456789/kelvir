use super::tokens::{Token, TokenType};

pub struct Lexer
{
    iter: Vec<char>,
    pos: usize,
    current_char: char,
    
}

impl Lexer
{
    fn advance(&mut self) {
        if self.pos >= self.iter.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.iter[self.pos];
            self.pos += 1;
        }
    }

    pub fn new(text: String) -> Self {
        let mut lexer = Lexer {
            iter: text.chars().collect(),
            pos: 0,
            current_char: '\0',
        };

        lexer.advance();
        return lexer;
    }

    pub fn generate_tokens(&mut self) -> Vec<Token> {
        let mut vec: Vec<Token> = Vec::new();
        let mut been_in_while_loop: bool = false;

        while self.current_char != '\0' {
            if self.current_char.is_whitespace() {
                self.advance();
                continue;
            } else if self.current_char == '.' || self.current_char.is_numeric() {
                vec.push(self.generate_number());
            } else if self.current_char == '+' {
                self.advance();
                vec.push(Token { _type: TokenType::PLUS, value: "".to_string() });
            } else if self.current_char == '-' {
                self.advance();
                vec.push(Token { _type: TokenType::MINUS, value: "".to_string() });
            } else if self.current_char == '*' {
                self.advance();
                vec.push(Token { _type: TokenType::MULTIPLY, value: "".to_string() });
            } else if self.current_char == '/' {
                self.advance();
                vec.push(Token { _type: TokenType::DIVIDE, value: "".to_string() });
            }

            been_in_while_loop = true;
        }

        if !been_in_while_loop {
            vec.push(Token { _type: TokenType::NULL, value: "".to_string() });
        }

        return vec;

    }

    pub fn generate_number(&mut self) -> Token {
        let mut decimal_point_count = 0;
        let mut number_str: String = String::from(self.current_char);
        
        self.advance();

        while self.current_char != '\0' && (self.current_char == '.' || self.current_char.is_numeric()) {
            if self.current_char == '.' {
                decimal_point_count += 1;
                if decimal_point_count > 1 {
                    break;
                }
            }
        }

        if number_str.starts_with(',') {
            number_str = format!("0{}", number_str);
        }

        if number_str.ends_with('.') {
            number_str.push_str("0");
        }

        Token { _type: TokenType::NUMBER, value: number_str}
    }
}
