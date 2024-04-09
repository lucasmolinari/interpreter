use super::object::{Boolean, Integer, Null, Object};
use crate::parser_utils::ast::{Expression, ExpressionStatement, Node, Program, Statement};


pub fn eval(program: &Program) -> Vec<Object> {
    let mut results: Vec<Object> = Vec::new();
    for stmt in &program.statements {
        let obj: Object = match stmt {
            Node::Statement(stmt) => evaluate_statement(stmt),
            _ => Object::Null(Null {}),
        };
        results.push(obj);
    }
    results
}

fn evaluate_statement(node: &Statement) -> Object {
    match node {
        Statement::ExpressionStatement(expr) => evaluate_expression_statement(&*expr.expression),
        _ => Object::Null(Null {}),
    }
}

fn evaluate_expression_statement(node: &Expression) -> Object {
    match node {
        Expression::IntegerLiteral(i) => Object::Integer(Integer { value: i.value }),
        Expression::BooleanExpression(b) => Object::Boolean(Boolean { value: b.value }),
        _ => Object::Null(Null {}),
    }
}
