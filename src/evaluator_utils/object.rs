use std::any::Any;

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Return,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Return(Return),
    Null(Null),
}
impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Return(_) => ObjectType::Return,
            Object::Null(_) => ObjectType::Null,
        }
    }
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(i) => i.inspect(),
            Object::Boolean(b) => b.inspect(),
            Object::Return(r) => r.inspect(),
            Object::Null(n) => n.inspect(),
        }
    }

    pub fn downcast<T: Any>(self) -> Option<T> {
        let obj: Box<dyn Any> = match self {
            Object::Integer(i) => Box::new(i),
            Object::Boolean(b) => Box::new(b),
            Object::Return(r) => Box::new(r),
            Object::Null(n) => Box::new(n),
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub value: bool,
}
impl Boolean {
    fn inspect(&self) -> String {
        self.value.to_string()
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Null {}
impl Null {
    fn inspect(&self) -> String {
        "null".to_string()
    }
}

fn main() {
    let five = Object::Integer(Integer { value: 5 });
    let ten = Object::Integer(Integer { value: 10 });
    let true_obj = Object::Boolean(Boolean { value: true });
    let false_obj = Object::Boolean(Boolean { value: false });
    let null_obj = Object::Null(Null {});

    println!("{}", five.inspect());
    println!("{}", ten.inspect());
    println!("{}", true_obj.inspect());
    println!("{}", false_obj.inspect());
    println!("{}", null_obj.inspect());
}
