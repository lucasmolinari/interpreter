use std::any::Any;

use crate::parser_utils::ast::BlockStatement;

use super::environment::Environment;

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Return,
    Function,
    Null,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Return(Return),
    Function(Function),
    Null(Null),
    Error(Error),
}
impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(i) => i.object_type(),
            Object::Boolean(b) => b.object_type(),
            Object::Return(r) => r.object_type(),
            Object::Function(f) => f.object_type(),
            Object::Null(n) => n.object_type(),
            Object::Error(e) => e.object_type(),
        }
    }
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(i) => i.inspect(),
            Object::Boolean(b) => b.inspect(),
            Object::Return(r) => r.inspect(),
            Object::Function(f) => f.inspect(),
            Object::Null(n) => n.inspect(),
            Object::Error(e) => e.inspect(),
        }
    }

    pub fn downcast<T: Any>(self) -> Option<T> {
        let obj: Box<dyn Any> = match self {
            Object::Integer(i) => Box::new(i),
            Object::Boolean(b) => Box::new(b),
            Object::Return(r) => Box::new(r),
            Object::Function(f) => Box::new(f),
            Object::Null(n) => Box::new(n),
            Object::Error(e) => Box::new(e),
        };
        let opt = obj.downcast().ok().map(|x| *x);
        match opt {
            Some(x) => Some(x),
            None => None,
        }
    }

    pub fn get_return_value(&self) -> Object {
        match self {
            Object::Return(r) => *r.value.clone(),
            _ => panic!("Expected ReturnObject, got {:?}", self.inspect()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: i64,
}
impl Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
    pub fn object_type(&self) -> ObjectType {
        ObjectType::Integer
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub value: bool,
}
impl Boolean {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
    pub fn object_type(&self) -> ObjectType {
        ObjectType::Boolean
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: Box<Object>,
}
impl Return {
    fn inspect(&self) -> String {
        self.value.inspect()
    }
    pub fn object_type(&self) -> ObjectType {
        ObjectType::Return
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: BlockStatement,
    pub env: Environment,
}
impl Function {
    fn inspect(&self) -> String {
        format!(
            "fn ({}) {{\n{}\n}}",
            self.parameters.join(", "),
            self.body.string()
        )
    }
    pub fn object_type(&self) -> ObjectType {
        ObjectType::Function
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Null {}
impl Null {
    fn inspect(&self) -> String {
        "null".to_string()
    }
    pub fn object_type(&self) -> ObjectType {
        ObjectType::Null
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub message: String,
}
impl Error {
    fn inspect(&self) -> String {
        self.message.clone()
    }
    pub fn object_type(&self) -> ObjectType {
        ObjectType::Error
    }
}
