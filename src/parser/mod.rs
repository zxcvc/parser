mod ast;

use self::ast::{PrimaryRow};

use super::scanner::{Position, Scanner, Token, TokenRow};
use ast::{
    BinaryExpression, BinaryOpeator, Exp, GroupExpression, PrimaryExpression, UanryExpression,
    UnaryOperator,
};

#[derive(Debug)]
pub struct Parser {
    token_list: Vec<Token>,
    index: usize,
}

impl<'a> Parser {
    pub fn new(s: &'a str) -> Self {
        let scanner = Scanner::new(s);
        let token_list = scanner
            .filter(|token| match token.token {
                TokenRow::Space(_) => false,
                _ => true,
            })
            .collect();
        Self {
            token_list,
            index: 0,
        }
    }

    pub fn advance(&mut self) -> Option<Token> {
        if self.index >= self.token_list.len() {
            return None;
        }
        let token = self.token_list[self.index].clone();
        self.index += 1;
        Some(token)
    }

    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        self.token_list.get(self.index + n)
    }

    pub fn next_n_match(&self, n_list: Vec<TokenRow>) -> bool {
        let token = self.peek_n(0);
        if let Some(token) = token {
            return n_list.iter().any(|x| *x == token.token);
        }
        false
    }

    pub fn expresson(&mut self) -> Option<Box<dyn Exp>> {
        let e = self.equality();
        if let Some(ex) = e {
            return Some(Box::new(ex));
        }
        None
    }

    pub fn equality(&mut self) -> Option<Box<dyn Exp>> {
        let mut left: Box<dyn Exp> = self.comprsion().unwrap();
        while self.next_n_match(vec![TokenRow::DoubleEq, TokenRow::NotEq]) {
            let op = self.advance().unwrap();
            let right = self.comprsion().unwrap();
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Some(left)
    }

    pub fn comprsion(&mut self) -> Option<Box<dyn Exp>> {
        let mut left: Box<dyn Exp> = self.term().unwrap();
        while self.next_n_match(vec![
            TokenRow::Greater,
            TokenRow::GreaterEq,
            TokenRow::Less,
            TokenRow::LessEq,
        ]) {
            let op = self.advance().unwrap();
            let right = self.term().unwrap();
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Some(left)
    }

    pub fn term(&mut self) -> Option<Box<dyn Exp>> {
        let mut left: Box<dyn Exp> = self.factor().unwrap();
        while self.next_n_match(vec![TokenRow::Plus, TokenRow::Minus]) {
            let op = self.advance().unwrap();
            let right = self.factor().unwrap();
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Some(left)
    }

    pub fn factor(&mut self) -> Option<Box<dyn Exp>> {
        let mut left: Box<dyn Exp> = self.unary().unwrap();
        while self.next_n_match(vec![TokenRow::Start, TokenRow::Div]) {
            let op = self.advance().unwrap();
            let right = self.unary().unwrap();
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Some(left)
    }

    pub fn unary(&mut self) -> Option<Box<dyn Exp>> {
        if self.next_n_match(vec![TokenRow::Minus, TokenRow::Exclamation]) {
            let mut exp: Option<Box<dyn Exp>> = None;
            while self.next_n_match(vec![TokenRow::Minus, TokenRow::Exclamation]) {
                let op = self.advance().unwrap();
                let op = UnaryOperator::new(op);
                let e = self.unary().unwrap();
                exp = Some(UanryExpression::new(op, e));
            }
            return exp;
        } else {
            let ex = self.primary();
            if let Some(_) = ex {
                return ex;
            }
        }
        None
    }

    pub fn primary(&mut self) -> Option<Box<dyn Exp>> {
        let prim = match self.peek_n(0) {
            Some(&Token {
                token: TokenRow::Digital(n),
                position: _,
            }) => PrimaryRow::Number(n),
            Some(&Token {
                token: TokenRow::String(ref s),
                position: _,
            }) => PrimaryRow::String(s.clone()),
            Some(&Token {
                token: TokenRow::True,
                position: _,
            }) => PrimaryRow::True,
            Some(&Token {
                token: TokenRow::False,
                position: _,
            }) => PrimaryRow::False,
            Some(&Token {
                token: TokenRow::Null,
                position: _,
            }) => PrimaryRow::Null,
            Some(&Token {
                token: TokenRow::LeftParent,
                position: _,
            }) => {
                return self.group();
            }
            _ => return None,
        };
        let token = self.advance().unwrap();
        let exp = PrimaryExpression::new(prim, token.position);
        Some(exp)
    }

    pub fn group(&mut self) -> Option<Box<dyn Exp>> {
        if let Some(Token {
            token: TokenRow::LeftParent,
            position: _,
        }) = self.peek_n(0)
        {
            let left_parent = self.advance().unwrap();
            let exp = self.expresson();
            let right_parent = self.advance().unwrap();
            if let Some(e) = exp {
                return Some(GroupExpression::new(
                    e,
                    (left_parent.position, right_parent.position),
                ));
            }
        }
        None
    }
}
