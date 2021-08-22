
use super::checker::*;
use crate::parser::construct::*;
use super::expression::check_expression;


fn check_statement_assign(checker: &mut Checker, node_i: usize) {
    let child_opt = checker.ast.nodes[node_i].child;
    let child_i = child_opt.expect("Missing first child in assign statement");
    let datatype1 = check_expression(checker, child_i);
    let datatype2 = check_expression(checker, child_i);
    assert!(datatype1 == datatype2);
}

pub fn check_statement(checker: &mut Checker, node_i: usize) {
    match &checker.ast.nodes[node_i].construct {
        Construct::Statement(statement) => match statement {
            Statement::Assign => check_statement_assign(checker, node_i),
            _ => (),
        },
        _ => panic!("Node passed to check_statement is not a statement"),
    }
}
