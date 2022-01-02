mod scanner;

use scanner::Scanner;


use scanner::Token;
fn main() {
    let mut s = Scanner::new(r#"function fn(a){console.log(2);let a = 1; let b = a;}"#);
    for item in s.filter(|token|match token{&Token::Space(_)=>false, _ => true}){
        println!("{:?}",item);
    }
}
