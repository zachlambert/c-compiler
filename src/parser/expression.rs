
use crate::lexer::token::*;
use super::construct::*;
use super::parser::Parser;

fn match_expression_function(parser: &mut Parser) -> bool {
    parser.start_node();

    let identifier = match parser.consume_token() {
        Token::Identifier(identifier) => identifier,
        _ => {
            parser.discard_node();
            return false;
        }
    };

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

    let construct = Construct::Expression(Expression::Function(String::clone(identifier)));
    parser.confirm_node(&construct);

    return true;
}

// Match an expression that can be evaluated without needing to look at further tokens
fn match_expression_enclosed(parser: &mut Parser) -> bool {
    // Only enclosed as you approach from the left
    if match_expression_unary_op(parser) {
        return true;
    }

    if match_expression_function(parser) {
        return true;
    }

    // Check for (<expresion>)
    let expression = match parser.consume_token() {
        Token::LParen => {
            if !match_expression(parser) {
                panic!("No expression within ()");
            }
            match parser.consume_token() {
                Token::RParen => (),
                _ => panic!("No closing )"),
            }
            return true;
        }
        Token::Constant(constant) => Expression::Constant(Constant::clone(constant)),
        Token::Identifier(identifier) => Expression::Identifier(String::clone(identifier)),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    // Create node for Constant or Identifier
    parser.start_node();
    let construct = Construct::Expression(expression);
    parser.confirm_node(&construct);

    return true;
}

fn match_binary_op(parser: &mut Parser) -> Option<(BinaryOp, u8)> {
    let (op, priority) = match parser.consume_token() {
        Token::Plus => (BinaryOp::Add, 53),
        Token::Minus => (BinaryOp::Subtract, 52),
        Token::Asterisk => (BinaryOp::Multiply, 51),
        Token::RSlash => (BinaryOp::Divide, 51),
        Token::Ampersand =>
            match parser.peek_token() {
                Token::Ampersand => {
                    parser.consume_token();
                    (BinaryOp::LogicalAnd, 42)
                },
                _ => (BinaryOp::BitwiseAnd, 31),
            },
        Token::VBar =>
            match parser.peek_token() {
                Token::VBar => {
                    parser.consume_token();
                    (BinaryOp::LogicalOr, 43)
                },
                _ => (BinaryOp::BitwiseOr, 32),
            },
        _ => return None,
    };
    Some((op, priority))
}

fn match_expression_binary_chain(parser: &mut Parser, priority: u8) -> bool {
    if !match_expression_enclosed(parser) {
        parser.discard_node();
        return false;
    }

    parser.stash_state();
    let (op, op_priority) = match match_binary_op(parser) {
        Some((op, op_priority)) => (op, op_priority),
        None => {
            parser.rollback_state();
            return true;
        }
    };

    if op_priority >= priority {
        parser.rollback_state();
        return true;
    }

    parser.start_node_with_prev(1);

    // Higher priority. Create binary expression, then return.
    if !match_expression_binary_chain(parser, op_priority) {
        panic!("Missing expression after binary operation");
    }

    let construct = Construct::Expression(Expression::BinaryOp(op));
    parser.confirm_node(&construct);

    return true;
}

fn match_expression_unary_op(parser: &mut Parser) -> bool {
    parser.start_node();

    let (unary_op, priority) = match parser.consume_token() {
        Token::Minus => (UnaryOp::Negate, 52),
        Token::Exclamation => (UnaryOp::LogicalNot, 41),
        _ => {
            parser.discard_node();
            return false;
        }
    };

    if !match_expression_binary_chain(parser, priority) {
        panic!("Expected expression after unary opeation");
    }

    let construct = Construct::Expression(Expression::UnaryOp(unary_op));
    parser.confirm_node(&construct);

    return true;
}

pub fn match_expression(parser: &mut Parser) -> bool {
    if !match_expression_enclosed(parser) {
        parser.discard_node();
        return false;
    }

    // Find binary operations
    loop {
        parser.stash_state();
        let (op, priority) = match match_binary_op(parser) {
            Some((op, priority)) => (op, priority),
            None => {
                parser.rollback_state();
                break;
            }
        };
        parser.start_node_with_prev(1);
        if !match_expression_binary_chain(parser, priority) {
            panic!("Missing expression after binary operator");
        }

        let construct = Construct::Expression(Expression::BinaryOp(op));
        parser.confirm_node(&construct);
    }

    return true;
}
