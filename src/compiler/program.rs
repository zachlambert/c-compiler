
use crate::parser::construct::*;
use super::symbol::*;
use super::datatype::*;
use super::compiler::Compiler;

pub fn compile_program(compiler: &mut Compiler) {
    let mut node_opt = Some(compiler.ast.nodes.len()-1);
    loop {
        match node_opt {
            Some(node_i) => {
                let node = &compiler.ast.nodes[node_i];
                match &node.construct {
                    Construct::Function(function) => {
                        let return_type = match function.ret_type {
                            Type::Primitive(primitive) => {
                                Datatype::Primitive(primitive)
                            }
                            _ => panic!("todo"), // Resolve symbol
                        }
                        let function = Function::new();
                    },
                    _ => panic!("Unsupported construct in program root"),
                }
                node_opt = node.next;
            }
            None => break,
        }
    }
}
