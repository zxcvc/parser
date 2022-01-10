mod parser;
mod scanner;
mod utils;

use parser::Parser;
fn main() {
    let mut parser = Parser::new(r#"( 1 - 2 )"#);
    let exp = parser.expresson();
    dbg!(exp);
}
