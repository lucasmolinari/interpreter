use std::collections::HashMap;
use std::hash::Hash;

use crate::lexer_utils::lexer::*;
use crate::lexer_utils::token;
use crate::lexer_utils::token::*;
use crate::parser_utils::ast::{
    Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
    Node, PrefixExpression, Program, ReturnStatement, Statement,
};

type PrefixParse = fn(&mut Parser) -> Result<Expression, String>;
type InfixParse = fn(&mut Parser, Expression) -> Result<Expression, String>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Precedence {
    LOWEST = 0,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse: HashMap<TokenType, PrefixParse>,
    infix_parse: HashMap<TokenType, InfixParse>,
    precedence_table: HashMap<TokenType, Precedence>,
}
impl Parser {
    pub fn new(mut l: Lexer) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();

        let precedence_table = HashMap::from([
            (TokenType::EQ, Precedence::EQUALS),
            (TokenType::NOTEQ, Precedence::EQUALS),
            (TokenType::LT, Precedence::LESSGREATER),
            (TokenType::GT, Precedence::LESSGREATER),
            (TokenType::PLUS, Precedence::SUM),
            (TokenType::MINUS, Precedence::SUM),
            (TokenType::ASTERISK, Precedence::PRODUCT),
            (TokenType::SLASH, Precedence::PRODUCT),
        ]);

        let mut p = Parser {
            lexer: l,
            cur_token: cur_token,
            peek_token: peek_token,
            errors: Vec::new(),
            prefix_parse: HashMap::new(),
            infix_parse: HashMap::new(),
            precedence_table: precedence_table,
        };
        p.register_parsers();

        p
    }
    fn register_parsers(&mut self) {
        self.register_prefix(TokenType::IDENT, Self::parse_identifier);
        self.register_prefix(TokenType::INT, Self::parse_integer_literal);
        self.register_prefix(TokenType::BANG, Self::parse_prefix_expression);
        self.register_prefix(TokenType::MINUS, Self::parse_prefix_expression);

        self.register_infix(TokenType::PLUS, Self::parse_infix_expression);
        self.register_infix(TokenType::MINUS, Self::parse_infix_expression);
        self.register_infix(TokenType::SLASH, Self::parse_infix_expression);
        self.register_infix(TokenType::ASTERISK, Self::parse_infix_expression);
        self.register_infix(TokenType::EQ, Self::parse_infix_expression);
        self.register_infix(TokenType::NOTEQ, Self::parse_infix_expression);
        self.register_infix(TokenType::LT, Self::parse_infix_expression);
        self.register_infix(TokenType::GT, Self::parse_infix_expression);
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

    fn parse_statement(&mut self) -> Result<Node, String> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Node, String> {
        let token = self.cur_token.clone(); // This should be the LET token
        if self.expect_peek(TokenType::IDENT).is_err() {
            return Err("Expected identifier".to_string());
        }

        // This should be the variable name (Identifier)
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if self.expect_peek(TokenType::ASSIGN).is_err() {
            return Err("Expected ASSIGN token".to_string());
        }

        self.next_token();

        let stmt = Node::Statement(Statement::LetStatement(LetStatement {
            token: token,
            name: name,
            value: self.cur_token.literal.clone(),
        }));

        self.next_token();
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        Ok(stmt)
    }

    fn parse_return_statement(&mut self) -> Result<Node, String> {
        let token = self.cur_token.clone();
        self.next_token();

        let stmt = Node::Statement(Statement::ReturnStatement(ReturnStatement {
            token: token,
            return_value: self.cur_token.literal.clone(),
        }));

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        Ok(stmt)
    }

    fn parse_expression_statement(&mut self) -> Result<Node, String> {
        let token = self.cur_token.clone();
        
        let expression = match self.parse_expression(Precedence::LOWEST) {
            Ok(expr) => Box::new(expr),
            Err(err) => return Err(format!("Could not parse expression, received error: {}", err)),
        };

        let stmt = Node::Statement(Statement::ExpressionStatement(ExpressionStatement {
            token: token,
            expression: expression,
        }));

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        Ok(stmt)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let prefix = self.prefix_parse.get(&self.cur_token.token_type);
        
        // 1 = Expression Integer Literal 5
        let mut left = match prefix {
            Some(prefix_fn) => prefix_fn(self),
            None => return Err(format!("No prefix parse function for {:?}", self.cur_token.token_type)),
        };

        while !self.peek_token_is(TokenType::SEMICOLON) && &precedence < self.peek_precedence() {
            let infix_fn = self.infix_parse.get(&self.peek_token.token_type);
            if infix_fn.is_none() {
                return left;
            }
            left = infix_fn.unwrap()(self, left.unwrap());
        }
        left
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, String> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        
        let right = match self.parse_expression(Precedence::PREFIX) {
            Ok(expr) => expr,
            Err(err) => return Err(format!("Could not parse right expression in prefix, received error: {}", err)),
        };

        Ok(Expression::PrefixExpression(PrefixExpression {
            token: token,
            operator: operator,
            right: Box::new(right),
        }))
    }

    fn parse_infix_expression(&mut self, expr: Expression) -> Result<Expression, String> {
        self.next_token();
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let left = expr;
        let precedence = self.cur_precedence();

        self.next_token();
        
        let right = match self.parse_expression(    precedence) {
            Ok(expr) => expr,
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(format!("Could not parse right expression in infix, received error: {}", err))
            }
        };
     
        Ok(Expression::InfixExpression(InfixExpression {
            token: token,
            operator: operator,
            left: Box::new(left),
            right: Box::new(right),
        }))
    }

    pub fn parse_identifier(&mut self) -> Result<Expression, String> {
        Ok(Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    pub fn parse_integer_literal(&mut self) -> Result<Expression, String> {
        let converted = self.cur_token.literal.parse::<i64>();
        match converted {
            Ok(n) => Ok(Expression::IntegerLiteral(IntegerLiteral {
                token: self.cur_token.clone(),
                value: n,
            })),
            Err(_) => {
                let e = format!("Could not parse {} as integer", self.cur_token.literal);
                self.errors.push(e.clone());
                Err(e)
            }
        }
    }

    fn register_prefix(&mut self, t: TokenType, fn_ptr: PrefixParse) {
        self.prefix_parse.insert(t, fn_ptr);
    }

    fn register_infix(&mut self, t: TokenType, fn_ptr: InfixParse) {
        self.infix_parse.insert(t, fn_ptr);
    }

    fn cur_token_is(&mut self, t: TokenType) -> bool {
        t == self.cur_token.token_type
    }

    fn peek_token_is(&mut self, t: TokenType) -> bool {
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

    fn cur_precedence(&mut self) -> Precedence {
        match self.precedence_table.get(&self.cur_token.token_type) {
            Some(p) => p.clone(),
            None => Precedence::LOWEST,
        }
    }

    fn peek_precedence(&mut self) -> &Precedence {
        match self.precedence_table.get(&self.peek_token.token_type) {
            Some(p) => p,
            None => &Precedence::LOWEST,
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        let e = format!(
            "Expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        );
        self.errors.push(e);
    }
    pub fn errors(&mut self) -> &Vec<String> {
        &self.errors
    }
}
