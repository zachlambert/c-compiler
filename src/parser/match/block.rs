
use super::token::*;
use super::construct::*;
use super::parser::Parser;
use super::statement::match_statement;
use super::symbol::match_symbol;

pub fn match_block(parser: &mut Parser) -> bool {
    parser.start_node();

    // "{" , { statement | symbol } , "}"
    
    match parser.consume_token() {
        Token::LCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        }
    }

    loop {
        if match_statement(parser) {
            continue;
        }
        if match_symbol(parser) {
            continue;
        }
        break;
    }

    match parser.consume_token() {
        Token::RCBracket => (),
        _ => {
            parser.discard_node();
            return false;
        }
    }

    let construct = Construct::Block;
    parser.confirm_node(&construct);

    return true;
}
