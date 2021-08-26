
use super::token::*;
use super::construct::*;
use super::parser::Parser;
use super::datatype::match_datatype;
use super::block::match_block;


fn match_argument(parser: &mut Parser) -> bool {
    parser.start_node();

    // identifier , ":" , datatype

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

    let construct = Construct::Argument(String::clone(name));
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

    // identifier , ":" , "function" , ...

    let name = match parser.consume_token() {
        Token::Identifier(identifier) => identifier,
        _ => {
            parser.discard_node();
            return false;
        }
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

    // ... , "(" , [ argument , { "," , argument } ] , ")" ,

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

    // block

    if !match_block(parser) {
        parser.discard_node();
        return false;
    }

    let construct = Construct::Function(String::clone(name));
    parser.confirm_node(&construct);

    return true;
}
