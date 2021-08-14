
use crate::lexer::token::*;
use super::symbol::*;
use super::parser::Parser;

use super::expression::match_expression;

fn match_statement_declare(parser: &mut Parser) -> bool {
    parser.start_node();

    let init_type = match parser.consume_token() {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => {
                parser.discard_node();
                return false;
            }
        }
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    let identifier = match parser.consume_token() {
        Token::Identifier(identifier) => identifier,
        _ => {
            parser.discard_node();
            return false;
        }
    };

    match parser.consume_token() {
        Token::Semicolon => (),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    let symbol = Symbol::Statement(Statement::Declare(
        init_type,
        String::clone(identifier)
    ));
    parser.confirm_node(&symbol);

    return true;
}

fn match_statement_initialise(parser: &mut Parser) -> bool {
    parser.start_node();

    let init_type = match parser.consume_token() {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => {
                parser.discard_node();
                return false;
            }
        }
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    let identifier = match parser.consume_token() {
        Token::Identifier(identifier) => identifier,
        _ => {
            parser.discard_node();
            return false;
        }
    };

    match parser.consume_token() {
        Token::Equals => (),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    if !match_expression(parser) {
        parser.discard_node();
        return false;
    }

    match parser.consume_token() {
        Token::Semicolon => (),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    let symbol = Symbol::Statement(Statement::Initialise(
        init_type,
        String::clone(identifier)
    ));
    parser.confirm_node(&symbol);

    return true;
}

fn match_statement_assign(parser: &mut Parser) -> bool {
    parser.start_node();

    let identifier = match parser.consume_token() {
        Token::Identifier(identifier) => identifier,
        _ => {
            parser.discard_node();
            return false;
        }
    };

    match parser.consume_token() {
        Token::Equals => (),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    if !match_expression(parser) {
        parser.discard_node();
        return false;
    }

    match parser.consume_token() {
        Token::Semicolon => (),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    let symbol = Symbol::Statement(Statement::Assign(String::clone(identifier)));
    parser.confirm_node(&symbol);

    return true;
}

fn match_statement_return(parser: &mut Parser) -> bool {
    parser.start_node();
    match parser.consume_token() {
        Token::Keyword(keyword) => match keyword {
            Keyword::Return => (),
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

    if !match_expression(parser) {
        parser.discard_node();
        return false;
    }

    match parser.consume_token() {
        Token::Semicolon => (),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    println!("HERE");

    let symbol = Symbol::Statement(Statement::Return);
    parser.confirm_node(&symbol);

    return true;
}

pub fn match_statement(parser: &mut Parser) -> bool {
    if match_statement_declare(parser) {
        return true;
    }
    if match_statement_initialise(parser) {
        return true;
    }
    if match_statement_assign(parser) {
        return true;
    }
    if match_statement_return(parser) {
        return true;
    }
    return false;
}
