use std::{fs, time::Instant, rc::Rc};
use crate::{scope::{ScopeAnalysis, Scope}, type_checking::TypeChecker, generator::Generator};

mod lexer;
mod parser;
mod grammer;
mod token;
mod stack;
mod scope;
mod type_checking;
mod error;
mod generator;

fn main() {
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

    let mut scope_analysis = ScopeAnalysis::new(Rc::clone(&node));
    let scope = scope_analysis.scope();

    
    let mut type_checker = TypeChecker::new(Rc::clone(&scope), Rc::clone(&node));
    let typ = type_checker.type_check();

    ScopeAnalysis::create_table(typ);

    let mut generator = Generator::new(Rc::clone(&node));
    generator.generate();

    println!("{:?}", start.elapsed());
}
