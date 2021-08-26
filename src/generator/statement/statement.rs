
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;

use super::content::generate_content;
use super::content::resolve_content;
use super::expression::generate_expression_lvalue;
use super::expression::generate_expression_rvalue;


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
    // Current node = Statement::Assign
    // Children: expression1, expression2
    generator.down();
    let (instruction, dest_symbol, dest_datatype_i) = generate_expression_lvalue(generator);
    generator.next();
    let (src_symbol, src_datatype_i) = generate_expression_rvalue(generator);
    generator.up();

    // TODO: Assert src_datatype = dst_datatype

    generator.add_element(Element::Instruction(instruction)); // Move, load or store
    generator.add_element(Element::Operand(Operand::Symbol(src_symbol)));
    generator.add_element(Element::Operand(Operand::Symbol(dest_symbol)));
}

fn generate_statement_return(generator: &mut Generator) {
    // Children: expression
    // TODO: Set return values
    generator.add_element(Element::Instruction(Instruction::Return));
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
