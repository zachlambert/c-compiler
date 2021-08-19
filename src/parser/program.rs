
use super::construct::*;
use super::parser::Parser;
use super::function::match_function;
use super::structure::match_structure;

fn match_global_symbol(parser: &mut Parser) -> bool {
    if match_function(parser) {
        return true;
    }
    if match_structure(parser) {
        return true;
    }
    return false;
}

pub fn match_program(parser: &mut Parser) -> bool {
    parser.start_node();

    loop {
        if !match_global_symbol(parser) {
            break;
        }
    }

    let construct = Construct::Program;
    parser.confirm_node(&construct);

    return true;
}

