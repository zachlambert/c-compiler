
use crate::lexer::token::*;
use super::construct::*;
use super::parser::Parser;

use super::datatype::match_datatype;


fn match_member(parser: &mut Parser) -> bool {
    parser.start_node();

    // identifier , ":", type , ":"
    
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
        }
    }

    if !match_datatype(parser) {
        parser.discard_node();
        return false;
    }

    match parser.consume_token() {
        Token::Semicolon => (),
        _ => {
            parser.discard_node();
            return false;
        }
    }

    // Offset is calculated later
    let construct = Construct::Member(String::clone(name), 0);
    parser.confirm_node(&construct);

    return true;
}

pub fn match_structure(parser: &mut Parser) -> bool {
    parser.start_node();

    // identifier , ":" , "struct" , { member }

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
    }

    match parser.consume_token() {
        Token::Keyword(keyword) => match keyword {
            Keyword::Struct => (),
            _ => {
                parser.discard_node();
                return false;
            },
        },
        _ => {
            parser.discard_node();
            return false;
        },
    }

    match parser.consume_token() {
        Token::LCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }
    
    loop {
        if !match_member(parser) {
            break;
        }
    }

    match parser.consume_token() {
        Token::RCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }

    // Size is calculated later
    let construct = Construct::Structure(String::clone(name), 0);
    parser.confirm_node(&construct);

    return true;
}
