use std::fmt::Debug;

pub trait Exp: Debug {}

impl Exp for Box<dyn Exp> {}

#[derive(Debug)]
pub struct PrimaryExpression(Primary);

#[derive(Debug)]
pub struct UanryExpression<T: Exp> {
    pub op: UnaryOperator,
    pub exp: T,
}

#[derive(Debug)]
pub struct BinaryExpression<T: Exp, U: Exp> {
    pub op: BinaryOpeator,
    pub left: T,
    pub right: U,
}

#[derive(Debug)]
pub struct GroupExpression<T: Exp>(T);

#[derive(Debug)]
pub enum Primary {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

#[derive(Debug)]
pub enum BinaryOpeator {
    Eq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Plus,
    Minus,
    Multip,
    Div,
}
#[derive(Debug)]
pub enum UnaryOperator {
    Not,
    Negative,
}

impl Exp for PrimaryExpression {}
impl<T: Exp> Exp for UanryExpression<T> {}
impl<T: Exp, U: Exp> Exp for BinaryExpression<T, U> {}
impl<T: Exp> Exp for GroupExpression<T> {}



impl PrimaryExpression {
    pub fn new(exp: Primary) -> Box<dyn Exp> {
        Box::new(Self(exp))
    }
}

impl<T: Exp + 'static> UanryExpression<T> {
    pub fn new(op: UnaryOperator, exp: T) -> Box<dyn Exp> {
        let e = Self { op, exp };
        Box::new(e)
    }
}

impl<T: Exp + 'static, U: Exp + 'static> BinaryExpression<T, U> {
    pub fn new(op: BinaryOpeator, left: T, right: U) -> Box<dyn Exp> {
        let e = Self { op, left, right };
        Box::new(e)
    }
}

impl<T: Exp + 'static> GroupExpression<T> {
    pub fn new(exp: T) -> Box<dyn Exp> {
        let e = Self(exp);
        Box::new(e)
    }
}
