#[macro_use] extern crate lazy_static;
extern crate regex;

mod lexer;
mod grammar;
mod syntax;

use crate::syntax::Parser;

fn main() {
    //let script = "if (balance > 0) { PAY address 0.6 ZNT }".to_string();
    let script = "if balance > 0 { stack [ PAY address 6 ZNT ] };".to_string();
    let parser = Parser;
    let parse_result = match parser.parse_script(&script) {
        Ok(v) => v,
        Err(e) => panic!("ERROR PARSING: {}", e)
    };

    println!("");
    println!("{:?}", parse_result);
    println!("");
}
