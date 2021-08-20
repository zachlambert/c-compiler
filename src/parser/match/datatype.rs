
use super::token::*;
use super::construct::*;
use super::parser::Parser;
use super::common::match_identifier;


fn match_qualifier(parser: &mut Parser) -> bool {
    parser.start_node();

    let qualifier = match parser.consume_token() {
        Token::Keyword(keyword) => match keyword {
            Keyword::Mut => {
                Qualifier::Mut
            }
            _ => {
                parser.discard_node();
                return false;
            }
        },
        _ => {
            parser.discard_node();
            return false;
        },
    };

    let construct = Construct::Qualifier(qualifier);
    parser.confirm_node(&construct);
    return true;
}

fn match_datatype_primitive(parser: &mut Parser) -> bool {
    parser.start_node();

    let primitive = match parser.consume_token() {
        Token::Keyword(keyword) => match keyword {
            Keyword::U8 => Primitive::U8,
            Keyword::U16 => Primitive::U16,
            Keyword::U32 => Primitive::U32,
            Keyword::U64 => Primitive::U64,
            Keyword::I8 => Primitive::I8,
            Keyword::I16 => Primitive::I16,
            Keyword::I32 => Primitive::I32,
            Keyword::I64 => Primitive::I64,
            Keyword::F32 => Primitive::F32,
            Keyword::F64 => Primitive::F64,
            Keyword::C8 => Primitive::C8,
            _ => {
                parser.discard_node();
                return false;
            }
        },
        _ => {
            parser.discard_node();
            return false;
        },
    };

    let construct = Construct::Primitive(primitive);
    parser.confirm_node(&construct);

    return true;
}

fn match_datatype_terminal(parser: &mut Parser) -> bool {
    if match_datatype_primitive(parser) {
        return true;
    }
    if match_identifier(parser) {
        return true;
    }
    return false;
}

// Note: won't match inferred datatype

pub fn match_datatype(parser: &mut Parser) -> bool {
    parser.start_node();

    loop {
        if !match_qualifier(parser) {
            break;
        }
    }

    let datatype = match parser.peek_token() {
        Token::Ampersand => {
            parser.consume_token();
            if !match_datatype(parser) {
                panic!("Expected data type after &");
            }
            Datatype::Pointer
        },
        _ => {
            if !match_datatype_terminal(parser) {
                parser.discard_node();
                return false;
            }
            Datatype::Terminal
        },
    };

    let construct = Construct::Datatype(datatype);
    parser.confirm_node(&construct);

    return true;
}
