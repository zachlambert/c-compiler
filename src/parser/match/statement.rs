
use crate::lexer::token::*;
use super::construct::*;
use super::parser::Parser;

use super::expression::match_expression;


fn match_statement_assign(parser: &mut Parser) -> bool {
    parser.start_node();

    // <expression> = <expression> ;

    if !match_expression(parser) {
        parser.discard_node();
        return false;
    }

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

    let construct = Construct::Statement(Statement::Assign);
    parser.confirm_node(&construct);

    return true;
}

fn match_statement_return(parser: &mut Parser) -> bool {
    parser.start_node();

    // return <expression> ;

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

    let construct = Construct::Statement(Statement::Return);
    parser.confirm_node(&construct);

    return true;
}

pub fn match_statement(parser: &mut Parser) -> bool {
    if match_statement_assign(parser) {
        return true;
    }
    if match_statement_return(parser) {
        return true;
    }
    return false;
}
