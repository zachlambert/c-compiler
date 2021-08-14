
use crate::parser::Ast;
use crate::parser::Symbol;
use crate::parser::SymbolFunction;
use crate::parser::SymbolArgument;

//      .globl	main
//  main:
// 	    movl	$42, %eax
// 	    ret
//

fn compile_statement(ast: &Ast, code: &mut String, parent_i: usize) {
    // let node_opt = ast.nodes[parent_i].child;
    // match node_opt {
    //     Some(node_i) => {
    //         let node = &ast.nodes[node_i];
    //         match node.symbol {
    //             Symbol::
    //         };
    //     }
    //     None => (),
    // };
    let line = format!("\tmovl\t$3, %eax\n");
    code.push_str(&line);
    let line = format!("\tret\n");
    code.push_str(&line);
}

fn compile_function(ast: &Ast, code: &mut String, parent_i: usize, function: &SymbolFunction) {
    let line = format!(
        "\t.globl {}\n", function.name
    );
    code.push_str(&line);
    let line = format!(
        "{}:\n", function.name
    );
    code.push_str(&line);

    let mut node_opt = ast.nodes[parent_i].child;

    // Collect references to arguments
    let mut args: Vec<&SymbolArgument> = Vec::new();
    {
        loop {
            match node_opt {
                Some(node_i) => {
                    let node = &ast.nodes[node_i];
                    match &node.symbol {
                        Symbol::Argument(argument) => {
                            node_opt = node.next;
                            args.push(&argument);
                        },
                        Symbol::Statement => break,
                        _ => panic!("Unexpected child symbol of function"),
                    };
                },
                None => break,
            }
        }
    }
    // TODO: Do something with the arguments

    let mut body = String::new();
    loop {
        match node_opt {
            Some(node_i) => {
                let node = &ast.nodes[node_i];
                match &node.symbol {
                    Symbol::Statement => {
                        node_opt = node.next;
                        compile_statement(ast, &mut body, node_i);
                    },
                    _ => panic!("Unexpected child symbol of function"),
                };
            },
            None => break,
        };
    }
    code.push_str(&body);
}

fn compile_program(ast: &Ast, code: &mut String, parent_i: usize) {
    let mut node_i = ast.nodes[parent_i].child
        .expect("Program doesn't contain anything");

    loop {
        let node = &ast.nodes[node_i];
        match &node.symbol {
            Symbol::Function(function) => {
                compile_function(ast, code, node_i, &function);
            },
            _ => break,
        };
        node_i = match node.next {
            Some(next_i) => next_i,
            None => break,
        };
    }
}

pub fn compile_ast(ast: &Ast, code: &mut String) {
    let node_i = ast.nodes.len() - 1;
    let node = &ast.nodes[node_i];
    match &node.symbol {
        Symbol::Program => compile_program(ast, code, node_i),
        _ => panic!("First node isn't program"),
    }
}
