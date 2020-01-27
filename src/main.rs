extern crate regex;

mod lexer;
mod grammar;
mod syntax;

use crate::syntax::Parser;

fn main() {
    let script = 
    "
    set coin = 0;
    set address1 = 'A0f932582395';
    set address2 = 'B13509813fg1';

    if coin == 0 {
        stack [ PAY address1 1 ZNT ]
    } else {
        stack [ PAY address2 1 ZNT ]
    }"
    .to_string();

    let parser = Parser;
    let parse_result = match parser.parse_script(&script) {
        Ok(v) => v,
        Err(e) => panic!("ERROR PARSING: {}", e)
    };

    println!("");
    println!("{:?}", parse_result);
    println!("");
}
