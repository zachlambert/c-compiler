
use std::fmt;

use crate::token::Token;
use crate::token::Keyword;
use crate::token::Constant;
use crate::token::Primitive;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub ret_type: Type,
}

impl fmt::Display for Function {
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
pub struct Argument {
    pub name: String,
    pub arg_type: Type,
}

impl fmt::Display for Argument {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Argument[arg_type: {}, name: {}]", self.arg_type, self.name)
    }
}


#[derive(Clone)]
pub enum UnaryOp {
    Negate,
    LogicalNot,
}

impl fmt::Display for UnaryOp {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Negate => write!(fmt, "UnaryOp(Negate)"),
            LogicalNot => write!(fmt, "UnaryOp(LogicalNot)"),
        }
    }
}

#[derive(Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    LogicalAnd,
    LogicalOr,
}

impl fmt::Display for BinaryOp {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Add => write!(fmt, "BinaryOp(Add)"),
            Subtract => write!(fmt, "BinaryOp(Subtract)"),
            Multiply => write!(fmt, "BinaryOp(Multiply)"),
            Divide => write!(fmt, "BinaryOp(Divide)"),
            LogicalAnd => write!(fmt, "BinaryOp(LogicalAnd)"),
            LogicalOr => write!(fmt, "BinaryOp(LogicalOr)"),
        }
    }
}

#[derive(Clone)]
pub enum Statement {
    Declare(Type, String),    // <type> <identifier>;
    Initialise(Type, String), // <type> <identifier> = <expression>;
    Assign(String),           // <identifier> = <expression>;
    Return,                   // return <expression>;
}

impl fmt::Display for Statement {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Declare(statement_type, identifier) =>
                write!(fmt,
                       "Statement[declare, type: {}, identifier: {}]",
                       statement_type, identifier),
            Statement::Initialise(statement_type, identifier) =>
                write!(fmt,
                       "Statement[initialise, type: {}, identifier: {}]",
                       statement_type, identifier),
            Statement::Assign(identifier) =>
                write!(fmt,
                       "Statement[assign, identifier: {}]",
                       identifier),
            Statement::Return =>
                write!(fmt, "Statement[return]"),
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Function(String), // n child expressions for args
    UnaryOp(UnaryOp), // one child expression
    BinaryOp(BinaryOp), // Two child expressions
    Constant(Constant), // Terminal
    Identifier(String), // Terminal
}

impl fmt::Display for Expression {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Function(name) => write!(fmt, "Expression(function, name: {})", name),
            Expression::UnaryOp(op) => write!(fmt, "Expression({})", op),
            Expression::BinaryOp(op) => write!(fmt, "Expression({})", op),
            Expression::Constant(constant) => write!(fmt, "Expression({})", constant),
            Expression::Identifier(identifier) => write!(fmt, "Expression(Identifier({}))", identifier),
        }
    }
}

#[derive(Clone)]
pub enum Symbol {
    Program,
    Function(Function),
    Statement(Statement),
    Expression(Expression),
    Argument(Argument),
}

impl fmt::Display for Symbol {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Program => write!(fmt, "Program"),
            Symbol::Function(function) => write!(fmt, "{}", function),
            Symbol::Statement(statement) => write!(fmt, "{}", statement),
            Symbol::Expression(expression) => write!(fmt, "{}", expression),
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

fn match_expression_binary_op(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    None
}

fn match_expression_unary_op(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    None
}

fn match_expression_function(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    None
}

fn match_expression_terminal(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;

    let expression = match &tokens[state.token_i] {
        Token::Constant(constant) => Expression::Constant(Constant::clone(constant)),
        Token::Identifier(identifier) => Expression::Identifier(String::clone(identifier)),
        _ => return None,
    };
    state.step_token();

    let symbol = Symbol::Expression(expression);
    ast.set_node(state.node_i, &symbol, None);
    state.step_node();

    return Some(state);
}

fn match_expression(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    match match_expression_binary_op(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }
    match match_expression_unary_op(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }
    match match_expression_function(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }
    match match_expression_terminal(start, tokens, ast) {
        Some(state) => return Some(state),
        None => (),
    }
    return None;
}

fn match_statement_declare(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
    let mut state = start;

    let init_type = match &tokens[state.token_i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => return None,
        }
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => return None,
    };
    state.step_token();

    let identifier = match &tokens[state.token_i] {
        Token::Identifier(identifier) => identifier,
        _ => return None,
    };
    state.step_token();

    match &tokens[state.token_i] {
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

    let init_type = match &tokens[state.token_i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Primitive(primitive) => Type::Primitive(Primitive::clone(primitive)),
            _ => return None,
        }
        Token::Identifier(identifier) => Type::Identifier(String::clone(identifier)),
        _ => return None,
    };
    state.step_token();

    let identifier = match &tokens[state.token_i] {
        Token::Identifier(identifier) => identifier,
        _ => return None,
    };
    state.step_token();

    match &tokens[state.token_i] {
        Token::Equals => (),
        _ => return None,
    };
    state.step_token();

    match match_expression(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            nodes.push(state.node_i-1);
        },
        None => return None,
    }

    match &tokens[state.token_i] {
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

    let identifier = match &tokens[state.token_i] {
        Token::Identifier(identifier) => identifier,
        _ => return None,
    };
    state.step_token();

    match &tokens[state.token_i] {
        Token::Equals => (),
        _ => return None,
    };
    state.step_token();

    match match_expression(state, tokens, ast) {
        Some(new_state) => {
            state = new_state;
            nodes.push(state.node_i-1);
        },
        None => return None,
    }

    match &tokens[state.token_i] {
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

    match &tokens[state.token_i] {
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
            nodes.push(state.node_i-1);
        },
        None => return None,
    }

    match &tokens[state.token_i] {
        Token::Semicolon => (),
        _ => return None,
    };
    state.step_token();

    let symbol = Symbol::Statement(Statement::Return);
    ast.set_node(state.node_i, &symbol, Some(&nodes));
    state.step_node();

    return Some(state);
}

fn match_statement(start: ParserState, tokens: &Vec<Token>, ast: &mut Ast) -> Option<ParserState> {
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

    let function = Function {
        name: String::clone(name),
        ret_type: ret_type,
    };
    let symbol = Symbol::Function(function);
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
