mod error;
mod parser;
mod scanner;
mod utils;

use std::{rc::Rc, cell::RefCell};

use parser::Parser;
fn main() {
    let mut parser = Parser::new(
        r#"for(; ; ){let a =2;for(let a = 0;;){;;;;}};;;;;;return 1;return; return 2+2;function ff(){let a = 2;}"#,
    );
    let res = parser.programing();
    dbg!(res);
}
