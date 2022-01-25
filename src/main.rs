mod error;
mod parser;
mod scanner;
mod utils;

use parser::Parser;
fn main() {
    let mut parser = Parser::new(r#"for(; ; ){let a =2;for(let a = 0;;){}}"#);
    let res = parser.programing();
    dbg!(res);
}
