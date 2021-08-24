
use super::generator::Generator;
use crate::parser::construct::*;
use super::symbol::check_for_symbol;
use super::structure::fully_define_structure;

pub fn generate_program(generator: &mut Generator) {
    match generator.current() {
        Construct::Program => (),
        _ => panic!("Node at generate_program() is not a program"),
    }
    generator.down();

    // Add symbols in current scope
    loop {
        check_for_symbol(generator);
        if !generator.next() {
            break;
        }
    }

    // Fully define structs
    generator.restart();
    loop {
        if let Construct::Structure(_, _) = generator.current() {
            fully_define_structure(generator);
        }
        if !generator.next() {
            break;
        }
    }

    // generator.restart();
    // loop {
    //     resolve_symbol(generator);
    //     if !generator.next() {
    //         break;
    //     }
    // }

    generator.up();
}
