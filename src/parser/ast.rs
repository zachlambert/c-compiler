use std::fmt;

use super::symbol::Symbol;

pub struct Node {
    pub symbol: Symbol,
    pub next: Option<usize>,
    pub child: Option<usize>,
}

pub struct Ast {
    pub nodes: Vec<Node>,
}
impl Ast {
    pub fn new() -> Ast {
        Ast {
            nodes: Vec::new(),
        }
    }

    pub fn set_node(&mut self, node_i: usize, symbol: &Symbol, children: &[usize]) {
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
        if children.len() > 0 {
            self.nodes[node_i].child = Some(children[0]);
            for i in 0..(children.len()-1) {
                self.nodes[children[i]].next = Some(children[i+1]);
            }
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
