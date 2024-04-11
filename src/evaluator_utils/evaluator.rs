use super::object::{Boolean, Error, Integer, Null, Object, ObjectType, Return};
use crate::parser_utils::ast::{
    BlockStatement, Expression, ExpressionStatement, IfExpression, Node, Program, ReturnStatement,
    Statement,
};

pub fn eval(statements: &Vec<Node>) -> Object {
    let mut result: Object = Object::Null(Null {});
    for stmt in statements {
        let obj: Object = match stmt {
            Node::Statement(stmt) => evaluate_statement(stmt),
            _ => Object::Null(Null {}),
        };
        if obj.object_type() == ObjectType::Return {
            return obj.get_return_value();
        } 
        if obj.object_type() == ObjectType::Error {
            return obj;
        }
        result = obj;
    }
    result
}

fn evaluate_block_statement(node: &BlockStatement) -> Object {
    let mut result: Object = Object::Null(Null {});
    for stmt in &node.statements {
        let obj: Object = match stmt {
            Node::Statement(stmt) => evaluate_statement(stmt),
            _ => Object::Null(Null {}),
        };
        if obj.object_type() == ObjectType::Return || obj.object_type() == ObjectType::Error{
            return obj;
        } 
        result = obj;
    }
    result
}

fn evaluate_statement(node: &Statement) -> Object {
    match node {
        Statement::ExpressionStatement(expr) => evaluate_expression_statement(&expr.expression),
        Statement::ReturnStatement(rs) => eval_return_statement(rs),
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
        Expression::BlockStatement(bs) => evaluate_block_statement(&bs),
        Expression::IfExpression(ie) => eval_if_else_expression(&ie),
        _ => new_error(format!("Unknown expression: {:?}", node)),
    }
}

fn eval_prefix_expression(operator: &String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_prefix_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => new_error(format!("Unknown operator: {}{:?}", operator, right.object_type())),
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
        _ => new_error(format!("Unknown operator: !{:?}", right.object_type())),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(i) => Object::Integer(Integer { value: -i.value }),
        _ => new_error(format!("Unknown operator: -{:?}", right.object_type())),
    }
}

fn eval_infix_expression(operator: &String, left: Object, right: Object) -> Object {
    if left.object_type() == ObjectType::Integer && right.object_type() == ObjectType::Integer {
        return eval_integer_infix_expression(
            operator,
            left.downcast().unwrap(),
            right.downcast().unwrap(),
        );
    }
    if left.object_type() != right.object_type() {
        return new_error(format!(
            "Type mismatch: {:?} {} {:?}",
            left.object_type(),
            operator,
            right.object_type()
        ));
    }
    match operator.as_str() {
        "==" => Object::Boolean(Boolean {
            value: left == right,
        }),
        "!=" => Object::Boolean(Boolean {
            value: left != right,
        }),
        _ => new_error(format!(
            "Unknown operator: {:?} {} {:?}",
            left.object_type(),
            operator,
            right.object_type()
        )), 
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
        _ => new_error(format!(
            "Unknown operator: {:?} {} {:?}",
            left.object_type(),
            operator,
            right.object_type(),
        )),
    }
}

fn eval_if_else_expression(ie: &IfExpression) -> Object {
    let condition = evaluate_expression_statement(&ie.condition);
    let alternative = &ie.alternative;

    if is_truthy(condition) {
        evaluate_block_statement(&ie.consequence)
    } else if alternative.is_some() {
        evaluate_block_statement(&ie.alternative.as_ref().unwrap())
    } else {
        Object::Null(Null {})
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Boolean(b) => b.value,
        Object::Null(_) => false,
        _ => true,
    }
}

fn eval_return_statement(rs: &ReturnStatement) -> Object {
    let val = evaluate_expression_statement(&rs.return_value);
    Object::Return(Return {
        value: Box::new(val),
    })
}

fn new_error(msg: String) -> Object {
    Object::Error(Error { message: msg })
}
