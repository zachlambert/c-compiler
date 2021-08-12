
use std::str::Chars;
use std::iter::Peekable;

use crate::token;
use token::Keyword;
use token::Constant;
use token::Token;

pub struct Lexer<'a> {
    input_iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        Lexer {
            input_iter: input.chars().peekable(),
            tokens: Vec::new(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input_iter.next()
    }

    fn is_letter(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn lookup_keyword(id: &String) -> Option<Keyword> {
        match id.as_str() {
            "return" => Some(Keyword::Return),
            "int" => Some(Keyword::Int),
            _ => None,
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input_iter.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                let _ = self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self, c: char) -> String {
        let mut ident = String::new();
        ident.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_alphabetic() {
                ident.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self, c: char) -> String {
        let mut number = String::new();
        number.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                number.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        number
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            match c {
                '{' => Some(Token::LParen),
                '}' => Some(Token::RParen),
                '(' => Some(Token::LBrace),
                ')' => Some(Token::RBrace),
                ';' => Some(Token::Semicolon),
                _ => {
                    if Self::is_letter(c) {
                        let keyword = self.read_identifier(c);
                        match Self::lookup_keyword(&keyword) {
                            Some(keyword) => Some(Token::Keyword(keyword)),
                            None => Some(Token::Identifier(keyword)),
                        }
                    } else if c.is_digit(10) {
                        Some(Token::Constant(
                            Constant::Int(self.read_number(c))))
                    } else {
                        Some(Token::Illegal)
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn read_tokens(&mut self) {
        loop {
            match self.next_token() {
                Some(token) => self.tokens.push(token),
                None => break,
            }
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn print_tokens(&self) {
        for token in self.tokens.iter() {
            println!("{}", token);
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
