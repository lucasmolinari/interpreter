use crate::lexer_utils::token::Token;

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
}
impl Node {
    pub fn get_node(&self) -> &Node {
        self
    }

    pub fn get_token(&self) -> &Token {
        match self {
            Node::Statement(stmt) => stmt.token(),
            Node::Expression(expr) => expr.token(),
        }
    }

    pub fn is_stmt(&self) -> bool {
        match self {
            Node::Statement(_) => true,
            _ => false,
        }
    }

    pub fn is_expr(&self) -> bool {
        match self {
            Node::Expression(_) => true,
            _ => false,
        }
    }

    pub fn is_return_stmt(&self) -> bool {
        match self {
            Node::Statement(Statement::ReturnStatement(_)) => true,
            _ => false,
        }
    }

    pub fn is_let_stmt(&self) -> bool {
        match self {
            Node::Statement(Statement::LetStatement(_)) => true,
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            Node::Expression(Expression::Identifier(_)) => true,
            _ => false,
        }
    }

    pub fn is_integer_literal(&self) -> bool {
        match self {
            Node::Expression(Expression::IntegerLiteral(_)) => true,
            _ => false,
        }
    }

    pub fn is_prefix_expr(&self) -> bool {
        match self {
            Node::Expression(Expression::PrefixExpression(_)) => true,
            _ => false,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}
impl Statement {
    fn token(&self) -> &Token {
        match self {
            Statement::LetStatement(stmt) => &stmt.token,
            Statement::ReturnStatement(stmt) => &stmt.token,
            Statement::ExpressionStatement(stmt) => &stmt.token,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
}
impl Expression {
    fn token(&self) -> &Token {
        match self {
            Expression::Identifier(expr) => &expr.token,
            Expression::IntegerLiteral(expr) => &expr.token,
            Expression::PrefixExpression(expr) => &expr.token,
            Expression::InfixExpression(expr) => &expr.token,
        }
    }
}
#[derive(Debug, PartialEq)]

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: String,
}
#[derive(Debug, PartialEq)]

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: String,
}
#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
#[derive(Debug, PartialEq)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
pub struct Program {
    pub statements: Vec<Node>,
}
