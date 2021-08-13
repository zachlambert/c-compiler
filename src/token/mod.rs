
use std::fmt;

#[derive(Clone)]
pub enum Keyword {
    Int,
    Return,
}

#[derive(Clone)]
pub enum Constant {
    Int(String),
}

#[derive(Clone)]
pub enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,
    Constant(Constant),
    Keyword(Keyword),
    Identifier(String),
    Illegal,
}

impl fmt::Display for Token {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Token::LParen => write!(fmt, "{{"),
            Token::RParen => write!(fmt, "}}"),
            Token::LBrace => write!(fmt, "("),
            Token::RBrace => write!(fmt, ")"),
            Token::Semicolon => write!(fmt, ";"),
            Token::Comma => write!(fmt, ","),
            Token::Constant(constant) => match constant {
                Constant::Int(int) => write!(fmt, "Int({})", int),
            },
            Token::Keyword(keyword) => match keyword {
                Keyword::Int => write!(fmt, "Keyword(int)"),
                Keyword::Return => write!(fmt, "Keyword(return)"),
            },
            Token::Identifier(identifier) =>
                write!(fmt, "Identifier({})", identifier),
            Token::Illegal => write!(fmt, "Illegal"),
        }
    }
}
