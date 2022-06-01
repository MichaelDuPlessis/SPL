use std::{rc::Rc, fs::{File, self}, io::Write, collections::HashMap};
use crate::{token::LNode, grammer::{Grammer, Terminal, NonTerminal, Boolean}, scope::ScopeNode};

pub struct Generator {
    ast: LNode,
    scope: ScopeNode,
    file: File,
    line_no: usize,
    var_no: u8,
    var_letter: u8,
    proc_pos: HashMap<String, usize>,
    vname: HashMap<String, String>
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
            var_letter: 0,
            proc_pos: HashMap::new(),
            vname: HashMap::new(),
        }
    }

    pub fn generate(&mut self) {
        self.gen_vars();

        self.write_line("main");
        
        self._generate(Rc::clone(&self.ast));
        
        self.write_line("END");

        // self.file.rewind().unwrap();
        // self.file.write(format!("10 goto {}\n", 0).as_bytes()).unwrap();

        let temp = &fs::read_to_string("./out.bas").unwrap();
        fs::write("./out.bas", temp.replace("main", &format!("goto {}", self.proc_pos.get("main").unwrap().to_string()))).unwrap();
    }
    
    fn _generate(&mut self, node: LNode) {
        for c in &node.borrow().children {
            let grammer = c.borrow().symbol;

            match grammer {
                Grammer::Terminal(t) => match t {
                    Terminal::Halt => {
                        // self.write_line("STOP");
                        return;
                    }, // don't care about VarDecl
                    Terminal::Main => {
                        self.proc_pos.insert("main".to_string(), self.line_no);
                    },
                    _ => (),
                },
                Grammer::NonTerminal(nt) => match nt {
                    NonTerminal::Algorithm => {
                        let code = self.algorithm(c);
                        for c in code.split('\n') {
                            self.write_line(c);
                        }
                    },
                    NonTerminal::PD => self.pd(c),
                    _ => self._generate(Rc::clone(c)),
                },
            }
        }
    }

    fn gen_vars(&mut self) {
        let mut vars = String::new();

        for (name, si) in &Rc::clone(&self.scope).borrow().vtable {
            // let var_name = self.scope.borrow().get_gen_name(name, si.is_array);
            let var_name = self.gen_var_name(name, si.is_array);
            if si.is_array {
                vars.push_str(&format!("DIM {var_name}({})\n", si.array_size));
            } else {
                if var_name.contains('$') {
                    vars.push_str(&format!("LET {var_name} = \"\"\n"));
                } else {
                    vars.push_str(&format!("LET {var_name} = 0\n"));
                }
            }
        }

        for v in vars.trim().split('\n') {
            self.write_line(v);
        }

        for (name, si) in &Rc::clone(&self.scope).borrow().vtable {
            if si.is_proc {
                self.enter(name);
                self.gen_vars();
                self.exit();
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

    fn next_var(&mut self, arr: bool) -> String {
        let var_name = if arr {
            format!("{}{}$", (self.var_letter % 26 + 65) as char, self.var_no % 10)
        } else {
            format!("{}{}", (self.var_letter % 26 + 65) as char, self.var_no % 10)
        };
        self.var_no += 1;
        self.var_letter = self.var_no/10;

        var_name
    }

    fn gen_var_name(&mut self, name: &str, arr: bool) -> String {
        let key = self.scope.borrow().get_gen_name(name, arr);
        if let Some(n) = self.vname.get(&key) {
            n.clone()
        } else {
            let val = if key.contains('$') {
                self.next_var(true)
            } else {
                self.next_var(false)
            };
            self.vname.insert(key, val.clone());

            val
        }
    }

    fn pd(&mut self, node: &LNode) {
        let children = &node.borrow().children;
        let name = children[1].borrow();
        let name = name.str_value.as_ref().unwrap();

        self.enter(name);

        if !children[3].borrow().children.is_empty() {
            self.pd(&children[3].borrow().children[0]);
        }

        // let gen_name = self.scope.borrow().get_gen_name(name, false);
        let gen_name = self.gen_var_name(name, false);
        self.proc_pos.insert(gen_name, self.line_no);
        
        let algo = &children[4];
        let code = self.algorithm(algo);
        
        for c in format!("{}\nreturn", code).split('\n') {
            self.write_line(c);
        }

        self.exit();
    }

    fn algorithm(&mut self, node: &LNode) -> String {
        if node.borrow().children.is_empty() {
            return String::new();
        }

        let instr = &node.borrow().children[0];
        let algo = &node.borrow().children[2];

        let mut code = self.instruction(instr);

        if !algo.borrow().children.is_empty() {
            let aglo_size = self.size_of_algo(&code) + 10;
            self.line_no += aglo_size;
            code = format!("{}\n{}", code, self.algorithm(algo));
            self.line_no -= aglo_size;
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
                NonTerminal::Loop => self.lop(insr_node),
                NonTerminal::PCall => self.call(insr_node),
                _ => panic!("Should not get here."),
            },
        }
    }

    fn call(&mut self, node: &LNode) -> String {
        let name = &node.borrow().children[1];
        let name = name.borrow();
        let name = name.str_value.as_ref().unwrap();

        self.enter(name);

        // let gen_name = self.scope.borrow().get_gen_name(name, false);
        let gen_name = self.gen_var_name(name, false);

        self.exit();


        format!("gosub {}", self.proc_pos.get(&gen_name).unwrap())
    }

    fn lop(&mut self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let kind = children[0].borrow().symbol;

        match kind {
            Grammer::Terminal(t) => match t {
                Terminal::While => {
                    if self.bool_val(&children[2]) == Boolean::False {
                        return String::new();
                    }

                    let expr = self.expression(&children[2]);
                    self.line_no += 20;
                    let algo = self.algorithm(&children[6]);
                    self.line_no -= 20;
                    let end_while = self.line_no + self.size_of_algo(&algo) + 40;

                    format!("if {expr} then {}\ngoto {}\n{algo}\ngoto {}", self.line_no + 20, end_while, self.line_no)
                },
                Terminal::Do => {
                    let expr = self.expression(&children[6]);
                    let algo = self.algorithm(&children[2]);
                    let end_while = self.size_of_algo(&algo);

                    format!("{algo}\nif {expr} then {}\ngoto {}\n{algo}\ngoto {}", self.line_no + end_while + 30, self.line_no + 2*end_while + 50, self.line_no + end_while + 10)
                },
                _ => panic!("Shouldn't get here"),
            },
            Grammer::NonTerminal(_) => todo!(),
        }
    }

    fn branch(&mut self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let expr = self.expression(&children[2]);
        let algo = self.algorithm(&children[6]);
        let alternat = &children[8];

        // checking for else
        if !alternat.borrow().children.is_empty() {
            let alt_algo = self.algorithm(&alternat.borrow().children[2]);
            let line_num = self.size_of_algo(&algo) + self.line_no + 40;
            let alt_line_num = self.size_of_algo(&alt_algo) + line_num + 10;

            if self.bool_val(&children[2]) == Boolean::True {
                return algo;
            } else if self.bool_val(&children[2]) == Boolean::False {
                return alt_algo;
            }
            
            format!("if {} then {}\ngoto {}\n{}\ngoto {}\n{}", expr, self.line_no + 20, line_num, algo, alt_line_num, alt_algo)
        } else {
            if self.bool_val(&children[2]) == Boolean::False {
                return String::new();
            }
            format!("if {} then {}\n{}", expr, self.line_no + 10, algo)
        }
    }

    fn assign(&mut self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let lhs = children[0].borrow();

        let name = &lhs.children[0];
        
        if name.borrow().symbol == Grammer::Terminal(Terminal::Out) {
            let expr = self.expression(&children[2]);
            return format!("PRINT {expr}");
        }

        let name = name.borrow();
        let name = name.str_value.as_ref().unwrap();

        let var_field = lhs.children[1].borrow();

        let is_arr = if var_field.children.is_empty() {
            false
        } else {
            true
        };

        // let var_name = self.scope.borrow().get_gen_name(name, is_arr);
        let var_name = self.gen_var_name(name, is_arr);
        
        // cause input is special
        if children[2].borrow().children[0].borrow().symbol == Grammer::NonTerminal(NonTerminal::UnOp) {
            if children[2].borrow().children[0].borrow().children[0].borrow().symbol == Grammer::Terminal(Terminal::Input) {
                return format!("INPUT {}", var_name);
            }
        }

        let expr = self.expression(&children[2]);

        if is_arr {
            let indexer = self.expression(&var_field.children[1]);
            format!("{var_name}({indexer}) = {expr}")
        } else {
            format!("{var_name} = {expr}")
        }
    }

    fn expression(&mut self, node: &LNode) -> String {
        let child_node = &node.borrow().children[0];
        let grammer = child_node.borrow().symbol;

        match grammer {
            Grammer::Terminal(t) => match t {
                Terminal::UserDefined => {
                    let name = child_node.borrow();
                    let name = name.str_value.as_ref().unwrap();
                        
                    let var_field = node.borrow();
                    let var_field = var_field.children[1].borrow();

                    let is_arr = if var_field.children.is_empty() {
                        false
                    } else {
                        true
                    };
            
                    // let var_name = self.scope.borrow().get_gen_name(name, is_arr);
                    let var_name = self.gen_var_name(name, is_arr);

                    if is_arr {
                        let indexer = self.expression(&var_field.children[1]);
                        format!("{var_name}({indexer})")
                    } else {
                        format!("{var_name}")
                    }
                },
                _ => panic!("Should not get here")
            },
            Grammer::NonTerminal(nt) => match nt {
                NonTerminal::Var => self.gen_var_name(child_node.borrow().children[0].borrow().str_value.as_ref().unwrap(), false),
                NonTerminal::Const => self.cnst(child_node),
                NonTerminal::UnOp => self.unop(child_node),
                NonTerminal::BinOp => self.binop(child_node),
                _ => panic!("Should not get here")
            },
        }
    }

    fn unop(&mut self, node: &LNode) -> String {
        let children = &node.borrow().children;
        let opp = children[0].borrow().symbol;
        let expr = self.expression(&children[2]);

        match opp {
            Grammer::Terminal(t) => match t {
                Terminal::Not => format!("(({} + 1) % 2)", expr),
                Terminal::Input => todo!(),
                _ => panic!("Should not get here"),
            },
            Grammer::NonTerminal(_) => panic!("should not get here"),
        }
    }

    fn binop(&mut self, node: &LNode) -> String {
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

    fn bool_val(&self, node: &LNode) -> Boolean {
        let child_node = &node.borrow().children[0];
        let grammer = child_node.borrow().symbol;

        match grammer {
            Grammer::Terminal(t) => Boolean::Unknown,
            Grammer::NonTerminal(nt) => match nt {
                NonTerminal::Var => Boolean::Unknown,
                NonTerminal::Const => match child_node.borrow().children[0].borrow().symbol {
                    Grammer::Terminal(t) => match t {
                        Terminal::True => Boolean::True,
                        Terminal::False => Boolean::False,
                        _ => Boolean::Unknown,
                    },
                    _ => todo!(),
                },
                NonTerminal::UnOp => self.un_op_type(child_node),
                NonTerminal::BinOp => self.bin_op_type(child_node),
                _ => panic!("Should not get here")
            },
        }
    }

    fn bin_op_type(&self, node: &LNode) -> Boolean {
        let bin_type = &node.borrow().children[0]; // kind of bin op
        let sym = bin_type.borrow().symbol;

        let expr1 = &node.borrow().children[2]; // expr 1
        let expr2 = &node.borrow().children[4]; // expr 2

        let type1 = self.bool_val(expr1);
        let type2 = self.bool_val(expr2);

        if sym == Grammer::Terminal(Terminal::And) {
            if type1 == Boolean::False || type2 == Boolean::False {
                return Boolean::False;
            }
        } else if sym == Grammer::Terminal(Terminal::Or) {
            if type1 == Boolean::True || type2 == Boolean::True {
                return Boolean::True;
            }
        }

        Boolean::Unknown
    }

    fn un_op_type(&self, node: &LNode) -> Boolean {
        let bin_type = &node.borrow().children[0]; // kind of bin op
        let sym = bin_type.borrow().symbol;

        let expr = &node.borrow().children[2]; // expr

        match sym {
            Grammer::Terminal(t) => match t {
                Terminal::Not => {
                    let typ = self.bool_val(expr);
                    if typ == Boolean::True {
                        Boolean::False
                    } else if typ == Boolean::False {
                        Boolean::True
                    } else {
                        Boolean::Unknown
                    }
                },
                _ => panic!("bin_op_type should not get here terminal"), // should never get here
            },
            Grammer::NonTerminal(_) => panic!("bin_op_type should not get here nonterminal"), // should never get here
        }
    }

    fn size_of_algo(&self, algo: &str) -> usize {
        algo.matches('\n').count()*10
    }

    fn enter(&mut self, name: &str) {
        let child = self.scope.borrow().child_scope(name);
        if let Some(c) = child {
            self.scope = c;
        }
    }
    
    fn exit(&mut self) {
        let parent = Rc::clone(&self.scope.borrow().parent());
        self.scope = parent;
    }
}
