
pub mod instructions;
mod generator;
mod symbol;
mod datatype;
mod structure;
mod program;
// mod content;
// mod function;
// mod statement;
// mod expression;

use crate::parser::ast::Ast;
use instructions::Element;
use generator::Generator;
use program::generate_program;

pub fn generate_instructions(ast: &mut Ast) -> Vec<Element> {
    let mut instructions: Vec<Element> = Vec::new();
    let mut generator = Generator::new(ast, &mut instructions);
    generate_program(&mut generator);
    return instructions;
}

pub fn print_instructions(instructions: &Vec<Element>) {
    for element in instructions {
        println!("{}", element);
    }
}
