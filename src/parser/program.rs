
use crate::lexer::token::*;
use super::symbol::*;
use super::ast::*;

use super::statement::match_statement;

fn match_argument(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;

    let arg_type = match state.peek_token(tokens) {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => return None,
        },
        _ => return None,
    };
    state.step_token();

    let name = match state.peek_token(tokens) {
        Token::Identifier(name) => name,
        _ => return None,
    };
    state.step_token();

    let argument = Argument {
        name: String::clone(name),
        arg_type: arg_type,
    };
    let symbol = Symbol::Argument(argument);
    ast.set_node(state.node_i, &symbol, None);
    state.step_node();

    return Some(state);
}

fn match_function(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    let ret_type = match state.peek_token(tokens) {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => panic!("Invalid return type for function"),
        },
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => return None,
    };
    state.step_token();

    let name = match state.peek_token(tokens) {
        Token::Identifier(name) => name,
        _ => return None,
    };
    state.step_token();

    match state.peek_token(tokens) {
        Token::LParen => (),
        _ => return None,
    }
    state.step_token();

    match match_argument(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            state.push_last_node(&mut nodes);
            loop {
                match state.peek_token(tokens) {
                    Token::Comma => (),
                    _ => break,
                };
                state.step_token();
                match match_argument(state, tokens, ast) {
                    Some(new_state) => {
                        state = new_state;
                        state.push_last_node(&mut nodes);
                    },
                    None => panic!("Expected argument after comma"),
                };
            }
        },
        None => (),
    }

    match state.peek_token(tokens) {
        Token::RParen => (),
        _ => return None,
    }
    state.step_token();

    match state.peek_token(tokens) {
        Token::LCBracket => (),
        _ => return None,
    }
    state.step_token();

    loop {
        match match_statement(state, tokens, ast) {
            Some(new_state) => {
                state = new_state;
                state.push_last_node(&mut nodes);
            },
            None => break,
        }
    }

    match state.peek_token(tokens) {
        Token::RCBracket => (),
        _ => return None,
    }
    state.step_token();

    let function = Function {
        name: String::clone(name),
        ret_type: ret_type,
    };
    let symbol = Symbol::Function(function);
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

pub fn match_program(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    loop {
        match match_function(state , tokens, ast) {
            Some(new_state) => {
                state = new_state;
                state.push_last_node(&mut nodes);
            },
            None => break,
        };
    }

    let symbol = Symbol::Program;
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state)
}

