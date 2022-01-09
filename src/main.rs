// mod parser;
mod scanner;
mod utils;

// use parser::Parser;
fn main() {
    // let mut parser = Parser::new(r#"(1-(-12/-2))-2+4<2+28*23/11-2+666-(12/22-(232+3))"#);
    // let exp = parser.expresson();
    // dbg!(exp);
    let mut s = scanner::Scanner::new("let 
let 
    let 
    let 1").collect::<Vec<scanner::Token>>();
    dbg!(s);

}
