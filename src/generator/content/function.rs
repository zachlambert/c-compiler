
use super::construct;
use super::construct::Construct;
use super::construct::Primitive;
use super::generator::Generator;
use super::instructions::*;

use super::content::resolve_content;
use super::content::generate_content;


fn create_pass_location(generator: &mut Generator, index: usize) -> PassLocation {
    // Current node = Argument or Returned
    // Child is a datatype
    generator.down();
    let (size, datatype) = match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            construct::Datatype::Terminal => {
                generator.down();
                let result = match generator.current() {
                    Construct::Primitive(primitive) => match primitive {
                        Primitive::U8 => (1, Datatype::Integer),
                        Primitive::U16 => (2, Datatype::Integer),
                        Primitive::U32 => (4, Datatype::Integer),
                        Primitive::U64 => (8, Datatype::Integer),
                        Primitive::I8 => (1, Datatype::Integer),
                        Primitive::I16 => (2, Datatype::Integer),
                        Primitive::I32 => (4, Datatype::Integer),
                        Primitive::I64 => (8, Datatype::Integer),
                        Primitive::F32 => (4, Datatype::Float),
                        Primitive::F64 => (8, Datatype::Float),
                        Primitive::C8 => (1, Datatype::Integer),
                    },
                    Construct::Structure(_, size) => (*size, Datatype::Struct),
                    _ => panic!("Invalid child of Datatype in create_pass_location"),
                };
                generator.up();
                result
            },
            construct::Datatype::Pointer => (8, Datatype::Pointer),
        },
        _ => panic!("Node at create_pass_location isn't Datatype"),
    };
    let pass_location = PassLocation {
        index: index,
        size: size,
        datatype: datatype,
    };
    generator.up();
    return pass_location;
}

fn generate_argument_get(generator: &mut Generator, argument: &PassLocation, name: &String) {
    let symbol = Symbol {
        name: String::clone(name),
        version: 0,
        size: argument.size,
        datatype: Datatype::clone(&argument.datatype),
    };
    generator.add_element(Element::Instruction(Instruction::GetArgument));
    generator.add_element(Element::Argument(Argument::PassLocation(PassLocation::clone(argument))));
    generator.add_element(Element::Argument(Argument::Symbol(symbol)));
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
    generator.add_element(Element::Argument(Argument::Label(name)));

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
