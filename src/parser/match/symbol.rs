
use crate::lexer::token::*;
use super::construct::*;
use super::parser::Parser;

use super::datatype::match_datatype;
use super::structure::match_structure;
use super::function::match_function;


fn match_variable(parser: &mut Parser) -> bool {
    parser.start_node();

    let name = match parser.consume_token() {
        Token::Identifier(identifier) => identifier,
        _ => {
            parser.discard_node();
            return false;
        },
    };

    match parser.consume_token() {
        Token::Colon => (),
        _ => {
            parser.discard_node();
            return false;
        },
    };

    if !match_datatype(parser) {
        parser.discard_node();
        return false;
    }

    match parser.consume_token() {
        Token::Semicolon => (),
        _ => {
            parser.discard_node();
            return false;
        },
    };

    let construct = Construct::Variable(String::clone(name));
    parser.confirm_node(&construct);

    return true;
}

pub fn match_symbol(parser: &mut Parser) -> bool {
    if match_function(parser) {
        return true;
    }
    if match_structure(parser) {
        return true;
    }
    if match_variable(parser) {
        return true;
    }
    return false;
}

