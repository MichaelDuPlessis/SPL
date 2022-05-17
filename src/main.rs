use std::{fs, rc::Rc};

mod lexer;
mod parser;
mod grammer;
mod token;
mod stack;

fn main() {
    let mut input = String::new();

    println!("Enter a file path:");
    std::io::stdin().read_line(&mut input).unwrap();
    let input = &input[..input.len() - 1];

    let file = match fs::read_to_string(input) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let mut lexer = lexer::Lexer::new(&file);
    let tokens = lexer.tokenize();

    let parser = parser::Parser::new(tokens);
    let node = parser.parse();
    parser::Parser::create_xml(Rc::clone(&node));
}
