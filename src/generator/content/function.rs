
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;

use super::content::resolve_content;
use super::content::generate_content;


pub fn generate_function(generator: &mut Generator) {
    let mut name = match generator.current() {
        Construct::Function(name) => String::clone(name),
        _ => panic!("Node at generate_function() is not a function"),
    };
    generator.increase_scope_function();

    // Add function label
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
