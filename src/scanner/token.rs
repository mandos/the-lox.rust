#![allow(dead_code)]
use crate::scanner::token_type::TokenType;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_to_string() {
        let token = Token::new(
            TokenType::Nil,
            "lexeme".to_string(),
            "literal".to_string(),
            42,
        );
        assert_eq!(token.to_string(), "Nil lexeme literal");
    }
}
