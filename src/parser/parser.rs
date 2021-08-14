
use crate::lexer::token::Token;

use super::ast::Ast;
use super::symbol::Symbol;

#[derive(Clone, Copy)]
struct State {
    token_i: usize,
    node_i: usize,
    child_i: usize,
}

pub struct Parser<'a> {
    ast: &'a mut Ast,
    tokens: &'a Vec<Token>,
    state_stack: Vec<State>,
    children: Vec<usize>,
    state: State,
}

impl<'a> Parser<'a> {
    pub fn new(
        ast: &'a mut Ast,
        tokens: &'a Vec<Token>,
        max_stack_size: usize,
        max_num_children: usize) -> Parser<'a>
    {
        let mut parser = Parser {
            ast: ast,
            tokens: tokens,
            state_stack: Vec::new(),
            children: Vec::new(),
            state: State { 
                token_i: 0,
                node_i: 0,
                child_i: 0,
            },
        };
        parser.state_stack.reserve(max_stack_size);
        parser.children.reserve(max_num_children);
        parser
    }

    pub fn start_node(&mut self) {
        if self.state_stack.len() == self.state_stack.capacity() {
            panic!("Reached max stack size. Would need to resize to continue.");
        }
        self.state_stack.push(self.state);
    }

    pub fn confirm_node(&mut self, symbol: &Symbol) {
        let start = self.state_stack.pop()
            .expect("Trying to confirm a node without starting one.");

        let child_nodes = &self.children[start.child_i..self.state.child_i];
        self.ast.set_node(self.state.node_i, symbol, child_nodes);

        self.state.child_i = start.child_i;
        while self.children.len() > self.state.child_i {
            self.children.pop();
        }
        if self.children.len() == self.children.capacity() {
            panic!("Reached max children list size. Would need to resize to continue.");
        }

        self.children.push(self.state.node_i);
        self.state.node_i+=1;
        self.state.child_i+=1;
    }

    pub fn discard_node(&mut self){ 
        self.state = self.state_stack.pop()
            .expect("Trying to discard a node without starting one.");
        while self.children.len() > self.state.child_i {
            self.children.pop();
        }
    }

    pub fn peek_token(&self) -> &'a Token {
        if self.state.token_i >= self.tokens.len() {
            panic!("No tokens left. Should stop at the End token.");
        } else {
            return &self.tokens[self.state.token_i];
        }
    }

    pub fn consume_token(&mut self) -> &'a Token {
        if self.state.token_i >= self.tokens.len() {
            panic!("No tokens left. Should stop at the End token.");
        } else {
            self.state.token_i+=1;
            return &self.tokens[self.state.token_i-1];
        }
    }
}

