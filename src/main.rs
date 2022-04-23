use std::{fs, time::Instant};

mod lexer;
mod parser;
mod grammer;
mod token;
mod stack;

fn main() {
    let start = Instant::now();
    
    let file = fs::read_to_string("./test.spl").unwrap();
    let mut lexer = lexer::Lexer::new(&file);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let parser = parser::Parser::new(tokens);
    parser.parse();

    println!("{:?}", start.elapsed());
}
