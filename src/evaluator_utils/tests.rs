use core::panic;
use std::vec;

use crate::lexer_utils::lexer::Lexer;
use crate::parser_utils::parser::Parser;

use super::evaluator::eval;
use super::object::{Boolean, Integer, Object};

fn evaluate(input: String) -> Vec<Object> {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    eval(&program)
}

#[test]
fn test_eval_integer_expression() {
    struct EvalIntTest {
        input: String,
        expected: i64,
    }
    let tests = vec![
        EvalIntTest {
            input: "5".to_string(),
            expected: 5,
        },
        EvalIntTest {
            input: "10".to_string(),
            expected: 10,
        },
        EvalIntTest {
            input: "-5".to_string(),
            expected: -5,
        },
        EvalIntTest {
            input: "-10".to_string(),
            expected: -10,
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        for obj in res {
            test_integer_object(obj, tt.expected)
        }
    }
}

#[test]
fn test_eval_boolean_expression() {
    struct EvalBooleanTest {
        input: String,
        expected: bool,
    }
    let tests = vec![
        EvalBooleanTest {
            input: "true".to_string(),
            expected: true,
        },
        EvalBooleanTest {
            input: "false".to_string(),
            expected: false,
        },
    ];

    for tt in tests {
        let res = evaluate(tt.input.clone());
        for obj in res {
            test_boolean_object(obj, tt.expected)
        }
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
        for obj in res {
            test_boolean_object(obj, tt.expected)
        }
    }
}

fn test_integer_object(object: Object, expected: i64) {
    let inspect = &object.inspect();
    let obj: Integer = match object.downcast() {
        Some(x) => x,
        None => panic!("Could not downcast {} to Integer", inspect),
    };
    assert_eq!(
        obj.value, expected,
        "Test [{}] - Integer Object has wrong value. Got {}, Expected {}",
        inspect, obj.value, expected
    );
}

fn test_boolean_object(object: Object, expected: bool) {
    let inspect = &object.inspect();
    let obj: Boolean = match object.downcast() {
        Some(x) => x,
        None => panic!("Could not downcast {} to Boolean", inspect),
    };
    assert_eq!(
        obj.value, expected,
        "Test [{}] - Boolean Object has wrong value. Got {}, Expected {}",
        inspect, obj.value, expected
    );
}
