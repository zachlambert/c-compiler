use std::env;
use std::fs;
use std::str::Chars;
use std::iter::Peekable;
use std::io::{BufReader, Read};

enum Keyword {
    Int,
    Return,
}

enum Constant {
    Int(String),
}

enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Constant(Constant),
    Keyword(Keyword),
    Identifier(String),
    Illegal,
}

struct Lexer<'a> {
    input_iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        Lexer {
            input_iter: input.chars().peekable()
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    println!("Compiling {}", file_name);

    let file = fs::File::open(file_name)
        .expect("Failed to open file.");

    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)
        .expect("Failed to read file.");

    let mut lexer = Lexer::new(&mut content);
    loop {
        match lexer.next_token() {
            Some(token) => {
                match token {
                    Token::LParen => println!("{{"),
                    Token::RParen => println!("}}"),
                    Token::LBrace => println!("("),
                    Token::RBrace => println!(")"),
                    Token::Semicolon => println!(";"),
                    Token::Constant(constant) => match constant {
                        Constant::Int(int) => println!("Int({})", int),
                    },
                    Token::Keyword(keyword) => match keyword {
                        Keyword::Int => println!("Keyword(int)"),
                        Keyword::Return => println!("Keyword(return)"),
                    },
                    Token::Identifier(identifier) =>
                        println!("Identifier({})", identifier),
                    Token::Illegal => println!("Illegal"),
                };
            },
            None => break,
        }
    }
    println!("End");
}
