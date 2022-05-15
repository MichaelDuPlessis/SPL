use std::{cell::RefCell, rc::Rc, collections::HashMap};
use crate::token::LNode;

pub struct ScopeAnalysis {
    head: LNode,
    scope: ScopeNode,
    current_id: usize,
}

impl ScopeAnalysis {
    pub fn new(head: LNode) -> Self {
        Self {
            head,
            scope: Rc::new(RefCell::new(Default::default())),
            current_id: 0,
        }
    }

    pub fn scope(&mut self) {

    }

    fn analysis(&self, mut node: LNode) {
        let x = Rc::get_mut(&mut node);
    }

    fn enter(&mut self, scope: &ScopeNode) {
        let child = Rc::clone(scope);
        self.scope = child;
    }

    fn exit(&mut self) {
        let parent = Rc::clone(&self.scope.borrow().parent.as_ref().unwrap());
        self.scope = parent;
    }

    fn bind(&self, name: String, node: &LNode) {
        self.scope.borrow_mut().bind(name, Rc::clone(node));
    }

    fn lookup(&self, name: &str) -> Option<LNode> {
        self.scope.borrow().lookup(name)
    }

    fn new_id(&mut self) {
        self.current_id += 1;
    }
}

type ScopeNode = Rc<RefCell<Scope>>;

#[derive(Debug)]
struct Scope {
    scope_id:usize,
    vtable: HashMap<String, LNode>,
    parent: Option<ScopeNode>,
    children: Vec<ScopeNode>,
}

impl Scope {
    fn bind(&mut self, name: String, node: LNode) {
        self.vtable.insert(name, node);
    }

    fn lookup(&self, name: &str) -> Option<LNode> {
        match self.vtable.get(name) {
            Some(n) => Some(Rc::clone(n)),
            None => None,
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            scope_id: 0,
            vtable: HashMap::new(),
            parent: None,
            children: Vec::new(),
        }
    }
}