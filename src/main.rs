use std::env;
use std::fs;
use std::io::{BufReader, Read, BufWriter, Write};

mod lexer;
mod parser;
mod checker;
// mod compiler;

use lexer::read_tokens;
use lexer::print_tokens;
use parser::build_ast;
use parser::print_ast;
use checker::resolve_ast;
// use compiler::compile_ast;

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

    // 1. Read source file to a string
    let input_file = fs::File::open(input_name)
        .expect("Failed to open file.");
    let mut reader = BufReader::new(input_file);
    let mut content = String::new();
    reader.read_to_string(&mut content)
        .expect("Failed to read file.");

    // 2. Read tokens
    let tokens = read_tokens(&content);
    // print_tokens(&tokens);

    // 3. Build abstract syntax tree
    let mut ast = build_ast(&tokens)
        .expect("Failed to build ast");
    print_ast(&ast);

    // 4. Resolve ast
    resolve_ast(&mut ast);

    // 4. Compile ast to string
    let code = String::new();//compile_ast(&ast);

    // 5. Write code to file
    let output_file = fs::File::create(output_name)
        .expect("Failed to create output file.");
    let mut writer = BufWriter::new(output_file);
    writer.write_all(code.as_bytes())
        .expect("Unable to write data");

    println!("End");
}
