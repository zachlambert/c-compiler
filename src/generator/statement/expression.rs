
use super::construct::*;
use super::generator::Generator;
use super::instructions::*;


fn get_symbol_datatype_i(generator: &mut Generator, symbol_i: usize) -> usize {
    match generator.current() {
        Construct::Variable(_) => (),
        _ => panic!(""),
    }
    generator.down();
    let datatype_i = generator.get_ref_id();
    generator.up();
    return datatype_i;
}

fn get_pointer_datatype_i(generator: &mut Generator, symbol_i: usize) -> usize {
    match generator.current() {
        Construct::Variable(_) => (),
        _ => panic!(""),
    }
    generator.down();
    match generator.current() {
        Construct::Datatype(datatype) => match datatype {
            Datatype::Pointer => (),
            _ => panic!(""),
        },
        _ => panic!(""),
    }
    let datatype_i = generator.get_ref_id();
    generator.up();
    return datatype_i;
}


fn get_lvalue_identifier(generator: &mut Generator) -> (Symbol, usize) {
    // Current node = Construct::Identifier
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
                let version = generator.get_symbol_version(&name);
                generator.up();
                let symbol = Symbol {

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
                let (symbol, datatype_i) = get_lvalue_identifier(generator);
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

}
