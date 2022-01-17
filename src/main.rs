mod error;
mod parser;
mod scanner;
mod utils;

use parser::Parser;
fn main() {
    let mut parser =
        Parser::new(r#"if(1){1;}else{2;}"#);
    let res = parser.programing();
    dbg!(res);
}
