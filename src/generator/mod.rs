
mod generator;
mod program;
mod content;
mod symbol;
mod function;
mod statement;
mod expression;

use crate::parser::ast::Ast;
use generator::Generator;
use program::generate_program;

pub use generator::Config;

pub fn generate_instructions(ast: &mut Ast, config: Config) {
    let mut generator = Generator::new(ast);
    generate_program(&mut generator);
}
