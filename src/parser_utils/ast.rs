use crate::lexer_utils::token::Token;
use std::fmt;
pub trait Node {
    fn token_literal(&self) -> String {
        "".to_string()
    }
    fn string(&self) -> String;
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
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
    fn string(&self) -> String {
        self.value.clone()
    }
}
impl Expression for Identifier {
    fn expression_node(&self) {}
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
        self.token.literal.to_string()
    }
    fn string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.value,
            self.value
        )
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}
#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    // pub return_value: Box<dyn Expression>,
    pub return_value: String,
}
impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
    fn string(&self) -> String {
        format!("{} {};", self.token_literal(), self.return_value,)
    }
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

pub struct ExpressionStatement {
    pub token: Token,
    // pub expression: Box<dyn Expression>,
    pub expression: Box<dyn Expression>,
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!(""));

        result
    }
}
impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
    pub fn string(&self) -> String {
        let mut result = String::new();
        for stmt in &self.statements {
            result.push_str(&format!("{:?}", stmt))
        }
        result
    }
}
impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for stmt in &self.statements {
            result.push_str(&format!("{:?}", stmt));
        }
        write!(f, "{}", result)
    }
}
