use std::rc::Rc;
use crate::{token::LNode, scope::ScopeNode, grammer::{Terminal, NonTerminal, Grammer, Type, Boolean, Number}, error::error};

pub struct TypeChecker {
    scope: ScopeNode,
    ast: LNode,
    current_call: String,
    input_found: bool,
}

impl TypeChecker {
    pub fn new(scope: ScopeNode, ast: LNode) -> Self {
        Self {
            scope,
            ast,
            current_call: String::new(),
            input_found: false,
        }
    }

    pub fn type_check(&mut self) -> ScopeNode {
        // Do anaylsis there and if encounter call, search AST to find it and do analysis there
        // ignore any procdefs and only analysis when matching call found
        
        self.analysis(Rc::clone(&self.ast));
        
        // after anaylsis check to make sure each variable is defined
        self.check_defined(Rc::clone(&self.scope));
        
        println!("{:?}", self.scope);

        return Rc::clone(&self.scope);
    }

    // if function completes tahn all vars defined
    fn check_defined(&self, node: ScopeNode) {
        let node = node.borrow();
        
        for (name, si) in &node.vtable {
            if !si.is_defined && !si.is_proc {
                error(&format!("{} in scope {} is never assigned to.", name, node.scope_id))
            }
        }

        for c in &node.children {
            self.check_defined(Rc::clone(c));
        }
    }

    fn analysis(&mut self, node: LNode) {
        let mut skip_if = false;
        let mut skip_else = false;
        let mut skip_while = false;
        let current_call = self.current_call.clone();

        for (i, c) in node.borrow().children.iter().enumerate() {
            let grammer = c.borrow().symbol;
            if skip_if {
                if grammer == Grammer::NonTerminal(NonTerminal::Alternat) {
                    skip_if = false;
                    self.analysis(Rc::clone(c));
                }
                continue;
            }

            if skip_while {
                if grammer == Grammer::Terminal(Terminal::RBrace) {
                    skip_while = false;
                    self.analysis(Rc::clone(c));
                }
                continue;
            }

            match grammer {
                Grammer::Terminal(t) => match t {
                    Terminal::Call => {
                        let name = &node.borrow().children[i + 1];

                        let name = name.borrow();
                        let name = name.str_value.as_ref().unwrap();

                        if current_call == *name {
                            return;
                        }
                        self.current_call = String::from(name);

                        self.enter(name);

                        let scope = self.scope.borrow().exist_down(name);
                        if let Some(si) = scope {
                            let node_id = si.node_id;

                            let proc_node = self.ast.borrow().find(node_id, &self.ast);
                            if let Some(proc) = proc_node {
                                self.analysis(Rc::clone(&proc));
                            } else {
                                panic!("don't think it should get here");
                            }
                        } else {
                            let scope = self.scope.borrow().exist_proc(name);
                            if let Some(si) = scope {
                                let node_id = si.node_id;
    
                                let proc_node = self.ast.borrow().find(node_id, &self.ast);
                                if let Some(proc) = proc_node {
                                    self.analysis(Rc::clone(&proc));
                                } else {
                                    panic!("don't think it should get here");
                                }
                            } else {
                                error(&format!("Call to {} not in scope", name));
                            }
                        }

                        self.exit();
                    }
                    // conditionals
                    Terminal::If => {
                        let typ = self.expr_type(&node.borrow().children[2]);
                        if let Type::Boolean(typ) = typ {
                            if typ == Boolean::True {
                                skip_else = true;
                            } else if typ == Boolean::False {
                                skip_if = true;
                            } // else do nothing evaluate both
                        } else {
                            error("Type of expression in if must evaluate to bool.")
                        }
                    },
                    Terminal::While => {
                        let typ = self.expr_type(&node.borrow().children[2]);
                        if let Type::Boolean(typ) = typ {
                            if typ == Boolean::True {
                                skip_while = false;
                            } else if typ == Boolean::False {
                                skip_while = true;
                            } // else do nothing evaluate both
                        } else {
                            error("Type of expression in while must evaluate to bool.")
                        }
                    },
                    Terminal::Until => {
                        let typ = self.expr_type(&node.borrow().children[6]);
                        if typ != Type::Boolean(Boolean::True) {
                            error("Type of expression in while must evaluate to bool.")
                        }
                    },
                    _ => (),
                },
                Grammer::NonTerminal(nt) => match nt {
                    NonTerminal::Assign => self.check_assign(c),
                    NonTerminal::ProcDefs => continue, // if encounter procdefs leave
                    NonTerminal::Alternat => if !skip_else {
                        self.analysis(Rc::clone(c));
                    } else {
                        skip_else = false;
                    },
                    _ => self.analysis(Rc::clone(c)),
                },
            }
        }
    }

    fn check_assign(&self, node: &LNode) {
        let children = &node.borrow().children; // children of assign
        let lhs = children[0].borrow();
        let child1 = lhs.children[0].borrow(); // var to assign to

        if child1.symbol == Grammer::Terminal(Terminal::UserDefined) {
            let name = child1.str_value.as_ref().unwrap();

            let typ = self.expr_type(&children[2]);            

            // if array check paramter valid
            // change to use created functions at some point
            let var_field = &lhs.children[1].borrow().children;
            if !var_field.is_empty() {
                let indexer = &var_field[1];
                let t = self.expr_type(indexer);
                let symbol = indexer.borrow().children[0].borrow().symbol;

                if symbol == Grammer::NonTerminal(NonTerminal::Var) {
                    if let Type::Number(num) = t {
                        if num != Number::N {
                            error(&format!("Type of var indexer must be Number"));
                        }
                    } else {
                        error(&format!("Type of var indexer must be Number"));
                    }
                } else if symbol == Grammer::NonTerminal(NonTerminal::Const) {
                    if let Type::Number(num) = t {
                        if num != Number::NN {
                            error(&format!("Type of const indexer must be non-negative Number"));
                        }
                    } else {
                        error(&format!("Type of const indexer must be non-negative Number"));
                    }
                }

                self.scope.borrow_mut().add_type(name, typ, true);
            } else {
                self.scope.borrow_mut().add_type(name, typ, false);
            }
        } else if child1.symbol == Grammer::Terminal(Terminal::Out) {
            let typ = self.expr_type(&children[2]);

            match typ {
                Type::Number(_) => (),
                Type::String => (),
                Type::Mixed => (),
                Type::Boolean(_) => error("Output cannot be of type boolean"),
                Type::Unknown => error("Output cannot be of type Unknown"),
            }
        }
    }

    fn expr_type(&self, node: &LNode) -> Type {
        let kind = &node.borrow().children[0];
        let sym = kind.borrow().symbol;

        match sym {
            Grammer::Terminal(t) => match t {
                Terminal::UserDefined => self.udn_type(node),
                _ => panic!("Should never get here expr_type terminal"),
            }, 
            Grammer::NonTerminal(nt) => match nt {
                NonTerminal::Var => self.udn_type(kind),
                NonTerminal::Const => self.const_type(kind),
                NonTerminal::UnOp => self.un_op_type(kind),
                NonTerminal::BinOp => self.bin_op_type(kind),
                _ => Type::Unknown,
            },
        }
    }

    fn check_arr_valid(&self, node: &LNode) {
        let indexer = &node;
        let t = self.expr_type(indexer);
        let symbol = indexer.borrow().children[0].borrow().symbol;

        if symbol == Grammer::NonTerminal(NonTerminal::Var) {
            if let Type::Number(num) = t {
                if num != Number::N {
                    error(&format!("Type of var indexer must be Number"));
                }
            } else {
                error(&format!("Type of var indexer must be Number"));
            }
        } else if symbol == Grammer::NonTerminal(NonTerminal::Const) {
            if let Type::Number(num) = t {
                if num != Number::NN {
                    error(&format!("Type of const indexer must be non-negative Number"));
                }
            } else {
                error(&format!("Type of const indexer must be non-negative Number"));
            }
        }
    }

    fn array(&self, node: &LNode) -> Option<LNode> {
        if node.borrow().children.is_empty() {
            None
        } else {
            Some(Rc::clone(&node.borrow().children[1]))
        }
    }

    fn udn_type(&self, node: &LNode) -> Type {
        let child = &node.borrow();

        let var = if node.borrow().children.is_empty() {
            node
        } else {
            &child.children[0]
        };

        let name = var.borrow();
        let name = name.str_value.as_ref().unwrap();

        let scope_info = self.scope.borrow();
        let scope_info = if node.borrow().children.len() > 1 {
            if node.borrow().children[1].borrow().children.is_empty() {
                scope_info.exist_up(name, false)
            } else {
                if let Some(n) = self.array(&node.borrow().children[1]) {
                    self.check_arr_valid(&n);
                    scope_info.exist_up(name, true)
                } else {
                    error(&format!("No index provided for array {} at {}", name, var.borrow().pos.unwrap()));
                    None
                }
            }
        } else {
            scope_info.exist_up(name, false)
        };

        if let Some(si) = scope_info {
            if !si.is_defined {
                error(&format!("{} has not been assigned to.", name));
            }

            si.data_type
        } else {
            panic!("Should never get here udn_type");
        }
    }

    fn un_op_type(&self, node: &LNode) -> Type {
        let bin_type = &node.borrow().children[0]; // kind of bin op
        let sym = bin_type.borrow().symbol;

        let expr = &node.borrow().children[2]; // expr

        match sym {
            Grammer::Terminal(t) => match t {
                Terminal::Not => {
                    let typ = self.expr_type(expr);
                    match typ {
                        Type::Boolean(_) => Type::Boolean(Boolean::Unknown),
                        _ => Self::bad_type(typ),
                }
                },
                Terminal::Input => {
                    let child = &expr.borrow().children[0];

                    let name = child.borrow();
                    let name = name.str_value.as_ref().unwrap();

                    let scope_info = self.scope.borrow();
                    scope_info.exist_up(name, false);
                    drop(scope_info);
                    self.scope.borrow_mut().add_type(name, Type::Mixed, false);

                    Type::Mixed
                },
                _ => panic!("bin_op_type should not get here terminal"), // should never get here
            },
            Grammer::NonTerminal(_) => panic!("bin_op_type should not get here nonterminal"), // should never get here
        }
    }

    // pass in binop to get type
    fn bin_op_type(&self, node: &LNode) -> Type {
        let bin_type = &node.borrow().children[0]; // kind of bin op
        let sym = bin_type.borrow().symbol;

        let expr1 = &node.borrow().children[2]; // expr 1
        let expr2 = &node.borrow().children[4]; // expr 2

        let type1 = self.expr_type(expr1);
        let type2 = self.expr_type(expr2);

        match sym {
            Grammer::Terminal(t) => match t {
                // logical operators
                Terminal::And => match (type1, type2) {
                    (Type::Boolean(Boolean::True), Type::Boolean(Boolean::True)) => Type::Boolean(Boolean::True),
                    (Type::Boolean(_), Type::Boolean(_)) => Type::Boolean(Boolean::False),
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Or => match (type1, type2) {
                    (Type::Boolean(Boolean::False), Type::Boolean(Boolean::False)) => Type::Boolean(Boolean::False),
                    (Type::Boolean(_), Type::Boolean(_)) => Type::Boolean(Boolean::True),
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Equal => match (type1, type2) {
                    (Type::Boolean(x), Type::Boolean(y)) => if x == y { Type::Boolean(Boolean::True) } else { Type::Boolean(Boolean::Unknown) },
                    (Type::String, Type::String) => Type::Boolean(Boolean::Unknown),
                    (Type::Number(_), Type::Number(_)) => Type::Boolean(Boolean::Unknown),
                    _ => Type::Boolean(Boolean::False),
                },
                Terminal::Add => match (type1, type2) {
                    (Type::Number(_), Type::Number(_)) => Type::Number(Number::N),
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Mult => match (type1, type2) {
                    (Type::Number(_), Type::Number(_)) => Type::Number(Number::N),
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Sub => match (type1, type2) {
                    (Type::Number(_), Type::Number(_)) => Type::Number(Number::N),
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Larger => match (type1, type2) {
                    (Type::Number(_), Type::Number(_)) => Type::Number(Number::N),
                    _ => Self::incompatible(type1, type2),
                },
                _ => panic!("bin_op_type should not get here terminal"),
            },
            Grammer::NonTerminal(_) => panic!("bin_op_type should not get here nonterminal"), // should never get here
        }
    }

    fn incompatible(t1: Type, t2: Type) -> Type {
        error(&format!("Incompatible types {} and {}", t1, t2));
        Type::Unknown
    }

    fn bad_type(t: Type) -> Type {
        error(&format!("Bad type {} on unary operator", t));
        Type::Unknown
    }

    // pass in const to get type
    fn const_type(&self, node: &LNode) -> Type {
        let con = &node.borrow().children[0];

        let typ = match con.borrow().symbol {
            Grammer::Terminal(t) => match t {
                Terminal::Number => {
                    let num = con.borrow().num_value.unwrap();
                    if num < 0 {
                        Type::Number(Number::N)
                    } else {
                        Type::Number(Number::NN)
                    }
                },
                Terminal::ShortString => Type::String,
                Terminal::True => Type::Boolean(Boolean::True),
                Terminal::False => Type::Boolean(Boolean::False),
                _ => Type::Unknown,
            },
            Grammer::NonTerminal(_) => Type::Unknown,
        };

        typ
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