use std::rc::Rc;

use crate::{token::LNode, scope::ScopeNode, grammer::{Terminal, NonTerminal, Grammer, Type, Boolean, Number}};

pub struct TypeChecker {
    scope: ScopeNode,
    ast: LNode,
    enter_scope: bool,
    exit_scope: bool,
    assign: bool,
}

impl TypeChecker {
    pub fn new(scope: ScopeNode, ast: LNode) -> Self {
        Self {
            scope,
            ast,
            enter_scope: false,
            exit_scope: false,
            assign: false,
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
                            self.exit_scope = false;
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
        let children = &node.borrow().children;
        let child1 = children[0].borrow();
        let child1 = child1.children[0].borrow();

        if child1.symbol == Grammer::Terminal(Terminal::UserDefined) {
            let name = child1.str_value.as_ref().unwrap();

            if let Some(s) = self.scope.borrow().exist_up(name) {
                if s.is_proc {
                    println!("Error: cannot assign to procedure {} at {}", name, children[0].borrow().pos.unwrap());
                    std::process::exit(1);
                }
            }

            let child2 = children[2].borrow();
            let child2 = child2.children[0].borrow();

            match child2.symbol {
                Grammer::Terminal(_) => (),
                Grammer::NonTerminal(nt) => match nt {
                    NonTerminal::Const => {
                        let con = &child2.children[0];
                        let typ;

                        match con.borrow().symbol {
                            Grammer::Terminal(t) => match t {
                                Terminal::Number => {
                                    if con.borrow().num_value.unwrap() < 0 {
                                        typ = Type::Number(Number::N);
                                    } else {
                                        typ = Type::Number(Number::NN);
                                    }
                                },
                                Terminal::ShortString => typ = Type::String,
                                Terminal::True => typ = Type::Boolean(Boolean::True),
                                Terminal::False => typ = Type::Boolean(Boolean::False),
                                _ => typ = Type::Unknown,
                            },
                            Grammer::NonTerminal(_) => typ = Type::Unknown,
                        }

                        self.scope.borrow_mut().add_type(name, typ);
                    },
                    _ => (),
                },
            }
        }
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