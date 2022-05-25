use std::{rc::Rc, fs::File, io::Write, cell::RefCell};

use crate::{token::LNode, grammer::{Grammer, Terminal, NonTerminal}};

pub struct Generator {
    ast: LNode,
    file: File,
    line_no: usize,
    var_no: usize,
}

impl Generator {
    pub fn new(ast: LNode) -> Self {
        let file = File::create("out.bas").unwrap();

        Self {
            ast,
            file,
            line_no: 0,
            var_no: 0,
        }
    }

    pub fn generate(&mut self) {
        self._generate(Rc::clone(&self.ast));
        self.write_line("END");
    }
    
    fn _generate(&mut self, node: LNode) {
        for c in &node.borrow().children {
            let grammer = c.borrow().symbol;

            match grammer {
                Grammer::Terminal(t) => match t {
                    Terminal::Halt => {
                        self.write_line("STOP");
                        return;
                    }, // don't care about VarDecl
                    _ => (),
                },
                Grammer::NonTerminal(nt) => match nt {
                    NonTerminal::Instr => self.instruction(&c.borrow().children[0]),
                    _ => self._generate(Rc::clone(c)),
                },
            }
        }
    }

    fn write_line(&mut self, line: &str) {
        let line_num = self.next_line();
        self.file.write_all(format!("{} {}\n", line_num, line).as_bytes()).unwrap();
    }

    fn next_line(&mut self) -> usize {
        self.line_no += 10;
        self.line_no
    }

    fn next_var(&mut self) -> String {
        let v = format!("T{}", self.var_no);
        self.var_no += 1;
        return v;
    }

    fn instruction(&mut self, node: &LNode) { // pass in child
        let grammer = node.borrow().symbol;

        match grammer {
            Grammer::Terminal(_) => (),
            Grammer::NonTerminal(nt) => match nt {
                NonTerminal::Assign => {
                    let to_write = self.assign(node);
                    self.write_line(&to_write);
                },
                NonTerminal::Branch => todo!(),
                NonTerminal::Loop => todo!(),
                NonTerminal::PCall => todo!(),
                _ => panic!("Should not get here."),
            },
        }
    }

    fn assign(&self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let var_name = &children[0].borrow().children[0];
        let expr = self.expression(&children[2]);
        
        format!("{}={}", RefCell::borrow(var_name).str_value.as_ref().unwrap(), expr)
    }

    fn expression(&self, node: &LNode) -> String {
        let node = &node.borrow().children[0];
        let grammer = node.borrow().symbol;

        match grammer {
            Grammer::Terminal(t) => match t {
                Terminal::UserDefined => todo!(),
                _ => panic!("Should not get here")
            },
            Grammer::NonTerminal(nt) => match nt {
                NonTerminal::Const => self.cnst(node),
                NonTerminal::UnOp => self.unop(node),
                NonTerminal::BinOp => self.binop(node),
                _ => panic!("Should not get here")
            },
        }
    }

    fn unop(&self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let opp = children[0].borrow().symbol;
        let expr = self.expression(&children[2]);

        match opp {
            Grammer::Terminal(t) => match t {
                Terminal::Not => format!("({} + 1) mod 2", expr),
                Terminal::Input => todo!(),
                _ => panic!("Should not get here"),
            },
            Grammer::NonTerminal(_) => panic!("should not get here"),
        }
    }

    fn binop(&self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let opp = children[0].borrow().symbol;
        let expr1 = self.expression(&children[2]);
        let expr2 = self.expression(&children[4]);

        match opp {
            Grammer::Terminal(t) => match t {
                Terminal::Add => format!("({}+{})", expr1, expr2),
                Terminal::Mult => format!("({}*{})", expr1, expr2),
                Terminal::Sub => format!("({}-{})", expr1, expr2),
                _ => panic!("Should not get here"),
            },
            Grammer::NonTerminal(_) => panic!("should not get here"),
        }
    }

    fn cnst(&self, node: &LNode) -> String {
        let con = &node.borrow().children[0];

        let typ = match con.borrow().symbol {
            Grammer::Terminal(t) => match t {
                Terminal::Number => {
                    let num = con.borrow().num_value.unwrap();
                    num.to_string()
                },
                Terminal::ShortString => format!("\"{}\"", con.borrow().str_value.as_ref().unwrap()),
                Terminal::True => "1".to_string(),
                Terminal::False => "0".to_string(),
                _ => panic!("Should not get here"),
            },
            Grammer::NonTerminal(_) => panic!("Should not get here"),
        };

        typ
    }
}