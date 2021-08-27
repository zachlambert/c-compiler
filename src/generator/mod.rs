
pub mod instructions;
mod generator;
mod generate;
mod resolve;

use crate::parser::ast::Ast;
use crate::parser::construct;

use instructions::Element;
use generator::Generator;


pub fn generate_instructions(ast: &mut Ast) -> Vec<Element> {
    let mut instructions: Vec<Element> = Vec::new();
    let mut generator = Generator::new(ast, &mut instructions);

    match generator.current() {
        construct::Construct::Program => (),
        _ => panic!("Node at generate_program() is not a program"),
    }
    resolve::resolve_content(&mut generator);
    // TODO: Generate code for global variables?
    generate::generate_content(&mut generator);

    return instructions;
}

pub fn print_instructions(instructions: &Vec<Element>) {
    for element in instructions {
        println!("{}", element);
    }
}
