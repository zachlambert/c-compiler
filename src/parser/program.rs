
use crate::lexer::token::*;
use super::construct::*;
use super::parser::Parser;
use super::statement::match_statement;
use super::common::match_identifier;
use super::datatype::match_datatype;

pub fn match_argument(parser: &mut Parser) -> bool {
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

pub fn match_returned(parser: &mut Parser) -> bool {
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

fn match_function(parser: &mut Parser) -> bool {
    parser.start_node();

    // identifier , ":" , "=",  "function" , ...

    if !match_identifier(parser) {
        parser.discard_node();
        return false;
    }
    println!("a");

    match parser.consume_token() {
        Token::Colon => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }
    println!("b");

    match parser.consume_token() {
        Token::Equals => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }
    println!("c");

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
    println!("d");

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
    println!("d");

    match parser.consume_token() {
        Token::RParen => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }
    println!("e");

    // [ "->" , ( return , ( "(" , {return} , ")" ) ) ] , ...

    match parser.peek_token() {
        Token::RArrow => {
            parser.consume_token();
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
    println!("f");

    // "{" , { statement } , "}"

    match parser.consume_token() {
        Token::LCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }
    println!("g");

    loop {
        if !match_statement(parser) {
            break;
        }
    }
    println!("h");

    match parser.consume_token() {
        Token::RCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }
    println!("i");

    let construct = Construct::Function;
    parser.confirm_node(&construct);

    return true;
}

pub fn match_program(parser: &mut Parser) -> bool {
    parser.start_node();

    loop {
        if !match_function(parser) {
            break;
        }
    }

    let construct = Construct::Program;
    parser.confirm_node(&construct);

    return true;
}

