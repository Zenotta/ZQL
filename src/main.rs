#[macro_use] extern crate lazy_static;
extern crate regex;

mod lexer;
mod grammar;

use crate::lexer::Lexer;

fn main() {
    let script = "if (balance > 0) { PAY address 0.6 ZNT }".to_string();
    let mut lex_inst = Lexer::new();
    lex_inst.lex(&script);

    println!("{:?}", lex_inst.tokens);
}
