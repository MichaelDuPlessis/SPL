use std::{collections::LinkedList, cell::RefCell, rc::Rc, borrow::Borrow};
use crate::{token::Node, stack::Stack, grammer::{Grammer, Terminal, NonTerminal}};

pub struct ScopeAnalysis {
    head: Rc<RefCell<Node>>,
    ftable: ScopeNode,
    scope: usize,
}

impl ScopeAnalysis {
    pub fn new(head: Rc<RefCell<Node>>) -> Self {
        Self {
            head,
            ftable: Rc::new(RefCell::new(Default::default())),
            scope: 0,
        }
    }

    pub fn analysis(&mut self) {
        self.scope(Rc::clone(&self.head));
    }

    fn scope(&mut self, current: Rc<RefCell<Node>>) {
        let mut bindf = false;
        let mut bindv = false;

        for c in &RefCell::borrow(&current).children {
            if bindf {
                bindf = false;
                RefCell::borrow_mut(&self.ftable)
                .children.push(
                    Scope::new_node(
                        self.scope,
                        RefCell::borrow_mut(c).str_value.take().unwrap(),
                        Rc::clone(&self.ftable))
                );
            } else if bindv {
            } else {
                match RefCell::borrow(c).symbol {
                    Grammer::Terminal(t) => {
                        if t == Terminal::Proc {
                            bindf = true;
                            self.next_scope();
                        }
                    },
                    Grammer::NonTerminal(_) => (),
                }
            }

        }

        println!("{:?}", self.ftable);
    }

    fn next_scope(&mut self) {
        self.scope += 1;
    }
}

type ScopeNode = Rc<RefCell<Scope>>;

#[derive(Debug)]
struct Scope {
    scope_id: usize,
    name: String,
    parent: Option<ScopeNode>,
    children: Vec<ScopeNode>,
}

impl Scope {
    fn new(scope_id: usize, name: String, parent: ScopeNode) -> Self {
        let parent = Some(parent);

        Self {
            scope_id,
            name,
            parent,
            children: Vec::new()
        }
    }

    fn new_node(scope_id: usize, name: String, parent: ScopeNode) -> ScopeNode {
        Rc::new(RefCell::new(Scope::new(scope_id, name, parent)))
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            scope_id: 0,
            name: String::new(),
            parent: None,
            children: Vec::new()
        }
    }
}