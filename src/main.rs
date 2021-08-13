use std::env;
use std::fs;
use std::io::{BufReader, Read, BufWriter, Write};

mod lexer;
mod token;
mod parser;
mod compiler;

use lexer::Lexer;
use parser::create_ast;
use compiler::compile_ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: compiler <input> <output>");
        return;
    }
    assert!(args.len()>=3);
    let input_name = &args[1];
    let output_name = &args[2];
    println!("Compiling {}", input_name);

    let input_file = fs::File::open(input_name)
        .expect("Failed to open file.");

    let mut reader = BufReader::new(input_file);
    let mut content = String::new();
    reader.read_to_string(&mut content)
        .expect("Failed to read file.");

    let mut lexer = Lexer::new(&mut content);
    lexer.read_tokens();
    lexer.print_tokens();

    let ast = create_ast(lexer.get_tokens())
        .expect("Failed to build ast");
    println!("{}", ast);

    let mut code = String::new();
    compile_ast(&ast, &mut code);

    let output_file = fs::File::create(output_name)
        .expect("Failed to create output file.");
    let mut writer = BufWriter::new(output_file);
    writer.write_all(code.as_bytes())
        .expect("Unable to write data");

    println!("End");
}
