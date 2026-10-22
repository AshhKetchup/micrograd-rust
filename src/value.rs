use std::cell::{Ref, RefCell};
use std::fmt;
use std::ops::{Add, Mul};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Mul,
    Sub,
    Div,
}

#[derive(Clone)]
pub struct ValueRef(Rc<RefCell<Value>>);

pub struct Value {
    data: i32,
    parents: Option<Vec<ValueRef>>,
    op: Option<Op>,
    grad: f32,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value(Data: {})", self.data)
    }
}

impl ValueRef {
    pub fn new(data: i32, parents: Option<Vec<ValueRef>>, op: Option<Op>) -> ValueRef {
        ValueRef(Rc::new(RefCell::new(Value {
            data,
            parents,
            op,
            grad: 0.0,
        })))
    }

    pub fn leaf(data: i32) -> ValueRef {
        Self::new(data, None, None)
    }

    pub fn data(&self) -> i32 {
        self.borrow().data
    }

    pub fn grad(&self) -> f32 {
        self.borrow().grad
    }

    fn borrow(&self) -> Ref<'_, Value> {
        self.0.borrow()
    }

    pub(crate) fn graph_snapshot(&self) -> (usize, i32, f32, Option<&'static str>, Vec<ValueRef>) {
        let value = self.borrow();
        (
            Rc::as_ptr(&self.0) as usize,
            value.data,
            value.grad,
            value.op.as_ref().map(Op::symbol),
            value.parents.clone().unwrap_or_default(),
        )
    }
}

impl Op {
    pub(crate) fn symbol(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Mul => "*",
            Self::Sub => "-",
            Self::Div => "/",
        }
    }
}

impl Add for ValueRef {
    type Output = ValueRef;

    fn add(self, other: ValueRef) -> ValueRef {
        let data = self.borrow().data + other.borrow().data;
        ValueRef::new(data, Some(vec![self, other]), Some(Op::Add))
    }
}

impl Mul for ValueRef {
    type Output = ValueRef;

    fn mul(self, other: ValueRef) -> ValueRef {
        let data = self.borrow().data * other.borrow().data;
        ValueRef::new(data, Some(vec![self, other]), Some(Op::Mul))
    }
}
