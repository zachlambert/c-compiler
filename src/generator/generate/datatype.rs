
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;


pub fn create_pass_location(generator: &mut Generator, index: usize) -> PassLocation {
    // Current node = Argument or Returned
    // Child is a datatype
    generator.down();
    let info = get_datatype_info(generator);
    let pass_location = PassLocation {
        index: index,
        size: info.size,
        regtype: info.regtype,
    };
    generator.up();
    return pass_location;
}

pub fn generate_argument_get(generator: &mut Generator, argument: &PassLocation, name: &String) {
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

pub fn generate_return_set(generator: &mut Generator, symbol: &Symbol, return_index: usize) {
    // let symbol = Symbol {
    //     name: String::clone(name),
    //     version: 0,
    //     size: argument.size,
    //     regtype: Regtype::clone(&argument.regtype),
    // };
    // generator.add_element(Element::Instruction(Instruction::GetArgument));
    // generator.add_element(Element::Operand(Operand::PassLocation(PassLocation::clone(argument))));
    // generator.add_element(Element::Operand(Operand::Symbol(symbol)));
}

fn skip_qualifiers(generator: &mut Generator) {
    loop {
        match generator.current() {
            Construct::Qualifier(_) => {
                if !generator.next() {
                    panic!("");
                }
            },
            _ => break,
        }
    }
}

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
                skip_qualifiers(generator);
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

    // Current level = { qualifier } , datatype
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

fn check_mutable(generator: &mut Generator) -> bool {
    // Current node = datatype
    generator.down();
    let mut mutable = false;
    loop {
        match generator.current() {
            Construct::Qualifier(qualifier) => match qualifier {
                Qualifier::Mut => {
                    generator.up();
                    mutable = true;
                },
                // _ => (),
            },
            _ => (),
        }
        if !generator.next() {
            break;
        }
    }
    generator.up();
    return mutable;
}

fn match_datatype_pointer(generator: &mut Generator, other: usize) -> bool {
    generator.down_ref(other);
    let matches = match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            Datatype::Pointer => true,
            _ => false,
        },
        _ => panic!(""),
    };
    generator.up();
    return matches;
}

fn match_datatype_reference(generator: &mut Generator, ref_i: usize, other: usize) -> bool {
    // Each struct is a specific node, so can compare ref_i.
    generator.down_ref(other);
    let matches = match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            Datatype::Terminal => {
                generator.down();
                skip_qualifiers(generator);
                let matches = match generator.current() {
                    Construct::Reference(other_ref_i) => (*other_ref_i == ref_i),
                    _ => false,
                };
                generator.up();
                matches
            },
            _ => false,
        },
        _ => false,
    };
    generator.up();
    return matches;
}

fn match_datatype_primitive(generator: &mut Generator, primitive: Primitive, other: usize) -> bool {
    // Each struct is a specific node, so can compare ref_i.
    generator.down_ref(other);
    let matches = match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            Datatype::Terminal => {
                generator.down();
                skip_qualifiers(generator);
                let matches = match generator.current() {
                    Construct::Primitive(other_primitive) => (*other_primitive == primitive),
                    _ => false,
                };
                generator.up();
                matches
            },
            _ => false,
        },
        _ => false,
    };
    generator.up();
    return matches;
}

pub fn validate_datatypes(generator: &mut Generator, lhs: usize, rhs: usize, lhs_mutable: bool, rhs_mutable: bool) -> bool {

    generator.down_ref(lhs);
    match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            Datatype::Pointer => {
                if !match_datatype_pointer(generator, rhs) {
                    return false;
                }
            },
            Datatype::Terminal => {
                generator.down();
                skip_qualifiers(generator);
                match generator.current() {
                    Construct::Reference(node_i_) => {
                        let node_i = *node_i_;
                        if !match_datatype_reference(generator, node_i, rhs) {
                            return false;
                        }
                    },
                    Construct::Primitive(primitive_) => {
                        let primitive = Primitive::clone(primitive_);
                        if !match_datatype_primitive(generator, primitive, rhs) {
                            return false;
                        }
                    },
                    _ => return false,
                }
            }
        },
        _ => panic!(""),
    }
    generator.up();

    generator.down_ref(lhs);
    if lhs_mutable && !check_mutable(generator) {
        generator.up();
        return false;
    }

    generator.down_ref(rhs);
    if rhs_mutable && !check_mutable(generator) {
        generator.up();
        return false;
    }
    generator.up();

    return true;
}
