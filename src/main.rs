use std::{fs, time::Instant};

use grammer::Terminal;

mod lexer;
mod parser;
mod grammer;

fn main() {
    let start = Instant::now();
    
    let file = fs::read_to_string("./test.spl").unwrap();
    let mut lexer = lexer::Lexer::new(&file);
    let tokens = lexer.tokenize();
    // println!("{:?}", tokens);

    let parser = parser::Parser::new(tokens);

    println!("{:?}", start.elapsed());
}
