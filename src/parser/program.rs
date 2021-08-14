
use crate::lexer::token::*;
use super::symbol::*;
use super::parser::Parser;

// use super::statement::match_statement;

fn match_argument(parser: &mut Parser) -> bool {
    parser.start_node();

    let arg_type = match parser.consume_token() {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => {
                parser.discard_node();
                return false;
            }
        },
        _ => {
            parser.discard_node();
            return false;
        }
    };

    let name = match parser.consume_token() {
        Token::Identifier(name) => name,
        _ => {
            parser.discard_node();
            return false;
        },
    };

    let argument = Argument {
        name: String::clone(name),
        arg_type: arg_type,
    };
    let symbol = Symbol::Argument(argument);
    parser.confirm_node(&symbol);

    return true;
}

fn match_function(parser: &mut Parser) -> bool {
    parser.start_node();

    let ret_type = match parser.consume_token() {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => panic!("Invalid return type for function"),
        },
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => {
            parser.discard_node();
            return false;
        },
    };

    let name = match parser.consume_token() {
        Token::Identifier(name) => name,
        _ => {
            parser.discard_node();
            return false;
        },
    };

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

    match parser.consume_token() {
        Token::LCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }

    // loop {
    //     if !match_statement(parser) {
    //         break;
    //     }
    // }

    match parser.consume_token() {
        Token::RCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        },
    }

    let function = Function {
        name: String::clone(name),
        ret_type: ret_type,
    };
    let symbol = Symbol::Function(function);
    parser.confirm_node(&symbol);

    return true;
}

pub fn match_program(parser: &mut Parser) -> bool {
    parser.start_node();

    loop {
        if !match_function(parser) {
            break;
        }
    }

    let symbol = Symbol::Program;
    parser.confirm_node(&symbol);

    return true;
}

