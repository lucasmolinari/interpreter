use crate::lexer_utils::{lexer::Lexer, token::TokenType};
use crate::parser_utils::ast::Program;
use crate::parser_utils::{
    ast::{Expression, ExpressionStatement, Node, Statement},
    parser::Parser,
};

fn init_program(input: String) -> Program {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    p.parse_program()
}

#[test]
fn test_lexer() {
    let input = String::from("let x = 5; let add = fn(x, y) { x + y }; let result = add(5, 5);");
    let mut l = Lexer::new(input);
    let tests = vec![
        ("let", TokenType::LET),
        ("x", TokenType::IDENT),
        ("=", TokenType::ASSIGN),
        ("5", TokenType::INT),
        (";", TokenType::SEMICOLON),
        ("let", TokenType::LET),
        ("add", TokenType::IDENT),
        ("=", TokenType::ASSIGN),
        ("fn", TokenType::FUNCTION),
        ("(", TokenType::LPAREN),
        ("x", TokenType::IDENT),
        (",", TokenType::COMMA),
        ("y", TokenType::IDENT),
        (")", TokenType::RPAREN),
        ("{", TokenType::LBRACE),
        ("x", TokenType::IDENT),
        ("+", TokenType::PLUS),
        ("y", TokenType::IDENT),
        ("}", TokenType::RBRACE),
        (";", TokenType::SEMICOLON),
        ("let", TokenType::LET),
        ("result", TokenType::IDENT),
        ("=", TokenType::ASSIGN),
        ("add", TokenType::IDENT),
        ("(", TokenType::LPAREN),
        ("5", TokenType::INT),
        (",", TokenType::COMMA),
        ("5", TokenType::INT),
        (")", TokenType::RPAREN),
        (";", TokenType::SEMICOLON),
        ("\0", TokenType::EOF),
    ];
    for (i, tt) in tests.iter().enumerate() {
        let tok = l.next_token();
        assert_eq!(tok.literal, tt.0, "Test [{}] - Token Literal is wrong", i);
    }
}

#[test]
fn test_let_statements() {
    let tests = vec![
        ("let", "x", "5"),
        ("let", "y", "true"),
        ("let", "foobar", "y"),
    ];

    for tt in tests.iter() {
        let input = format!("{} {} = {};", tt.0, tt.1, tt.2);
        let program = init_program(input.clone());

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Test [{}] Statement length is wrong", input);

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            TokenType::LET,
            "Test [{}] Statement Token Type is wrong",
            input
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.0,
            "Test [{}] Statement Token Literal is wrong",
            input
        );
    }
}

#[test]
fn test_return_statements() {
    let tests = vec![("return", "5"), ("return", "true"), ("return", "foobar")];

    for tt in tests.iter() {
        let input = format!("{} {};", tt.0, tt.1);
        let program = init_program(input.clone());

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Test [{}] Statement length is wrong", input);

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            TokenType::RETURN,
            "Test [{}] Statement Token Type is wrong",
            input
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.0,
            "Test [{}] Statement Token Literal is wrong",
            input
        );
    }
}

#[test]
fn test_identifier_expression() {
    let input = String::from("foobar;");
    let program = init_program(input);

    let stmts = program.statements;
    assert_eq!(stmts.len(), 1);

    let stmt = stmts.get(0).unwrap();
    assert_eq!(
        stmt.get_token().token_type,
        TokenType::IDENT,
        "Statement Token Type is wrong"
    );
    assert_eq!(
        stmt.get_token().literal,
        "foobar",
        "Statement Token Literal is wrong"
    );

    let expr = &stmt.get_statement_expr().expression;
    assert_eq!(
        expr.get_identifer().value,
        "foobar",
        "Expression Identifier is wrong"
    );

    let string = expr.string();
    assert_eq!(string, "foobar", "Expression String is wrong");
}

#[test]
fn test_integer_literal_expression() {
    let input = String::from("5;");
    let program = init_program(input);

    let stmts = program.statements;
    assert_eq!(stmts.len(), 1, "Statement length is wrong");

    let stmt = stmts.get(0).unwrap();
    assert_eq!(
        stmt.get_token().token_type,
        TokenType::INT,
        "Statement Token Type is wrong"
    );
    assert_eq!(
        stmt.get_token().literal,
        "5",
        "Statement Token Literal is wrong"
    );

    let expr = &stmt.get_statement_expr().expression;
    assert_eq!(
        expr.get_integer_literal().value,
        5,
        "Expression Integer Literal Value is wrong"
    );

    let string = expr.string();
    assert_eq!(string, "5", "Expression String is wrong");
}

#[test]
fn test_boolean_expresion() {
    struct BooleanTest {
        input: String,
        value: bool,
        token_type: TokenType,
    }
    let tests = vec![
        BooleanTest {
            input: "true;".to_string(),
            value: true,
            token_type: TokenType::TRUE,
        },
        BooleanTest {
            input: "false;".to_string(),
            value: false,
            token_type: TokenType::FALSE,
        },
    ];

    for tt in tests {
        let input = tt.input;
        let p = init_program(input.clone());

        let stmts = p.statements;
        assert_eq!(stmts.len(), 1, "Test [{}] Statement length is wrong", input);

        let stmt = stmts.get(0).unwrap();
        let boolean = stmt
            .get_statement_expr()
            .expression
            .get_boolean_expression();

        assert_eq!(boolean.value, tt.value, "Test [{}] Boolean Value is wrong", input);
        assert_eq!(
            boolean.token.token_type, tt.token_type,
            "Test [{}] Boolean Token Type is wrong",
            input
        );
        assert_eq!(
            boolean.token.literal,
            tt.value.to_string(),
            "Test [{}] Boolean Token Literal is wrong",
            input
        );
    }
}

#[test]
fn test_prefix_expression() {
    struct PrefixTest {
        input: String,
        operator: String,
        right: String,
        operator_token: TokenType,
        right_token: TokenType,
    }
    let tests = vec![
        PrefixTest {
            input: "!5;".to_string(),
            operator: "!".to_string(),
            right: "5".to_string(),
            operator_token: TokenType::BANG,
            right_token: TokenType::INT,
        },
        PrefixTest {
            input: "-15;".to_string(),
            operator: "-".to_string(),
            right: "15".to_string(),
            operator_token: TokenType::MINUS,
            right_token: TokenType::INT,
        },
        PrefixTest {
            input: "!foobar;".to_string(),
            operator: "!".to_string(),
            right: "foobar".to_string(),
            operator_token: TokenType::BANG,
            right_token: TokenType::IDENT,
        },
        PrefixTest {
            input: "-foobar;".to_string(),
            operator: "-".to_string(),
            right: "foobar".to_string(),
            operator_token: TokenType::MINUS,
            right_token: TokenType::IDENT,
        },
        PrefixTest {
            input: "!true;".to_string(),
            operator: "!".to_string(),
            right: "true".to_string(),
            operator_token: TokenType::BANG,
            right_token: TokenType::TRUE,
        },
        PrefixTest {
            input: "!false;".to_string(),
            operator: "!".to_string(),
            right: "false".to_string(),
            operator_token: TokenType::BANG,
            right_token: TokenType::FALSE,
        },
    ];

    for tt in tests {
        let input = tt.input;
        let program = init_program(input.clone());

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Test [{}] Statement length is wrong", input);

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            tt.operator_token,
            "Test [{}] Expression Statement Token Type is wrong",
            input
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.operator,
            "Test [{}] Expression Statement Literal is wrong",
            input
        );

        let prefix_expr = &stmt.get_statement_expr().expression.get_prefix_expr();
        assert_eq!(
            prefix_expr.token.token_type, tt.operator_token,
            "Test [{}] Prefix Expression Token Type is wrong",
            input
        );
        assert_eq!(
            prefix_expr.token.literal, tt.operator,
            "Test [{}] Prefix Expression Token Literal is wrong",
            input
        );
        assert_eq!(
            prefix_expr.operator, tt.operator,
            "Test [{}] Prefix Expression Operator is wrong",
            input
        );

        test_literal_expression(&prefix_expr.right, &tt.right);
        assert_eq!(
            prefix_expr.string(),
            format!("({}{})", tt.operator, tt.right),
            "Test [{}] Prefix Expression String is wrong",
            input
        );
    }
}

#[test]
fn test_infix_expression() {
    struct InfixTest {
        input: String,
        left: String,
        operator: String,
        right: String,
        left_token: TokenType,
        operator_token: TokenType,
        right_token: TokenType,
    }
    let tests = vec![
        InfixTest {
            input: "5 + 5;".to_string(),
            left: "5".to_string(),
            operator: "+".to_string(),
            right: "5".to_string(),
            left_token: TokenType::INT,
            operator_token: TokenType::PLUS,
            right_token: TokenType::INT,
        },
        InfixTest {
            input: "5 - 5;".to_string(),
            left: "5".to_string(),
            operator: "-".to_string(),
            right: "5".to_string(),
            left_token: TokenType::INT,
            operator_token: TokenType::MINUS,
            right_token: TokenType::INT,
        },
        InfixTest {
            input: "5 * 5;".to_string(),
            left: "5".to_string(),
            operator: "*".to_string(),
            right: "5".to_string(),
            left_token: TokenType::INT,
            operator_token: TokenType::ASTERISK,
            right_token: TokenType::INT,
        },
        InfixTest {
            input: "5 / 5;".to_string(),
            left: "5".to_string(),
            operator: "/".to_string(),
            right: "5".to_string(),
            left_token: TokenType::INT,
            operator_token: TokenType::SLASH,
            right_token: TokenType::INT,
        },
        InfixTest {
            input: "5 > 5;".to_string(),
            left: "5".to_string(),
            operator: ">".to_string(),
            right: "5".to_string(),
            left_token: TokenType::INT,
            operator_token: TokenType::GT,
            right_token: TokenType::INT,
        },
        InfixTest {
            input: "5 < 5;".to_string(),
            left: "5".to_string(),
            operator: "<".to_string(),
            right: "5".to_string(),
            left_token: TokenType::INT,
            operator_token: TokenType::LT,
            right_token: TokenType::INT,
        },
        InfixTest {
            input: "5 == 5;".to_string(),
            left: "5".to_string(),
            operator: "==".to_string(),
            right: "5".to_string(),
            left_token: TokenType::INT,
            operator_token: TokenType::EQ,
            right_token: TokenType::INT,
        },
        InfixTest {
            input: "5 != 5;".to_string(),
            left: "5".to_string(),
            operator: "!=".to_string(),
            right: "5".to_string(),
            left_token: TokenType::INT,
            operator_token: TokenType::NOTEQ,
            right_token: TokenType::INT,
        },
        InfixTest {
            input: "true == true;".to_string(),
            left: "true".to_string(),
            operator: "==".to_string(),
            right: "true".to_string(),
            left_token: TokenType::TRUE,
            operator_token: TokenType::EQ,
            right_token: TokenType::TRUE,
        },
        InfixTest {
            input: "true != false;".to_string(),
            left: "true".to_string(),
            operator: "!=".to_string(),
            right: "false".to_string(),
            left_token: TokenType::TRUE,
            operator_token: TokenType::NOTEQ,
            right_token: TokenType::FALSE,
        },
        InfixTest {
            input: "false == false;".to_string(),
            left: "false".to_string(),
            operator: "==".to_string(),
            right: "false".to_string(),
            left_token: TokenType::FALSE,
            operator_token: TokenType::EQ,
            right_token: TokenType::FALSE,
        },
    ];

    for tt in tests {
        let input = tt.input;
        let program = init_program(input.clone());

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Test [{}] Statement length is wrong", input);

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            tt.left_token,
            "Test [{}] Expression Statement Token Type is wrong",
            input
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.left,
            "Test [{}] Expression Statement Literal is wrong",
            input
        );

        let infix_expr = stmt.get_statement_expr().expression.get_infix_expr();
        test_literal_expression(&infix_expr.left, &tt.left);
        test_literal_expression(&infix_expr.right, &tt.right);
        assert_eq!(&infix_expr.operator, &tt.operator, "Operator is wrong");
    }
}

#[test]
fn test_operator_precedence() {
    struct PrecedenceTest {
        input: String,
        expected: String,
    }

    let tests = vec![
        PrecedenceTest {
            input: "-a * b;".to_string(),
            expected: "((-a) * b)".to_string(),
        },
        PrecedenceTest {
            input: "!-a;".to_string(),
            expected: "(!(-a))".to_string(),
        },
        PrecedenceTest {
            input: "a + b + c;".to_string(),
            expected: "((a + b) + c)".to_string(),
        },
        PrecedenceTest {
            input: "a + b - c;".to_string(),
            expected: "((a + b) - c)".to_string(),
        },
        PrecedenceTest {
            input: "a * b * c;".to_string(),
            expected: "((a * b) * c)".to_string(),
        },
        PrecedenceTest {
            input: "a * b / c;".to_string(),
            expected: "((a * b) / c)".to_string(),
        },
        PrecedenceTest {
            input: "a + b / c;".to_string(),
            expected: "(a + (b / c))".to_string(),
        },
        PrecedenceTest {
            input: "a + b * c + d / e - f;".to_string(),
            expected: "(((a + (b * c)) + (d / e)) - f)".to_string(),
        },
        PrecedenceTest {
            input: "3 + 4; -5 * 5;".to_string(),
            expected: "(3 + 4)((-5) * 5)".to_string(),
        },
        PrecedenceTest {
            input: "5 > 4 == 3 < 4;".to_string(),
            expected: "((5 > 4) == (3 < 4))".to_string(),
        },
        PrecedenceTest {
            input: "5 < 4 != 3 > 4;".to_string(),
            expected: "((5 < 4) != (3 > 4))".to_string(),
        },
        PrecedenceTest {
            input: "3 + 4 * 5 == 3 * 1 + 4 * 5;".to_string(),
            expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".to_string(),
        },
        PrecedenceTest {
            input: "true;".to_string(),
            expected: "true".to_string(),
        },
        PrecedenceTest {
            input: "false;".to_string(),
            expected: "false".to_string(),
        },
        PrecedenceTest {
            input: "3 < 5 == true;".to_string(),
            expected: "((3 < 5) == true)".to_string(),
        },
        PrecedenceTest {
            input: "3 > 5 == false;".to_string(),
            expected: "((3 > 5) == false)".to_string(),
        },
        PrecedenceTest {
            input: "1 + (2 + 3) + 4;".to_string(),
            expected: "((1 + (2 + 3)) + 4)".to_string(),
        },
        PrecedenceTest {
            input: "(5 + 5) * 2;".to_string(),
            expected: "((5 + 5) * 2)".to_string(),
        },
        PrecedenceTest {
            input: "2 / (5 + 5);".to_string(),
            expected: "(2 / (5 + 5))".to_string(),
        },
        PrecedenceTest {
            input: "-(5 + 5);".to_string(),
            expected: "(-(5 + 5))".to_string(),
        },
        PrecedenceTest {
            input: "!(true == true);".to_string(),
            expected: "(!(true == true))".to_string(),
        },
    ];

    for tt in tests {
        let input = tt.input;
        let program = init_program(input.clone());
        
        let string = program.string();
        assert_eq!(string, tt.expected, "Test [{}] Expression String is wrong", input);
    }
}

#[test]
fn test_if_expression(){
    let input = "if (x < y) { x }".to_string();
    let p = init_program(input.clone());

    let stmts = p.statements;
    assert_eq!(stmts.len(), 1, "Statement length is wrong");

    let stmt = stmts.get(0).unwrap();
    
    let if_expr = stmt.get_statement_expr().expression.get_if_expr();
    assert_eq!(if_expr.token.literal, "if", "Token Literal is wrong");

    test_literal_expression(&if_expr.condition, "x < y");
    assert_eq!(if_expr.condition.string(), "(x < y)", "Condition String is wrong");
    assert_eq!(if_expr.consequence.string(), "x", "Consequence String is wrong");

}   

fn test_literal_expression(expr: &Expression, expected: &str) {
    match expr {
        Expression::Identifier(_) => test_identifier(expr, expected),
        Expression::IntegerLiteral(_) => test_integer(expr, expected.parse::<i64>().unwrap()),
        Expression::BooleanExpression(_) => test_boolean(expr, expected.parse::<bool>().unwrap()),
        _ => panic!("Not a literal expression"),
    }
}

fn test_identifier(expr: &Expression, value: &str) {
    let ident = expr.get_identifer();
    assert_eq!(ident.value, value, "Identifier Value is wrong");
    assert_eq!(
        ident.token.token_type,
        TokenType::IDENT,
        "Identifier Token Type is wrong"
    );
    assert_eq!(
        ident.token.literal, value,
        "Identifier Token Literal is wrong"
    );
}

fn test_integer(expr: &Expression, value: i64) {
    let int = expr.get_integer_literal();
    assert_eq!(int.value, value, "Integer Value is wrong");
    assert_eq!(
        int.token.token_type,
        TokenType::INT,
        "Integer Token Type is wrong"
    );
    assert_eq!(
        int.token.literal,
        value.to_string(),
        "Integer Token Literal is wrong"
    );
}

fn test_boolean(expr: &Expression, value: bool) {
    let boolean = expr.get_boolean_expression();
    assert_eq!(boolean.value, value, "Boolean Value is wrong");
    assert_eq!(
        boolean.token.token_type,
        if value { TokenType::TRUE } else { TokenType::FALSE },
        "Boolean Token Type is wrong"
    );
    assert_eq!(
        boolean.token.literal,
        value.to_string(),
        "Boolean Token Literal is wrong"
    );
}
