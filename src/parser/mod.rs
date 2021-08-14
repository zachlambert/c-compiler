
use std::fmt;

use crate::token::Token;
use crate::token::Keyword;
use crate::token::Constant;
use crate::token::Primitive;

#[derive(Clone)]
pub struct SymbolFunction {
    pub name: String,
    pub ret_type: Type,
}

impl fmt::Display for SymbolFunction {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Function[ret_type: {}, name: {}]", self.ret_type, self.name)
    }
}

#[derive(Clone)]
pub enum Type {
    Primitive(Primitive),
    Identifier(String),
}

impl fmt::Display for Type {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Primitive(primitive) => write!(fmt, "Type({})", primitive),
            Type::Identifier(identifier) => write!(fmt, "Type({})", identifier),
        }
    }
}

#[derive(Clone)]
pub struct SymbolArgument {
    pub name: String,
    pub arg_type: Type,
}

impl fmt::Display for SymbolArgument {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Argument[arg_type: {}, name: {}]", self.arg_type, self.name)
    }
}

#[derive(Clone)]
pub enum Symbol {
    Program,
    Function(SymbolFunction),
    Statement,
    Expression,
    Argument(SymbolArgument),
}

impl fmt::Display for Symbol {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Program => write!(fmt, "Program"),
            Symbol::Function(function) => write!(fmt, "{}", function),
            Symbol::Statement => write!(fmt, "Statement"),
            Symbol::Expression => write!(fmt, "Expression"),
            Symbol::Argument(argument) => write!(fmt, "{}", argument),
        }
    }
}

pub struct Node {
    pub symbol: Symbol,
    pub next: Option<usize>,
    pub child: Option<usize>,
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
                    for _ in 0..depth {
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

    let arg_type = match &tokens[state.token_i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
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
        arg_type: Type::clone(&arg_type),
    };
    let symbol = Symbol::Argument(symbol_argument);
    ast.set_node(state.node_i, &symbol, None);
    state.step_node();

    return Some(state);
}

fn match_function(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;
    let mut nodes: Vec<usize> = Vec::new();

    let ret_type = match &tokens[state.token_i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => panic!("Invalid return type for function"),
        },
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => return None,
    };
    state.step_token();

    let name = match &tokens[state.token_i] {
        Token::Identifier(name) => name,
        _ => return None,
    };
    state.step_token();

    match &tokens[state.token_i] {
        Token::LParen => (),
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
        Token::RParen => (),
        _ => return None,
    }
    state.step_token();

    match &tokens[state.token_i] {
        Token::LCBracket => (),
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
        Token::RCBracket => (),
        _ => return None,
    }
    state.step_token();

    let symbol_function = SymbolFunction {
        name: String::clone(name),
        ret_type: Type::clone(&ret_type),
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
        None => {
            println!("Failed to match function");
        },
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
        println!("Failed to match program");
        return None;
    }
    Some(ast)
}
