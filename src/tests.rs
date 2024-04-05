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
        let program = init_program(input);

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Statement length is wrong");

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            TokenType::LET,
            "Statement Token Type is wrong"
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.0,
            "Statement Token Literal is wrong"
        );
    }
}

#[test]
fn test_return_statements() {
    let tests = vec![("return", "5"), ("return", "true"), ("return", "foobar")];

    for tt in tests.iter() {
        let input = format!("{} {};", tt.0, tt.1);
        let program = init_program(input);

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Statement length is wrong");

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            TokenType::RETURN,
            "Statement Token Type is wrong"
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.0,
            "Statement Token Literal is wrong"
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
    ];

    for tt in tests {
        let program = init_program(tt.input);

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Statement length is wrong");

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            tt.operator_token,
            "Expression Statement Token Type is wrong"
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.operator,
            "Expression Statement Literal is wrong"
        );

        let prefix_expr = &stmt.get_statement_expr().expression.get_prefix_expr();
        assert_eq!(
            prefix_expr.token.token_type, tt.operator_token,
            "Prefix Expression Token Type is wrong"
        );
        assert_eq!(
            prefix_expr.token.literal, tt.operator,
            "Prefix Expression Token Literal is wrong"
        );
        assert_eq!(
            prefix_expr.operator, tt.operator,
            "Prefix Expression Operator is wrong"
        );

        test_literal_expression(&prefix_expr.right, &tt.right);
        assert_eq!(
            prefix_expr.string(),
            format!("({}{})", tt.operator, tt.right),
            "Prefix Expression String is wrong"
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
    ];

    for tt in tests {
        let program = init_program(tt.input);

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Statement length is wrong");

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            tt.left_token,
            "Expression Statement Token Type is wrong"
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.left,
            "Expression Statement Literal is wrong"
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
    ];

    for tt in tests {
        let program = init_program(tt.input);
        let string = program.string();

        assert_eq!(string, tt.expected, "Expression String is wrong");
    }
}

fn test_literal_expression(expr: &Expression, expected: &str) {
    match expr {
        Expression::Identifier(_) => test_identifier(expr, expected),
        Expression::IntegerLiteral(_) => test_integer(expr, expected.parse::<i64>().unwrap()),
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
