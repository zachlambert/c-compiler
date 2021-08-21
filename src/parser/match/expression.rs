
use super::token::*;
use super::construct::*;
use super::parser::Parser;
use super::common::match_identifier;


fn match_expression_function(parser: &mut Parser) -> bool {
    parser.start_node();

    if !match_identifier(parser) {
        parser.discard_node();
        return false;
    }

    match parser.consume_token() {
        Token::LParen => (),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    loop {
        if !match_expression(parser) {
            break;
        };
        loop {
            match parser.consume_token() {
                Token::Comma => (),
                _ => break,
            };
            if !match_expression(parser) {
                panic!("Expected expression after comma in function call");
            };
        }
        break;
    }

    match parser.consume_token() {
        Token::RParen => (),
        _ => panic!("Function missing closing )"),
    };

    let construct = Construct::Expression(Expression::Function);
    parser.confirm_node(&construct);

    return true;
}

pub fn match_expression_parentheses(parser: &mut Parser) -> bool {
    match parser.peek_token() {
        Token::LParen => {
            parser.consume_token();
            if !match_expression(parser) {
                panic!("No expression within ()");
            }
            match parser.consume_token() {
                Token::RParen => (),
                _ => panic!("No closing )"),
            }
            return true;
        }
        _ => return false,
    }
}

pub fn match_expression_constant(parser: &mut Parser) -> bool {
    parser.start_node();

    let constant = match parser.consume_token() {
        Token::Constant(constant) => constant,
        _ => {
            parser.discard_node();
            return false;
        },
    };

    let construct = Construct::Expression(Expression::Constant(Constant::clone(constant)));
    parser.confirm_node(&construct);
    return true;
}

pub fn match_expression_identifier(parser: &mut Parser) -> bool {
    parser.start_node();

    if !match_identifier(parser) {
        parser.discard_node();
        return false;
    }

    let construct = Construct::Expression(Expression::Identifier);
    parser.confirm_node(&construct);
    return true;
}

// Match an expression that can be evaluated without needing to look at further tokens
fn match_expression_enclosed(parser: &mut Parser) -> bool {
    if match_expression_unary_op(parser) {
        return true;
    }
    if match_expression_function(parser) {
        return true;
    }
    if match_expression_parentheses(parser) {
        return true;
    }
    if match_expression_constant(parser) {
        return true;
    }
    if match_expression_identifier(parser) {
        return true;
    }
    return false;
}

fn match_binary_op(parser: &mut Parser) -> Option<(BinaryOp, u8)> {
    let (op, priority) = match parser.consume_token() {
        Token::Ampersand => {
            match parser.peek_token() {
                Token::Ampersand => {
                    parser.consume_token();
                    (BinaryOp::LogicalAnd, 42)
                }
                _ => (BinaryOp::BitwiseAnd, 31)
            }
        }
        Token::VBar => {
            match parser.peek_token() {
                Token::VBar => {
                    parser.consume_token();
                    (BinaryOp::LogicalOr, 43)
                }
                _ => (BinaryOp::BitwiseOr, 32)
            }
        }
        Token::Equals => {
            match parser.peek_token() {
                Token::Equals => {
                    parser.consume_token();
                    (BinaryOp::LogicalEquals, 41)
                },
                _ => return None,
            }
        }
        Token::Plus => (BinaryOp::Add, 53),
        Token::Minus => (BinaryOp::Subtract, 52),
        Token::Asterisk => (BinaryOp::Multiply, 51),
        Token::RSlash => (BinaryOp::Divide, 51),

        Token::Period => (BinaryOp::Access, 1),
        _ => return None,
    };
    Some((op, priority))
}

fn match_expression_unary_op(parser: &mut Parser) -> bool {
    parser.stash_state();
    let (unary_op, priority) = match parser.consume_token() {
        Token::Minus => (UnaryOp::Negate, 52),
        Token::Exclamation => (UnaryOp::LogicalNot, 41),
        Token::Ampersand => (UnaryOp::Ref, 11),
        Token::Asterisk => (UnaryOp::Deref, 11),
        _ => {
            parser.rollback_state();
            return false;
        }
    };

    if !match_expression_enclosed(parser) {
        panic!("Missing expression after unary operator");
    }

    match_binary_expression(parser, priority);

    parser.start_node_with_prev(1);
    let construct = Construct::Expression(Expression::UnaryOp(unary_op));
    parser.confirm_node(&construct);

    return true;
}

pub fn match_binary_expression(parser: &mut Parser, priority: u8) {
    parser.stash_state();
    let (op, op_priority) = match match_binary_op(parser) {
        Some((op, priority)) => (op, priority),
        None => {
            parser.rollback_state();
            return;
        }
    };

    if op_priority >= priority {
        parser.rollback_state();
        return;
    }

    parser.start_node_with_prev(1);
    // Add node for right
    if !match_expression_enclosed(parser) {
        panic!("Missing expression after binary operator");
    }
    match_binary_expression(parser, op_priority);

    let construct = Construct::Expression(Expression::BinaryOp(op));
    parser.confirm_node(&construct);

    match_binary_expression(parser, priority);
}

pub fn match_expression(parser: &mut Parser) -> bool {
    if !match_expression_enclosed(parser) {
        return false;
    }
    match_binary_expression(parser, 255);
    return true;
}
