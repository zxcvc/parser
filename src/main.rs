mod error;
mod parser;
mod scanner;
mod utils;

use std::{cell::RefCell, rc::Rc};

use parser::Parser;
fn main() {
    let mut parser = Parser::new(
        r#"for(; ; ){let a =2;for(let a = 0;;){;;;;}};;;;;;return 1;return; return 2+2;function ff(){let a = 2;}
function fn(afd,bfs,xf    ,pd){
function a(){}
}
        "#,
    );
    let res = parser.programing();
    dbg!(res);
}
