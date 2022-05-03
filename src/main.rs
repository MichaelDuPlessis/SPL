use std::{fs, time::Instant};

mod lexer;
mod parser;
mod grammer;
mod token;
mod stack;

fn main() {
    let start = Instant::now();
    
    let file = fs::read_to_string("./input.spl").unwrap();
    let mut lexer = lexer::Lexer::new(&file);
    let tokens = lexer.tokenize();
    // println!("{:?}", tokens);

    let parser = parser::Parser::new(tokens);
    let node = parser.parse();
    parser::Parser::create_xml(node);

    println!("{:?}", start.elapsed());
}
