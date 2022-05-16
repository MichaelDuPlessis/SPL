use std::{fs, time::Instant, rc::Rc};

use crate::scope::ScopeAnalysis;

mod lexer;
mod parser;
mod grammer;
mod token;
mod stack;
mod scope;

fn main() {
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let input = &input[..input.len() - 1];

    let start = Instant::now();

    let file = match fs::read_to_string("./test.spl") {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let mut lexer = lexer::Lexer::new(&file);
    let tokens = lexer.tokenize();
    // println!("{:?}", tokens);

    let parser = parser::Parser::new(tokens);
    let node = parser.parse();
    parser::Parser::create_xml(Rc::clone(&node));

    let mut scope = ScopeAnalysis::new(node);
    println!("{:?}", scope.scope());

    println!("{:?}", start.elapsed());
}
