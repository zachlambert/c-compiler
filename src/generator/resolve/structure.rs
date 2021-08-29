
use super::construct::*;
use super::generator::Generator;


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
        Primitive::F64 => 8,
        Primitive::C8 => 1,
    }
}

fn get_reference_size(generator: &mut Generator, ref_id: usize) -> usize {
    // Current node = Primitive::Reference(ref_id)
    // Instead of down to child, follows ref to new ref_id
    // on up() again, will return to ref node.
    generator.down_ref(ref_id); 
    let mut size = match generator.current() {
        Construct::Structure(_, size) => *size,
        _ => panic!("Reference doesn't point to a structure node"),
    };
    if size == 0 {
        // Need to calculate size for this structure
        size = fully_define_structure(generator);
    }
    generator.up();
    return size;
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
                        Construct::Reference(ref_id_) => {
                            let ref_id = *ref_id_;
                            get_reference_size(generator, ref_id)
                        }
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

pub fn fully_define_structure(generator: &mut Generator) -> usize {
    // Current node = structure

    let (identifier, mut size) = match generator.current() {
        Construct::Structure(identifier, size) => (String::clone(identifier), *size),
        _ => panic!("Node not Structure at fully_define_structure()"),
    };
    if size != 0 {
        // Already fully defined
        return size;
    }
    let mut alignment: usize = 0;
    generator.down();
    loop {
        let member_size = find_member_size(generator);
        // Need to offset size to align the member
        if member_size > alignment {
            alignment = member_size;
        }
        size += (member_size - size % member_size) % member_size;

        let construct = match generator.current() {
            Construct::Member(identifier, _) => Construct::Member(String::clone(identifier), size),
            _ => panic!("Child of structure node is not a member"),
        };
        generator.replace_construct(&construct);
        
        size += member_size;

        if !generator.next() {
            break;
        }
    }
    generator.up();
    // Need size to be a multiple of alignment
    size += (alignment - size % alignment) % alignment;

    let construct = Construct::Structure(identifier, size);
    generator.replace_construct(&construct);

    return size;

    // TODO: Give warning when structure isn't tightly packed
}
