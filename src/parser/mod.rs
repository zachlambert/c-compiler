
use std::fmt;

use crate::token::Token;
use crate::token::Keyword;
use crate::token::Constant;

#[derive(Clone)]
struct SymbolFunction {
    name: String,
    ret_type: String, // TODO: Use enum for this later
}

#[derive(Clone)]
struct SymbolArgument {
    name: String,
    arg_type: String, // TODO: Use enum for this later
}

#[derive(Clone)]
enum Symbol {
    Program,
    Function(SymbolFunction),
    Statement,
    Expression,
    Argument(SymbolArgument),
    List(String),
}

impl fmt::Display for Symbol {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Symbol::Program => write!(fmt, "Program"),
            Symbol::Function(function) =>
                write!(fmt, "Function: {}, {}", function.ret_type, function.name),
            Symbol::Statement => write!(fmt, "Statement"),
            Symbol::Expression => write!(fmt, "Expression"),
            Symbol::Argument(argument) =>
                write!(fmt, "Argument: {} {}", argument.arg_type, argument.name),
            Symbol::List(name) => write!(fmt, "List({})", name),
        }
    }
}

pub struct Node {
    symbol: Symbol,
    next: Option<usize>,
    child: Option<usize>,
}

pub struct Ast {
    pub nodes: Vec<Node>,
}

#[derive(Clone, Copy)]
struct ParserState {
    token_i: usize,
    node_i: usize, // In ast
}

impl ParserState {
    pub fn step_both(&mut self) {
        self.token_i+=1;
        self.node_i+=1;
    }
    pub fn step_node(&mut self) {
        self.node_i+=1;
    }
    pub fn step_token(&mut self) {
        self.token_i+=1;
    }
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            nodes: Vec::new(),
        }
    }

    pub fn set_node(&mut self, node_i: usize, symbol: &Symbol, children: Option<&Vec<usize>>) {
        if node_i == self.nodes.len() {
            self.nodes.push( Node {
                symbol: Symbol::clone(symbol),
                next: Option::None,
                child: Option::None,
            });
        } else if node_i < self.nodes.len() {
            self.nodes[node_i].symbol = Symbol::clone(symbol);
            self.nodes[node_i].child = Option::None;
            self.nodes[node_i].next = Option::None;
        } else {
            panic!("Unexpected node_i in Ast::set_node");
        }
        match children {
            Some(children) => {
                if children.len() > 0 {
                    self.nodes[node_i].child = Some(children[0]);
                    for i in 0..(children.len()-1) {
                        self.nodes[children[i]].next = Some(children[i+1]);
                    }
                }
            }
            None => (),
        }
    }
}

impl fmt::Display for Ast {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result
    {
        let mut stack: Vec<usize> = Vec::new();
        let mut depths: Vec<u8> = Vec::new();
        stack.push(self.nodes.len()-1);
        depths.push(0);
        loop {
            match stack.pop() {
                Some(node) => {
                    let depth = depths.pop()
                        .expect("Should have depth");
                    for j in 0..depth {
                        write!(fmt, "  ")?;
                    }
                    write!(fmt, "{}", self.nodes[node].symbol)?;
                    match self.nodes[node].next {
                        Some(next) => {
                            stack.push(next);
                            depths.push(depth);
                        },
                        None => (),
                    };
                    match self.nodes[node].child {
                        Some(child) => {
                            stack.push(child);
                            depths.push(depth+1);
                        },
                        None => (),
                    };

                    if stack.len() != 0 {
                        write!(fmt, "\n")?;
                    }
                },
                None => break,
            };
        }
        Ok(())
    }
}

fn match_expression(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;

    match &tokens[state.token_i] {
        Token::Constant(constant) => match constant {
            Constant::Int(_) => (),
            _ => return None,
        },
        _ => return None,
    };
    state.step_token();

    let symbol = Symbol::Expression;
    ast.set_node(state.node_i, &symbol, None);
    state.step_node();

    return Some(state);
}

fn match_statement(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    // Match return keyword

    match &tokens[state.token_i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Return => (),
            _ => return None,
        },
        _ => return None,
    };
    state.step_token();

    // Match expression

    match match_expression(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            nodes.push(state.node_i-1);
        },
        None => return None,
    }

    // Match semi-colon

    match &tokens[state.token_i] {
        Token::Semicolon => (),
        _ => return None,
    };
    state.step_token();

    let symbol = Symbol::Statement;
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

fn match_argument(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    let arg_type = match &tokens[state.token_i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Int => String::from("Int"), // TODO: Change to enum
            _ => return None,
        },
        _ => return None,
    };
    state.step_token();

    let name = match &tokens[state.token_i] {
        Token::Identifier(name) => name,
        _ => return None,
    };
    state.step_token();

    let symbol_argument = SymbolArgument {
        name: String::clone(name),
        arg_type: String::clone(&arg_type),
    };
    let symbol = Symbol::Argument(symbol_argument);
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

fn match_function(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    // Match int keyword

    let ret_type = match &tokens[state.token_i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Int => String::from("Int"), // TODO: Change to enum
            _ => return None,
        },
        _ => return None,
    };
    state.step_token();

    // Match function name identifier

    let name = match &tokens[state.token_i] {
        Token::Identifier(name) => name,
        _ => return None,
    };
    state.step_token();

    // Match LBrace

    match &tokens[state.token_i] {
        Token::LBrace => (),
        _ => return None,
    }
    state.step_token();

    match match_argument(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            nodes.push(state.node_i-1);
            loop {
                match &tokens[state.token_i] {
                    Token::Comma => (),
                    _ => break,
                };
                state.step_token();
                match match_argument(state, tokens, ast) {
                    Some(new_state) => {
                        state = new_state;
                        nodes.push(state.node_i-1);
                    },
                    None => panic!("Expected argument after comma"),
                };
            }
        },
        None => (),
    }

    match &tokens[state.token_i] {
        Token::RBrace => (),
        _ => return None,
    }
    state.step_token();

    match &tokens[state.token_i] {
        Token::LParen => (),
        _ => return None,
    }
    state.step_token();

    loop {
        match match_statement(state, tokens, ast) {
            Some(new_state) => {
                state = new_state;
                nodes.push(state.node_i-1);
            },
            None => break,
        }
    }

    match &tokens[state.token_i] {
        Token::RParen => (),
        _ => return None,
    }
    state.step_token();

    let symbol_function = SymbolFunction {
        name: String::clone(name),
        ret_type: String::clone(&ret_type),
    };
    let symbol = Symbol::Function(symbol_function);
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

fn match_program(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    // Match function
    match match_function(state , tokens, ast) {
        Some(new_state) => {
            state = new_state;
            nodes.push(state.node_i-1);
        },
        None => return None,
    };

    let symbol = Symbol::Program;
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state)
}

pub fn create_ast(tokens: &Vec<Token>) -> Option<Ast> {
    let mut ast = Ast::new();
    let state = ParserState {
        token_i: 0,
        node_i: 0,
    };
    if match_program(state, tokens, &mut ast).is_none() {
        return None;
    }
    Some(ast)
}
