pub mod token;
mod token_type;

use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.chars().count();
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.source.chars().nth(self.current);
        self.current += 1;
        return char;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            String::from(""),
            String::from(""),
            42,
        ))
    }

    fn scan_token(&mut self) {
        let char = self.advance();
        match char {
            Some('(') => self.add_token(TokenType::LeftParen),
            Some(')') => self.add_token(TokenType::RightParen),
            Some('{') => self.add_token(TokenType::LeftBrace),
            Some('}') => self.add_token(TokenType::RightBrace),
            Some(',') => self.add_token(TokenType::Comma),
            Some('.') => self.add_token(TokenType::Dot),
            Some('-') => self.add_token(TokenType::Minus),
            Some('+') => self.add_token(TokenType::Plus),
            Some(';') => self.add_token(TokenType::Semicolon),
            Some('*') => self.add_token(TokenType::Star),
            //_ => println!("Unexpected character: {:?}", char),
            Some(_) => (),
            None => panic!("dupa"),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.scan_token()
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            String::from(""),
            42,
        ));

        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name_of_val;

    #[test]
    fn scan_empty() {
        let mut scanner = Scanner::new(String::from(""));
        let tokens = scanner.scan_tokens();

        // Basic check if we get vectors of Tokens
        assert!(type_name_of_val(&tokens).contains("vec::Vec<"));
        assert!(type_name_of_val(&tokens).contains("token::Token>"));
        // Should be end of file token
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn scan_simple_lexemes() {
        let mut scanner = Scanner::new(String::from("(){},.-+;*"));
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 11, "Number of scanned tokens is incorrect.")
    }
}
