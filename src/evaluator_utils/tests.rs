use crate::lexer_utils::lexer::Lexer;
use crate::parser_utils::parser::Parser;

use super::evaluator::eval;
use super::object::{Integer, Object};

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
    ];

    for (i, tt) in tests.iter().enumerate() {
        let res = evaluate(tt.input.clone());
        for obj in res {
            test_integer_object(obj, tt.expected)
        } 
    }
}

fn test_integer_object(object: Object, expected: i64) {
    let inspect = &object.inspect();
    let obj: Integer = object.downcast();
    assert_eq!(
        obj.value, expected,
        "Test [{}] - Integer Object has wrong value. Got {}, Expected {}",
        inspect, obj.value, expected
    );
}

fn evaluate(input: String) -> Vec<Object> {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    eval(&program)
}

