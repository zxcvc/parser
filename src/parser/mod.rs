pub mod ast;
use self::ast::{error::ParseError, Expression::PrimaryRow};
use super::error::{NoContentError, SyntaxError as AllError};
use super::scanner::{error::ScanError, Position, Scanner, Token, TokenRow};
use ast::right_value::RightValueExpression;
use ast::Expression::{
    BinaryExpression, BinaryOpeator, Exp, GroupExpression, PrimaryExpression, UanryExpression,
    UnaryOperator,
};
use ast::StateMent::{
    AssignStatement, Block, DeclareStatement, ExpressionStatement, ForStatement,
    FunctionDeclareStatement, IfStatement, ReturnStatement, StateMent, WhileStatement,
};

#[derive(Debug)]
pub struct Parser {
    pub token_list: Result<Vec<Token>,ScanError>,
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

    pub fn advance(&mut self) -> Result<Option<Token>, ScanError> {
        if self.is_end()? {
            return Ok(None);
        }
        let token_list = self.token_list.as_ref();
        let list = match token_list{
            Ok(list) => list,
            Err(e) => return Err(e.clone())
        };
        let token = list[self.index].clone();
        self.index += 1;
        Ok(Some(token.clone()))
    }

    pub fn is_end(&self) -> Result<bool,ScanError> {
        match self.token_list.as_ref(){
            Err(e) => Err(e.clone()),
            Ok(list) => Ok(self.index >= list.len())
        }
    }

    pub fn peek_n(&self, n: usize) -> Result<Option<&Token>, ScanError> {
        match self.token_list.as_ref(){
            Err(e) => Err(e.clone()),
            Ok(list) => Ok(list.get(self.index+n)),
        }
    }

    pub fn next_n_match(&self, match_list: Vec<TokenRow>) -> Result<bool, AllError> {
        let token = self.peek_n(0)?;
        match token {
            Some(v) => {
                Ok(match_list.iter().any(|x| v.token == *x))
            },
            None => Ok(false),
        }
    }

    pub fn expect(&self, position: Position, expected: TokenRow) -> Result<bool, AllError> {
        let position = Position {
            col: position.col + 1,
            ..position
        };
        let next = self.peek_n(0)?;
        let s = match expected {
            TokenRow::Dot => ".",
            TokenRow::Comma => "//",
            TokenRow::Semicolon => ";",
            TokenRow::Plus => "+",
            TokenRow::Minus => "-",
            TokenRow::Start => "*",
            TokenRow::Div => "/",
            TokenRow::Eq => "=",
            TokenRow::DoubleEq => "==",
            TokenRow::Exclamation => "!",
            TokenRow::NotEq => "!=",
            TokenRow::Greater => ">",
            TokenRow::Less => "<",
            TokenRow::GreaterEq => ">=",
            TokenRow::LessEq => "<=",
            TokenRow::LeftParent => "(",
            TokenRow::RightParent => ")",
            TokenRow::LeftBrace => "{",
            TokenRow::RightBrace => "}",
            TokenRow::Digital(_) => todo!(),
            TokenRow::String(_) => todo!(),
            TokenRow::Space(_) => todo!(),
            TokenRow::Identifier(_) => todo!(),
            TokenRow::Let => todo!(),
            TokenRow::Function => "function",
            TokenRow::Return => todo!(),
            TokenRow::If => todo!(),
            TokenRow::Else => todo!(),
            TokenRow::For => todo!(),
            TokenRow::While => todo!(),
            TokenRow::Continue => todo!(),
            TokenRow::Break => todo!(),
            TokenRow::True => todo!(),
            TokenRow::False => todo!(),
            TokenRow::Null => todo!(),
            TokenRow::This => todo!(),
        };
        match next {
            Some(Token {
                token: _expected,
                position: _,
            }) => Ok(true),
            _ => Err(ParseError {
                code: 400,
                position,
                describe: format!(r#""{}" is expected"#, s),
            }
            .into()),
        }
    }

    pub fn next_n_is(&self, n: usize, match_list: Vec<TokenRow>) -> Result<bool,AllError> {
        let n = self.peek_n(n)?;
        match n {
            Some(Token {
                token: t,
                position: _p,
            }) => Ok(match_list.iter().any(|token| token == t)),
            _ => Ok(false),
        }
    }

    pub fn expresson(&mut self) -> Result<Box<dyn Exp>, AllError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let mut left = self.comprsion()?;
        while self.next_n_match(vec![TokenRow::DoubleEq, TokenRow::NotEq])? {
            let op = self.advance()?.unwrap();
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
            let op = self.advance()?.unwrap();
            let right = self.term()?;
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Ok(left)
    }

    pub fn term(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let mut left: Box<dyn Exp> = self.factor()?;
        while self.next_n_match(vec![TokenRow::Plus, TokenRow::Minus])? {
            let op = self.advance()?.unwrap();
            let right = self.factor()?;
            let op = BinaryOpeator::new(op);
            left = BinaryExpression::new(op, left, right);
        }
        Ok(left)
    }

    pub fn factor(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let mut left: Box<dyn Exp> = self.unary()?;
        while self.next_n_match(vec![TokenRow::Start, TokenRow::Div])? {
            let op = self.advance()?.unwrap();
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
                let op = self.advance()?.unwrap();
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
        let next_token = self.peek_n(0)?;
        let prim = match next_token {
            Some(v) => match v.clone() {
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
        let token = self.advance()?.unwrap();
        let exp = PrimaryExpression::new(prim, token.position);
        Ok(exp)
    }

    pub fn group(&mut self) -> Result<Box<dyn Exp>, AllError> {
        let left_parent = self.advance()?.unwrap();
        let exp = self.expresson()?;
        if self.next_n_match(vec![TokenRow::RightParent])? {
            let right_parent = self.advance()?.unwrap();
            return Ok(GroupExpression::new(
                exp,
                (left_parent.position, right_parent.position),
            ));
        } else {
            let next_token = self.peek_n(0)?;
            match next_token {
                None => Err(NoContentError::new().into()),
                Some(res) => {
                    Err(ParseError::from(res.position.clone()).into())
                }
            }
        }
    }

    pub fn statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        match self.peek_n(0)? {
            None => todo!(),
            Some(res) => match res.clone() {
                Token {
                    token: t,
                    position: _p,
                } => match t {
                    TokenRow::Let => self.declare_statement(),
                    TokenRow::Identifier(_ident) if self.next_n_is(1, vec![TokenRow::Eq])? => {
                        self.assign_statement()
                    }
                    TokenRow::If => self.if_statement(),
                    TokenRow::For => self.for_statement(),
                    TokenRow::While => self.while_statement(),
                    TokenRow::Return => self.return_statement(),
                    TokenRow::LeftBrace => self.block(),
                    TokenRow::Function => self.function_declare_statement(),
                    _ => self.expression_statement(),
                },
            },
        }
    }

    pub fn right_value(&mut self) -> Result<RightValueExpression, AllError> {
        match self.peek_n(0)? {
            Some(v) => match v {
                Token {
                    token: _t,
                    position: _p,
                } => (Ok(self.expresson()?.into())),
            },
            None => Err(NoContentError::new().into()),
        }
    }

    pub fn declare_statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let let_ident = self.advance()?.unwrap();
        let assign_statement = self.assign_statement_row()?;
        let mut declare_statement: DeclareStatement = assign_statement.into();
        declare_statement.start = let_ident.position;
        Ok(Box::new(declare_statement))
    }

    pub fn assign_statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let assign_statement_row = self.assign_statement_row()?;
        Ok(Box::new(assign_statement_row))
    }

    fn assign_statement_row(&mut self) -> Result<AssignStatement, AllError> {
        let variable_token = self.advance()?.unwrap();
        self.advance();
        let right_value = self.right_value()?;
        let value_end = right_value.0.get_position().1;
        Ok(AssignStatement::new(
            variable_token.token,
            right_value,
            (variable_token.position, value_end),
        ))
    }

    pub fn if_statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let if_token = self.advance()?.unwrap();
        self.expect(if_token.position.clone(), TokenRow::LeftParent)?;
        self.advance();
        let condition = self.expresson()?;
        self.expect(if_token.position.clone(), TokenRow::RightParent)?;
        self.advance();
        let then_branch = self.statement()?;
        let mut end_position = then_branch.get_position().1;
        let mut else_branch = None;
        if self.next_n_match(vec![TokenRow::Else])? {
            self.advance()?.unwrap();
            let _else_branch = self.statement()?;
            end_position = _else_branch.get_position().1;
            else_branch = Some(_else_branch);
        }
        let if_statement = IfStatement::new(
            condition,
            then_branch,
            else_branch,
            (if_token.position, end_position),
        );
        Ok(Box::new(if_statement))
    }

    pub fn for_statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let for_token = self.advance()?.unwrap();
        let start = for_token.position.clone();

        self.expect(for_token.position, TokenRow::LeftParent)?;
        self.advance();

        let mut init_statement: Option<Box<dyn StateMent>> = None;
        if !self.next_n_is(0, vec![TokenRow::Semicolon])? {
            let _init_statement = self.statement()?;
            self.expect(_init_statement.get_position().1, TokenRow::Semicolon)?;
            self.advance();
            init_statement = Some(_init_statement);
        } else {
            self.advance();
        }

        let mut condition: Option<Box<dyn Exp>> = None;
        if !self.next_n_is(0, vec![TokenRow::Semicolon])? {
            let _condition = self.expresson()?;
            self.expect(_condition.get_position().1, TokenRow::Semicolon)?;
            self.advance();
            condition = Some(_condition);
        } else {
            self.advance();
        }

        let mut next_statement: Option<Box<dyn StateMent>> = None;
        if !self.next_n_is(0, vec![TokenRow::RightParent])? {
            let _next_statement = self.statement()?;
            self.expect(_next_statement.get_position().1, TokenRow::RightParent)?;
            self.advance();
            next_statement = Some(_next_statement);
        } else {
            self.advance();
        }

        let body = self.statement()?;

        let end = body.get_position().1;

        let for_statement = ForStatement::new(
            init_statement,
            condition,
            next_statement,
            body,
            (start, end),
        );
        Ok(Box::new(for_statement))
    }

    pub fn while_statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let while_token = self.advance()?.unwrap();
        self.expect(while_token.position.clone(), TokenRow::LeftParent)?;
        self.advance();
        let condition = self.expresson()?;
        self.expect(condition.get_position().1, TokenRow::RightParent)?;
        self.advance();
        let body = self.statement()?;
        let end_position = body.get_position().1;
        let while_statement =
            WhileStatement::new(condition, body, (while_token.position, end_position));
        Ok(Box::new(while_statement))
    }

    pub fn return_statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let return_token = self.advance()?.unwrap();
        let mut exp: Option<Box<dyn Exp>> = None;
        let start = return_token.position;
        let mut end = Position::new(start.row, start.col + 6);
        if !self.next_n_is(0, vec![TokenRow::Semicolon])? {
            let _exp = self.expresson()?;
            end = _exp.get_position().1;
            exp = Some(_exp);
        }
        let return_statement = ReturnStatement::new(exp, (start, end));
        Ok(Box::new(return_statement))
    }

    pub fn expression_statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let exp = self.expresson()?;
        let statement = ExpressionStatement::new(exp);
        Ok(Box::new(statement))
    }

    pub fn block(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let left_brace = self.advance()?.unwrap();
        let mut body = vec![];
        while !self.next_n_match(vec![TokenRow::RightBrace])? {
            while !self.is_end()? && self.next_n_is(0, vec![TokenRow::Semicolon])? {
                self.advance();
            }
            if !self.next_n_match(vec![TokenRow::RightBrace])? {
                let statement = self.statement()?;
                if statement.need_semi() {
                    self.expect(statement.get_position().1, TokenRow::Semicolon)?;
                    self.advance();
                }
                body.push(statement);
            }

            while !self.is_end()? && self.next_n_is(0, vec![TokenRow::Semicolon])? {
                self.advance();
            }
        }
        let right_brace = self.advance()?.unwrap();
        let block = Block::new(body, (left_brace.position, right_brace.position));
        Ok(Box::new(block))
    }

    pub fn get_args(&mut self)->Result<Vec<Token>,AllError>{
        let mut args = vec![];
        while !self.next_n_match(vec![TokenRow::RightParent])?{
            let arg = self.advance()?.unwrap();
            args.push(arg);
            match self.peek_n(0)?{
                Some(ret) =>{
                    todo!("重构scanner的错误处理");
                }
                None =>{}
            }
        }
        Ok(args)
    }

    pub fn function_declare_statement(&mut self) -> Result<Box<dyn StateMent>, AllError> {
        let function_token = self.advance()?.unwrap();
        let identity = self.advance()?;
        match identity {
            Some(Token {
                token: TokenRow::Identifier(name),
                position,
            }) => {
                self.expect(position, TokenRow::LeftParent)?;
                dbg!(self.advance());
                let mut args = vec![];
                while !self.next_n_match(vec![TokenRow::RightParent])? {
                    self.advance();
                    todo!();
                }
                self.advance();
                let _body = self.block()?;
                let body = std::mem::take(unsafe { &mut *(Box::into_raw(_body) as *mut Block) });
                let end = body.get_position().1;
                let function_declare_statement =
                    FunctionDeclareStatement::new(name, args, body, (function_token.position, end));
                Ok(Box::new(function_declare_statement))
            }
            _ => Err(AllError::ParseError(ParseError {
                code: 400,
                describe: "function name is expected".to_string(),
                position: Position {
                    row: function_token.position.row,
                    col: function_token.position.col + 1,
                },
            })),
        }
    }

    pub fn programing(&mut self) -> Result<Vec<Box<dyn StateMent>>, AllError> {
        let mut programing = vec![];
        while !self.is_end()? {
            while !self.is_end()? && self.next_n_is(0, vec![TokenRow::Semicolon])? {
                self.advance();
            }
            let statement = self.statement()?;
            if statement.need_semi() {
                self.expect(statement.get_position().1, TokenRow::Semicolon)?;
                self.advance();
            }
            programing.push(statement);
            while !self.is_end()? && self.next_n_is(0, vec![TokenRow::Semicolon])? {
                self.advance();
            }
        }
        Ok(programing)
    }
}
