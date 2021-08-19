use std::fmt;

#[derive(Clone)]
pub enum Keyword {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    C8,
    Mut,
    Return,
    Function,
    Struct,
}

impl fmt::Display for Keyword {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::U8 => write!(fmt, "Keyword(u8)"),
            Keyword::U16 => write!(fmt, "Keyword(u16)"),
            Keyword::U32 => write!(fmt, "Keyword(u32)"),
            Keyword::U64 => write!(fmt, "Keyword(u64)"),
            Keyword::I8 => write!(fmt, "Keyword(i8)"),
            Keyword::I16 => write!(fmt, "Keyword(i16)"),
            Keyword::I32 => write!(fmt, "Keyword(i32)"),
            Keyword::I64 => write!(fmt, "Keyword(i64)"),
            Keyword::F32 => write!(fmt, "Keyword(f32)"),
            Keyword::F64 => write!(fmt, "Keyword(f64)"),
            Keyword::C8 => write!(fmt, "Keyword(c8)"),
            Keyword::Mut => write!(fmt, "Keyword(mut)"),
            Keyword::Return => write!(fmt, "Keyword(return)"),
            Keyword::Function => write!(fmt, "Keyword(function)"),
            Keyword::Struct => write!(fmt, "Keyword(struct)"),
        }
    }
}

#[derive(Clone)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Str(String),
}

impl fmt::Display for Constant {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Int(int) => write!(fmt, "Int({})", int),
            Constant::Float(float) => write!(fmt, "Float({})", float),
            Constant::Str(string) => write!(fmt, "String({})", string),
        }
    }
}

#[derive(Clone)]
pub enum Token {
    LParen, //      (
    RParen, //      )
    LCBracket, //   { (curly brackets)
    RCBracket, //   }
    LSBracket, //   [ (square brackets)
    RSBracket, //   ]
    Ampersand, //   &
    Circumflex, //  ^
    Percent, //     %
    Equals, //      =
    Semicolon, //   ;
    Colon, //       :
    Comma, //       ,
    Period, //      .
    Plus, //        +
    Minus, //       -
    Asterisk, //    *
    RSlash, //      /
    LSlash, //      \
    LessThan, //    >
    GreaterThan, // <
    Apostrophe, //  '
    Tilde, //       ~
    VBar, //        |
    Underscore, //  _
    Dollar, //      $
    Exclamation, // !
    Question, //    ?
    Grave, //       `
    Constant(Constant),
    Keyword(Keyword),
    Identifier(String),
    Illegal,
    End,
}

impl fmt::Display for Token {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Token::LParen => write!(fmt, "("),
            Token::RParen => write!(fmt, ")"),
            Token::LCBracket => write!(fmt, "{{"),
            Token::RCBracket => write!(fmt, "}}"),
            Token::LSBracket => write!(fmt, "["),
            Token::RSBracket => write!(fmt, "]"),
            Token::Ampersand => write!(fmt, "&"),
            Token::Circumflex => write!(fmt, "^"),
            Token::Percent => write!(fmt, "%"),
            Token::Equals => write!(fmt, "="),
            Token::Semicolon => write!(fmt, ";"),
            Token::Colon => write!(fmt, ":"),
            Token::Comma => write!(fmt, ","),
            Token::Period => write!(fmt, ",."),
            Token::Plus => write!(fmt, ",+"),
            Token::Minus => write!(fmt, ",-"),
            Token::Asterisk => write!(fmt, "*"),
            Token::RSlash => write!(fmt, "/"),
            Token::LSlash => write!(fmt, "\\"),
            Token::LessThan => write!(fmt, ">"),
            Token::GreaterThan => write!(fmt, "<"),
            Token::Apostrophe => write!(fmt, "'"),
            Token::Tilde => write!(fmt, "~"),
            Token::VBar => write!(fmt, "|"),
            Token::Underscore => write!(fmt, "_"),
            Token::Dollar => write!(fmt, "$"),
            Token::Exclamation => write!(fmt, "!"),
            Token::Question => write!(fmt, "?"),
            Token::Grave => write!(fmt, "`"),
            Token::Constant(constant) => write!(fmt, "Constant({})", constant),
            Token::Keyword(keyword) => write!(fmt, "Keyword({})", keyword),
            Token::Identifier(identifier) => write!(fmt, "Identifier({})", identifier),
            Token::Illegal => write!(fmt, "Illegal"),
            Token::End => write!(fmt, "End"),
        }
    }
}
