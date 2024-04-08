use std::string;

use crate::lexer_utils::token::Token;

#[derive(Debug, PartialEq, Clone)]
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
            _ => panic!("{:?} is not an expression statement", self),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Node::Statement(stmt) => stmt.string(),
            Node::Expression(expr) => expr.string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

    pub fn string(&self) -> String {
        match self {
            Statement::LetStatement(stmt) => stmt.string(),
            Statement::ReturnStatement(stmt) => stmt.string(),
            Statement::ExpressionStatement(stmt) => stmt.string(),
        }
    }

    pub fn get_let_stmt(&self) -> &LetStatement {
        match self {
            Statement::LetStatement(stmt) => stmt,
            _ => panic!("Not a let statement"),
        }
    }
    
    pub fn get_return_stmt(&self) -> &ReturnStatement {
        match self {
            Statement::ReturnStatement(stmt) => stmt,
            _ => panic!("Not a return statement"),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    BooleanExpression(BooleanExpression),
    BlockStatement(BlockStatement),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
}
impl Expression {
    fn token(&self) -> &Token {
        match self {
            Expression::Identifier(expr) => &expr.token,
            Expression::IntegerLiteral(expr) => &expr.token,
            Expression::PrefixExpression(expr) => &expr.token,
            Expression::InfixExpression(expr) => &expr.token,
            Expression::BooleanExpression(expr) => &expr.token,
            Expression::IfExpression(expr) => &expr.token,
            Expression::BlockStatement(expr) => &expr.token,
            Expression::FunctionLiteral(expr) => &expr.token,
            Expression::CallExpression(expr) => &expr.token,
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

    pub fn get_boolean_expression(&self) -> &BooleanExpression {
        match self {
            Expression::BooleanExpression(expr) => expr,
            _ => panic!("Not a boolean expression"),
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

    pub fn get_block_statement(&self) -> &BlockStatement {
        match self {
            Expression::BlockStatement(expr) => expr,
            _ => panic!("Not a block statement"),
        }
    }

    pub fn get_if_expr(&self) -> &IfExpression {
        match self {
            Expression::IfExpression(expr) => expr,
            _ => panic!("Not an if expression"),
        }
    }

    pub fn get_function_expr(&self) -> &FunctionLiteral {
        match self {
            Expression::FunctionLiteral(expr) => expr,
            _ => panic!("Not a function expression"),
        }
    }

    pub fn get_call_expr(&self) -> &CallExpression {
        match self {
            Expression::CallExpression(expr) => expr,
            _ => panic!("Not a call expression"),
        }
    }

    pub fn is_integer_literal(&self) -> bool {
        match self {
            Expression::IntegerLiteral(_) => true,
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            Expression::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(expr) => expr.value.clone(),
            Expression::IntegerLiteral(expr) => expr.value.to_string(),
            Expression::PrefixExpression(expr) => expr.string(),
            Expression::InfixExpression(expr) => expr.precedence(),
            Expression::BooleanExpression(expr) => expr.string(),
            Expression::IfExpression(expr) => expr.string(),
            Expression::BlockStatement(expr) => expr.string(),
            Expression::FunctionLiteral(expr) => expr.string(),
            Expression::CallExpression(expr) => expr.string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}
impl LetStatement {
    pub fn string(&self) -> String {
        format!("{} = {};", self.name.value, self.value.string())
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}
impl ReturnStatement {
    pub fn string(&self) -> String {
        format!("return {};", self.return_value.string())
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<Expression>,
}
impl ExpressionStatement {
    pub fn string(&self) -> String {
        self.expression.string()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Node>,
}
impl BlockStatement {
    pub fn string(&self) -> String {
        let mut block = String::new();
        for stmt in &self.statements {
            match stmt {
                Node::Statement(stmt) => block.push_str(&stmt.string()),
                Node::Expression(expr) => block.push_str(&expr.string()),
            }
        }
        block
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Identifier {
    pub fn string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanExpression {
    pub token: Token,
    pub value: bool,
}
impl BooleanExpression {
    fn string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub token: Token, // Token for the operator
    pub operator: String,
    pub right: Box<Expression>,
}
impl PrefixExpression {
    pub fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub token: Token, // Token for the operator
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
impl InfixExpression {
    pub fn precedence(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.operator,
            self.right.string()
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
impl IfExpression {
    pub fn string(&self) -> String {
        let mut if_expr = String::new();
        if_expr.push_str(&format!(
            "if {} {{ {} }}",
            self.condition.string(),
            self.consequence.string()
        ));
        let alternative = match &self.alternative {
            Some(alt) => if_expr.push_str(&format!(" else {{ {} }}", alt.string())),
            None => if_expr.push_str(""),
        };
        if_expr
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionLiteral {
    pub token: Token, // fn token
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}
impl FunctionLiteral {
    pub fn string(&self) -> String {
        let parameters = self
            .parameters
            .iter()
            .map(|x| x.string())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "{} ( {} ) {{ {} }}",
            self.token.literal,
            parameters,
            self.body.string()
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub token: Token,              // '(' Token
    pub function: Box<Expression>, // Identifier | Function Literal
    pub arguments: Vec<Expression>,
}
impl CallExpression {
    pub fn string(&self) -> String {
        let args = self
            .arguments
            .iter()
            .map(|x| x.string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("{}({})", self.function.string(), args)
    }
}

pub struct Program {
    pub statements: Vec<Node>,
}
impl Program {
    pub fn string(&self) -> String {
        let mut program = String::new();
        let mut string = String::new();
        for stmt in &self.statements {
            match stmt {
                Node::Statement(stmt) => string.push_str(&stmt.string()),
                Node::Expression(expr) => string.push_str(&expr.string()),
            }
        }
        string
    }
}
