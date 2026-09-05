use std::fmt;
use std::ops::{Add, Mul};

struct Value {
    data: i32,
    parents: Option<Vec<Value>>,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Value {
    fn new(data: i32, parents: Option<Vec<Value>>) -> Self {
        Self { data, parents }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        let data = self.data + other.data;
        Value::new(data, Some(vec![self, other]))
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Value {
        Value::new(self.data * other.data, Some(vec![self, other]))
    }
}

fn main() {
    let a = Value::new(9, None);
    let b = Value::new(10, None);
    let d = a * b;
    print!("{}", d);
}
