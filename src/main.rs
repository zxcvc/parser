mod error;
mod parser;
mod scanner;
mod utils;

use parser::Parser;
fn main() {
    let mut parser = Parser::new(
        r#"let a = 2;;;;;;;a=2;let d = 2;while(1==2){let a = 2;2 <= 5;1;}
        let a = 2;
        a = 2;
        a = 2 == 5;
        let b = 3 >= (3-2);
    "#,
    );
    let res = parser.programing();
    dbg!(res);
}
