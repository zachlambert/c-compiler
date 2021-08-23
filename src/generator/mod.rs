
pub mod instructions;
mod generator;
mod program;
// mod content;
// mod symbol;
// mod function;
// mod statement;
// mod expression;

use crate::parser::ast::Ast;
use generator::Generator;
use program::generate_program;
use instructions::Element;

pub use generator::Config;

pub fn generate_instructions(ast: &mut Ast, config: Config) -> Vec<Element> {
    let mut instructions: Vec<Element> = Vec::new();
    let mut generator = Generator::new(ast, config, &mut instructions);
    generate_program(&mut generator);
    return instructions;
}

pub fn print_instructions(instructions: &Vec<Element>) {
    for element in instructions {
        println!("{}", element);
    }
}
