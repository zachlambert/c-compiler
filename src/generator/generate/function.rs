
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;

use super::resolve::resolve_content;
use super::content::generate_content;
use super::datatype::create_pass_location;
use super::datatype::generate_argument_get;


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
    generator.add_element(Element::Operand(Operand::Label(name)));

    generator.down();

    // For arguments: create symbols and create instruction to load from pass location
    // For returns: store pass locations in generator, to use later
    let mut arg_count: usize = 0;
    loop {
        match generator.current() {
            Construct::Argument(name_) => {
                let name = String::clone(name_);
                generator.add_symbol(&name, true);
                let pass_location = create_pass_location(generator, arg_count);
                arg_count+=1;
                generate_argument_get(generator, &pass_location, &name);
            }
            Construct::Returned => {
                generator.down();
                generator.push_return_datatype();
                generator.up();
            },
            Construct::Block => break,
            _ => panic!("Unexpected child node of function"),
        };

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
