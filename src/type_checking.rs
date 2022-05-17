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

            self.scope.borrow_mut().add_type(name, self.expr_type(&children[2]));
        }
    }

    fn expr_type(&self, node: &LNode) -> Type {
        let kind = &node.borrow().children[0];
        let sym = kind.borrow().symbol;

        match sym {
            Grammer::Terminal(_) => Type::Unknown, // for array indexing do later
            Grammer::NonTerminal(nt) => match nt {
                NonTerminal::Const => self.const_type(kind),
                NonTerminal::BinOp => self.bin_op_type(kind),
                _ => Type::Unknown,
            },
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
                    Type::Boolean(b) => match b {
                        Some(b) => Type::Boolean(Some(!b)),
                        None => Type::Boolean(None),
                    }
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
                Terminal::And => match (type1, type2) {
                    (Type::Boolean(bool1), Type::Boolean(bool2)) => match (bool1, bool2) {
                        (Some(bool1), Some(bool2)) => Type::Boolean(Some(bool1 && bool2)),
                        _ => Type::Boolean(None), // if unkown, i.e. one is a uDN from user input
                    },
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Or => match (type1, type2) {
                    (Type::Boolean(bool1), Type::Boolean(bool2)) => match (bool1, bool2) {
                        (Some(bool1), Some(bool2)) => Type::Boolean(Some(bool1 || bool2)),
                        _ => Type::Boolean(None), // if unkown, i.e. one is a uDN from user input
                    },
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Equal => match (type1, type2) {
                    (Type::Boolean(bool1), Type::Boolean(bool2)) => match (bool1, bool2) {
                        (Some(bool1), Some(bool2)) => Type::Boolean(Some(bool1 == bool2)),
                        _ => Type::Boolean(None), // if unkown, i.e. one is a uDN from user input
                    },
                    _ => Type::Boolean(Some(false)),
                },
                Terminal::Add => match (type1, type2) {
                    (Type::Number(num1), Type::Number(num2)) => match (num1, num2) {
                        (Some((_, num1)), Some((_, num2))) => {
                            let num = num1 + num2;
                            if num >= 0 {
                                Type::Number(Some((Number::NN, num)))
                            } else {
                                Type::Number(Some((Number::N, num)))
                            }
                        },
                        _ => Type::Number(None),
                    },
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Mult => match (type1, type2) {
                    (Type::Number(num1), Type::Number(num2)) => match (num1, num2) {
                        (Some((_, num1)), Some((_, num2))) => {
                            let num = num1 * num2;
                            if num >= 0 {
                                Type::Number(Some((Number::NN, num)))
                            } else {
                                Type::Number(Some((Number::N, num)))
                            }
                        },
                        _ => Type::Number(None),
                    },
                    _ => Self::incompatible(type1, type2),
                },
                Terminal::Sub => match (type1, type2) {
                    (Type::Number(num1), Type::Number(num2)) => match (num1, num2) {
                        (Some((_, num1)), Some((_, num2))) => {
                            let num = num1 - num2;
                            if num >= 0 {
                                Type::Number(Some((Number::NN, num)))
                            } else {
                                Type::Number(Some((Number::N, num)))
                            }
                        },
                        _ => Type::Number(None),
                    },
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
                        Type::Number(Some((Number::N, num)))
                    } else {
                        Type::Number(Some((Number::NN, num)))
                    }
                },
                Terminal::ShortString => Type::String,
                Terminal::True => Type::Boolean(Some(true)),
                Terminal::False => Type::Boolean(Some(false)),
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