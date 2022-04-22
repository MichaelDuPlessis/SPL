use crate::lexer::{Token, Nonterminal};

// all nonterminals of the grammer
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

// Since a grammer symbol may be either one
enum Grammer {
    Terminal(Terminal),
    Nonterminal(Nonterminal),
}

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
        }
    }

    pub fn parse() {

    }

    fn spl_progr_rime() {

    }

    fn spl_progr() {

    }

    fn spl_progr_prime() {

    }

    fn proc_defs() {

    }

    fn pd() {

    }

    fn algorithm() {

    }

    fn instr() {

    }

    fn assign() {

    }

    fn branch() {

    }

    fn alternat() {

    }

    fn spl_loop() {

    }

    fn lhs() {

    }

    fn expr() {

    }

    fn var_field() {

    }

    fn field() {

    }

    fn f_type() {

    }

    fn spl_const() {

    }

    fn un_op() {

    }

    fn bin_op() {

    }

    fn var_decl() {

    }

    fn dec() {

    }

    fn typ() {

    }
}