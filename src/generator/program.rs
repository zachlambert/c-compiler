
use super::generator::Generator;
use crate::parser::construct::*;

pub fn generate_program(generator: &mut Generator) {
    match generator.current() {
        Construct::Program => (),
        _ => panic!("Node at generate_program() is not a program"),
    }
}
