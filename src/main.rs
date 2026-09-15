use std::cell::RefCell;
use std::fmt;
use std::ops::{Add, Mul};
use std::rc::Rc;

enum Op {
    Add,
    Mul,
    Sub,
    Div,
}

type ValueRef = Rc<RefCell<Value>>;

struct Value {
    data: i32,
    parents: Option<Vec<ValueRef>>,
    op: Option<Op>,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value(Data: {})", self.data)
    }
}

impl Value {
    fn new(data: i32, parents: Option<Vec<ValueRef>>, op: Option<Op>) -> ValueRef {
        Rc::new(RefCell::new(Value { data, parents, op }))
    }
}

impl Add for ValueRef {
    type Output = ValueRef;

    fn add(self, other: ValueRef) -> ValueRef {
        let data = self.borrow().data + other.borrow().data;
        Value::new(
            data,
            Some(Rc::new(RefCell::new(vec![self, other])), Some(Op::Add)),
        )
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Value {
        Value::new(
            self.data * other.data,
            Some(vec![self, other]),
            Some(Op::Mul),
        )
    }
}

fn main() {
    let a = Rc::new(Value::new(9, None, None));
    let b = Rc::new(Value::new(10, None, none));
    let d = Rc::clone(&a) * Rc::clone(&b);
    print!("{}", d);
}
