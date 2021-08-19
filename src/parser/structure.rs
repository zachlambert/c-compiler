
use crate::lexer::token::*;
use super::construct::*;
use super::parser::Parser;

use super::datatype::match_datatype;
use super::common::match_identifier;

fn match_member(parser: &mut Parser) -> bool {
    parser.start_node();

    // identifier , ":", type , ":"
    
    if !match_identifier(parser) {
        parser.discard_node();
        return false;
    }

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

    let construct = Construct::Member;
    parser.confirm_node(&construct);

    return true;
}

pub fn match_structure(parser: &mut Parser) -> bool {
    parser.start_node();

    // identifier , ":" , "=" , "struct" , { member }

    if !match_identifier(parser) {
        parser.discard_node();
        return false;
    }

    match parser.consume_token() {
        Token::Colon => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }

    match parser.consume_token() {
        Token::Equals => (),
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

    let construct = Construct::Structure;
    parser.confirm_node(&construct);

    return true;
}
