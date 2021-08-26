
use super::construct;
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;

use super::content::resolve_content;
use super::content::generate_content;


fn create_pass_location(generator: &mut Generator, index: usize) -> PassLocation {
    // Current node = Argument or Returned
    // Child is a datatype
    generator.down();
    let (size, regtype) = match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            construct::Datatype::Terminal => {
                generator.down();
                let result = match generator.current() {
                    Construct::Primitive(primitive) => match primitive {
                        Primitive::U8 => (1, Regtype::Integer),
                        Primitive::U16 => (2, Regtype::Integer),
                        Primitive::U32 => (4, Regtype::Integer),
                        Primitive::U64 => (8, Regtype::Integer),
                        Primitive::I8 => (1, Regtype::Integer),
                        Primitive::I16 => (2, Regtype::Integer),
                        Primitive::I32 => (4, Regtype::Integer),
                        Primitive::I64 => (8, Regtype::Integer),
                        Primitive::F32 => (4, Regtype::Float),
                        Primitive::F64 => (8, Regtype::Float),
                        Primitive::C8 => (1, Regtype::Integer),
                    },
                    Construct::Structure(_, size) => (*size, Regtype::Struct),
                    _ => panic!("Invalid child of Datatype in create_pass_location"),
                };
                generator.up();
                result
            },
            construct::Datatype::Pointer => (8, Regtype::Pointer),
        },
        _ => panic!("Node at create_pass_location isn't Datatype"),
    };
    let pass_location = PassLocation {
        index: index,
        size: size,
        regtype: regtype,
    };
    generator.up();
    return pass_location;
}

fn generate_argument_get(generator: &mut Generator, argument: &PassLocation, name: &String) {
    let symbol = Symbol {
        name: String::clone(name),
        version: 0,
        size: argument.size,
        regtype: Regtype::clone(&argument.regtype),
    };
    generator.add_element(Element::Instruction(Instruction::GetArgument));
    generator.add_element(Element::Operand(Operand::PassLocation(PassLocation::clone(argument))));
    generator.add_element(Element::Operand(Operand::Symbol(symbol)));
}

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
        let name = match generator.current() {
            Construct::Argument(name_) => {
                String::clone(name_)
            }
            Construct::Returned => {
                let pass_location = create_pass_location(generator, generator.return_count());
                generator.push_return(pass_location);
                continue;
            },
            Construct::Block => break,
            _ => panic!("Unexpected child node of function"),
        };

        // Add symbol if the current node was an argument
        generator.add_symbol(&name, true);
        let pass_location = create_pass_location(generator, arg_count);
        arg_count+=1;
        generate_argument_get(generator, &pass_location, &name);

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
