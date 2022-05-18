use std::rc::Rc;

use crate::{token::LNode, scope::ScopeNode, grammer::{Terminal, NonTerminal, Grammer, Type, Boolean, Number}};

pub struct TypeChecker {
    scope: ScopeNode,
    ast: LNode,
    enter_scope: bool,
    exit_scope: bool,
}

impl TypeChecker {
    pub fn new(scope: ScopeNode, ast: LNode) -> Self {
        Self {
            scope,
            ast,
            enter_scope: false,
            exit_scope: false,
        }
    }

    pub fn type_check(&mut self) {
        self.analysis(Rc::clone(&self.ast));
        println!("{:?}", self.scope);
    }

    fn analysis(&mut self, node: LNode) {
        for c in &node.borrow().children {
            let grammer = c.borrow().symbol;
            match grammer {
                Grammer::Terminal(t) => match t {
                    Terminal::Proc => self.enter_scope = true,
                    Terminal::UserDefined => {
                        if self.enter_scope {
                            self.enter_scope = false;
                            self.enter(c.borrow().str_value.as_ref().unwrap());
                        }
                    }
                    Terminal::Return => self.exit_scope = true,
                    Terminal::RBrace => {
                        if self.exit_scope {
                            self.exit_scope = false;
                            self.exit();
                        }
                    }
                    _ => (),
                },
                Grammer::NonTerminal(nt) => match nt {
                    NonTerminal::Assign => self.check_assign(c),
                    _ => self.analysis(Rc::clone(c)),
                },
            }
        }
    }

    fn check_assign(&self, node: &LNode) {
        let children = &node.borrow().children; // children of assign
        let child1 = children[0].borrow();
        let child1 = child1.children[0].borrow(); // var to assign to

        if child1.symbol == Grammer::Terminal(Terminal::UserDefined) {
            let name = child1.str_value.as_ref().unwrap();

            if let Some(s) = self.scope.borrow().exist_up(name) {
                if s.is_proc {
                    println!("Error: cannot assign to procedure {} at {}", name, children[0].borrow().pos.unwrap());
                    std::process::exit(1);
                }
            }

            let typ = self.expr_type(&children[2]);
            self.scope.borrow_mut().add_type(name, typ);
        }
    }

    fn expr_type(&self, node: &LNode) -> Type {
        let kind = &node.borrow().children[0];
        let sym = kind.borrow().symbol;

        match sym {
            Grammer::Terminal(t) => match t {
                Terminal::UserDefined => self.udn_type(kind),
                _ => panic!("Should never get here expr_type terminal"),
            }, 
            Grammer::NonTerminal(nt) => match nt {
                NonTerminal::Const => self.const_type(kind),
                NonTerminal::UnOp => self.un_op_type(kind),
                NonTerminal::BinOp => self.bin_op_type(kind),
                _ => Type::Unknown,
            },
        }
    }

    fn udn_type(&self, node: &LNode) -> Type {
        let name = node.borrow();
        let name = name.str_value.as_ref().unwrap();

        let scope_info = self.scope.borrow();
        let scope_info = scope_info.exist_up(name);

        if let Some(si) = scope_info {
            si.data_type
        } else {
            panic!("Should never get here udn_type");
        }
    }

    fn un_op_type(&self, node: &LNode) -> Type {
        let bin_type = &node.borrow().children[0]; // kind of bin op
        let sym = bin_type.borrow().symbol;

        let expr = &node.borrow().children[2]; // expr

        let typ = self.expr_type(expr);

        match sym {
            Grammer::Terminal(t) => match t {
                Terminal::Not => match typ {
                    Type::Boolean(_) => Type::Boolean(Boolean::Unknown),
                    _ => Self::bad_type(typ),
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
                Terminal::And | Terminal::Or => match (type1, type2) {
                    (Type::Boolean(_), Type::Boolean(_)) => Type::Boolean(Boolean::Unknown),
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Equal => match (type1, type2) {
                    (Type::Boolean(_), Type::Boolean(_)) => Type::Boolean(Boolean::Unknown),
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
                _ => panic!("bin_op_type should not get here terminal"),
            },
            Grammer::NonTerminal(_) => panic!("bin_op_type should not get here nonterminal"), // should never get here
        }
    }

    fn incompatible(t1: Type, t2: Type) -> Type {
        println!("Error: incompatible types {} and {}", t1, t2);
        std::process::exit(1);
    }

    fn bad_type(t: Type) -> Type {
        println!("Error: bade type {} on unary operator", t);
        std::process::exit(1);
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
                Terminal::True => Type::Boolean(Boolean::Unknown),
                Terminal::False => Type::Boolean(Boolean::Unknown),
                _ => Type::Unknown,
            },
            Grammer::NonTerminal(_) => Type::Unknown,
        };

        typ
    }

    fn enter(&mut self, name: &str) {
        let child = self.scope.borrow().child_scope(name);
        self.scope = child;
    }
    
    fn exit(&mut self) {
        let parent = Rc::clone(&self.scope.borrow().parent());
        self.scope = parent;
    }
}