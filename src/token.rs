
pub enum Token {
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
