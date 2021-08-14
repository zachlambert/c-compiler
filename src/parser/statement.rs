
use crate::lexer::token::*;
use super::ast::*;
use super::symbol::*;

use super::expression::match_expression;

fn match_statement_declare(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;

    let init_type = match state.peek_token(tokens) {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => return None,
        }
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => return None,
    };
    state.step_token();

    let identifier = match state.peek_token(tokens) {
        Token::Identifier(identifier) => identifier,
        _ => return None,
    };
    state.step_token();

    match state.peek_token(tokens) {
        Token::Semicolon => (),
        _ => return None,
    };
    state.step_token();

    let symbol = Symbol::Statement(Statement::Declare(
        init_type,
        String::clone(identifier)
    ));
    ast.set_node(state.node_i, &symbol, None);
    state.step_node();

    return Some(state);
}

fn match_statement_initialise(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    let init_type = match state.peek_token(tokens) {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => return None,
        }
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => return None,
    };
    state.step_token();

    let identifier = match state.peek_token(tokens) {
        Token::Identifier(identifier) => identifier,
        _ => return None,
    };
    state.step_token();

    match state.peek_token(tokens) {
        Token::Equals => (),
        _ => return None,
    };
    state.step_token();

    match match_expression(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            state.push_last_node(&mut nodes);
        },
        None => return None,
    }

    match state.peek_token(tokens) {
        Token::Semicolon => (),
        _ => return None,
    };
    state.step_token();

    let symbol = Symbol::Statement(Statement::Initialise(
        init_type,
        String::clone(identifier)
    ));
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

fn match_statement_assign(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    let identifier = match state.peek_token(tokens) {
        Token::Identifier(identifier) => identifier,
        _ => return None,
    };
    state.step_token();

    match state.peek_token(tokens) {
        Token::Equals => (),
        _ => return None,
    };
    state.step_token();

    match match_expression(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            state.push_last_node(&mut nodes);
        },
        None => return None,
    }

    match state.peek_token(tokens) {
        Token::Semicolon => (),
        _ => return None,
    };
    state.step_token();

    let symbol = Symbol::Statement(Statement::Assign(String::clone(identifier)));
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

fn match_statement_return(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    match state.peek_token(tokens) {
        Token::Keyword(keyword) => match keyword {
            Keyword::Return => (),
            _ => return None,
        },
        _ => return None,
    };
    state.step_token();

    match match_expression(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            state.push_last_node(&mut nodes);
        },
        None => return None,
    }

    match state.peek_token(tokens) {
        Token::Semicolon => (),
        _ => return None,
    };
    state.step_token();

    let symbol = Symbol::Statement(Statement::Return);
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

pub fn match_statement(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    match match_statement_declare(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }
    match match_statement_initialise(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }
    match match_statement_assign(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }
    match match_statement_return(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }
    return None;
}
