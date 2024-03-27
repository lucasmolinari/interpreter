use crate::lexer_utils::lexer::*;
use crate::lexer_utils::token::*;
use crate::parser_utils::ast::{Identifier, LetStatement, Program};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}
impl Parser {
    pub fn new(mut l: Lexer) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        let p = Parser {
            lexer: l,
            cur_token: cur_token,
            peek_token: peek_token,
            errors: Vec::new(),
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
        while !self.cur_token_is(TokenType::EOF) {
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

        if self.expect_peek(TokenType::IDENT).is_err() {
            return None;
        }

        // This should be the variable name (Identifier)
        let name = Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        if self.expect_peek(TokenType::ASSIGN).is_err() {
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
