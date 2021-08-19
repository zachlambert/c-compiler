
use crate::lexer::token::*;
use super::construct::*;
use super::parser::Parser;
use super::statement::match_statement;
use super::common::match_identifier;
use super::datatype::match_datatype;

fn match_argument(parser: &mut Parser) -> bool {
    parser.start_node();

    // identifier : datatype

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
    };

    if !match_datatype(parser) {
        parser.discard_node();
        return false;
    }

    let construct = Construct::Argument;
    parser.confirm_node(&construct);

    return true;
}

fn match_returned(parser: &mut Parser) -> bool {
    parser.start_node();

    // datatype

    if !match_datatype(parser) {
        parser.discard_node();
        return false;
    }

    let construct = Construct::Returned;
    parser.confirm_node(&construct);

    return true;
}

pub fn match_function(parser: &mut Parser) -> bool {
    parser.start_node();

    // identifier , ":" , "=",  "function" , ...

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
            Keyword::Function => (),
            _ => {
                parser.discard_node();
                return false;
            }
        }
        _ => {
            parser.discard_node();
            return false;
        },
    }

    // ... , "function" , "(" , [ argument , { "," , argument } ] , ")" ,

    match parser.consume_token() {
        Token::LParen => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }

    if match_argument(parser) {
        loop {
            match parser.peek_token() {
                Token::Comma => parser.consume_token(),
                _ => break,
            };
            if !match_argument(parser) {
                panic!("Expected argument after comma");
            };
        }
    }

    match parser.consume_token() {
        Token::RParen => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }

    // [ "->" , ( return , ( "(" , {return} , ")" ) ) ] , ...

    match parser.peek_token() {
        Token::Minus => {
            parser.consume_token();
            match parser.consume_token() {
                Token::LessThan => (),
                _ => panic!("Expected '>' after '-' to form ->"),
            }
            if !match_returned(parser) {
                match parser.consume_token() {
                    Token::LParen => (),
                    _ => panic!("Expected return type or '(' after function(...)->"),
                }
                if !match_returned(parser) {
                    panic!("Expected return type in ->()");
                }
                loop {
                    match parser.peek_token() {
                        Token::Comma => (),
                        _ => break,
                    }
                    parser.consume_token();
                    if !match_returned(parser) {
                        panic!("Expected return type after ,");
                    }
                }
                match parser.consume_token() {
                    Token::RParen => (),
                    _ => panic!("Expected ) after return type list"),
                }
            }
        },
        _ => (),
    }

    // "{" , { statement } , "}"

    match parser.consume_token() {
        Token::LCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }

    loop {
        if !match_statement(parser) {
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

    let construct = Construct::Function;
    parser.confirm_node(&construct);

    return true;
}
