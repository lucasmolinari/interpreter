use std::collections::HashMap;

use crate::lexer_utils::lexer::*;
use crate::lexer_utils::token::*;
use crate::parser_utils::ast::Node;
use crate::parser_utils::ast::{Expression, Identifier, LetStatement, Program, ReturnStatement};

use super::ast::ExpressionStatement;
use super::ast::Statement;

type PrefixParse = fn(&Parser) -> Box<dyn Expression>;
type InfixParse = fn(&Parser, Box<dyn Expression>) -> Box<dyn Expression>;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse: HashMap<TokenType, PrefixParse>,
    infix_parse: HashMap<TokenType, InfixParse>,
}
impl Parser {
    pub fn new(mut l: Lexer) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        let mut p = Parser {
            lexer: l,
            cur_token: cur_token,
            peek_token: peek_token,
            errors: Vec::new(),
            prefix_parse: HashMap::new(),
            infix_parse: HashMap::new(),
        };
        p.register_parsers();
        return p;
    }
    fn register_parsers(&mut self){
        self.register_prefix(TokenType::IDENT, Self::parse_identifier);

    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut prg = Program {
            statements: Vec::new(),
        };
        while !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            if stmt.is_ok() {
                prg.statements.push(stmt.unwrap());
            }
            self.next_token();
        }
        return prg;
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ()> {
        match self.cur_token.token_type {
            TokenType::LET => return self.parse_let_statement(),
            TokenType::RETURN => return self.parse_return_statement(),
            _ => return self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, ()> {
        let token = self.cur_token.clone(); // This should be the LET token

        if self.expect_peek(TokenType::IDENT).is_err() {
            return Err(());
        }

        // This should be the variable name (Identifier)
        let name = Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        if self.expect_peek(TokenType::ASSIGN).is_err() {
            return Err(());
        }

        let stmt = LetStatement {
            token: token,
            name: name,
            value: "LetValue".to_string(),
        };
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        println!("LetStatement: {}", stmt.string());
        return Ok(Box::new(stmt));
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, ()> {
        let token = self.cur_token.clone();
        self.next_token();
        let stmt = ReturnStatement {
            token: token,
            return_value: "ReturnValue".to_string(),
        };
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        println!("ReturnStatement: {}", stmt.string());
        return Ok(Box::new(stmt));
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, ()> {
        let expression = match self.parse_expression(Precedence::LOWEST) {
            Ok(expr) => expr,
            Err(()) => return Err(())
        };

        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: expression
        };
        if self.peek_token_is(TokenType::SEMICOLON){
            self.next_token()
        }
        return Ok(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Box<dyn Expression>, ()> {
        let prefix = self.prefix_parse.get(&self.cur_token.token_type);
        match prefix {
            Some(prefix_fn) => Ok(prefix_fn(self)),
            None => Err(())
        }
    }

    fn register_prefix(&mut self, t: TokenType, fn_ptr: PrefixParse) {
        self.prefix_parse.insert(t, fn_ptr);
    }

    fn register_infix(&mut self, t: TokenType, fn_ptr: InfixParse) {
        self.infix_parse.insert(t, fn_ptr);
    }

    pub fn parse_identifier(&self) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        return t == self.cur_token.token_type;
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        return t == self.peek_token.token_type;
    }

    fn expect_peek(&mut self, t: TokenType) -> Result<(), ()> {
        if self.peek_token_is(t) {
            self.next_token();
            return Ok(());
        } else {
            self.peek_error(t);
            return Err(());
        }
    }
    fn peek_error(&mut self, t: TokenType) {
        let e = format!(
            "Expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        );
        self.errors.push(e);
    }
    pub fn errors(&self) -> &Vec<String> {
        return &self.errors;
    }
}

enum Precedence {
    LOWEST = 0,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL
}