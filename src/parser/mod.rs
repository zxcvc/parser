pub mod ast;

use self::ast::{error::ParseError, Exppression::PrimaryRow};
use super::error::{NoContentError, SyntaxError as AllError};
use super::scanner::{error::ScanError, Position, Scanner, Token, TokenRow};
use ast::Exppression::{
    BinaryExpression, BinaryOpeator, Exp, GroupExpression, PrimaryExpression, UanryExpression,
    UnaryOperator,
};
use ast::StateMent::{StateMent,ExpressionStatement};

#[derive(Debug)]
pub struct Parser {
    pub token_list: Vec<Result<Token, ScanError>>,
    index: usize,
}

impl<'a> Parser {
    pub fn new(s: &'a str) -> Self {
        let scanner = Scanner::new(s);
        let token_list = scanner
            .filter(|token| match token {
                Ok(Token {
                    token: TokenRow::Space(_),
                    ..
                }) => false,
                _ => true,
            })
            .collect();
        Self {
            token_list,
            index: 0,
        }
    }

    pub fn advance(&mut self) -> Option<Result<Token, ScanError>> {
        if self.is_end() {
            return None;
        }
        let token = self.token_list[self.index].clone();
        self.index += 1;
        Some(token)
    }

    pub fn is_end(&self)->bool{
        self.index >= self.token_list.len()
    }

    pub fn peek_n(&self, n: usize) -> Option<&Result<Token, ScanError>> {
        self.token_list.get(self.index + n)
    }

    pub fn next_n_match(&self, match_list: Vec<TokenRow>) -> Result<bool, AllError> {
        let token = self.peek_n(0);
        match token {
            Some(v) => match v.as_ref() {
                Ok(token) => Ok(match_list.iter().any(|x| token.token == *x)),
                Err(error) => Err(ParseError::from(error.position.clone()).into()),
            },
            None => Ok(false),
        }
    }

    pub fn expect(&self,position:Position,expected:&'static str)->Result<bool,AllError>{
        let position = Position{col:position.col+1,..position};
        let next = self.peek_n(0);
        match next{
            Some(Ok(Token { token:TokenRow::Semicolon, position:_})) => Ok(true),
            _ => Err(ParseError{code:400,position,describe:format!(r#""{}" is expected"#,expected)}.into()),
        }
    }
    
    pub fn expresson(&mut self) -> Result<Box<dyn Exp>, AllError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let mut left = self.comprsion()?;
        while self.next_n_match(vec![TokenRow::DoubleEq, TokenRow::NotEq])? {
            let op = self.advance().unwrap()?;
            let right = self.comprsion()?;
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }

        Ok(left)
    }

    pub fn comprsion(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let mut left: Box<dyn Exp> = self.term()?;
        while self.next_n_match(vec![
            TokenRow::Greater,
            TokenRow::GreaterEq,
            TokenRow::Less,
            TokenRow::LessEq,
        ])? {
            let op = self.advance().unwrap()?;
            let right = self.term()?;
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Ok(left)
    }

    pub fn term(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let mut left: Box<dyn Exp> = self.factor()?;
        while self.next_n_match(vec![TokenRow::Plus, TokenRow::Minus])? {
            let op = self.advance().unwrap()?;
            let right = self.factor()?;
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Ok(left)
    }

    pub fn factor(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let mut left: Box<dyn Exp> = self.unary()?;
        while self.next_n_match(vec![TokenRow::Start, TokenRow::Div])? {
            let op = self.advance().unwrap()?;
            let right = self.unary()?;
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Ok(left)
    }

    pub fn unary(&mut self) -> Result<Box<dyn Exp>, AllError> {
        if self.next_n_match(vec![TokenRow::Minus, TokenRow::Exclamation])? {
            let mut exp: Result<Box<dyn Exp>, AllError> = Err(NoContentError::new().into());
            if self.next_n_match(vec![TokenRow::Minus, TokenRow::Exclamation])? {
                let op = self.advance().unwrap()?;
                let op = UnaryOperator::new(op);
                let e = self.unary()?;
                exp = Ok(UanryExpression::new(op, e));
            }
            exp
        } else {
            self.primary()
        }
    }

    pub fn primary(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let next_token = self.peek_n(0);
        let prim = match next_token {
            Some(v) => match (v.clone())? {
                Token {
                    token: t,
                    position: p,
                } => match t {
                    TokenRow::Digital(n) => PrimaryRow::Number(n),
                    TokenRow::String(ref s) => PrimaryRow::String(s.clone()),
                    TokenRow::True => PrimaryRow::True,
                    TokenRow::False => PrimaryRow::False,
                    TokenRow::Null => PrimaryRow::Null,
                    TokenRow::LeftParent => return self.group(),
                    _ => return Err(ParseError::from(p).into()),
                },
            },
            None => return Err(NoContentError::new().into()),
        };
        let token = self.advance().unwrap()?;
        let exp = PrimaryExpression::new(prim, token.position);
        Ok(exp)
    }

    pub fn group(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let left_parent = self.advance().unwrap()?;
        let exp = self.expresson()?;
        if self.next_n_match(vec![TokenRow::RightParent])? {
            let right_parent = self.advance().unwrap()?;
            return Ok(GroupExpression::new(
                exp,
                (left_parent.position, right_parent.position),
            ));
        } else {
            let next_token = self.peek_n(0);
            match next_token {
                None => Err(NoContentError::new().into()),
                Some(res) => match res {
                    Ok(token) => Err(ParseError::from(token.position.clone()).into()),
                    Err(err) => Err(ParseError::from(err.position.clone()).into()),
                },
            }
        }
    }

    pub fn statement(&mut self)->Result<Box<dyn StateMent>,AllError>{
        self.expression_statement()
    }

    pub fn expression_statement(&mut self)->Result<Box<dyn StateMent>,AllError>{
        let exp = self.expresson()?;
        self.expect(exp.get_position().1, ";")?;
        let mut semi = None;
        while self.next_n_match(vec![TokenRow::Semicolon])? {
            semi = self.advance();
        };
        let statement = ExpressionStatement::new(exp, semi.unwrap()?);
        Ok(Box::new(statement))
    }

    pub fn programing(&mut self)->Result<Vec<Box<dyn StateMent>>,AllError>{
        let mut programing = vec![];
        while !self.is_end(){
            programing.push(self.statement()?);
        }
        Ok(programing)
    }
}
