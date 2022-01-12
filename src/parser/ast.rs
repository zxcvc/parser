use super::{Position, Token, TokenRow};
use error::ParseError;
use std::fmt::Debug;

pub mod error {
    use crate::scanner::{Position, Token};

    #[derive(Debug, Clone)]
    pub struct ParseError {
        pub code: i32,
        pub describe: String,
        pub position: super::Position,
    }

    impl From<Position> for ParseError {
        fn from(position: Position) -> Self {
            Self {
                code: 400,
                describe: "unexpected token".to_string(),
                position,
            }
        }
    }
    impl From<Token> for ParseError {
        fn from(token: Token) -> Self {
            Self {
                code: 400,
                describe: "unexpected token".to_string(),
                position: token.position,
            }
        }
    }
}

pub trait Exp: Debug {
    fn get_position(&self) -> (Position, Position);
}

impl Exp for Box<dyn Exp> {
    fn get_position(&self) -> (Position, Position) {
        (**self).get_position()
    }
}

#[derive(Debug)]
pub struct PrimaryExpression {
    pub exp: PrimaryRow,
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct UanryExpression<T: Exp> {
    pub op: UnaryOperator,
    pub exp: T,
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct BinaryExpression<T: Exp, U: Exp> {
    pub op: BinaryOpeator,
    pub left: T,
    pub right: U,
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct GroupExpression<T: Exp> {
    pub exp: T,
    pub start: Position,
    pub end: Position,
}

// #[derive(Debug)]
// pub struct Primary {
//     pub exp: PrimaryRow,
//     pub start: Position,
//     pub end: Position,
// }

#[derive(Debug)]
pub enum PrimaryRow {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

#[derive(Debug)]
pub struct BinaryOpeator {
    pub op: BinaryOpeatorRow,
    pub start: Position,
    pub end: Position,
}
#[derive(Debug)]
pub enum BinaryOpeatorRow {
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
pub struct UnaryOperator {
    pub op: UnaryOperatorRow,
    pub start: Position,
    pub end: Position,
}
#[derive(Debug)]
pub enum UnaryOperatorRow {
    Not,
    Negative,
}

impl Exp for PrimaryExpression {
    fn get_position(&self) -> (Position, Position) {
        (self.start.clone(), self.end.clone())
    }
}
impl<T: Exp> Exp for UanryExpression<T> {
    fn get_position(&self) -> (Position, Position) {
        (self.start.clone(), self.end.clone())
    }
}
impl<T: Exp, U: Exp> Exp for BinaryExpression<T, U> {
    fn get_position(&self) -> (Position, Position) {
        (self.start.clone(), self.end.clone())
    }
}
impl<T: Exp> Exp for GroupExpression<T> {
    fn get_position(&self) -> (Position, Position) {
        (self.start.clone(), self.end.clone())
    }
}

impl BinaryOpeatorRow {
    pub fn len(&self) -> usize {
        match self {
            &BinaryOpeatorRow::Eq => 2,
            &BinaryOpeatorRow::NotEq => 2,
            &BinaryOpeatorRow::Greater => 1,
            &BinaryOpeatorRow::GreaterEq => 2,
            &BinaryOpeatorRow::Less => 1,
            &BinaryOpeatorRow::LessEq => 2,
            &BinaryOpeatorRow::Plus => 1,
            &BinaryOpeatorRow::Minus => 1,
            &BinaryOpeatorRow::Multip => 1,
            &BinaryOpeatorRow::Div => 1,
        }
    }
}

impl UnaryOperatorRow {
    pub fn len(&self) -> usize {
        match self {
            &UnaryOperatorRow::Not => 1,
            &UnaryOperatorRow::Negative => 1,
        }
    }
}

impl BinaryOpeator {
    pub fn new(token: Token) -> Self {
        let op_row = match token.token {
            TokenRow::DoubleEq => BinaryOpeatorRow::Eq,
            TokenRow::NotEq => BinaryOpeatorRow::NotEq,

            TokenRow::Greater => BinaryOpeatorRow::Greater,
            TokenRow::GreaterEq => BinaryOpeatorRow::GreaterEq,
            TokenRow::Less => BinaryOpeatorRow::Less,
            TokenRow::LessEq => BinaryOpeatorRow::LessEq,

            TokenRow::Plus => BinaryOpeatorRow::Plus,
            TokenRow::Minus => BinaryOpeatorRow::Minus,
            TokenRow::Start => BinaryOpeatorRow::Multip,
            TokenRow::Div => BinaryOpeatorRow::Div,
            _ => BinaryOpeatorRow::Eq,
        };
        let op_len = op_row.len().saturating_sub(1);
        Self {
            op: op_row,
            start: token.position.clone(),
            end: Position::new(token.position.row, token.position.col + op_len),
        }
    }
}

impl UnaryOperator {
    pub fn new(token: Token) -> Self {
        let op_row = match token.token {
            TokenRow::Exclamation => UnaryOperatorRow::Not,
            TokenRow::Minus => UnaryOperatorRow::Negative,
            _ => UnaryOperatorRow::Negative,
        };
        let op_len = op_row.len().saturating_sub(1);
        Self {
            op: op_row,
            start: token.position.clone(),
            end: Position::new(token.position.row, token.position.col + op_len),
        }
    }
}

impl PrimaryRow {
    pub fn len(&self) -> usize {
        match self {
            PrimaryRow::Number(n) => n.to_string().len(),
            PrimaryRow::String(s) => s.len(),
            PrimaryRow::True => 4,
            PrimaryRow::False => 4,
            PrimaryRow::Null => 4,
        }
    }
}

impl PrimaryExpression {
    pub fn new(exp: PrimaryRow, position: Position) -> Box<dyn Exp> {
        let op_len = exp.len().saturating_sub(1);
        let e = Self {
            exp: exp,
            start: position.clone(),
            end: Position::new(position.row, position.col + op_len),
        };
        Box::new(e)
    }
}

impl<T: Exp + 'static> UanryExpression<T> {
    pub fn new(op: UnaryOperator, exp: T) -> Box<dyn Exp> {
        let start = op.start.clone();
        let end = exp.get_position().1;
        let e = Self {
            op,
            exp,
            start,
            end,
        };
        Box::new(e)
    }
}

impl<T: Exp + 'static, U: Exp + 'static> BinaryExpression<T, U> {
    pub fn new(op: BinaryOpeator, left: T, right: U) -> Box<dyn Exp> {
        let start = left.get_position().0;
        let end = right.get_position().1;
        let e = Self {
            op,
            left,
            right,
            start,
            end,
        };
        Box::new(e)
    }
}

impl<T: Exp + 'static> GroupExpression<T> {
    pub fn new(exp: T, position: (Position, Position)) -> Box<dyn Exp> {
        let (start, end) = position;
        let e = Self { exp, start, end };
        Box::new(e)
    }
}
