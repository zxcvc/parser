mod ast;

use super::scanner::{Scanner, Token};
use ast::{
    BinaryExpression, BinaryOpeator, Exp, GroupExpression, Primary, PrimaryExpression,
    UanryExpression, UnaryOperator,
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
            .filter(|token| match token {
                &Token::Space(_) => false,
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

    pub fn next_n_match(&self, n_list: Vec<Token>) -> bool {
        let token = self.peek_n(0);
        if let Some(token) = token {
            return n_list.iter().any(|x| *x == *token);
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
        while self.next_n_match(vec![Token::DoubleEq, Token::NotEq]) {
            let op = self.advance().unwrap();
            let right = self.comprsion().unwrap();
            let op = match op {
                Token::DoubleEq => BinaryOpeator::Eq,
                Token::NotEq => BinaryOpeator::NotEq,
                _ => return None,
            };
            left = BinaryExpression::new(op, left, right);
        }
        Some(left)
    }

    pub fn comprsion(&mut self) -> Option<Box<dyn Exp>> {
        let mut left: Box<dyn Exp> = self.term().unwrap();
        while self.next_n_match(vec![
            Token::Greater,
            Token::GreaterEq,
            Token::Less,
            Token::LessEq,
        ]) {
            let op = self.advance().unwrap();
            let right = self.term().unwrap();
            let op = match op {
                Token::Greater => BinaryOpeator::Greater,
                Token::GreaterEq => BinaryOpeator::GreaterEq,
                Token::Less => BinaryOpeator::Less,
                Token::LessEq => BinaryOpeator::LessEq,
                _ => return None,
            };
            left = BinaryExpression::new(op, left, right);
        }
        Some(left)
    }

    pub fn term(&mut self) -> Option<Box<dyn Exp>> {
        let mut left: Box<dyn Exp> = self.factor().unwrap();
        while self.next_n_match(vec![Token::Plus, Token::Minus]) {
            let op = self.advance().unwrap();
            let right = self.factor().unwrap();
            let op = match op {
                Token::Plus => BinaryOpeator::Plus,
                Token::Minus => BinaryOpeator::Minus,
                _ => return None,
            };
            left = BinaryExpression::new(op, left, right);
        }
        Some(left)
    }

    pub fn factor(&mut self) -> Option<Box<dyn Exp>> {
        let mut left: Box<dyn Exp> = self.unary().unwrap();
        while self.next_n_match(vec![Token::Start, Token::Div]) {
            let op = self.advance().unwrap();
            let right = self.unary().unwrap();
            let op = match op {
                Token::Start => BinaryOpeator::Multip,
                Token::Div => BinaryOpeator::Div,
                _ => return None,
            };
            left = BinaryExpression::new(op, left, right);
        }
        Some(left)
    }

    pub fn unary(&mut self) -> Option<Box<dyn Exp>> {
        if self.next_n_match(vec![Token::Minus, Token::Exclamation]) {
            let mut exp: Option<Box<dyn Exp>> = None;
            while self.next_n_match(vec![Token::Minus, Token::Exclamation]) {
                let op = self.advance().unwrap();
                let op = match op {
                    Token::Exclamation => UnaryOperator::Not,
                    Token::Minus => UnaryOperator::Negative,
                    _ => return None,
                };
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
            Some(&Token::Digital(n)) => Primary::Number(n),
            Some(&Token::String(ref s)) => Primary::String(s.clone()),
            Some(&Token::True) => Primary::True,
            Some(&Token::False) => Primary::False,
            Some(&Token::Null) => Primary::Null,
            Some(&Token::LeftParent) => {
                // self.advance();
                // let e = self.expresson();
                // if let Some(&Token::RightParent) = self.peek_n(0) {
                //     Primary::Group(GroupExpression::new(e.unwrap()))
                // } else {
                //     return None;
                // }
                // let group = self.group();
                return self.group();
            }
            _ => return None,
        };
        self.advance();
        let exp = PrimaryExpression::new(prim);
        Some(exp)
    }

    pub fn group(&mut self) -> Option<Box<dyn Exp>> {
        if let Some(Token::LeftParent) = self.peek_n(0) {
            self.advance();
            let exp = self.expresson();
            self.advance();
            if let Some(e) = exp {
                return Some(GroupExpression::new(e));
            }
        }
        None
    }
}
