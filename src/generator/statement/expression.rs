
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;

// TODO: Move this to separate file
fn get_datatype_reg(generator: &mut Generator) -> (usize, Regtype) {
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
                    _ => panic!("Invalid child of Datatype in create_pass_location"),
                };
                generator.up();
                result
            },
            Datatype::Pointer => (8, Regtype::Pointer),
        },
        _ => panic!("Node at create_pass_location isn't Datatype"),
    };
    return (size, regtype);
}

fn get_symbol_datatype_i(generator: &mut Generator, symbol_i: usize) -> (usize, usize, Regtype) {
    // Current node = <doesn't matter>
    generator.down_ref(symbol_i);
    // Current node = Variable
    match generator.current() {
        Construct::Variable(_) => (),
        _ => panic!(""),
    }
    generator.down();
    // Current node = datatype, return this
    let datatype_i = generator.get_ref_id();
    let (size, regtype) = get_datatype_reg(generator);
    generator.up();
    generator.up();
    return (datatype_i, size, regtype);
}

fn get_pointer_datatype_i(generator: &mut Generator, symbol_i: usize) -> usize {
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
    let datatype_i = generator.get_ref_id();

    generator.up();
    generator.up();
    generator.up();
    return datatype_i;
}


fn get_symbol_identifier(generator: &mut Generator, lvalue: bool) -> (Symbol, usize) {
    // Current node = Expression::Identifier
    let name = match generator.current() {
        Construct::Identifier(name_) => String::clone(name_),
        _ => panic!(""),
    };
    let symbol_i = generator.find_symbol(&name).expect("Failed to resolve symbol");
    let (datatype_i, size, regtype) = get_symbol_datatype_i(generator, symbol_i);
    let version = generator.get_symbol_version(&name, lvalue); // Increment if setting lvalue

    let symbol = Symbol {
        name: String::clone(&name),
        version: version,
        size: size,
        regtype: regtype,
    };
    return (symbol, datatype_i);
}

fn get_lvalue_pointer(generator: &mut Generator) -> (Symbol, usize) {
    // Current node = Construct::Expression
    match generator.current() {
        Construct::Expression(expression) => match expression {
            Expression::Identifier => {
                generator.down();
                let name = match generator.current() {
                    Construct::Identifier(name_) => String::clone(name_),
                    _ => panic!(""),
                };
                let symbol_i = generator.find_symbol(&name).expect("Failed to resolve symbol");
                let datatype_i = get_pointer_datatype_i(generator, symbol_i);
                let version = generator.get_symbol_version(&name, true);
                generator.up();
                let symbol = Symbol {
                    name: String::clone(&name),
                    version: version,
                    size: 8,
                    regtype: Regtype::Pointer,
                };
                return (symbol, datatype_i);
            },
            _ => panic!("Dereferencing expressions not implemented yet"),
        },
        _ => panic!(""),
    }
}

pub fn generate_expression_lvalue(generator: &mut Generator) -> (Instruction, Symbol, usize) {
    // Current node = Expression
    match generator.current() {
        Construct::Expression(expression) => match expression {
            Expression::Identifier => {
                generator.down();
                // Current node = Construct::Identifier
                let (symbol, datatype_i) = get_symbol_identifier(generator, true);
                generator.up();
                return (Instruction::Move, symbol, datatype_i);
            },
            Expression::UnaryOp(op) => match op {
                UnaryOp::Deref => {
                    generator.down();
                    // Current node = Expression
                    let (symbol, datatype_i) = get_lvalue_pointer(generator);
                    generator.up();
                    return (Instruction::Store, symbol, datatype_i);
                },
                _ => panic!("Invalid lvalue expression"),
            },
            _ => panic!("Invalid lvalue expresion"),
        },
        _ => panic!("Node at generate_expression_lvalue not Expression"),
    }
}

pub fn generate_expression_rvalue(generator: &mut Generator) -> (Symbol, usize) {
    // Current node = Expression
    let expression = match generator.current() {
        Construct::Expression(expression) => Expression::clone(expression),
        _ => panic!(""),
    };
    match expression {
        Expression::Identifier => {
            generator.down();
            // Current node = Construct::Identifier
            let (symbol, datatype_i) = get_symbol_identifier(generator, false);
            generator.up();
            return (symbol, datatype_i);
        },
        Expression::UnaryOp(op) => {
            generator.down();
            // Current node = expression
            let (operand_symbol, operand_datatype_i) = generate_expression_rvalue(generator);
            let result = match op {
                UnaryOp::Negate => {
                    let result_symbol = Symbol {
                        name: String::from("__temp"),
                        version: generator.get_temp_version(),
                        size: operand_symbol.size,
                        regtype: operand_symbol.regtype,
                    };
                    generator.add_element(Element::Instruction(Instruction::ALUOp(ALUOp::Negate)));
                    generator.add_element(Element::Operand(Operand::Symbol(Symbol::clone(&operand_symbol))));
                    generator.add_element(Element::Operand(Operand::Symbol(Symbol::clone(&result_symbol))));
                    (Symbol::clone(&result_symbol), operand_datatype_i)
                }
                _ => panic!("{} not implemented yet", op),
            };
            generator.up();
            return result;
        },
        Expression::BinaryOp(op) => {
            generator.down();
            // Current children = expression(lhs) , expression(rhs)
            let (lhs_symbol, lhs_datatype_i) = generate_expression_rvalue(generator);
            generator.next();
            let (rhs_symbol, rhs_datatype_i) = generate_expression_rvalue(generator);
            // TODO: Assert datatypes match
            let result = match op {
                BinaryOp::Add => {
                    let result_symbol = Symbol {
                        name: String::from("__temp"),
                        version: generator.get_temp_version(),
                        size: lhs_symbol.size,
                        regtype: lhs_symbol.regtype,
                    };
                    generator.add_element(Element::Instruction(Instruction::ALUOp(ALUOp::Add)));
                    generator.add_element(Element::Operand(Operand::Symbol(Symbol::clone(&lhs_symbol))));
                    generator.add_element(Element::Operand(Operand::Symbol(Symbol::clone(&rhs_symbol))));
                    generator.add_element(Element::Operand(Operand::Symbol(Symbol::clone(&result_symbol))));
                    (Symbol::clone(&result_symbol), lhs_datatype_i)
                }
                _ => panic!("{} not implemented yet", op),
            };
            generator.up();
            return result;
        },
        _ => panic!("{} not implemented", expression),
    }
}
