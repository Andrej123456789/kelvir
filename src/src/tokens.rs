#![allow(unreachable_patterns)]
#![allow(dead_code)]

pub enum TokenType {
    NUMBER,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    LPAREN,
    RPAREN,
    NULL,
}

pub struct Token {
    pub _type: TokenType,
    pub value: String,
}

impl Token {
    fn enum_to_string(&self) -> String {
        match self._type {
            TokenType::NUMBER => return "NUMBER".to_string(),
            TokenType::PLUS => return "PLUS".to_string(),
            TokenType::MINUS => return "MINUS".to_string(),
            TokenType::MULTIPLY => return "MULTIPLY".to_string(),
            TokenType::DIVIDE => return "DIVIDE".to_string(),
            TokenType::LPAREN => return "LPAREN".to_string(),
            TokenType::RPAREN => return "RPAREN".to_string(),
            TokenType::NULL => return "NULL".to_string(),

            _ => return "N/A".to_string(),
        }
    }

    pub fn __repr__(&self) -> String {
        if self.value != "" {
            return self.enum_to_string() + ": " + &format!("{}", self.value);
        }

        return self.enum_to_string();
    }
}
