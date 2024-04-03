
use crate::lexer_utils::{lexer::Lexer, token::TokenType};
use crate::parser_utils::{ast::{ExpressionStatement, Node, Statement}, parser::Parser};

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
        assert_eq!(tok.literal, tt.0, "Test [{:?}] - Token Literal is wrong", i);
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
        assert_eq!(stmts.len(), 1);

        let stmt = stmts.get(0).unwrap();
        assert_eq!(stmt.get_token().token_type, TokenType::LET);
        assert_eq!(stmt.get_token().literal, tt.0);
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
        assert_eq!(stmts.len(), 1);

        let stmt = stmts.get(0).unwrap();
        assert_eq!(stmt.get_token().token_type, TokenType::RETURN);
        assert_eq!(stmt.get_token().literal, tt.0);
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
    assert_eq!(stmt.get_token().token_type, TokenType::IDENT);
    assert_eq!(stmt.get_token().literal, "foobar");

    let expr = &stmt.get_statement_expr().expression;
    assert_eq!(expr.get_identifer().value, "foobar");
}

#[test]
fn test_integer_literal_expression() {
    let input = String::from("5;");
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    let stmts = program.statements;
    assert_eq!(stmts.len(), 1);

    let stmt = stmts.get(0).unwrap();
    assert_eq!(stmt.get_token().token_type, TokenType::INT);
    assert_eq!(stmt.get_token().literal, "5");

    let expr = &stmt.get_statement_expr().expression;
    assert_eq!(expr.get_integer_literal().value, 5);
}

#[test]
fn test_prefix_expression() {
    let tests = vec![
        ("!5;", "!", "5", TokenType::INT, TokenType::BANG),
        ("-15;", "-", "15", TokenType::INT, TokenType::MINUS),
        ("!foobar;", "!", "foobar", TokenType::IDENT, TokenType::BANG),
        ("-foobar;", "-", "foobar", TokenType::IDENT, TokenType::MINUS),
    ];

    for tt in tests {
        let l = Lexer::new(tt.0.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        let stmts = program.statements;
        assert_eq!(stmts.len(), 1);

        let stmt = stmts.get(0).unwrap();
        assert_eq!(stmt.get_token().token_type, tt.3);
        assert_eq!(stmt.get_token().literal, tt.2);

        let prefix_expr = &stmt.get_statement_expr().expression.get_prefix_expr();
        assert_eq!(prefix_expr.token.literal, tt.1);
        assert_eq!(prefix_expr.token.token_type, tt.4);
        assert_eq!(prefix_expr.operator, tt.1);

        let right = &prefix_expr.right;
        if tt.3 == TokenType::INT {
            assert_eq!(right.get_integer_literal().value, tt.2.parse::<i64>().unwrap());
        } else {
            assert_eq!(right.get_identifer().value, tt.2);
        }
    }
}
