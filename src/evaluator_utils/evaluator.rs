use super::{
    environment::Environment,
    object::{Boolean, Error, Function, Integer, Null, Object, ObjectType, Return},
};
use crate::parser_utils::ast::{
    BlockStatement, CallExpression, Expression, ExpressionStatement, IfExpression, LetStatement,
    Node, Program, ReturnStatement, Statement,
};

pub fn eval(statements: &Vec<Node>, env: &mut Environment) -> Object {
    let mut result: Object = Object::Null(Null {});
    for stmt in statements {
        let obj: Object = match stmt {
            Node::Statement(stmt) => evaluate_statement(stmt, env),
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

fn evaluate_block_statement(node: &BlockStatement, env: &mut Environment) -> Object {
    let mut result: Object = Object::Null(Null {});
    for stmt in &node.statements {
        let obj: Object = match stmt {
            Node::Statement(stmt) => evaluate_statement(stmt, env),
            _ => Object::Null(Null {}),
        };
        if obj.object_type() == ObjectType::Return || obj.object_type() == ObjectType::Error {
            return obj;
        }
        result = obj;
    }
    result
}

fn evaluate_statement(node: &Statement, env: &mut Environment) -> Object {
    match node {
        Statement::ExpressionStatement(expr) => {
            evaluate_expression_statement(&expr.expression, env)
        }
        Statement::ReturnStatement(rs) => eval_return_statement(rs, env),
        Statement::LetStatement(ls) => eval_let_statement(ls, env),
    }
}

fn evaluate_expression_statement(node: &Expression, env: &mut Environment) -> Object {
    match node {
        Expression::IntegerLiteral(i) => Object::Integer(Integer { value: i.value }),
        Expression::BooleanExpression(b) => Object::Boolean(Boolean { value: b.value }),
        Expression::PrefixExpression(p) => {
            let right = evaluate_expression_statement(&p.right, env);
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(&p.operator, right)
        }
        Expression::InfixExpression(ie) => {
            let left = evaluate_expression_statement(&ie.left, env);
            if is_error(&left) {
                return left;
            }
            let right = evaluate_expression_statement(&ie.right, env);
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(&ie.operator, left, right)
        }
        Expression::BlockStatement(bs) => evaluate_block_statement(&bs, env),
        Expression::Identifier(id) => match env.get(&id.value) {
            Some(obj) => obj.clone(),
            None => new_error(format!("Identifier not found: {}", id.value)),
        },
        Expression::IfExpression(ie) => eval_if_else_expression(&ie, env),
        Expression::FunctionLiteral(fl) => Object::Function(Function {
            parameters: fl.get_parameters(),
            body: fl.body.clone(),
            env: env.clone(),
        }),
        Expression::CallExpression(ce) => eval_call_expression(&ce, env), // TODO: Implement recursive functions
        _ => new_error(format!("Unknown expression: {:?}", node)),
    }
}

fn eval_prefix_expression(operator: &String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_prefix_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => new_error(format!(
            "Unknown operator: {}{:?}",
            operator,
            right.object_type()
        )),
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

fn eval_if_else_expression(ie: &IfExpression, env: &mut Environment) -> Object {
    let condition = evaluate_expression_statement(&ie.condition, env);
    if is_error(&condition) {
        return condition;
    }
    let alternative = &ie.alternative;

    if is_truthy(condition) {
        evaluate_block_statement(&ie.consequence, env)
    } else if alternative.is_some() {
        evaluate_block_statement(&ie.alternative.as_ref().unwrap(), env)
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

fn eval_return_statement(rs: &ReturnStatement, env: &mut Environment) -> Object {
    let val = evaluate_expression_statement(&rs.return_value, env);
    if is_error(&val) {
        return val;
    }
    Object::Return(Return {
        value: Box::new(val),
    })
}

fn eval_let_statement(ls: &LetStatement, env: &mut Environment) -> Object {
    let val = evaluate_expression_statement(&ls.value, env);
    if is_error(&val) {
        return val;
    }
    env.set(ls.name.value.clone(), val.clone());
    return val;
}

fn eval_call_expression(ce: &CallExpression, env: &mut Environment) -> Object {
    let function = evaluate_expression_statement(&ce.function, env);
    if is_error(&function) {
        return function;
    }
    let args: Vec<Object> = ce
        .arguments
        .iter()
        .map(|arg| evaluate_expression_statement(arg, env))
        .collect();
    if args.iter().any(|arg| is_error(arg)) {
        return args[0].clone();
    }
    apply_function(function, args)
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    if function.object_type() != ObjectType::Function {
        return new_error(format!("Not a function: {:?}", function.object_type()));
    }
    let func_obj = function.downcast().unwrap();
    let mut extended_env = extend_function_env(&func_obj, args);
    let evaluated = eval(&func_obj.body.statements, &mut extended_env);
    unwrap_return_value(evaluated)
}

fn extend_function_env(function: &Function, args: Vec<Object>) -> Environment {
    let mut env = Environment::new_enclosed(function.env.clone());
    for (i, param) in function.parameters.iter().enumerate() {
        env.set(param.to_owned(), args[i].clone())
    }
    env
}

fn unwrap_return_value(obj: Object) -> Object {
    if obj.object_type() == ObjectType::Return {
        return obj.get_return_value();
    }
    obj
}

fn new_error(msg: String) -> Object {
    Object::Error(Error { message: msg })
}

fn is_error(obj: &Object) -> bool {
    obj.object_type() == ObjectType::Error
}
