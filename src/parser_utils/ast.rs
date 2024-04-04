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

    pub fn get_expression(&self) -> &Expression {
        match self {
            Node::Expression(expr) => expr,
            _ => panic!("Not an expression"),
        }
    }

    pub fn get_statement(&self) -> &Statement {
        match self {
            Node::Statement(stmt) => stmt,
            _ => panic!("Not a statement"),
        }
    }

    pub fn get_statement_expr(&self) -> &ExpressionStatement {
        match self {
            Node::Statement(Statement::ExpressionStatement(expr)) => expr,
            _ => panic!("Not an expression statement"),
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
#[derive(Debug, Clone, PartialEq)]
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
    pub fn get_identifer(&self) -> &Identifier {
        match self {
            Expression::Identifier(expr) => expr,
            _ => panic!("Not an identifier"),
        }
    }
    pub fn get_integer_literal(&self) -> &IntegerLiteral {
        match self {
            Expression::IntegerLiteral(expr) => expr,
            _ => panic!("Not an integer literal"),
        }
    }
    pub fn get_prefix_expr(&self) -> &PrefixExpression {
        match self {
            Expression::PrefixExpression(expr) => expr,
            _ => panic!("Not a prefix expression"),
        }
    }
    pub fn get_infix_expr(&self) -> &InfixExpression {
        match self {
            Expression::InfixExpression(expr) => expr,
            _ => panic!("Not an infix expression"),
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

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub token: Token, // Token for the operator
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
pub struct Program {
    pub statements: Vec<Node>,
}
