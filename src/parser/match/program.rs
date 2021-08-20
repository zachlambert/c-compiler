
use super::construct::*;
use super::parser::Parser;
use super::symbol::match_symbol;


pub fn match_program(parser: &mut Parser) -> bool {
    parser.start_node();

    loop {
        if !match_symbol(parser) {
            break;
        }
    }

    let construct = Construct::Program;
    parser.confirm_node(&construct);

    return true;
}

