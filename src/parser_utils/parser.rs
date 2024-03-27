use std::ptr::null;

use crate::lexer_utils::lexer::*;
use crate::lexer_utils::token::*;
use crate::parser_utils::ast::{Identifier, LetStatement, Program, Statement};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}
impl Parser {
    pub fn new(mut l: Lexer) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        let mut p = Parser {
            lexer: l,
            cur_token: cur_token,
            peek_token: peek_token,
        };
        return p;
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut prg = Program {
            statements: Vec::new(),
        };
        while self.cur_token.token_type != TokenType::EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                prg.statements.push(Box::new(stmt.unwrap()));
            }
            self.next_token();
        }
        return prg;
    }

    fn parse_statement(&mut self) -> Option<LetStatement> {
        match self.cur_token.token_type {
            TokenType::LET => return self.parse_let_statement(),
            _ => return None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone(); // This should be the LET token
        
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }
        // This should be the variable name (Identifier)
        let name = Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        let stmt = LetStatement {
            token: token,
            name: name,
            value: "value".to_string(),
        };
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        println!("LetStatement: {:?}", stmt);
        return Some(stmt);
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        return t == self.cur_token.token_type;
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        return t == self.peek_token.token_type;
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}
