use self::error::ScanError;

use super::utils::{is_alpha, is_alphadigital, is_digital, is_space};
use lazy_static::lazy_static;
use std::{collections::HashMap, error::Error, iter::Peekable, ops::Deref, str::Chars};

pub mod error {
    use super::Position;

    #[derive(Debug, Clone)]
    pub struct ScanError {
        pub code: i32,
        pub describe: String,
        pub position: Position,
    }

    impl From<Position> for ScanError {
        fn from(position: Position) -> Self {
            Self {
                code: 300,
                describe: "unexpected char".to_string(),
                position,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenRow {
    Dot,
    Comma,
    Semicolon, //分号
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
    For,
    While,
    Continue,
    Break,
    True,
    False,
    Null,
    This,
}

impl TokenRow {
    pub fn to_string(&self) -> String {
        match self {
            TokenRow::Dot => ".".to_string(),
            TokenRow::Comma => ",".to_string(),
            TokenRow::Semicolon => ";".to_string(),
            TokenRow::Plus => "+".to_string(),
            TokenRow::Minus => "-".to_string(),
            TokenRow::Start => "*".to_string(),
            TokenRow::Div => "/".to_string(),
            TokenRow::Eq => "=".to_string(),
            TokenRow::DoubleEq => "==".to_string(),
            TokenRow::Exclamation => "!".to_string(),
            TokenRow::NotEq => "!=".to_string(),
            TokenRow::Greater => ">".to_string(),
            TokenRow::Less => "<".to_string(),
            TokenRow::GreaterEq => ">=".to_string(),
            TokenRow::LessEq => "<=".to_string(),
            TokenRow::LeftParent => "(".to_string(),
            TokenRow::RightParent => ")".to_string(),
            TokenRow::LeftBrace => "{".to_string(),
            TokenRow::RightBrace => "}".to_string(),
            TokenRow::Digital(digital) => digital.to_string(),
            TokenRow::String(string) => string.to_string(),
            TokenRow::Space(space) => space.to_string(),
            TokenRow::Identifier(s) => s.to_string(),
            TokenRow::Let => "let".to_string(),
            TokenRow::Function => "function".to_string(),
            TokenRow::Return => "return".to_string(),
            TokenRow::If => "if".to_string(),
            TokenRow::Else => "else".to_string(),
            TokenRow::For => "for".to_string(),
            TokenRow::While => "while".to_string(),
            TokenRow::Continue => "continue".to_string(),
            TokenRow::Break => "break".to_string(),
            TokenRow::True => "true".to_string(),
            TokenRow::False => "false".to_string(),
            TokenRow::Null => "null".to_string(),
            TokenRow::This => "this".to_string(),
        }
    }
}

impl Deref for Token {
    type Target = TokenRow;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

lazy_static! {
    static ref IDENTIFER_MAP: HashMap<&'static str, TokenRow> = {
        let mut map = HashMap::new();
        map.insert("let", TokenRow::Let);
        map.insert("function", TokenRow::Function);
        map.insert("return", TokenRow::Return);
        map.insert("if", TokenRow::If);
        map.insert("else", TokenRow::Else);
        map.insert("for", TokenRow::For);
        map.insert("while", TokenRow::While);
        map.insert("continue", TokenRow::Continue);
        map.insert("break", TokenRow::Break);
        map.insert("true", TokenRow::True);
        map.insert("false", TokenRow::False);
        map.insert("null", TokenRow::Null);
        map.insert("this", TokenRow::This);
        map
    };
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenRow,
    pub position: Position,
}
impl Token {
    pub fn new(token: TokenRow, position: Position) -> Self {
        Self { token, position }
    }
}

#[derive(Debug)]
pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    current_string: String,
    position: Position,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            current_string: String::new(),
            position: Position::default(),
        }
    }

    pub fn get_position(&self) -> Position {
        // let col = self.position.col - self.current_string.len();
        // Position::new(self.position.row, col)
        self.position.clone()
    }

    pub fn clear(&mut self) {
        self.current_string.clear();
    }

    pub fn advance(&mut self) -> Option<char> {
        let ch = self.source.next();
        match ch {
            Some(c) => {
                self.current_string.push(c);
                if c == '\n' {
                    self.position.col = 0;
                    self.position.row += 1;
                } else {
                    self.position.col += 1;
                }
            }
            None => return None,
        }
        ch
    }

    pub fn get_next(&mut self) -> Option<&char> {
        self.source.peek()
    }

    pub fn next_is_expected(&mut self, expected: char) -> bool {
        match self.get_next() {
            Some(&ch) => ch == expected,
            None => false,
        }
    }

    pub fn next_is_expected_by(&mut self, func: &dyn Fn(char) -> bool) -> bool {
        match self.get_next() {
            Some(&ch) => func(ch),
            None => false,
        }
    }

    pub fn advance_until(&mut self, expected: char) {
        //一直推进 直到遇见expected（expected不会被推进）
        while !self.next_is_expected(expected) {
            self.advance();
        }
    }

    pub fn advance_until_by(&mut self, func: &dyn Fn(char) -> bool) {
        while self.next_is_expected_by(func) {
            self.advance();
        }
    }

    pub fn get_number(&mut self) -> Result<f64, impl Error> {
        self.advance_until_by(&is_digital);
        let digital = self.current_string.clone();
        digital.parse::<f64>()
    }

    pub fn get_string(&mut self) -> String {
        self.advance_until('"');
        let s = self.current_string.clone();
        self.advance();
        s
    }

    pub fn get_identifier(&mut self) -> String {
        self.advance_until_by(&is_alphadigital);

        self.current_string.clone()
    }

    pub fn get_space(&mut self) -> String {
        self.advance_until_by(&is_space);

        self.current_string.clone()
    }

    pub fn scan(&mut self) -> Option<Result<Token, ScanError>> {
        let position = self.get_position();
        let ch = self.advance();
        let token_row = match ch {
            Some('.') => TokenRow::Dot,
            Some(',') => TokenRow::Comma,
            Some(';') => TokenRow::Semicolon,
            Some('+') => TokenRow::Plus,
            Some('-') => TokenRow::Minus,
            Some('*') => TokenRow::Start,
            Some('/') => TokenRow::Div,
            Some('(') => TokenRow::LeftParent,
            Some(')') => TokenRow::RightParent,
            Some('{') => TokenRow::LeftBrace,
            Some('}') => TokenRow::RightBrace,
            Some(' ') | Some('\n') | Some('\t') => TokenRow::Space(self.get_space()),
            Some('=') => {
                if let Some(&'=') = self.get_next() {
                    self.advance();
                    TokenRow::DoubleEq
                } else {
                    TokenRow::Eq
                }
            }
            Some('!') => {
                if let Some(&'=') = self.get_next() {
                    self.advance();
                    TokenRow::NotEq
                } else {
                    TokenRow::Exclamation
                }
            }
            Some('>') => {
                if let Some(&'=') = self.get_next() {
                    self.advance();
                    TokenRow::GreaterEq
                } else {
                    TokenRow::Greater
                }
            }
            Some('<') => {
                if let Some(&'=') = self.get_next() {
                    self.advance();
                    TokenRow::LessEq
                } else {
                    TokenRow::Less
                }
            }
            Some('"') => {
                let s = self.get_string();
                TokenRow::String(s)
            }
            Some(c) if is_digital(c) => {
                let dig = self.get_number().unwrap();
                TokenRow::Digital(dig)
            }

            Some(c) if is_alpha(c) => {
                let identifer = self.get_identifier();
                let token_row = IDENTIFER_MAP.get(&*identifer);
                match token_row {
                    Some(t) => t.clone(),
                    _ => TokenRow::Identifier(identifer),
                }
            }

            _ => {
                self.clear();
                return None;
            }
        };
        self.clear();
        Some(Ok(Token::new(token_row, position)))
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, ScanError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scan()
    }
}
