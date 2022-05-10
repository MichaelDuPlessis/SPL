use std::{rc::Rc, cell::{RefCell}, fs::File, io::Write, mem, process::exit};

use crate::{grammer::*, token::{TokenList, Token, Pos}, stack};
const ROWS: usize = 22;
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

        table[Terminal::Main + NonTerminal::SPLProgr*COLS] = Some(vec![Grammer::from(NonTerminal::ProcDefs), Grammer::from(Terminal::Main), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::Halt), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl), Grammer::from(Terminal::RBrace)]);
        table[Terminal::Proc + NonTerminal::SPLProgr*COLS] = Some(vec![Grammer::from(NonTerminal::ProcDefs), Grammer::from(Terminal::Main), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::Halt), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl), Grammer::from(Terminal::RBrace)]);
        
        table[Terminal::Main + NonTerminal::ProcDefs*COLS] = Some(vec![]);
        table[Terminal::Proc + NonTerminal::ProcDefs*COLS] = Some(vec![Grammer::from(NonTerminal::PD), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::ProcDefs)]);
        table[Terminal::Return + NonTerminal::ProcDefs*COLS] = Some(vec![]);
        table[Terminal::If + NonTerminal::ProcDefs*COLS] = Some(vec![]);
        table[Terminal::Do + NonTerminal::ProcDefs*COLS] = Some(vec![]);
        table[Terminal::While + NonTerminal::ProcDefs*COLS] = Some(vec![]);
        table[Terminal::Out + NonTerminal::ProcDefs*COLS] = Some(vec![]);
        table[Terminal::True + NonTerminal::ProcDefs*COLS] = Some(vec![]);
        table[Terminal::UserDefined + NonTerminal::ProcDefs*COLS] = Some(vec![]);
        
        table[Terminal::Proc + NonTerminal::PD*COLS] = Some(vec![Grammer::from(Terminal::Proc), Grammer::from(Terminal::UserDefined), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::ProcDefs), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::Return), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl), Grammer::from(Terminal::RBrace)]);
        
        table[Terminal::RBrace + NonTerminal::Algorithm*COLS] = Some(vec![]);
        table[Terminal::Halt + NonTerminal::Algorithm*COLS] = Some(vec![]);
        table[Terminal::Return + NonTerminal::Algorithm*COLS] = Some(vec![]);
        table[Terminal::If + NonTerminal::Algorithm*COLS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[Terminal::Do + NonTerminal::Algorithm*COLS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[Terminal::While + NonTerminal::Algorithm*COLS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[Terminal::Out + NonTerminal::Algorithm*COLS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[Terminal::Call + NonTerminal::Algorithm*COLS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        table[Terminal::UserDefined + NonTerminal::Algorithm*COLS] = Some(vec![Grammer::from(NonTerminal::Instr), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::Algorithm)]);
        
        table[Terminal::If + NonTerminal::Instr*COLS] = Some(vec![Grammer::from(NonTerminal::Branch)]);
        table[Terminal::Do + NonTerminal::Instr*COLS] = Some(vec![Grammer::from(NonTerminal::Loop)]);
        table[Terminal::While + NonTerminal::Instr*COLS] = Some(vec![Grammer::from(NonTerminal::Loop)]);
        table[Terminal::Out + NonTerminal::Instr*COLS] = Some(vec![Grammer::from(NonTerminal::Assign)]);
        table[Terminal::Call + NonTerminal::Instr*COLS] = Some(vec![Grammer::from(NonTerminal::PCall)]);
        table[Terminal::UserDefined + NonTerminal::Instr*COLS] = Some(vec![Grammer::from(NonTerminal::Assign)]);
        
        table[Terminal::UserDefined + NonTerminal::Assign*COLS] = Some(vec![Grammer::from(NonTerminal::LHS), Grammer::from(Terminal::Assignment), Grammer::from(NonTerminal::Expr)]);
        table[Terminal::Out + NonTerminal::Assign*COLS] = Some(vec![Grammer::from(NonTerminal::LHS), Grammer::from(Terminal::Assignment), Grammer::from(NonTerminal::Expr)]);
        
        table[Terminal::If + NonTerminal::Branch*COLS] = Some(vec![Grammer::from(Terminal::If), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses), Grammer::from(Terminal::Then), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::RBrace), Grammer::from(NonTerminal::Alternat)]);

        table[Terminal::Semicolon + NonTerminal::Alternat*COLS] = Some(vec![]);
        table[Terminal::Else + NonTerminal::Alternat*COLS] = Some(vec![Grammer::from(Terminal::Else), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::RBrace)]);
        
        table[Terminal::Do + NonTerminal::Loop*COLS] = Some(vec![Grammer::from(Terminal::Do), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::RBrace), Grammer::from(Terminal::Until), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[Terminal::While + NonTerminal::Loop*COLS] = Some(vec![Grammer::from(Terminal::While), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses), Grammer::from(Terminal::Do), Grammer::from(Terminal::LBrace), Grammer::from(NonTerminal::Algorithm), Grammer::from(Terminal::RBrace)]);
        
        table[Terminal::UserDefined + NonTerminal::LHS*COLS] = Some(vec![Grammer::from(Terminal::UserDefined), Grammer::from(NonTerminal::VarField)]);
        table[Terminal::Out + NonTerminal::LHS*COLS] = Some(vec![Grammer::from(Terminal::Out)]);

        table[Terminal::True + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::Const)]);
        table[Terminal::False + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::Const)]);
        table[Terminal::Input + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::UnOp)]);
        table[Terminal::Not + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::UnOp)]);
        table[Terminal::And + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[Terminal::Or + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[Terminal::Equal + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[Terminal::Larger + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[Terminal::Add + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[Terminal::Sub + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[Terminal::Mult + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::BinOp)]);
        table[Terminal::Number + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::Const)]);
        table[Terminal::UserDefined + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(Terminal::UserDefined), Grammer::from(NonTerminal::VarField)]);
        table[Terminal::ShortString + NonTerminal::Expr*COLS] = Some(vec![Grammer::from(NonTerminal::Const)]);
        
        table[Terminal::RParentheses + NonTerminal::VarField*COLS] = Some(vec![]);
        table[Terminal::LBracket + NonTerminal::VarField*COLS] = Some(vec![Grammer::from(Terminal::LBracket), Grammer::from(NonTerminal::FType)]);
        table[Terminal::Semicolon + NonTerminal::VarField*COLS] = Some(vec![]);
        table[Terminal::Comma + NonTerminal::VarField*COLS] = Some(vec![]);
        table[Terminal::Assignment + NonTerminal::VarField*COLS] = Some(vec![]);
        
        table[Terminal::Call + NonTerminal::PCall*COLS] = Some(vec![Grammer::from(Terminal::Call), Grammer::from(Terminal::UserDefined)]);
        
        table[Terminal::UserDefined + NonTerminal::Var*COLS] = Some(vec![Grammer::from(Terminal::UserDefined)]);
        
        table[Terminal::True + NonTerminal::FType*COLS] = Some(vec![Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket)]);
        table[Terminal::False + NonTerminal::FType*COLS] = Some(vec![Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket)]);
        table[Terminal::Number + NonTerminal::FType*COLS] = Some(vec![Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket)]);
        table[Terminal::UserDefined + NonTerminal::FType*COLS] = Some(vec![Grammer::from(NonTerminal::Var), Grammer::from(Terminal::RBracket)]);
        table[Terminal::ShortString + NonTerminal::FType*COLS] = Some(vec![Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket)]);
        
        table[Terminal::True + NonTerminal::Const*COLS] = Some(vec![Grammer::from(Terminal::True)]);
        table[Terminal::False + NonTerminal::Const*COLS] = Some(vec![Grammer::from(Terminal::False)]);
        table[Terminal::Number + NonTerminal::Const*COLS] = Some(vec![Grammer::from(Terminal::Number)]);
        table[Terminal::ShortString + NonTerminal::Const*COLS] = Some(vec![Grammer::from(Terminal::ShortString)]);
        
        table[Terminal::Input + NonTerminal::UnOp*COLS] = Some(vec![Grammer::from(Terminal::Input), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Var), Grammer::from(Terminal::RParentheses)]);
        table[Terminal::Not + NonTerminal::UnOp*COLS] = Some(vec![Grammer::from(Terminal::Not), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        
        table[Terminal::And + NonTerminal::BinOp*COLS] = Some(vec![Grammer::from(Terminal::And), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[Terminal::Or + NonTerminal::BinOp*COLS] = Some(vec![Grammer::from(Terminal::Or), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[Terminal::Equal + NonTerminal::BinOp*COLS] = Some(vec![Grammer::from(Terminal::Equal), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[Terminal::Larger + NonTerminal::BinOp*COLS] = Some(vec![Grammer::from(Terminal::Larger), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[Terminal::Add + NonTerminal::BinOp*COLS] = Some(vec![Grammer::from(Terminal::Add), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[Terminal::Sub + NonTerminal::BinOp*COLS] = Some(vec![Grammer::from(Terminal::Sub), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        table[Terminal::Mult + NonTerminal::BinOp*COLS] = Some(vec![Grammer::from(Terminal::Mult), Grammer::from(Terminal::LParentheses), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::Comma), Grammer::from(NonTerminal::Expr), Grammer::from(Terminal::RParentheses)]);
        
        table[Terminal::RBrace + NonTerminal::VarDecl*COLS] = Some(vec![]);
        table[Terminal::Array + NonTerminal::VarDecl*COLS] = Some(vec![Grammer::from(NonTerminal::Dec), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl)]);
        table[Terminal::Num + NonTerminal::VarDecl*COLS] = Some(vec![Grammer::from(NonTerminal::Dec), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl)]);
        table[Terminal::Boolean + NonTerminal::VarDecl*COLS] = Some(vec![Grammer::from(NonTerminal::Dec), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl)]);
        table[Terminal::String + NonTerminal::VarDecl*COLS] = Some(vec![Grammer::from(NonTerminal::Dec), Grammer::from(Terminal::Semicolon), Grammer::from(NonTerminal::VarDecl)]);
        
        table[Terminal::Array + NonTerminal::Dec*COLS] = Some(vec![Grammer::from(Terminal::Array), Grammer::from(NonTerminal::TYP), Grammer::from(Terminal::LBracket), Grammer::from(NonTerminal::Const), Grammer::from(Terminal::RBracket), Grammer::from(NonTerminal::Var)]);
        table[Terminal::Num + NonTerminal::Dec*COLS] = Some(vec![Grammer::from(NonTerminal::TYP), Grammer::from(NonTerminal::Var)]);
        table[Terminal::Boolean + NonTerminal::Dec*COLS] = Some(vec![Grammer::from(NonTerminal::TYP), Grammer::from(NonTerminal::Var)]);
        table[Terminal::String + NonTerminal::Dec*COLS] = Some(vec![Grammer::from(NonTerminal::TYP), Grammer::from(NonTerminal::Var)]);
        
        table[Terminal::Num + NonTerminal::TYP*COLS] = Some(vec![Grammer::from(Terminal::Num)]);
        table[Terminal::Boolean + NonTerminal::TYP*COLS] = Some(vec![Grammer::from(Terminal::Boolean)]);
        table[Terminal::String + NonTerminal::TYP*COLS] = Some(vec![Grammer::from(Terminal::String)]);        

        tokens.push(Token::new_struct_token(Terminal::Dollar, Pos::new(0, 0)));

        Self {
            tokens,
            table,
        }
    }

    // actual parsing
    pub fn parse(&self) -> Node {
        let mut stack = stack::Stack::from(
            vec![Grammer::from(NonTerminal::SPLProgr), Grammer::from(Terminal::Dollar)]
            .into_iter()
            .map(|g| {
                Rc::new(RefCell::new(Node::new(g, 0)))
            })
            .collect::<Vec<Rc<RefCell<Node>>>>()
        );

        let mut id = 0;
        
        let mut head = Rc::new(RefCell::new(Node::new(Grammer::from(Terminal::Dollar), 0)));
        let mut first = true;

        let mut input = 0usize;

        let mut last_node = Rc::new(RefCell::new(Node::new(Grammer::from(Terminal::Dollar), 0)));

        while !stack.is_empty() {
            let top = RefCell::borrow(stack.peek()).symbol;
            // println!("{:?}", stack);
            // println!("{:?}", self.tokens[input].token());

            if let Grammer::Terminal(t) = top {
                if t == self.tokens[input].token() {
                    // adding data to non terminal nodes
                    for n in &last_node.borrow_mut().children {
                        let sym = RefCell::borrow(n).symbol;
                        if sym == top {
                            n.borrow_mut().pos = Some(self.tokens[input].pos());
                            n.borrow_mut().num_value = self.tokens[input].num_value();
                            n.borrow_mut().str_value = self.tokens[input].str_value();
                            break;
                        }
                    }

                    input += 1;
                    stack.pop();
                } else {
                    println!("Expected {} but found {} on Ln {}, Col {}", t, if let Some(val) = self.tokens[input].str_value() { val } else { self.tokens[input].token().to_string() }, self.tokens[input].row(), self.tokens[input].col());
                    exit(1);
                }
            } else if let Grammer::NonTerminal(t) = top {
                // println!("{} : {}", self.tokens[input].token(), t);
                if self.table[self.tokens[input].token() + t*COLS].is_none() {
                    println!("Error: {} cannot follow {}", self.tokens[input].token(), self.tokens[input-1].token());
                    exit(1);
                }

                let terminal = Rc::clone(&stack.pop());
                last_node = Rc::clone(&terminal);
                
                if first {
                    head = Rc::clone(&terminal);
                    first = false;
                }

                let mut rhs: Vec<Rc<RefCell<Node>>> = self.table[self.tokens[input].token() + t*COLS].as_ref().unwrap().clone() // now that it is not none
                .into_iter()
                .map(|g| {
                    id += 1;
                    Rc::new(RefCell::new(Node::new(g, id)))
                })
                .collect();

                terminal.borrow_mut().add_children(&rhs);

                rhs.append(&mut stack.to_vec());
                stack = stack::Stack::from(rhs);
            }
        }

        // create new head to return
        let mut node = Node::new(RefCell::borrow(&head).symbol, RefCell::borrow(&head).id);
        node.pos = RefCell::borrow(&head).pos;
        node.num_value = RefCell::borrow(&head).num_value;
        node.str_value = RefCell::borrow(&head).str_value.clone();
        mem::swap(&mut node.children, &mut RefCell::borrow_mut(&head).children);

        node
    }

    pub fn create_xml(node: Node) {
        let mut file = File::create("./parse.xml").unwrap();
        let mut to_write = String::new();

        Self::write_to_xml(Rc::new(RefCell::new(node)), &mut to_write, 0);

        file.write_all(to_write.as_bytes()).unwrap();
    }

    fn write_to_xml(node: Rc<RefCell<Node>>, to_write: &mut String, level: usize) {
        let mut tabs = String::new();
        for _ in 0..level {
            tabs.push('\t');
        }

        let sym = RefCell::borrow(&*node);

        if let Grammer::Terminal(_) = sym.symbol {
            if let Some(num) = sym.num_value {
                to_write.push_str(&format!("{}<{} id=\"{}\" value=\"{}\"/>\n", tabs, sym.symbol, sym.id, num));
            } else if let Some(string) = &sym.str_value {
                to_write.push_str(&format!("{}<{} id=\"{}\" value=\"{}\"/>\n", tabs, sym.symbol, sym.id, string));
            }
        } else {
            to_write.push_str(&format!("{}<{} id=\"{}\">\n", tabs, sym.symbol, sym.id));
    
            for c in &sym.children {
                Self::write_to_xml(Rc::clone(&c), to_write, level + 1);
            }
    
            to_write.push_str(&format!("{}</{}>\n", tabs, sym.symbol));
        }
    }
}

#[derive(Debug)]
pub struct Node {
    id: usize,
    symbol: Grammer,
    children: Vec<Rc<RefCell<Node>>>,
    pos: Option<Pos>,
    num_value: Option<isize>,
    str_value: Option<String>,
}

impl Node {
    fn new(symbol: Grammer, id: usize) -> Self {
        Self {
            id,
            symbol,
            children: Vec::new(),
            pos: None,
            num_value: None,
            str_value: None,
        }
    } 
    
    fn add_children(&mut self, children: &Vec<Rc<RefCell<Node>>>) {
        for c in children {
            self.children.push(Rc::clone(c));
        }
    }
}