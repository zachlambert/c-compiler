
pub mod token;
mod lexer;

use token::Token;
use lexer::Lexer;

pub fn read_tokens(content: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(&content, &mut tokens);
    lexer.read_tokens();
    tokens
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{}", token);
    }
}
