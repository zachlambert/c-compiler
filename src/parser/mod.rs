
use crate::token::Token;
use crate::token::Keyword;
use crate::token::Constant;

#[derive(Clone)]
struct SymbolFunction {
    name: String,
}

#[derive(Clone)]
enum Symbol {
    Program,
    Function(SymbolFunction),
    Keyword(Keyword),
    Constant(Constant),
}

pub struct Node {
    symbol: Symbol,
    parent: Option<usize>,
    children: Vec<usize>,
}

pub struct Ast {
    pub nodes: Vec<Node>,
}

struct MatchResult {
    i: usize,
    node_i: usize, // In ast
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            nodes: Vec::new(),
        }
    }
    pub fn add_node(&mut self, symbol: &Symbol) -> usize {
        self.nodes.push( Node {
            symbol: Symbol::clone(symbol),
            parent: Option::None,
            children: Vec::new(),
        });
        return self.nodes.len() - 1;
    }
}

fn match_expression(start: usize, tokens: &Vec<Token>) -> Option<usize> {
    let mut i = start;

    // Match int constant

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::Constant(constant) => match constant {
            Constant::Int(_) => i+=1,
            _ => return None,
        },
        _ => return None,
    };

    return Some(i);
}

fn match_statement(start: usize, tokens: &Vec<Token>) -> Option<usize> {
    let mut i = start;

    // Match return keyword

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Return => i+=1,
            _ => return None,
        },
        _ => return None,
    };

    // Match expression

    match match_expression(i, tokens) {
        Some(new_i) => i = new_i,
        None => return None,
    }

    // Match semi-colon

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::Semicolon => i+=1,
        _ => return None,
    };

    return Some(i);
}

fn match_function(start: usize, tokens: &Vec<Token>, ast: &mut Ast) -> Option<MatchResult> {
    let mut i = start;

    // Match int keyword

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::Keyword(keyword) => match keyword {
            Keyword::Int => i+=1,
            _ => return None,
        },
        _ => return None,
    };

    // Match function name identifier

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::Identifier(_) => i+=1,
        _ => return None,
    }

    // Match LBrace

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::LBrace => i+=1,
        _ => return None,
    }

    // Match RBrace

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::RBrace => i+=1,
        _ => return None,
    }

    // Match LParen

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::LParen => i+=1,
        _ => return None,
    }

    // Match statement

    match match_statement(i, tokens) {
        Some(new_i) => i = new_i,
        None => return None,
    }

    // Match RParen

    if i==tokens.len() { return None; }
    match &tokens[i] {
        Token::RParen => i+=1,
        _ => return None,
    }

    let symbol_function = SymbolFunction {
        name: String::from("TODO"),
    };
    let symbol = Symbol::Function(symbol_function);
    let node_i = ast.add_node(&symbol);

    // ast.nodes[node_i].children.push(function_i);

    // ast.nodes[function_i].parent = node_i;

    return Some(MatchResult{
        i: i,
        node_i: node_i
    });
}

fn match_program(start: usize, tokens: &Vec<Token>, ast: &mut Ast) -> Option<MatchResult> {
    let mut i = start;

    // Match function
    let function_i: usize;
    match match_function(i , tokens, ast) {
        Some(result) => {
            i = result.i;
            function_i = result.node_i;
        },
        None => return None,
    };

    let symbol = Symbol::Program;
    let node_i = ast.add_node(&symbol);
    ast.nodes[function_i].parent = Some(node_i);
    ast.nodes[node_i].children.push(function_i);

    return Some(MatchResult{
        i: i,
        node_i: node_i
    });
}

pub fn create_ast(tokens: &Vec<Token>) -> Option<Ast> {
    let mut ast = Ast::new();
    if match_program(0, tokens, &mut ast).is_none() {
        return None;
    }
    Some(ast)
}
