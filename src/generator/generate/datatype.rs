
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;

#[derive(Clone, Copy)]
pub struct DatatypeInfo {
    pub size: usize,
    pub regtype: Regtype,
}

pub fn get_datatype_info(generator: &mut Generator) -> DatatypeInfo {
    // Current node = Datatype
    let (size, regtype) = match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            Datatype::Terminal => {
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
                    Construct::Reference(symbol_i_) => {
                        let symbol_i = *symbol_i_;
                        generator.down_ref(symbol_i);
                        let size = match generator.current() {
                            Construct::Structure(_, size) => *size,
                            _ => panic!(""),
                        };
                        generator.up();
                        (size, Regtype::Struct)
                    },
                    _ => panic!("Invalid child of Datatype in get_datatype_info"),
                };
                generator.up();
                result
            },
            Datatype::Pointer => (8, Regtype::Pointer),
        },
        _ => {
            println!("{}", generator.current());
            panic!("Node at create_pass_location isn't Datatype");
        },
    };
    return DatatypeInfo { size: size, regtype: regtype };
}

pub fn get_symbol_datatype(generator: &mut Generator, symbol_i: usize) -> (usize, DatatypeInfo) {
    // Current node = <doesn't matter>
    generator.down_ref(symbol_i);
    // Current node = Variable
    match generator.current() {
        Construct::Variable(_) => (),
        _ => panic!(""),
    }
    generator.down();
    // Current node = datatype, return this
    let node_i = generator.get_ref_id();
    let info = get_datatype_info(generator);
    generator.up();
    generator.up();

    return (node_i, info);
}

pub fn get_pointer_datatype(generator: &mut Generator, symbol_i: usize) -> usize {
    // Current node = <doesn't matter>
    generator.down_ref(symbol_i);

    // Current node = Variable
    match generator.current() {
        Construct::Variable(_) => (),
        _ => panic!(""),
    }
    generator.down();

    // Current node = datatype, expect Datatype::Pointer
    match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            Datatype::Pointer => (),
            _ => panic!(""),
        },
        _ => panic!(""),
    }
    generator.down();

    // Current level = { qualiifier } , datatype
    // Ignore qualifiers, not relevant
    loop {
        match generator.current() {
            Construct::Datatype(_) => break,
            _ => (),
        }
        if !generator.next() {
            panic!("Pointer node didn't have datatype child");
        }
    }
    // Current node = datatype
    let node_i = generator.get_ref_id();

    generator.up();
    generator.up();
    generator.up();

    return node_i;
}
