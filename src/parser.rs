use std::ops::Mul;
use crate::lexer::{Token, Nonterminal};

#[allow(dead_code)]

// all nonterminals of the grammer
#[derive(Clone, Copy)]
enum Terminal {
    SPLProgrPrime,
    SPLProgr,
    ProcDefs,
    PD,
    Algorithm,
    Instr,
    Assign,
    Branch,
    Alternat,
    Loop,
    LHS,
    Expr,
    VarField,
    PCall,
    Var,
    Field,
    FType,
    Const,
    UnOp,
    BinOp,
    VarDecl,
    Dec,
    TYP,
}

impl Terminal {
    fn grammer(self) -> Grammer {
        Grammer::Terminal(self)
    }
}

impl Mul<usize> for Terminal {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        self as usize * rhs
    }
}

#[allow(dead_code)]
// Since a grammer symbol may be either one
#[derive(Clone)]
pub enum Grammer {
    Terminal(Terminal),
    Nonterminal(Nonterminal),
}

#[allow(dead_code)]
struct Parser {
    tokens: Vec<Token>,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        const ROWS: usize = 22;
        const COLS: usize = 39;
        let mut table = [(); ROWS*COLS].map(|_| Option::<Rhs>::default());

        table[Terminal::SPLProgrPrime*ROWS + Nonterminal::Main*COLS] = Some(Rhs::new(vec![Terminal::ProcDefs, Nonterminal::main, Nonterminal::LBrace, Terminal::Algorithm, Nonterminal::Halt, Nonterminal::Semicolon, Terminal::VarDecl, Nonterminal::RBrace]));

        Self {
            tokens,
        }
    }

    pub fn parse() {

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