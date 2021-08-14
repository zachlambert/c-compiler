
use std::str::Chars;
use std::iter::Peekable;

use crate::token;
use token::Keyword;
use token::Constant;
use token::Primitive;
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
            "char" => Some(Keyword::Primitive(Primitive::Char)),
            "int" => Some(Keyword::Primitive(Primitive::Int)),
            "float" => Some(Keyword::Primitive(Primitive::Float)),
            "double" => Some(Keyword::Primitive(Primitive::Double)),
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

    fn read_word(&mut self, c: char) -> String {
        let mut word = String::new();
        word.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_alphabetic() {
                word.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        word
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        let mut escape = false;
        while let Some(&c) = self.peek_char() {
            if !escape && c == '"' {
                let _ = self.read_char();
                break;
            } if !escape && c == '\\' {
                let _ = self.read_char();
                escape = true;
            } else {
                string.push(self.read_char().unwrap());
                escape = false;
            }
        }
        string
    }

    fn read_number(&mut self, c: char) -> Constant {
        let mut string = String::new();
        let mut float = false;
        string.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                string.push(self.read_char().unwrap());
            } else if c == '.' {
                if float {
                    panic!("Two periods in number");
                }
                float = true;
                string.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        if float {
            Constant::Float(string.parse::<f64>().unwrap())
        } else {
            Constant::Int(string.parse::<i64>().unwrap())
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            match c {
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                '{' => Some(Token::LCBracket),
                '}' => Some(Token::RCBracket),
                '[' => Some(Token::LSBracket),
                ']' => Some(Token::RSBracket),
                '&' => Some(Token::Ampersand),
                '^' => Some(Token::Circumflex),
                '%' => Some(Token::Percent),
                '=' => Some(Token::Equals),
                ';' => Some(Token::Semicolon),
                ':' => Some(Token::Colon),
                ',' => Some(Token::Comma),
                '.' => Some(Token::Period),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Asterisk),
                '/' => Some(Token::RSlash),
                '\\' => Some(Token::LSlash),
                '>' => Some(Token::LessThan),
                '<' => Some(Token::GreaterThan),
                '\'' => Some(Token::Apostrophe),
                '~' => Some(Token::Tilde),
                '|' => Some(Token::VBar),
                '_' => Some(Token::Underscore),
                '$' => Some(Token::Dollar),
                '!' => Some(Token::Exclamation),
                '?' => Some(Token::Question),
                '`' => Some(Token::Grave),
                '"' => Some(Token::Constant(Constant::Str(self.read_string()))),
                _ => {
                    if Self::is_letter(c) {
                        let word = self.read_word(c);
                        match Self::lookup_keyword(&word) {
                            Some(keyword) => Some(Token::Keyword(keyword)),
                            None => Some(Token::Identifier(word)),
                        }
                    } else if c.is_digit(10) { // 0 -> 9
                        Some(Token::Constant(self.read_number(c)))
                    } else {
                        Some(Token::Illegal)
                    }
                },
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
