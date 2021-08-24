
use super::generator::Generator;
use crate::parser::construct::*;
use super::datatype::resolve_datatype;


fn get_primitive_size(primitive: &Primitive) -> usize {
    match primitive {
        Primitive::U8 => 1,
        Primitive::U16 => 2,
        Primitive::U32 => 4,
        Primitive::U64 => 8,
        Primitive::I8 => 1,
        Primitive::I16 => 2,
        Primitive::I32 => 4,
        Primitive::I64 => 8,
        Primitive::F32 => 4,
        Primitive::F64 => 4,
        Primitive::C8 => 1,
    }
}

fn get_reference_size(generator: &mut Generator) -> usize {
    // Current node = Primitive::Reference(ref_id)
    // TODO
    return 0;
}

fn find_member_size(generator: &mut Generator) -> usize {
    generator.down();
    loop {
        if let Construct::Datatype(datatype) = generator.current() {
            let size = match datatype {
                Datatype::Terminal => {
                    generator.down();
                    let size = match generator.current() {
                        Construct::Primitive(primitive) => get_primitive_size(primitive),
                        Construct::Reference(ref_id) => get_reference_size(generator),
                        Construct::Identifier(_) => panic!("Struct datatype not resolved"),
                        _ => panic!("Unexpected child node of Datatype::Terminal"),
                    };
                    generator.up();
                    size
                },
                Datatype::Pointer => 8,
            };
            generator.up();
            return size;
        }
        if !generator.next() {
            break;
        }
    }
    panic!("Member had no datatype child");
}

pub fn fully_define_structure(generator: &mut Generator) {
    // Current node = structure

    let (identifier, mut size) = match generator.current() {
        Construct::Structure(identifier, size) => (String::clone(identifier), *size),
        _ => panic!("Node not Structure at fully_define_structure()"),
    };
    if size != 0 {
        // Already fully defined
        return;
    }
    generator.down();
    loop {
        resolve_datatype(generator);
        size += find_member_size(generator);
        if !generator.next() {
            break;
        }
    }
    generator.up();

    let construct = Construct::Structure(identifier, size);
    generator.replace_construct(&construct);
}
