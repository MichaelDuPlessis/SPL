use std::{rc::Rc, cell::{RefCell}, fs::File, io::Write, mem};

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
            // println!("{:?}", input);

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
                    panic!("Invalid Program");
                }
            } else if let Grammer::NonTerminal(t) = top {
                // println!("{:?}, {:?}", self.tokens[input].token()*1, t*1);
                // println!("{:?}", self.table[self.tokens[input].token() + t*ROWS]);
                if self.table[self.tokens[input].token() + t*ROWS].is_none() {
                    panic!("Invalid Program");
                }

                let terminal = Rc::clone(&stack.pop());
                last_node = Rc::clone(&terminal);
                
                if first {
                    head = Rc::clone(&terminal);
                    first = false;
                }

                let mut rhs: Vec<Rc<RefCell<Node>>> = self.table[self.tokens[input].token() + t*ROWS].as_ref().unwrap().clone() // now that it is not none
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

        if let Some(num) = sym.num_value {
            to_write.push_str(&format!("{}<{} id={} value={}>\n", tabs, sym.symbol, sym.id, num));
        } else if let Some(string) = &sym.str_value {
            to_write.push_str(&format!("{}<{} id={} value={}>\n", tabs, sym.symbol, sym.id, string));
        } else {
            to_write.push_str(&format!("{}<{} id={}>\n", tabs, sym.symbol, sym.id));
        }

        for c in &sym.children {
            Self::write_to_xml(Rc::clone(&c), to_write, level + 1);
        }

        to_write.push_str(&format!("{}</{}>\n", tabs, sym.symbol));
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