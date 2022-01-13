mod error;
mod parser;
mod scanner;
mod utils;

use parser::Parser;
fn main() {
    let mut parser = Parser::new(r#"1;
    
    
                2;
                232;2;
                (2);
                1+2;
                1/2;
                1*2;
    
    
    -21212/2;"#);
    let res = parser.programing();
    dbg!(res);
}
