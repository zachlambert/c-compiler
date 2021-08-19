
use std::str::Chars;
use std::iter::Peekable;

use super::token::*;

pub struct Lexer<'a> {
    input_iter: Peekable<Chars<'a>>,
    tokens: &'a mut Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String, tokens: &'a mut Vec<Token>) -> Lexer<'a> {
        Lexer {
            input_iter: input.chars().peekable(),
            tokens: tokens,
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
            "u8" => Some(Keyword::U8),
            "u16" => Some(Keyword::U16),
            "u32" => Some(Keyword::U32),
            "u64" => Some(Keyword::U64),
            "i8" => Some(Keyword::I8),
            "i16" => Some(Keyword::I16),
            "i32" => Some(Keyword::I32),
            "i64" => Some(Keyword::I64),
            "f32" => Some(Keyword::F32),
            "f64" => Some(Keyword::F64),
            "c8" => Some(Keyword::C8),
            "mut" => Some(Keyword::Mut),
            "return" => Some(Keyword::Return),
            "function" => Some(Keyword::Function),
            "struct" => Some(Keyword::Struct),
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
            if c.is_alphabetic() || c=='_' || c.is_digit(10) {
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

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            match c {
                // Handle multi-character symbols first.
                '&' => Some(Token::Ampersand),
                '|' => Some(Token::VBar),
                '=' => Some(Token::Equals),
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                '{' => Some(Token::LCBracket),
                '}' => Some(Token::RCBracket),
                '[' => Some(Token::LSBracket),
                ']' => Some(Token::RSBracket),
                '^' => Some(Token::Circumflex),
                '%' => Some(Token::Percent),
                ';' => Some(Token::Semicolon),
                ':' => Some(Token::Colon),
                ',' => Some(Token::Comma),
                '.' => Some(Token::Period),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Asterisk),
                '\\' => Some(Token::LSlash),
                '/' => Some(Token::RSlash),
                '<' => Some(Token::GreaterThan),
                '>' => Some(Token::LessThan),
                '\'' => Some(Token::Apostrophe),
                '~' => Some(Token::Tilde),
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
        self.tokens.push(Token::End);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
