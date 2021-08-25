
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;

use super::content::generate_content;
use super::content::resolve_content;


fn generate_statement_block(generator: &mut Generator) {
    // Children: block
    generator.increase_scope();
    generator.down();
    resolve_content(generator);
    generate_content(generator);
    generator.up();
    generator.decrease_scope();
}

fn generate_statement_assign(generator: &mut Generator) {
    // Children: expression1, expression2
}

fn generate_statement_return(generator: &mut Generator) {
    // Children: expression
}

pub fn generate_statement(generator: &mut Generator) {
    if let Construct::Statement(statement) = generator.current() {
        match statement {
            Statement::Block => generate_statement_block(generator),
            Statement::Assign => generate_statement_assign(generator),
            Statement::Return => generate_statement_return(generator),
            _ => panic!("{} generation not implemented yet", statement),
        }
    } else {
        panic!("Node at generate_statement isn't a statement");
    }
}
