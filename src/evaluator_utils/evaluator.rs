use super::object::{Boolean, Integer, Null, Object, ObjectType};
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
        Expression::PrefixExpression(p) => {
            let right = evaluate_expression_statement(&p.right);
            eval_prefix_expression(&p.operator, right)
        }
        Expression::InfixExpression(ie) => {
            let left = evaluate_expression_statement(&ie.left);
            let right = evaluate_expression_statement(&ie.right);
            eval_infix_expression(&ie.operator, left, right)
        }
        _ => Object::Null(Null {}),
    }
}

fn eval_prefix_expression(operator: &String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_prefix_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Object::Null(Null {}),
    }
}

fn eval_bang_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(b) => {
            if b.value {
                Object::Boolean(Boolean { value: false })
            } else {
                Object::Boolean(Boolean { value: true })
            }
        }
        Object::Integer(_) => Object::Boolean(Boolean { value: false }),
        Object::Null(_) => Object::Boolean(Boolean { value: true }),
        _ => Object::Null(Null {}),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(i) => Object::Integer(Integer { value: -i.value }),
        _ => Object::Null(Null {}),
    }
}

fn eval_infix_expression(operator: &String, left: Object, right: Object) -> Object {
    match (left.object_type(), right.object_type()) {
        (ObjectType::Integer, ObjectType::Integer) => eval_integer_infix_expression(
            operator,
            left.downcast().unwrap(),
            right.downcast().unwrap(),
        ),
        _ => Object::Null(Null {}),
    }
}

fn eval_integer_infix_expression(operator: &String, left: Integer, right: Integer) -> Object {
    match operator.as_str() {
        "+" => Object::Integer(Integer {
            value: left.value + right.value,
        }),
        "-" => Object::Integer(Integer {
            value: left.value - right.value,
        }),
        "*" => Object::Integer(Integer {
            value: left.value * right.value,
        }),
        "/" => Object::Integer(Integer {
            value: left.value / right.value,
        }),
        "<" => Object::Boolean(Boolean {
            value: left.value < right.value,
        }),
        ">" => Object::Boolean(Boolean {
            value: left.value > right.value,
        }),
        "==" => Object::Boolean(Boolean {
            value: left.value == right.value,
        }),
        "!=" => Object::Boolean(Boolean {
            value: left.value != right.value,
        }),
        _ => Object::Null(Null {}),
    }
}
