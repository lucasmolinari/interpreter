use crate::lexer_utils::token::Token;
use std::fmt;
pub trait Node {
    fn token_literal(&self) -> String;
}
pub trait Statement: Node {
    fn statement_node(&self);
}
impl fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.token_literal())
    }
}
pub trait Expression: Node {
    fn expression_node(&self);
}
#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Box<Identifier>,
    // pub value: Box<dyn Expression>,
    pub value: String,
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Identifier {
    fn expression_node(&self) {}
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return "".to_string();
        }
    }
}
impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for statement in &self.statements {
            result.push_str(&format!("{:?}", statement));
        }
        write!(f, "{}", result)
    }
}
