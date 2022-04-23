use std::{process::exit, ops::{Mul, Add}};

use crate::token::Pos;

// types of tokens
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

#[allow(dead_code)]#[allow(dead_code)]
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

// all nonterminals of the grammer
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
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

impl Mul<usize> for NonTerminal {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        (self as usize)*rhs
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
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