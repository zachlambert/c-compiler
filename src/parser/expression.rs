
use crate::lexer::token::*;
use super::symbol::*;
use super::ast::*;

fn match_expression_function(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    let identifier = match state.peek_token(tokens) {
        Token::Identifier(identifier) => identifier,
        _ => return None,
    };
    state.step_token();

    match state.peek_token(tokens) {
        Token::LParen => (),
        _ => return None,
    };
    state.step_token();

    loop {
        match match_expression(state, tokens, ast) {
            Some(new_state) => {
                state = new_state;
                state.push_last_node(&mut nodes);
            },
            None => break,
        };
        loop {
            match state.peek_token(tokens) {
                Token::Comma => (),
                _ => break,
            };
            state.step_token();
            match match_expression(state, tokens, ast) {
                Some(new_state) => {
                    state = new_state;
                    state.push_last_node(&mut nodes);
                },
                None => panic!("Expected expression after comma in function call"),
            };
        }
        break;
    }

    match state.peek_token(tokens) {
        Token::RParen => (),
        _ => panic!("Function missing closing )"),
    };
    state.step_token();

    let symbol = Symbol::Expression(Expression::Function(String::clone(identifier)));
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

// Match an expression that can be evaluated without needing to look at further tokens
fn match_expression_enclosed(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;

    // Only enclosed as you approach from the left
    match match_expression_unary_op(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }

    match match_expression_function(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }

    // Check for (<expresion>)
    let expression = match state.peek_token(tokens) {
        Token::LParen => {
            state.step_token();
            match match_expression(state, tokens, ast) {
                Some(new_state) => {
                    state = new_state
                }
                None => panic!("No expression within ()"),
            }
            match state.peek_token(tokens) {
                Token::RParen => (),
                _ => panic!("No closing )"),
            }
            state.step_token();
            return Some(state);
        }
        Token::Constant(constant) => Expression::Constant(Constant::clone(constant)),
        Token::Identifier(identifier) => Expression::Identifier(String::clone(identifier)),
        _ => return None,
    };
    state.step_token();

    // If here, encountered a constant or identifier, so need to create a new
    // node for these.
    let symbol = Symbol::Expression(expression);
    ast.set_node(state.node_i, &symbol, None);
    state.step_node();

    return Some(state);
}

fn match_binary_op(start: ParserState, tokens: &Vec<Token>) -> Option<(ParserState, BinaryOp, u8)> {
    let mut state = start;
    let (op, priority) = match state.peek_token(tokens) {
        Token::Plus => (BinaryOp::Add, 53),
        Token::Minus => (BinaryOp::Subtract, 52),
        Token::Asterisk => (BinaryOp::Multiply, 51),
        Token::RSlash => (BinaryOp::Divide, 51),
        Token::Ampersand =>
            match &tokens[state.token_i+1] {
                Token::Ampersand => {
                    state.step_token();
                    (BinaryOp::LogicalAnd, 42)
                },
                _ => (BinaryOp::BitwiseAnd, 31),
            },
        Token::VBar =>
            match &tokens[state.token_i+1] {
                Token::VBar => {
                    state.step_token();
                    (BinaryOp::LogicalOr, 43)
                },
                _ => (BinaryOp::BitwiseOr, 32),
            },
        _ => return None,
    };
    state.step_token();
    Some((state, op, priority))
}

fn match_expression_binary_chain(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast, priority: u8) -> Option<ParserState> {
    let mut state = start;

    match match_expression_enclosed(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
        }
        None => return None,
    }

    let (new_state, op, op_priority) = match match_binary_op(state, tokens) {
        Some((new_state, op, op_priority)) => (new_state, op, op_priority),
        None => return Some(state)
    };

    if op_priority >= priority {
        // Don't change state, leave as it is, just after matching enclosed expression
        // The node will be available as state.node_i - 1
        return Some(state);
    }

    state = new_state;

    // Higher priority. Create binary expression, then return.
    let mut nodes: Vec<usize> = Vec::new();
    state.push_last_node(&mut nodes);

    state = match_expression_binary_chain(state, tokens, ast, op_priority)
        .expect("Missing expression after binary operation");
    state.push_last_node(&mut nodes);

    let symbol = Symbol::Expression(Expression::BinaryOp(op));
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    Some(state)
}

fn match_expression_unary_op(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    let (unary_op, priority) = match &tokens[start.token_i] {
        Token::Minus => (UnaryOp::Negate, 52),
        Token::Exclamation => (UnaryOp::LogicalNot, 41),
        _ => return None,
    };
    state.step_token();

    let mut state = match_expression_binary_chain(state, tokens, ast, priority)
        .expect("Expected expression after unary opeation");
    state.push_last_node(&mut nodes);

    let symbol = Symbol::Expression(Expression::UnaryOp(unary_op));
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    Some(state)
}

pub fn match_expression(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    match match_expression_enclosed(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            state.push_last_node(&mut nodes);
        },
        None => return None,
    }

    // Find binary operations
    loop {
        let (new_state, op, op_priority) = match match_binary_op(state, tokens) {
            Some((new_state, op, op_priority)) => (new_state, op, op_priority),
            None => break,
        };
        state = new_state;
        state = match_expression_binary_chain(state, tokens, ast, op_priority)
            .expect("Missing expression after binary operator");
        state.push_last_node(&mut nodes);

        let symbol = Symbol::Expression(Expression::BinaryOp(op));
        ast.set_node(state.node_i, &symbol, Some(&nodes));
        state.step_node();
        nodes.clear();
        state.push_last_node(&mut nodes);
    }

    Some(state)
}
