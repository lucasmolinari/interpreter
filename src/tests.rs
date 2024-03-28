use std::collections::HashMap;

use crate::lexer_utils::lexer::Lexer;
use crate::parser_utils::parser::Parser;

#[derive(Eq, Hash, PartialEq)]
pub enum Test {
    IdentifierExpression,
    IntegerLiteralExpression,
}

pub struct TestError {
    pub errors: HashMap<Test, String>,
}

impl TestError {
    pub fn new() -> TestError {
        TestError {
            errors: HashMap::new(),
        }
    }

    pub fn test_all(&mut self) -> Vec<Result<String, String>> {
        let mut results: Vec<Result<String, String>> = Vec::new();
        results.push(self.test_identifier_expresssion());
        results.push(self.test_integer_literal_expression());
        results
    }

    pub fn test_identifier_expresssion(&mut self) -> Result<String, String> {
        let test_ident_expression = "foobar;".to_string();
        let mut p = self.init_parser(test_ident_expression);
        let stmts = p.parse_program().statements;

        let mut err = String::new();
        let e = p.errors();
        if e.len() > 0 {
            err = format!("Identifier Expression -> Parser has {} errors", e.len());
            return Err(err);
        }
        if stmts.len() != 1 {
            err = format!("Identifier Expression -> Parser has {} statements, expected 1", stmts.len());
            return Err(err);
        }

        let stmt = stmts[0].as_ref();
        if stmt.token_literal() != "foobar" {
            err = format!(
                "Identifier Expression -> Expected token_literal to be foobar but got {}",
                stmt.token_literal()
            );
            return Err(err);
        }
        if err != "" {
            self.errors.insert(Test::IdentifierExpression, err);
        }
        Ok("Identifier Expression -> OK".to_string())
    }

    pub fn test_integer_literal_expression(&mut self) -> Result<String, String> {
        let test_int_expression = "5;".to_string();
        let mut p = self.init_parser(test_int_expression);
        let stmts = p.parse_program().statements;

        let mut err = String::new();

        let e = p.errors();
        if e.len() > 0 {
            err = format!("Integer Literal Expression -> Parser has {} errors", e.len());
            return Err(err);
        }
        if stmts.len() != 1 {
            err = format!("Integer Literal Expression -> Parser has {} statements, expected 1", stmts.len());
            return Err(err);
        }
        let stmt = stmts[0].as_ref();

        if stmt.token_literal() != "5" {
            err = format!(
                "Integer Literal Expression -> Expected token_literal to be 5, got {}",
                stmt.token_literal()
            );
            return Err(err);
        }

        if stmt.string() != "5;" {
            err = format!("Integer Literal Expression -> Expected string to be 5;, got {}", stmt.string());
            return Err(err);
        }

        if err != "" {
            self.errors.insert(Test::IntegerLiteralExpression, err);
        }
        Ok("Integer Literal Expression -> OK".to_string())
    }

    fn init_parser(&self, lexer_input: String) -> Parser {
        let l = Lexer::new(lexer_input);
        Parser::new(l)
    }
}
