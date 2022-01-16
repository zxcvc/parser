mod error;
mod parser;
mod scanner;
mod utils;

use parser::Parser;
fn main() {
    let mut parser =
        Parser::new(r#"if(2/1+1){1+3;if(1){1;}} 1+2;-1/2;let a = 2;a = 2;if (2){let c = 5;}"#);
    let res = parser.programing();
    dbg!(res);
}
