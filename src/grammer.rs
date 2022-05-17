use std::{process::exit, ops::{Mul, Add}, fmt::Display};
use crate::token::{Pos};

// types of tokens

// terminal
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Terminal {
	LParentheses, // (
	RParentheses, // )
	LBrace, // {
	RBrace, // }
	LBracket, // [
	RBracket, // ]
	Semicolon, // ;
    Comma, // ,
    Main, // main
    Halt, // halt
    Proc, // proc
    Return, // return
    Assignment, // :=
    If, // if
    Else, // else
    Then, // then
    Do, // do
    Until, // until
    While, // while
    Out, // output
    Call, // call
    True, // true
    False, // false
    // --------------------| unary ops
    Input, // input
    Not, // not
    // --------------------| Binary ops
    And, // and
    Or, // or
    Equal, // eq
    Larger, // larger
    Add, // add
    Sub, // sub
    Mult, // mult
    // --------------------|
    Array, // arr
    Num, // num --------|
    Boolean, // bool-------| --- types
    String, // string --|
    Number, // numbers
    UserDefined, // userDefinedNames
    ShortString, // ShortStrings
    
    Dollar, // $
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminal::LParentheses => write!(f, "LParentheses"),
            Terminal::RParentheses => write!(f, "RParentheses"),
            Terminal::LBrace => write!(f, "LBrace"),
            Terminal::RBrace => write!(f, "RBrace"),
            Terminal::LBracket => write!(f, "LBracket"),
            Terminal::RBracket => write!(f, "RBracket"),
            Terminal::Semicolon => write!(f, "Semicolon"),
            Terminal::Comma => write!(f, "Comma"),
            Terminal::Main => write!(f, "main"),
            Terminal::Halt => write!(f, "halt"),
            Terminal::Proc => write!(f, "proc"),
            Terminal::Return => write!(f, "return"),
            Terminal::Assignment => write!(f, "Assignment"),
            Terminal::If => write!(f, "if"),
            Terminal::Else => write!(f, "else"),
            Terminal::Then => write!(f, "then"),
            Terminal::Do => write!(f, "do"),
            Terminal::Until => write!(f, "until"),
            Terminal::While => write!(f, "while"),
            Terminal::Out => write!(f, "output"),
            Terminal::Call => write!(f, "call"),
            Terminal::True => write!(f, "true"),
            Terminal::False => write!(f, "false"),
            Terminal::Input => write!(f, "input"),
            Terminal::Not => write!(f, "not"),
            Terminal::And => write!(f, "and"),
            Terminal::Or => write!(f, "or"),
            Terminal::Equal => write!(f, "equal"),
            Terminal::Larger => write!(f, "larger"),
            Terminal::Add => write!(f, "add"),
            Terminal::Sub => write!(f, "sub"),
            Terminal::Mult => write!(f, "mult"),
            Terminal::Array => write!(f, "arr"),
            Terminal::Num => write!(f, "num"),
            Terminal::Boolean => write!(f, "bool"),
            Terminal::String => write!(f, "string"),
            Terminal::Number => write!(f, "num"),
            Terminal::UserDefined => write!(f, "userDefinedName"),
            Terminal::ShortString => write!(f, "ShortString"),
            Terminal::Dollar => write!(f, "$"),
        }
    }
}

#[allow(dead_code)]
impl Terminal {
    pub fn basic_token(token: char, currnet_pos: Pos) -> Self {
        match token {
            '(' => Self::LParentheses,
            ')' => Self::RParentheses,
            '[' => Self::LBracket,
            ']' => Self::RBracket,
            '{' => Self::LBrace,
            '}' => Self::RBrace,
            ',' => Self::Comma,
            ';' => Self::Semicolon,
            _ => {
                println!("Internal error: non-basic token at {}", currnet_pos);
                exit(1)
            },
        }
    }
}

impl Mul<usize> for Terminal {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        (self as usize)*rhs
    }
}

impl Add<usize> for Terminal {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        (self as usize)+rhs
    }
}

// nonterminal
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NonTerminal {
    // SPLProgrPrime,
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
    FType,
    Const,
    UnOp,
    BinOp,
    VarDecl,
    Dec,
    TYP,
    Field,
}

impl Display for NonTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonTerminal::SPLProgr => write!(f, "SPLProgr"),
            NonTerminal::ProcDefs => write!(f, "ProcDefs"),
            NonTerminal::PD => write!(f, "PD"),
            NonTerminal::Algorithm => write!(f, "Algorithm"),
            NonTerminal::Instr => write!(f, "Instr"),
            NonTerminal::Assign => write!(f, "Assign"),
            NonTerminal::Branch => write!(f, "Branch"),
            NonTerminal::Alternat => write!(f, "Alternat"),
            NonTerminal::Loop => write!(f, "Loop"),
            NonTerminal::LHS => write!(f, "LHS"),
            NonTerminal::Expr => write!(f, "Expr"),
            NonTerminal::VarField => write!(f, "VarField"),
            NonTerminal::PCall => write!(f, "PCall"),
            NonTerminal::Var => write!(f, "Var"),
            NonTerminal::FType => write!(f, "FType"),
            NonTerminal::Const => write!(f, "Const"),
            NonTerminal::UnOp => write!(f, "UnOp"),
            NonTerminal::BinOp => write!(f, "BinOp"),
            NonTerminal::VarDecl => write!(f, "VarDecl"),
            NonTerminal::Dec => write!(f, "Dec"),
            NonTerminal::TYP => write!(f, "TYP"),
            NonTerminal::Field => write!(f, "Field"),
        }
    }
}

impl Mul<usize> for NonTerminal {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        (self as usize)*rhs
    }
}

// grammer
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Grammer {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

impl From<Terminal> for Grammer {
    fn from(t: Terminal) -> Self {
        Grammer::Terminal(t)
    }
}

impl From<NonTerminal> for Grammer {
    fn from(t: NonTerminal) -> Self {
        Grammer::NonTerminal(t)
    }
}

impl Display for Grammer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grammer::Terminal(t) => write!(f, "{}", t),
            Grammer::NonTerminal(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Number(Number),
    Boolean(Boolean),
    String,
    Unknown,
    Mixed,
}

#[derive(Debug, Clone, Copy)]
pub enum Number {
    N,
    NN,
}

#[derive(Debug, Clone, Copy)]
pub enum Boolean {
    True,
    False,
}