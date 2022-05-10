use std::{fs, time::Instant};

mod lexer;
mod parser;
mod grammer;
mod token;
mod stack;

fn main() {
    let start = Instant::now();

    // println!("{}", std::env::args().next().unwrap());
    
    let file = match fs::read_to_string("./test.spl") {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let mut lexer = lexer::Lexer::new(&file);
    let tokens = lexer.tokenize();
    // println!("{:?}", tokens);

    let parser = parser::Parser::new(tokens);
    let node = parser.parse();
    parser::Parser::create_xml(node);

    println!("{:?}", start.elapsed());
}
