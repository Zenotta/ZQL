#[macro_use] extern crate lazy_static;
extern crate regex;

mod lexer;
mod grammar;
mod syntax;

use crate::syntax::Parser;

fn main() {
    let script = 
    "
    stack [ funds = 0 ZNT ]
    stack [ goal = 10000 ZNT ]
    set receivers = [];
    
    stack [ encoding_scheme = NEW SDL ]

    while funds < goal {
        receivers.push(receiver.address)
        funds += receiver.funding_amount
    };

    stack [
        CREATE Death_Stranding.ps4 WHERE
        ENCODING encoding_scheme AND
        AMOUNT receivers.length
    ]

    stack [ 
        TRANSACT Death_Stranding.ps4 WHERE
        WHO receiver IN receivers
    ]"
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
