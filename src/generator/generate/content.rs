
use super::construct::*;
use super::generator::Generator;

use super::function::generate_function;
use super::statement::generate_statement;


pub fn generate_content(generator: &mut Generator) {
    generator.down();
    loop {
        match generator.current() {
            Construct::Function(_) => generate_function(generator),
            Construct::Statement(_) => generate_statement(generator),
            _ => (),
        }
        if !generator.next() {
            break;
        }
    }
    generator.up();
}
