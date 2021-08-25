
use super::generator::Generator;
use crate::parser::construct::*;

use super::content::resolve_content;
use super::content::generate_content;


pub fn generate_program(generator: &mut Generator) {
    match generator.current() {
        Construct::Program => (),
        _ => panic!("Node at generate_program() is not a program"),
    }
    resolve_content(generator);
    // TODO: Generate code for global variables?
    generate_content(generator);
}
