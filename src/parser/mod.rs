
use crate::lexer::token::*;

pub mod construct;
pub mod ast;
mod parser;
mod program;
mod statement;
mod expression;

use ast::Ast;
use parser::Parser;

pub fn build_ast(tokens: &Vec<Token>) -> Option<Ast> {
    let mut ast = Ast::new();
    let mut parser = Parser::new(
        &mut ast,
        tokens,
        128,
        32
    );
    if !program::match_program(&mut parser) {
        println!("Failed to match program");
        return None;
    }
    Some(ast)
}

pub fn print_ast(ast: &Ast) {
    let mut stack: Vec<usize> = Vec::new();
    let mut depths: Vec<u8> = Vec::new();
    stack.push(ast.nodes.len()-1);
    depths.push(0);
    loop {
        match stack.pop() {
            Some(node) => {
                let depth = depths.pop()
                    .expect("Should have depth");
                for _ in 0..depth {
                    print!("  ");
                }
                print!("{}", ast.nodes[node].construct);
                match ast.nodes[node].next {
                    Some(next) => {
                        stack.push(next);
                        depths.push(depth);
                    },
                    None => (),
                };
                match ast.nodes[node].child {
                    Some(child) => {
                        stack.push(child);
                        depths.push(depth+1);
                    },
                    None => (),
                };
                print!("\n");
            },
            None => break,
        };
    }
}
