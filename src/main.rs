use std::env;
use std::fs;
use std::io::{BufReader, Read};

mod lexer;
use lexer::Lexer;

mod token;
use token::Token;
use token::Constant;
use token::Keyword;

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
