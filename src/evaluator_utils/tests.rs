use core::panic;
use std::vec;

use crate::evaluator_utils::object::{Function, ObjectType};
use crate::lexer_utils::lexer::Lexer;
use crate::parser_utils::parser::Parser;

use super::environment::Environment;
use super::evaluator::eval;
use super::object::{Boolean, Error, Integer, Null, Object};

fn evaluate(input: String) -> Object {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    eval(&program.statements, &mut Environment::new())
}

#[test]
fn test_eval_integer_expression() {
    struct EvalInt {
        input: String,
        expected: i64,
    }
    let tests = vec![
        EvalInt {
            input: "5".to_string(),
            expected: 5,
        },
        EvalInt {
            input: "10".to_string(),
            expected: 10,
        },
        EvalInt {
            input: "-5".to_string(),
            expected: -5,
        },
        EvalInt {
            input: "-10".to_string(),
            expected: -10,
        },
        EvalInt {
            input: "5 + 5 + 5 + 5 - 10".to_string(),
            expected: 10,
        },
        EvalInt {
            input: "2 * 2 * 2 * 2 * 2".to_string(),
            expected: 32,
        },
        EvalInt {
            input: "-50 + 100 + -50".to_string(),
            expected: 0,
        },
        EvalInt {
            input: "5 * 2 + 10".to_string(),
            expected: 20,
        },
        EvalInt {
            input: "5 + 2 * 10".to_string(),
            expected: 25,
        },
        EvalInt {
            input: "20 + 2 * -10".to_string(),
            expected: 0,
        },
        EvalInt {
            input: "50 / 2 * 2 + 10".to_string(),
            expected: 60,
        },
        EvalInt {
            input: "2 * (5 + 10)".to_string(),
            expected: 30,
        },
        EvalInt {
            input: "(5 + 10 * 2 + 15 / 3) * 2 + -10".to_string(),
            expected: 50,
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        test_integer_object(res, tt.expected)
    }
}

#[test]
fn test_eval_boolean_expression() {
    struct EvalBoolean {
        input: String,
        expected: bool,
    }
    let tests = vec![
        EvalBoolean {
            input: "true".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "false".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "1 < 2".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "1 > 2".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "1 < 1".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "1 > 1".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "1 == 1".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "1 != 1".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "1 == 2".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "1 != 2".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "true == true".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "false == false".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "true == false".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "true != false".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "false != true".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "(1 < 2) == true".to_string(),
            expected: true,
        },
        EvalBoolean {
            input: "(1 < 2) == false".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "(1 > 2) == true".to_string(),
            expected: false,
        },
        EvalBoolean {
            input: "(1 > 2) == false".to_string(),
            expected: true,
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        test_boolean_object(res, tt.expected)
    }
}

#[test]
fn test_eval_bang_prefix() {
    struct EvalBangPrefix {
        input: String,
        expected: bool,
    }

    let tests = vec![
        EvalBangPrefix {
            input: "!true".to_string(),
            expected: false,
        },
        EvalBangPrefix {
            input: "!false".to_string(),
            expected: true,
        },
        EvalBangPrefix {
            input: "!5".to_string(),
            expected: false,
        },
        EvalBangPrefix {
            input: "!!true".to_string(),
            expected: true,
        },
        EvalBangPrefix {
            input: "!!false".to_string(),
            expected: false,
        },
        EvalBangPrefix {
            input: "!!5".to_string(),
            expected: true,
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        test_boolean_object(res, tt.expected)
    }
}

#[test]
fn test_eval_if_else_expression() {
    struct EvalIfElse {
        input: String,
        expected: Result<i64, Null>,
    }
    let tests = vec![
        EvalIfElse {
            input: "if (true) { 10 }".to_string(),
            expected: Ok(10),
        },
        EvalIfElse {
            input: "if (false) { 10 }".to_string(),
            expected: Err(Null {}),
        },
        EvalIfElse {
            input: "if (1) { 10 }".to_string(),
            expected: Ok(10),
        },
        EvalIfElse {
            input: "if (1 < 2) { 10 }".to_string(),
            expected: Ok(10),
        },
        EvalIfElse {
            input: "if (1 > 2) { 10 }".to_string(),
            expected: Err(Null {}),
        },
        EvalIfElse {
            input: "if (1 > 2) { 10 } else { 20 }".to_string(),
            expected: Ok(20),
        },
        EvalIfElse {
            input: "if (1 < 2) { 10 } else { 20 }".to_string(),
            expected: Ok(10),
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        match tt.expected {
            Ok(x) => test_integer_object(res, x),
            Err(_) => test_null_object(res),
        }
    }
}

#[test]
fn test_eval_return_statement() {
    struct EvalReturn {
        input: String,
        expected: i64,
    }
    let tests = vec![
        EvalReturn {
            input: "return 10;".to_string(),
            expected: 10,
        },
        EvalReturn {
            input: "return 10; 9;".to_string(),
            expected: 10,
        },
        EvalReturn {
            input: "return 2 * 5; 9;".to_string(),
            expected: 10,
        },
        EvalReturn {
            input: "9; return 2 * 5; 20;".to_string(),
            expected: 10,
        },
        EvalReturn {
            input: "if (10 > 1) { if (10 > 1) { return 10; } return 1; }".to_string(),
            expected: 10,
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        test_integer_object(res, tt.expected)
    }
}

#[test]
fn test_eval_let_statement() {
    struct EvalLet {
        input: String,
        expected: i64,
    }
    let tests = vec![
        EvalLet {
            input: "let a = 5; a;".to_string(),
            expected: 5,
        },
        EvalLet {
            input: "let a = 5 * 5; a;".to_string(),
            expected: 25,
        },
        EvalLet {
            input: "let a = 5; let b = a; b;".to_string(),
            expected: 5,
        },
        EvalLet {
            input: "let a = 5; let b = a; let c = a + b + 5; c;".to_string(),
            expected: 15,
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        test_integer_object(res, tt.expected)
    }
}

#[test]
fn test_eval_function_application() {
    struct EvalFunction {
        input: String,
        expected: i64,
    }
    let tests = vec![
        EvalFunction {
            input: "let identity = fn(x) { x; }; identity(5);".to_string(),
            expected: 5,
        },
        EvalFunction {
            input: "let identity = fn(x) { return x; }; identity(5);".to_string(),
            expected: 5,
        },
        EvalFunction {
            input: "let double = fn(x) { x * 2; }; double(5);".to_string(),
            expected: 10,
        },
        EvalFunction {
            input: "let add = fn(x, y) { x + y; }; add(5, 5);".to_string(),
            expected: 10,
        },
        EvalFunction {
            input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));".to_string(),
            expected: 20,
        },
        EvalFunction {
            input: "fn(x) { x; }(5);".to_string(),
            expected: 5,
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        test_integer_object(res, tt.expected)
    }
}

#[test]
fn test_eval_function_object() {
    let input = "fn(x) { x + 2; };".to_string();
    let res = evaluate(input);
    assert_eq!(
        res.object_type(),
        ObjectType::Function,
        "Object is not a Function."
    );
    let fn_obj: Function = match res.downcast() {
        Some(x) => x,
        None => panic!("Could not downcast to Function"),
    };

    assert_eq!(fn_obj.parameters.len(), 1, "Function has wrong parameters.",);
    assert_eq!(
        fn_obj.parameters.get(0).unwrap(),
        "x",
        "Parameter is not 'x'.",
    );
}

#[test]
fn test_closures() {
    let input = "
    let newAdder = fn(x) {
        fn(y) { x + y };
    };
    let addTwo = newAdder(2);
    addTwo(3);
    "
    .to_string();
    let res = evaluate(input);
    test_integer_object(res, 5);
}

#[test]
fn test_error_handling() {
    struct ErrorHandling {
        input: String,
        expected: String,
    }
    let tests = vec![
        ErrorHandling {
            input: "5 + true;".to_string(),
            expected: "Type mismatch: Integer + Boolean".to_string(),
        },
        ErrorHandling {
            input: "5 + true; 5;".to_string(),
            expected: "Type mismatch: Integer + Boolean".to_string(),
        },
        ErrorHandling {
            input: "-true".to_string(),
            expected: "Unknown operator: -Boolean".to_string(),
        },
        ErrorHandling {
            input: "true + false;".to_string(),
            expected: "Unknown operator: Boolean + Boolean".to_string(),
        },
        ErrorHandling {
            input: "5; true + false; 5".to_string(),
            expected: "Unknown operator: Boolean + Boolean".to_string(),
        },
        ErrorHandling {
            input: "if (10 > 1) { true + false; }".to_string(),
            expected: "Unknown operator: Boolean + Boolean".to_string(),
        },
        ErrorHandling {
            input: "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }".to_string(),
            expected: "Unknown operator: Boolean + Boolean".to_string(),
        },
        ErrorHandling {
            input: "foobar".to_string(),
            expected: "Identifier not found: foobar".to_string(),
        },
    ];

    for tt in tests {
        let evaluated = evaluate(tt.input.clone());
        assert_eq!(
            evaluated.object_type(),
            ObjectType::Error,
            "No error object returned"
        );
        assert_eq!(evaluated.inspect(), tt.expected, "Wrong error message");
    }
}

fn test_integer_object(object: Object, expected: i64) {
    let obj_type = &object.object_type();
    let inspect = &object.inspect();
    let obj: Integer = match object.downcast() {
        Some(x) => x,
        None => panic!("Could not downcast {:?} to Integer", obj_type),
    };
    assert_eq!(
        obj.value, expected,
        "Test [{}] - Integer Object has wrong value. Got {}, Expected {}",
        inspect, obj.value, expected
    );
}

fn test_boolean_object(object: Object, expected: bool) {
    let obj_type = &object.object_type();
    let inspect = &object.inspect();
    let obj: Boolean = match object.downcast() {
        Some(x) => x,
        None => panic!("Could not downcast {:?} to Boolean", obj_type),
    };
    assert_eq!(
        obj.value, expected,
        "Test [{}] - Boolean Object has wrong value. Got {}, Expected {}",
        inspect, obj.value, expected
    );
}

fn test_null_object(object: Object) {
    let obj_type = &object.object_type();
    let inspect = &object.inspect();
    let obj: Null = match object.downcast() {
        Some(x) => x,
        None => panic!("Could not downcast {:?} to Null", obj_type),
    };
}
