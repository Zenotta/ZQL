extern crate regex;

mod lexer;
mod grammar;
mod syntax;
mod kernel;
mod utils;
mod compiler;

use crate::syntax::Parser;
use crate::compiler::Compiler;

fn main() {
    let script = 
    "
    set other_value = 3;
    set coin = 10 / other_value;
    set address1 = 'A0f932582395';
    set address2 = 'B13509813fg1';

    stack [ PAY address1 coin ZNT ]
    "
    .to_string();

    // Parse the script
    let parser = Parser;
    let parse_result = match parser.parse_script(&script) {
        Ok(v) => v,
        Err(e) => panic!("ERROR PARSING: {}", e)
    };

    println!("");
    println!("{:?}", parse_result);
    println!("");

    // Execute the script
    let mut compiler_instance = Compiler::new();
    compiler_instance.parse_heap_expression(&parse_result);

    println!("ASSIGNMENTS: {:?}", compiler_instance.0.assignments);
}
