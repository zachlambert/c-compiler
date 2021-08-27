
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;
use super::datatype::get_symbol_datatype;
use super::datatype::get_pointer_datatype;


fn get_symbol_identifier(generator: &mut Generator, lvalue: bool) -> (Symbol, usize) {
    // Current node = Expression::Identifier
    let name = match generator.current() {
        Construct::Identifier(name_) => String::clone(name_),
        _ => panic!(""),
    };
    let symbol_i = generator.find_symbol(&name).expect("Failed to resolve symbol");
    let (datatype_node_i, datatype_info) = get_symbol_datatype(generator, symbol_i);
    let version = generator.get_symbol_version(&name, lvalue); // Increment if setting lvalue

    let symbol = Symbol {
        name: String::clone(&name),
        version: version,
        size: datatype_info.size,
        regtype: datatype_info.regtype,
    };
    return (symbol, datatype_node_i);
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
                let datatype_node_i = get_pointer_datatype(generator, symbol_i);
                let version = generator.get_symbol_version(&name, true);
                generator.up();
                let symbol = Symbol {
                    name: String::clone(&name),
                    version: version,
                    size: 8,
                    regtype: Regtype::Pointer,
                };
                return (symbol, datatype_node_i);
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
