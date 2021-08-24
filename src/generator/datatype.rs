
use super::generator::Generator;
use crate::parser::construct::*;


fn resolve_datatype_terminal(generator: &mut Generator) {
    // Current node is Datatype::Terminal
    // Child is identifier, terminal (or reference when resolved)
    // If it is an identifier, change construct to a reference
    
    generator.down();
    let identifier = match generator.current() {
        Construct::Identifier(identifier) => identifier,
        _ => {
            generator.up();
            return;
        },
    };

    let ref_node_i = match generator.find_symbol(identifier) {
        Some(node_i) => node_i,
        None => panic!("Couldn't find symbol for identifier {}", identifier),
    };
    let construct = Construct::Reference(ref_node_i);

    generator.replace_construct(&construct);
    generator.up();
}

pub fn resolve_datatype(generator: &mut Generator) {
    // Current node has datatype as one of its children
    // If a datatype is terminal, it will have a primitive or identifier as child
    // If a datatype is a pointer, one of its children will also be a datatype
    generator.down();
    loop {
        if let Construct::Datatype(datatype) = generator.current() {
            match datatype {
                Datatype::Terminal => {
                    resolve_datatype_terminal(generator);
                },
                Datatype::Pointer => {
                    resolve_datatype(generator);
                }
            }
        }
        if !generator.next() {
            break;
        }
    }
    generator.up();
}
