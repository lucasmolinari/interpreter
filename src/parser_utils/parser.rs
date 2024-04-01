use std::collections::HashMap;

use crate::lexer_utils::lexer::*;
use crate::lexer_utils::token::*;
use crate::parser_utils::ast::Node;
use crate::parser_utils::ast::PrefixExpression;
use crate::parser_utils::ast::{
    Expression, Identifier, IntegerLiteral, LetStatement, Program, ReturnStatement,
};

use super::ast::ExpressionStatement;
use super::ast::Statement;

type PrefixParse = fn(&mut Parser) -> Result<Box<dyn Expression>, ()>;
type InfixParse = fn(&mut Parser, Box<dyn Expression>) -> Result<Box<dyn Expression>, ()>;

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

        p
    }
    fn register_parsers(&mut self) {
        self.register_prefix(TokenType::IDENT, Self::parse_identifier);
        self.register_prefix(TokenType::INT, Self::parse_integer_literal);
        self.register_prefix(TokenType::BANG, Self::parse_prefix_expression);
        self.register_prefix(TokenType::MINUS, Self::parse_prefix_expression);
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
        prg
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ()> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
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

        self.next_token();

        let stmt = LetStatement {
            token: token,
            name: name,
            value: self.cur_token.literal.clone(),
        };

        self.next_token();
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        println!("LetStatement: {}", stmt.string());
        Ok(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, ()> {
        let token = self.cur_token.clone();
        self.next_token();
        let stmt = ReturnStatement {
            token: token,
            return_value: self.cur_token.literal.clone(),
        };
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        println!("ReturnStatement: {}", stmt.string());
        Ok(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, ()> {
        let expression = match self.parse_expression(Precedence::LOWEST) {
            Ok(expr) => expr,
            Err(()) => return Err(()),
        };

        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: expression,
        };
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        Ok(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Box<dyn Expression>, ()> {
        let prefix = self.prefix_parse.get(&self.cur_token.token_type);
        match prefix {
            Some(prefix_fn) => prefix_fn(self),
            None => Err(()),
        }
    }

    pub fn parse_identifier(&mut self) -> Result<Box<dyn Expression>, ()> {
        Ok(Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    pub fn parse_integer_literal(&mut self) -> Result<Box<dyn Expression>, ()> {
        let converted = self.cur_token.literal.parse::<i64>();
        match converted {
            Ok(n) => {
                Ok(Box::new(IntegerLiteral {
                    token: self.cur_token.clone(),
                    value: n,
                }))
            }
            Err(_) => {
                let e = format!("Could not parse {} as integer", self.cur_token.literal);
                self.errors.push(e);
                Err(())
            }
        }
    }

    pub fn parse_prefix_expression(&mut self) -> Result<Box<dyn Expression>, ()> {
        let token = self.cur_token.clone();
        let literal = self.cur_token.literal.clone();
        self.next_token();
        match self.parse_expression(Precedence::PREFIX) {
            Ok(right) => Ok(Box::new(PrefixExpression {
                token: token,
                operator: literal,
                right: right,
            })),
            Err(_) => Err(()),
        }
    }

    fn register_prefix(&mut self, t: TokenType, fn_ptr: PrefixParse) {
        self.prefix_parse.insert(t, fn_ptr);
    }

    fn register_infix(&mut self, t: TokenType, fn_ptr: InfixParse) {
        self.infix_parse.insert(t, fn_ptr);
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        t == self.cur_token.token_type
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        t == self.peek_token.token_type
    }

    fn expect_peek(&mut self, t: TokenType) -> Result<(), ()> {
        if self.peek_token_is(t) {
            self.next_token();
            Ok(())
        } else {
            self.peek_error(t);
            Err(())
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
        &self.errors
    }
}

enum Precedence {
    LOWEST = 0,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}
