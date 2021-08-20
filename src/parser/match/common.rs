
use super::token::*;
use super::construct::*;
use super::parser::Parser;


pub fn match_identifier(parser: &mut Parser) -> bool {
    parser.start_node();
    let identifier = match parser.consume_token() {
        Token::Identifier(identifier) => identifier,
        _ => {
            parser.discard_node();
            return false;
        }
    };

    let construct = Construct::Identifier(String::clone(identifier));
    parser.confirm_node(&construct);

    return true;
}
