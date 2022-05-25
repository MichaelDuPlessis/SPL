use std::{rc::Rc, fs::File, io::Write, cell::RefCell, fmt::format};

use crate::{token::LNode, grammer::{Grammer, Terminal, NonTerminal}, scope::ScopeNode};

pub struct Generator {
    ast: LNode,
    scope: ScopeNode,
    file: File,
    line_no: usize,
    var_no: usize,
}

impl Generator {
    pub fn new(ast: LNode, scope: ScopeNode) -> Self {
        let file = File::create("out.bas").unwrap();

        Self {
            ast,
            scope,
            file,
            line_no: 10,
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
                    NonTerminal::Algorithm => {
                        let code = self.algorithm(c);
                        for c in code.split('\n') {
                            self.write_line(c);
                        }
                    },
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
        let old = self.line_no;
        self.line_no += 10;
        old
    }

    fn next_var(&mut self) -> String {
        let v = format!("T{}", self.var_no);
        self.var_no += 1;
        return v;
    }

    fn algorithm(&mut self, node: &LNode) -> String{
        if node.borrow().children.is_empty() {
            return String::new();
        }

        let instr = &node.borrow().children[0];
        let algo = &node.borrow().children[2];

        let mut code = self.instruction(instr);

        if !algo.borrow().children.is_empty() {
            code = format!("{}\n{}", code, self.algorithm(algo));
        }

        code
    }

    fn instruction(&mut self, node: &LNode) -> String { // pass in child
        let insr_node = &node.borrow().children[0];
        let grammer = insr_node.borrow().symbol;

        match grammer {
            Grammer::Terminal(_) => todo!(),
            Grammer::NonTerminal(nt) => match nt {
                NonTerminal::Assign => self.assign(insr_node),
                NonTerminal::Branch => self.branch(insr_node),
                NonTerminal::Loop => todo!(),
                NonTerminal::PCall => todo!(),
                _ => panic!("Should not get here."),
            },
        }
    }

    // fn lop(&mut self, node: &LNode) -> String {
        
    // }

    fn branch(&mut self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let expr = self.expression(&children[2]);
        let algo = self.algorithm(&children[6]);
        let alternat = &children[8];

        // checking for else
        if !alternat.borrow().children.is_empty() {
            let alt_algo = self.algorithm(&alternat.borrow().children[2]);
            let line_num = algo.matches('\n').count()*10 + self.line_no + 40;
            let alt_line_num = alt_algo.matches('\n').count()*10 + line_num + 10;

            format!("if {} then {}\ngoto {}\n{}\ngoto {}\n{}", expr, self.line_no + 20, line_num, algo, alt_line_num, alt_algo)
        } else {
            format!("if {} then {}\n{}", expr, self.line_no + 10, algo)
        }
    }

    fn assign(&self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let var_name = self.scope.borrow().get_gen_name(children[0].borrow().children[0].borrow().str_value.as_ref().unwrap());
        
        // cause input is special
        if children[2].borrow().children[0].borrow().symbol == Grammer::NonTerminal(NonTerminal::UnOp) {
            if children[2].borrow().children[0].borrow().children[0].borrow().symbol == Grammer::Terminal(Terminal::Input) {
                return format!("INPUT {}", var_name);
            }
        }

        let expr = self.expression(&children[2]);
        format!("{} = {}", var_name, expr)
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
                Terminal::Add => format!("({} + {})", expr1, expr2),
                Terminal::Mult => format!("({} * {})", expr1, expr2),
                Terminal::Sub => format!("({} - {})", expr1, expr2),
                Terminal::Equal => format!("({} = {})", expr1, expr2),
                Terminal::Larger => format!("({} > {})", expr1, expr2),
                Terminal::And => format!("(({} + {}) = 2)", expr1, expr2),
                Terminal::Or => format!("(({} + {}) > 0)", expr1, expr2),
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