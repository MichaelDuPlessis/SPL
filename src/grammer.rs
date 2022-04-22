use std::{process::exit, ops::{Mul, Add}};

use crate::lexer::Pos;

// types of tokens
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Nonterminal {
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
    Number(isize), // numbers
    UserDefined(String), // userDefinedNames
    ShortString(String), // ShortStrings
}

#[allow(dead_code)]#[allow(dead_code)]
impl Nonterminal {
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

impl Add<usize> for Nonterminal {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Nonterminal::LParentheses => rhs+0,
            Nonterminal::RParentheses => rhs+1,
            Nonterminal::LBrace => rhs+2,
            Nonterminal::RBrace => rhs+3,
            Nonterminal::LBracket => rhs+4,
            Nonterminal::RBracket => rhs+5,
            Nonterminal::Semicolon => rhs+6,
            Nonterminal::Comma => rhs+7,
            Nonterminal::Main => rhs+8,
            Nonterminal::Halt => rhs+9,
            Nonterminal::Proc => rhs+10,
            Nonterminal::Return => rhs+11,
            Nonterminal::Assignment => rhs+12,
            Nonterminal::If => rhs+13,
            Nonterminal::Else => rhs+14,
            Nonterminal::Then => rhs+15,
            Nonterminal::Do => rhs+16,
            Nonterminal::Until => rhs+17,
            Nonterminal::While => rhs+18,
            Nonterminal::Out => rhs+19,
            Nonterminal::Call => rhs+20,
            Nonterminal::True => rhs+21,
            Nonterminal::False => rhs+22,
            Nonterminal::Input => rhs+23,
            Nonterminal::Not => rhs+24,
            Nonterminal::And => rhs+25,
            Nonterminal::Or => rhs+26,
            Nonterminal::Equal => rhs+27,
            Nonterminal::Larger => rhs+28,
            Nonterminal::Add => rhs+29,
            Nonterminal::Sub => rhs+30,
            Nonterminal::Mult => rhs+31,
            Nonterminal::Array => rhs+32,
            Nonterminal::Num => rhs+33,
            Nonterminal::Boolean => rhs+34,
            Nonterminal::String => rhs+35,
            Nonterminal::Number(_) => rhs+36,
            Nonterminal::UserDefined(_) => rhs+37,
            Nonterminal::ShortString(_) => rhs+38,
        }
    }
}

impl Mul<usize> for Nonterminal {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        match self {
            Nonterminal::LParentheses => rhs*0,
            Nonterminal::RParentheses => rhs*1,
            Nonterminal::LBrace => rhs*2,
            Nonterminal::RBrace => rhs*3,
            Nonterminal::LBracket => rhs*4,
            Nonterminal::RBracket => rhs*5,
            Nonterminal::Semicolon => rhs*6,
            Nonterminal::Comma => rhs*7,
            Nonterminal::Main => rhs*8,
            Nonterminal::Halt => rhs*9,
            Nonterminal::Proc => rhs*10,
            Nonterminal::Return => rhs*11,
            Nonterminal::Assignment => rhs*12,
            Nonterminal::If => rhs*13,
            Nonterminal::Else => rhs*14,
            Nonterminal::Then => rhs*15,
            Nonterminal::Do => rhs*16,
            Nonterminal::Until => rhs*17,
            Nonterminal::While => rhs*18,
            Nonterminal::Out => rhs*19,
            Nonterminal::Call => rhs*20,
            Nonterminal::True => rhs*21,
            Nonterminal::False => rhs*22,
            Nonterminal::Input => rhs*23,
            Nonterminal::Not => rhs*24,
            Nonterminal::And => rhs*25,
            Nonterminal::Or => rhs*26,
            Nonterminal::Equal => rhs*27,
            Nonterminal::Larger => rhs*28,
            Nonterminal::Add => rhs*29,
            Nonterminal::Sub => rhs*30,
            Nonterminal::Mult => rhs*31,
            Nonterminal::Array => rhs*32,
            Nonterminal::Num => rhs*33,
            Nonterminal::Boolean => rhs*34,
            Nonterminal::String => rhs*35,
            Nonterminal::Number(_) => rhs*36,
            Nonterminal::UserDefined(_) => rhs*37,
            Nonterminal::ShortString(_) => rhs*38,
        }
    }
}

// all nonterminals of the grammer
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Terminal {
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

impl Mul<usize> for Terminal {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        self as usize * rhs
    }
}

#[allow(dead_code)]
pub enum Grammer {
    // terminal
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
    // Nonterminal
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