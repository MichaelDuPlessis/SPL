use std::{rc::Rc, cell::RefCell};

use crate::{grammer::*, token::{TokenList, StructureToken, Pos}, stack};
const ROWS: usize = 21;
const COLS: usize = 40;

#[allow(dead_code)]
pub struct Parser {
    tokens: TokenList,
    table: [Option<Vec<Grammer>>; ROWS*COLS],
}

#[allow(dead_code)]
impl Parser {
    pub fn new(mut tokens: TokenList) -> Self {
        let mut table = [(); ROWS*COLS].map(|_| Option::<Vec<Grammer>>::default());

        // table[8 + NonTerminal::SPLProgrPrime*ROWS] = Some(vec![Grammer::from(NonTerminal::ProcDefs), Grammer::from(Terminal::Dollar)]);
        // table[10 + NonTerminal::SPLProgrPrime*ROWS] = Some(vec![Grammer::from(NonTerminal::ProcDefs), Grammer::from(Terminal::Dollar)]);
        
        table[8 + NonTerminal::SPLProgr*ROWS] = Some(vec![Grammer::from(NonTerminal::ProcDefs), Grammer::from(Terminal::Main), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::Halt), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl), Grammer::from(Terminal::RBrace)]);
        table[10 + NonTerminal::SPLProgr*ROWS] = Some(vec![Grammer::from(NonTerminal::ProcDefs), Grammer::from(Terminal::Main), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::Halt), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl), Grammer::from(Terminal::RBrace)]);
        
        table[8 + NonTerminal::ProcDefs*ROWS] = Some(vec![]);
        table[10 + NonTerminal::ProcDefs*ROWS] = Some(vec![Grammer::from(NonTerminal::PD), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::ProcDefs)]);
        table[11 + NonTerminal::ProcDefs*ROWS] = Some(vec![]);
        table[13 + NonTerminal::ProcDefs*ROWS] = Some(vec![]);
        table[16 + NonTerminal::ProcDefs*ROWS] = Some(vec![]);
        table[18 + NonTerminal::ProcDefs*ROWS] = Some(vec![]);
        table[19 + NonTerminal::ProcDefs*ROWS] = Some(vec![]);
        table[21 + NonTerminal::ProcDefs*ROWS] = Some(vec![]);
        
        table[10 + NonTerminal::PD*ROWS] = Some(vec![Grammer::from(Terminal::Proc), Grammer::from(Terminal::UserDefined), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::ProcDefs), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::Return), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl), Grammer::from(Terminal::RBrace)]);
        
        table[3 + NonTerminal::Algorithm*ROWS] = Some(vec![]);
        table[9 + NonTerminal::Algorithm*ROWS] = Some(vec![]);
        table[11 + NonTerminal::Algorithm*ROWS] = Some(vec![]);
        table[13 + NonTerminal::Algorithm*ROWS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[16 + NonTerminal::Algorithm*ROWS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[18 + NonTerminal::Algorithm*ROWS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[19 + NonTerminal::Algorithm*ROWS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[20 + NonTerminal::Algorithm*ROWS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[37 + NonTerminal::Algorithm*ROWS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        
        table[13 + NonTerminal::Instr*ROWS] = Some(vec![Grammer::from(NonTerminal::Branch)]);
        table[16 + NonTerminal::Instr*ROWS] = Some(vec![Grammer::from(NonTerminal::Loop)]);
        table[18 + NonTerminal::Instr*ROWS] = Some(vec![Grammer::from(NonTerminal::Loop)]);
        table[19 + NonTerminal::Instr*ROWS] = Some(vec![Grammer::from(NonTerminal::Assign)]);
        table[20 + NonTerminal::Instr*ROWS] = Some(vec![Grammer::from(NonTerminal::PCall)]);
        table[37 + NonTerminal::Instr*ROWS] = Some(vec![Grammer::from(NonTerminal::Assign)]);
        
        table[37 + NonTerminal::Assign*ROWS] = Some(vec![Grammer::from(NonTerminal::LHS), Grammer::from(Terminal::Assignment), Grammer::from(NonTerminal::Expr)]);
        
        table[13 + NonTerminal::Branch*ROWS] = Some(vec![Grammer::from(Terminal::If), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses), Grammer::from(Terminal::Then), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Alternat)]);

        table[6 + NonTerminal::Alternat*ROWS] = Some(vec![]);
        table[14 + NonTerminal::Alternat*ROWS] = Some(vec![Grammer::from(Terminal::Else), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::RBrace)]);
        
        table[16 + NonTerminal::Loop*ROWS] = Some(vec![Grammer::from(Terminal::Do), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::RBrace), Grammer::from(Terminal::Until), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[16 + NonTerminal::Loop*ROWS] = Some(vec![Grammer::from(Terminal::While), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses), Grammer::from(Terminal::Do), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::RBrace)]);
        
        table[37 + NonTerminal::LHS*ROWS] = Some(vec![Grammer::from(Terminal::UserDefined), Grammer::from(NonTerminal::VarField)]);
        table[37 + NonTerminal::LHS*ROWS] = Some(vec![Grammer::from(Terminal::Out)]);

        table[21 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::Const)]);
        table[22 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::Const)]);
        table[23 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::UnOp)]);
        table[24 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::UnOp)]);
        table[25 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[26 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[27 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[28 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[29 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[30 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[31 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[36 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::Const)]);
        table[37 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(Terminal::UserDefined), Grammer::from(NonTerminal::VarField)]);
        table[38 + NonTerminal::Expr*ROWS] = Some(vec![Grammer::from(NonTerminal::Const)]);
        
        table[1 + NonTerminal::VarField*ROWS] = Some(vec![]);
        table[4 + NonTerminal::VarField*ROWS] = Some(vec![Grammer::from(Terminal::LBracket), Grammer::from(NonTerminal::FType)]);
        table[6 + NonTerminal::VarField*ROWS] = Some(vec![]);
        table[7 + NonTerminal::VarField*ROWS] = Some(vec![]);
        table[14 + NonTerminal::VarField*ROWS] = Some(vec![]);
        
        table[20 + NonTerminal::PCall*ROWS] = Some(vec![Grammer::from(Terminal::Call), Grammer::from(Terminal::UserDefined)]);
        
        table[37 + NonTerminal::Var*ROWS] = Some(vec![Grammer::from(Terminal::UserDefined)]);
        
        table[21 + NonTerminal::FType*ROWS] = Some(vec![Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket)]);
        table[22 + NonTerminal::FType*ROWS] = Some(vec![Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket)]);
        table[36 + NonTerminal::FType*ROWS] = Some(vec![Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket)]);
        table[37 + NonTerminal::FType*ROWS] = Some(vec![Grammer::from(NonTerminal::Var), Grammer::from(Terminal::RBracket)]);
        table[38 + NonTerminal::FType*ROWS] = Some(vec![Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket)]);
        
        table[21 + NonTerminal::Const*ROWS] = Some(vec![Grammer::from(Terminal::True)]);
        table[22 + NonTerminal::Const*ROWS] = Some(vec![Grammer::from(Terminal::False)]);
        table[36 + NonTerminal::Const*ROWS] = Some(vec![Grammer::from(Terminal::Number)]);
        table[38 + NonTerminal::Const*ROWS] = Some(vec![Grammer::from(Terminal::ShortString)]);
        
        table[23 + NonTerminal::UnOp*ROWS] = Some(vec![Grammer::from(Terminal::Input), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Var), Grammer::from(Terminal::RParentheses)]);
        table[24 + NonTerminal::UnOp*ROWS] = Some(vec![Grammer::from(Terminal::Not), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        
        table[25 + NonTerminal::BinOp*ROWS] = Some(vec![Grammer::from(Terminal::And), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[26 + NonTerminal::BinOp*ROWS] = Some(vec![Grammer::from(Terminal::Or), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[27 + NonTerminal::BinOp*ROWS] = Some(vec![Grammer::from(Terminal::Equal), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[28 + NonTerminal::BinOp*ROWS] = Some(vec![Grammer::from(Terminal::Larger), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[29 + NonTerminal::BinOp*ROWS] = Some(vec![Grammer::from(Terminal::Add), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[30 + NonTerminal::BinOp*ROWS] = Some(vec![Grammer::from(Terminal::Sub), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[31 + NonTerminal::BinOp*ROWS] = Some(vec![Grammer::from(Terminal::Mult), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        
        table[3 + NonTerminal::VarDecl*ROWS] = Some(vec![]);
        table[32 + NonTerminal::VarDecl*ROWS] = Some(vec![Grammer::from(NonTerminal::Dec), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl)]);
        table[33 + NonTerminal::VarDecl*ROWS] = Some(vec![Grammer::from(NonTerminal::Dec), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl)]);
        table[34 + NonTerminal::VarDecl*ROWS] = Some(vec![Grammer::from(NonTerminal::Dec), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl)]);
        table[35 + NonTerminal::VarDecl*ROWS] = Some(vec![Grammer::from(NonTerminal::Dec), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl)]);
        
        table[32 + NonTerminal::Dec*ROWS] = Some(vec![Grammer::from(Terminal::Array), Grammer::from(NonTerminal::TYP), Grammer::from(Terminal::LBracket), Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket), Grammer::from(NonTerminal::Var)]);
        table[33 + NonTerminal::Dec*ROWS] = Some(vec![Grammer::from(NonTerminal::TYP), Grammer::from(NonTerminal::Var)]);
        table[34 + NonTerminal::Dec*ROWS] = Some(vec![Grammer::from(NonTerminal::TYP), Grammer::from(NonTerminal::Var)]);
        table[35 + NonTerminal::Dec*ROWS] = Some(vec![Grammer::from(NonTerminal::TYP), Grammer::from(NonTerminal::Var)]);
        
        table[33 + NonTerminal::TYP*ROWS] = Some(vec![Grammer::from(Terminal::Num)]);
        table[34 + NonTerminal::TYP*ROWS] = Some(vec![Grammer::from(Terminal::Boolean)]);
        table[35 + NonTerminal::TYP*ROWS] = Some(vec![Grammer::from(Terminal::String)]);
        

        tokens.push(StructureToken::new_box(Terminal::Dollar, Pos::new(0, 0)));

        Self {
            tokens,
            table,
        }
    }

    pub fn parse(&self) {
        let mut stack = stack::Stack::from(
            vec![Grammer::from(NonTerminal::SPLProgr), Grammer::from(Terminal::Dollar)]
            .into_iter()
            .map(|g| Rc::new(RefCell::new(Node::new(g))))
            .collect::<Vec<Rc<RefCell<Node>>>>()
        );

        let mut head = Rc::new(RefCell::new(Node::new(Grammer::from(Terminal::Dollar))));
        let mut first = true;

        let mut input = 0usize;

        while !stack.is_empty() {
            // println!("input: {}", input);
            // println!("{:?}", stack);

            let top = stack.peek().borrow().symbol;
            if let Grammer::Terminal(t) = top {
                if t == self.tokens[input].token() {
                    input += 1;
                    stack.pop();
                } else {
                    panic!("Invalid Program");
                }
            } else if let Grammer::NonTerminal(t) = top {
                // println!("stack_pos {}, {:?}", self.tokens[input].token() + 0,  t*1);
                if self.table[self.tokens[input].token() + t*ROWS].is_none() {
                    panic!("Invalid Program");
                }

                let terminal = Rc::clone(&stack.pop());
                
                if first {
                    head = Rc::clone(&terminal);
                    first = false;
                }

                let mut rhs: Vec<Rc<RefCell<Node>>> = self.table[self.tokens[input].token() + t*ROWS].as_ref().unwrap().clone() // now that it is not none
                .into_iter()
                .map(|g| Rc::new(RefCell::new(Node::new(g))))
                .collect();

                terminal.borrow_mut().add_children(&rhs);

                rhs.append(&mut stack.to_vec());
                stack = stack::Stack::from(rhs);
            } else {
                panic!("How did it get here");
            }
        }

        print_tree(head, 0);
    }
}

fn print_tree(head: Rc<RefCell<Node>>, level: usize) {
    let mut tabs = String::new();
    for _ in 0..level {
        tabs.push('\t');
    }

    println!("{}Symbol: {:?}", tabs, RefCell::borrow(&*head).symbol);
    println!("{}{:?}-Children-{:?}", tabs, RefCell::borrow(&*head).symbol, RefCell::borrow(&*head).symbol);
    for c in &head.borrow().children {
        print_tree(Rc::clone(c), level + 1);
    }
    println!("{}{:?}-End-{:?}", tabs, RefCell::borrow(&*head).symbol, RefCell::borrow(&*head).symbol);
}


#[derive(Debug, Clone)]
struct Node {
    symbol: Grammer,
    children: Vec<Rc<RefCell<Node>>>
}

impl Node {
    fn new(symbol: Grammer) -> Self {
        Self {
            symbol,
            children: Vec::new(),
        }
    } 
    
    fn add_children(&mut self, children: &Vec<Rc<RefCell<Node>>>) {
        for c in children {
            self.children.push(Rc::clone(c));
        }
    }
}