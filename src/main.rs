#[macro_use] extern crate lazy_static;
extern crate regex;

mod lexer;
mod grammar;

fn main() {
    let script = "if (balance > 0) { PAY address 0.6 ZNT }".to_string();
    let script_to_lex = lexer::insert_breaks(&script);
    let result = lexer::lex(&script_to_lex);

    println!("{:?}", result);
}
