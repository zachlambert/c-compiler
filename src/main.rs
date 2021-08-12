use std::env;
use std::fs;
use std::io::{BufReader, Read};

mod lexer;
use lexer::Lexer;

mod token;
use token::Token;

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
    let mut tokens : Vec<Token> = Vec::new();
    lexer.read_tokens(&mut tokens);

    for token in tokens {
        println!("{}", token);
    }
    println!("End");
}
