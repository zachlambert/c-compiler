
use super::generator::Generator;
use crate::parser::construct::*;
use super::symbol::check_for_symbol;
use super::symbol::resolve_symbol_datatypes;
use super::structure::fully_define_structure;
use super::instructions::*;

fn resolve_content(generator: &mut Generator) {
    // Current node = program or body

    generator.down();

    // Add symbols in current scope
    loop {
        check_for_symbol(generator);
        if !generator.next() {
            break;
        }
    }

    // Resolve symbol datatypes
    generator.restart();
    loop {
        resolve_symbol_datatypes(generator);
        if !generator.next() {
            break;
        }
    }

    // Fully define structs
    generator.restart();
    loop {
        match generator.current() {
            Construct::Structure(..) => {
                fully_define_structure(generator);
            }
            _ => (),
        }
        if !generator.next() {
            break;
        }
    }

    generator.up();
}

fn generate_content(generator: &mut Generator) {
    generator.down();
    loop {
        match generator.current() {
            Construct::Function(_) => generate_function(generator),
            Construct::Statement(statement) => match statement {
                // Statement::Block => generate_statement_block(generator),
                // Statement::Loop => generate_statement_block(generator),
                _ => panic!("{} generation not implemented yet", statement),
            }
            _ => (),
        }
        // Temporary
        generator.add_element(Element::Instruction(Instruction::Call));
        if !generator.next() {
            break;
        }
    }
    generator.up();
}

pub fn generate_function(generator: &mut Generator) {
    let mut name = match generator.current() {
        Construct::Function(name) => String::clone(name),
        _ => panic!("Node at generate_function() is not a function"),
    };
    generator.increase_scope_function();

    generator.add_element(Element::Instruction(Instruction::Label));
    if name != "main" {
        name.push_str("__");
        name.push_str(&generator.get_ref_id().to_string());
    } else {
        name = String::from("_start");
    }
    generator.add_element(Element::Argument(Argument::Label(name)));

    generator.down();

    // Add symbols for arguments and return values. (return values have pseudonames)
    let mut name: String;
    loop {
        name = match generator.current() {
            Construct::Argument(name_) => String::clone(name_),
            Construct::Returned(name_) => String::clone(name_),
            Construct::Block => break,
            _ => panic!("Unexpected child node of function"),
        };
        generator.add_symbol(&name, true);
        if !generator.next() {
            panic!("Function node has no body child");
        }
    }

    // Current node = Body

    // Resolve symbols within the base scope of the body and generate code

    resolve_content(generator);
    generate_content(generator);

    generator.up(); // Out of function
    generator.decrease_scope_function();
}

pub fn generate_program(generator: &mut Generator) {
    match generator.current() {
        Construct::Program => (),
        _ => panic!("Node at generate_program() is not a program"),
    }
    resolve_content(generator);
    // TODO: Generate code for global variables?
    generate_content(generator);
}
