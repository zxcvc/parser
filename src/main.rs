mod error;
mod parser;
mod scanner;
mod utils;

use parser::Parser;
fn main() {
    let mut parser = Parser::new(r#"1-21212/2k"#);
    let exp = parser.expresson();
    dbg!(exp);
}
