use crate::{lexer::Token, grammer::Grammer};

#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
    table: [Option<Rhs>; 858],
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        const ROWS: usize = 22;
        const COLS: usize = 39;
        let mut table = [(); ROWS*COLS].map(|_| Option::<Rhs>::default());

        table[8 + 1*ROWS] = Some(Rhs::new(vec![Grammer::ProcDefs, Grammer::Main, Grammer::LBrace, Grammer::Algorithm, Grammer::Halt, Grammer::Semicolon, Grammer::VarDecl, Grammer::RBrace]));
        table[10 + 1*ROWS] = Some(Rhs::new(vec![Grammer::ProcDefs, Grammer::Main, Grammer::LBrace, Grammer::Algorithm, Grammer::Halt, Grammer::Semicolon, Grammer::VarDecl, Grammer::RBrace]));
        
        table[10 + 2*ROWS] = Some(Rhs::new(vec![Grammer::PD, Grammer::Comma, Grammer::ProcDefs]));
        
        table[10 + 3*ROWS] = Some(Rhs::new(vec![Grammer::Proc, Grammer::UserDefined, Grammer::LBrace, Grammer::ProcDefs, Grammer::Algorithm, Grammer::Return, Grammer::Semicolon, Grammer::VarDecl, Grammer::RBrace]));
        
        table[13 + 4*ROWS] = Some(Rhs::new(vec![Grammer::Instr, Grammer::Semicolon, Grammer::Algorithm]));
        table[16 + 4*ROWS] = Some(Rhs::new(vec![Grammer::Instr, Grammer::Semicolon, Grammer::Algorithm]));
        table[18 + 4*ROWS] = Some(Rhs::new(vec![Grammer::Instr, Grammer::Semicolon, Grammer::Algorithm]));
        table[19 + 4*ROWS] = Some(Rhs::new(vec![Grammer::Instr, Grammer::Semicolon, Grammer::Algorithm]));
        table[20 + 4*ROWS] = Some(Rhs::new(vec![Grammer::Instr, Grammer::Semicolon, Grammer::Algorithm]));
        table[37 + 4*ROWS] = Some(Rhs::new(vec![Grammer::Instr, Grammer::Semicolon, Grammer::Algorithm]));
        
        table[13 + 5*ROWS] = Some(Rhs::new(vec![Grammer::Branch]));
        table[16 + 5*ROWS] = Some(Rhs::new(vec![Grammer::Loop]));
        table[18 + 5*ROWS] = Some(Rhs::new(vec![Grammer::Loop]));
        table[19 + 5*ROWS] = Some(Rhs::new(vec![Grammer::Assign]));
        table[20 + 5*ROWS] = Some(Rhs::new(vec![Grammer::PCall]));
        table[37 + 5*ROWS] = Some(Rhs::new(vec![Grammer::Assign]));
        
        table[37 + 6*ROWS] = Some(Rhs::new(vec![Grammer::LHS, Grammer::Assignment, Grammer::Expr]));
        
        table[13 + 7*ROWS] = Some(Rhs::new(vec![Grammer::If, Grammer::LParentheses, Grammer::Expr, Grammer::RParentheses, Grammer::Then, Grammer::LBrace, Grammer::Algorithm, Grammer::LBrace, Grammer::Alternat]));

        table[14 + 8*ROWS] = Some(Rhs::new(vec![Grammer::Else, Grammer::LBrace, Grammer::Algorithm, Grammer::RBrace]));
        
        table[16 + 9*ROWS] = Some(Rhs::new(vec![Grammer::Do, Grammer::LBrace, Grammer::Algorithm, Grammer::RBrace, Grammer::Until, Grammer::LParentheses, Grammer::Expr, Grammer::RParentheses]));
        
        table[37 + 10*ROWS] = Some(Rhs::new(vec![Grammer::UserDefined, Grammer::VarField]));

        table[21 + 11*ROWS] = Some(Rhs::new(vec![Grammer::Const]));
        table[22 + 11*ROWS] = Some(Rhs::new(vec![Grammer::Const]));
        table[23 + 11*ROWS] = Some(Rhs::new(vec![Grammer::UnOp]));
        table[24 + 11*ROWS] = Some(Rhs::new(vec![Grammer::UnOp]));
        table[25 + 11*ROWS] = Some(Rhs::new(vec![Grammer::BinOp]));
        table[26 + 11*ROWS] = Some(Rhs::new(vec![Grammer::BinOp]));
        table[27 + 11*ROWS] = Some(Rhs::new(vec![Grammer::BinOp]));
        table[28 + 11*ROWS] = Some(Rhs::new(vec![Grammer::BinOp]));
        table[29 + 11*ROWS] = Some(Rhs::new(vec![Grammer::BinOp]));
        table[30 + 11*ROWS] = Some(Rhs::new(vec![Grammer::BinOp]));
        table[31 + 11*ROWS] = Some(Rhs::new(vec![Grammer::BinOp]));
        table[36 + 11*ROWS] = Some(Rhs::new(vec![Grammer::Const]));
        table[37 + 11*ROWS] = Some(Rhs::new(vec![Grammer::UserDefined, Grammer::VarField]));
        table[38 + 11*ROWS] = Some(Rhs::new(vec![Grammer::Const]));
        
        table[4 + 12*ROWS] = Some(Rhs::new(vec![Grammer::LBracket, Grammer::FType]));
        
        table[20 + 13*ROWS] = Some(Rhs::new(vec![Grammer::Call, Grammer::UserDefined]));
        
        table[37 + 14*ROWS] = Some(Rhs::new(vec![Grammer::UserDefined]));
        
        table[21 + 15*ROWS] = Some(Rhs::new(vec![Grammer::Const, Grammer::RBracket]));
        table[22 + 15*ROWS] = Some(Rhs::new(vec![Grammer::Const, Grammer::RBracket]));
        table[36 + 15*ROWS] = Some(Rhs::new(vec![Grammer::Const, Grammer::RBracket]));
        table[37 + 15*ROWS] = Some(Rhs::new(vec![Grammer::Var, Grammer::RBracket]));
        table[38 + 15*ROWS] = Some(Rhs::new(vec![Grammer::Const, Grammer::RBracket]));
        
        table[21 + 16*ROWS] = Some(Rhs::new(vec![Grammer::True]));
        table[22 + 16*ROWS] = Some(Rhs::new(vec![Grammer::False]));
        table[36 + 16*ROWS] = Some(Rhs::new(vec![Grammer::Number]));
        table[38 + 16*ROWS] = Some(Rhs::new(vec![Grammer::ShortString]));
        
        table[23 + 17*ROWS] = Some(Rhs::new(vec![Grammer::Input, Grammer::LParentheses, Grammer::Var, Grammer::RParentheses]));
        table[24 + 17*ROWS] = Some(Rhs::new(vec![Grammer::Not, Grammer::LParentheses, Grammer::Expr, Grammer::RParentheses]));
        
        table[25 + 18*ROWS] = Some(Rhs::new(vec![Grammer::And, Grammer::LParentheses, Grammer::Expr, Grammer::Comma, Grammer::Expr, Grammer::RParentheses]));
        table[26 + 18*ROWS] = Some(Rhs::new(vec![Grammer::Or, Grammer::LParentheses, Grammer::Expr, Grammer::Comma, Grammer::Expr, Grammer::RParentheses]));
        table[27 + 18*ROWS] = Some(Rhs::new(vec![Grammer::Equal, Grammer::LParentheses, Grammer::Expr, Grammer::Comma, Grammer::Expr, Grammer::RParentheses]));
        table[28 + 18*ROWS] = Some(Rhs::new(vec![Grammer::Larger, Grammer::LParentheses, Grammer::Expr, Grammer::Comma, Grammer::Expr, Grammer::RParentheses]));
        table[29 + 18*ROWS] = Some(Rhs::new(vec![Grammer::Add, Grammer::LParentheses, Grammer::Expr, Grammer::Comma, Grammer::Expr, Grammer::RParentheses]));
        table[30 + 18*ROWS] = Some(Rhs::new(vec![Grammer::Sub, Grammer::LParentheses, Grammer::Expr, Grammer::Comma, Grammer::Expr, Grammer::RParentheses]));
        table[31 + 18*ROWS] = Some(Rhs::new(vec![Grammer::Mult, Grammer::LParentheses, Grammer::Expr, Grammer::Comma, Grammer::Expr, Grammer::RParentheses]));
        
        table[32 + 19*ROWS] = Some(Rhs::new(vec![Grammer::Dec, Grammer::Semicolon, Grammer::VarDecl]));
        table[33 + 19*ROWS] = Some(Rhs::new(vec![Grammer::Dec, Grammer::Semicolon, Grammer::VarDecl]));
        table[34 + 19*ROWS] = Some(Rhs::new(vec![Grammer::Dec, Grammer::Semicolon, Grammer::VarDecl]));
        table[35 + 19*ROWS] = Some(Rhs::new(vec![Grammer::Dec, Grammer::Semicolon, Grammer::VarDecl]));
        
        table[32 + 20*ROWS] = Some(Rhs::new(vec![Grammer::Array, Grammer::TYP, Grammer::LBracket, Grammer::Const, Grammer::RBracket, Grammer::Var]));
        table[33 + 20*ROWS] = Some(Rhs::new(vec![Grammer::TYP, Grammer::Var]));
        table[34 + 20*ROWS] = Some(Rhs::new(vec![Grammer::TYP, Grammer::Var]));
        table[35 + 20*ROWS] = Some(Rhs::new(vec![Grammer::TYP, Grammer::Var]));
        
        table[33 + 21*ROWS] = Some(Rhs::new(vec![Grammer::Num]));
        table[34 + 21*ROWS] = Some(Rhs::new(vec![Grammer::Boolean]));
        table[35 + 21*ROWS] = Some(Rhs::new(vec![Grammer::String]));
        
        Self {
            tokens,
            table,
        }
    }

    pub fn parse() {
        let mut stack = vec![Grammer::SPLProgrPrime];
        let mut input = 0usize;

        while !stack.is_empty() {
            
        }
    }
}

#[allow(dead_code)]
struct Rhs {
    rhs: Vec<Grammer>,
}

#[allow(dead_code)]
impl Rhs {
    fn new(rhs: Vec<Grammer>) -> Self {
        Self {
            rhs,
        }
    }
}