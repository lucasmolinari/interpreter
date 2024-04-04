use crate::lexer_utils::{lexer::Lexer, token::TokenType};
use crate::parser_utils::ast::Program;
use crate::parser_utils::{
    ast::{ExpressionStatement, Node, Statement},
    parser::Parser,
};

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
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

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
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

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
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();

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
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();

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
    let tests = vec![
        ("!5;", "!", "5", TokenType::BANG, TokenType::INT),
        ("-15;", "-", "15", TokenType::MINUS, TokenType::INT),
        ("!foobar;", "!", "foobar", TokenType::BANG, TokenType::IDENT),
        (
            "-foobar;",
            "-",
            "foobar",
            TokenType::MINUS,
            TokenType::IDENT,
        ),
    ];

    for tt in tests {
        let l = Lexer::new(tt.0.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Statement length is wrong");

        let stmt = stmts.get(0).unwrap();
        assert_eq!(
            stmt.get_token().token_type,
            tt.3,
            "Statement Token Type is wrong"
        );
        assert_eq!(
            stmt.get_token().literal,
            tt.1,
            "Statement Token Literal is wrong"
        );

        let prefix_expr = &stmt.get_statement_expr().expression.get_prefix_expr();
        assert_eq!(
            prefix_expr.token.token_type, tt.3,
            "Prefix Expression Token Type is wrong"
        );
        assert_eq!(
            prefix_expr.token.literal, tt.1,
            "Prefix Expression Token Literal is wrong"
        );
        assert_eq!(
            prefix_expr.operator, tt.1,
            "Prefix Expression Operator is wrong"
        );

        let right = &prefix_expr.right;
        if tt.4 == TokenType::INT {
            assert_eq!(
                right.get_integer_literal().value,
                tt.2.parse::<i64>().unwrap(),
                "Prefix Expression Integer Literal is wrong"
            );
        } else {
            assert_eq!(
                right.get_identifer().value,
                tt.2,
                "Prefix Expression Identifier is wrong"
            );
        }
        let string = prefix_expr.string();
        assert_eq!(
            string,
            format!("({}{})", tt.1, tt.2),
            "Prefix Expression String is wrong"
        );
    }
}

fn test_infix_expression() {
    let tests = vec![
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
        ("5 == 5;", 5, "==", 5),
        ("5 != 5;", 5, "!=", 5),
    ];

    for tt in tests {
        let l = Lexer::new(tt.0.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1, "Statement length is wrong");

        let stmt = stmts.get(0).unwrap();

        let infix_expr = stmt.get_statement_expr().expression.get_infix_expr();

        let left = &infix_expr.left;
        assert_eq!(
            left.get_integer_literal().value,
            tt.1,
            "Left Integer Literal is wrong"
        );

        let right = &infix_expr.right;
        assert_eq!(
            right.get_integer_literal().value,
            tt.3,
            "Right Integer Literal is wrong"
        );

        let operator = &infix_expr.operator;
        assert_eq!(operator, tt.2, "Operator is wrong");
    }
}

#[test]
fn test_operator_precedence() {
    let tests = vec![
        ("-a * b;", "((-a) * b)"),
        ("!-a;", "(!(-a))"),
        ("a + b + c;", "((a + b) + c)"),
        ("a + b - c;", "((a + b) - c)"),
        ("a * b * c;", "((a * b) * c)"),
        ("a * b / c;", "((a * b) / c)"),
        ("a + b / c;", "(a + (b / c))"),
        ("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5;", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4;", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4;", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5;",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
    ];

    for tt in tests {
        let l = Lexer::new(tt.0.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        let string = program.string();
        
        assert_eq!(string, tt.1, "Expression String is wrong");
    }
}
