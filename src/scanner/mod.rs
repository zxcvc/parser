mod utils;
use std::{iter::Peekable, str::Chars, error::Error, collections::HashMap};
use utils::{is_space,is_digital,is_alpha,is_alphadigital};
use lazy_static::lazy_static;


#[derive(Debug,Clone)]
pub enum Token {
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Start,
    Div,
    Eq,
    DoubleEq,
    Exclamation,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    LeftParent,
    RightParent,
    LeftBrace,
    RightBrace,
    Digital(f64),
    String(String),
    Space(String),

    Identifier(String),

    // 关键字：
    Let,
    Function,
    Return,
    If,
    Else,
    While,
    Continue,
    Break,
    True,
    False,
    This
}

lazy_static!{
    static ref IDENTIFER_MAP:HashMap<&'static str,Token> = {
        let mut map = HashMap::new();
        map.insert("let", Token::Let);
        map.insert("function", Token::Function);
        map.insert("return", Token::Return);
        map.insert("if", Token::If);
        map.insert("else", Token::Else);
        map.insert("while", Token::While);
        map.insert("continue", Token::Continue);
        map.insert("break", Token::Break);
        map.insert("true", Token::True);
        map.insert("false", Token::False);
        map.insert("this", Token::This);
        map
    };
    
}





#[derive(Default,Debug,Clone,)]
struct Position{
    row:u32,
    col:u32,
}

impl Position{
    pub fn new(row:u32,col:u32)->Self{
        Self{
            row,
            col,
        }
    }
}

#[derive(Debug)]
pub struct Scanner<'a>{
    source:Peekable<Chars<'a>>,
    current_string:String,
    position:Position
}

impl<'a> Scanner<'a>{
    pub fn new(source:&'a str)->Self{
        Self{
            source:source.chars().peekable(),
            current_string:String::new(),
            position:Position::default(),
        }
    }

    pub fn clear(&mut self){
        self.current_string.clear();
    }

    pub fn advance(&mut self)->Option<char>{
        let ch = self.source.next();
        match ch{
            Some(c) => {
                self.current_string.push(c);
                if c == '\n' {
                    self.position.col = 0;
                    self.position.row = 0;
                }else{
                    self.position.col += 1;
                }
            }
            None => return None
        }
        ch
    }

    pub fn get_next(&mut self) -> Option<&char>{
        self.source.peek()
    }

    pub fn next_is_expected(&mut self,expected:char)->bool{
        match self.get_next(){
            Some(&ch) => ch == expected,
            None => false
        }
    }

    pub fn next_is_expected_by(&mut self,func:&dyn Fn(char)->bool)->bool{
        match self.get_next(){
            Some(&ch) => func(ch),
            None => false
        }
    }


    pub fn advance_until(&mut self,expected:char){//一直推进 直到遇见expected（expected不会被推进）
        while !self.next_is_expected(expected){
            self.advance();
        }
    }

    pub fn advance_until_by(&mut self,func:&dyn Fn(char)->bool){
        while self.next_is_expected_by(func){
            self.advance();
        }
    }

    pub fn get_number(&mut self)->Result<f64,impl Error>{
        self.advance_until_by(&|ch|is_digital(ch));
        let digital = self.current_string.clone();
        digital.parse::<f64>()
    }

    pub fn get_string(&mut self)->String{
        self.advance_until('"');
        let s = self.current_string.clone();
        self.advance();
        s
    }

    pub fn get_identifier(&mut self)->String{
        self.advance_until_by(&|ch|is_alphadigital(ch));
        let s = self.current_string.clone();
        s
    }

    pub fn get_space(&mut self)->String{
        self.advance_until_by(&|ch|is_space(ch));
        let space = self.current_string.clone();
        space
    }

    pub fn scan(&mut self)->Result<Token,Option<Token>>{
        let ch = self.advance();
        let token = match ch {
            Some('.') => { self.clear();Token::Dot },
            Some(',') => { self.clear();Token::Comma },
            Some(';') => { self.clear();Token::Semicolon },
            Some('+') => { self.clear();Token::Plus },
            Some('-') => { self.clear();Token::Minus },
            Some('*') => { self.clear();Token::Start },
            Some('/') => { self.clear();Token::Div },
            Some('(') => { self.clear();Token::LeftParent },
            Some(')') => { self.clear();Token::RightParent },
            Some('{') => { self.clear();Token::LeftBrace },
            Some('}') => { self.clear();Token::RightBrace },
            Some(' ') => {
                Token::Space(self.get_space())
            },
            Some('=') => {
                if let Some(&'=') = self.get_next(){
                    self.advance();
                    Token::DoubleEq
                }else{
                    Token::Eq
                }
            },
            Some('!') => {
                if let Some(&'=') = self.get_next(){
                    self.advance();
                    Token::NotEq
                }else{
                    Token::Exclamation
                }
            },
            Some('>') => {
                if let Some(&'=') = self.get_next(){
                    self.advance();
                    Token::GreaterEq
                }else{
                    Token::Greater
                }
            },
            Some('<') => {
                if let Some(&'=') = self.get_next(){
                    self.advance();
                    Token::LessEq
                }else{
                    Token::Less
                }
            },
            Some('"') => {
                let s = self.get_string();
                Token::String(s)
            }
            Some(c) if is_digital(c) => {
                let dig = self.get_number().unwrap();
                Token::Digital(dig)
            },

            Some(c) if is_alpha(c) => {
                let identifer = self.get_identifier();
                let token = IDENTIFER_MAP.get(&*identifer);
                match token{
                    Some(t) => t.clone(),
                    _ => Token::Identifier(identifer),
                }
            }

            _ => {
                self.clear();
                return Err(None);
            }
        };
        self.clear();
        Ok(token)
    }
}

impl<'a> Iterator for Scanner<'a>{
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.scan().ok()
    }
}



