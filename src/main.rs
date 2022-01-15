mod error;
mod parser;
mod scanner;
mod utils;

use parser::Parser;
fn main() {
    let mut parser = Parser::new(r#"let a = 1;a = 2;let c = 2;let bb = 222;let d = 2;d = 2;"#);
    let res = parser.programing();
    dbg!(res);
}
