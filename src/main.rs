mod error;
mod parser;
mod scanner;
mod utils;

use std::{cell::RefCell, rc::Rc};

use parser::Parser;

fn main() {
    let mut parser = Parser::new(
        r#"fn(1,2);
        function ff(){}
        let a = 2;
        "#,
    );
    let res = parser.programing();
    dbg!(res);
}
