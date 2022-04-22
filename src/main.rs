use std::{fs, time::Instant};

mod lexer;
mod parser;

fn main() {
    let start = Instant::now();
    
    let file = fs::read_to_string("./test.spl").unwrap();
    let mut lexer = lexer::Lexer::new(&file);
    println!("{:?}", lexer.tokenize());

    println!("{:?}", start.elapsed());
}
