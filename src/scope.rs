use std::{cell::RefCell, rc::Rc, collections::HashMap, fmt::{Debug, Display}, default};
use crate::{token::{LNode, Type}, grammer::{Terminal, Grammer}};

pub struct ScopeAnalysis {
    head: LNode,
    scope: ScopeNode,
    current_scope: ScopeNode,
    current_id: usize,
    proc_found: bool,
    array_found: bool,
    return_found: bool,
    type_found: bool,
    call_found: bool,
}

impl ScopeAnalysis {
    pub fn new(head: LNode) -> Self {
        let scope = Rc::new(RefCell::new(Default::default()));
        Self {
            head,
            current_scope: Rc::clone(&scope),
            scope,
            current_id: 0,
            array_found: false,
            proc_found: false,
            return_found: false,
            type_found: false,
            call_found: false,
        }
    }

    pub fn scope(&mut self) -> ScopeNode {
        // builds the scope tree
        self.analysis(Rc::clone(&self.head));
        // println!("{:?}", self.scope);

        self.return_found = false;
        self.proc_found = false;

        // // checks the scope tree
        self.check_scope(Rc::clone(&self.head));

        Rc::clone(&self.current_scope)
    }

    fn check_scope(&mut self, node: LNode) {
        for c in &node.borrow().children {
            match c.borrow().symbol {
                Grammer::Terminal(t) => match t {
                    Terminal::UserDefined => {
                        let name = c.borrow();
                        let name = name.str_value.as_ref().unwrap();
                        let pos = c.borrow().pos.unwrap();

                        if self.proc_found {
                            self.enter();
                            self.proc_found = false;
                            continue;
                        }

                        if self.call_found {
                            if self.exist_up(name).is_none() {
                                println!("Error: proc call {} at {} is not defined.", name, pos);
                                std::process::exit(1);
                            }
                            self.call_found = false;
                            continue;
                        }

                        if let Some(si) = self.exist_up(name) {
                            if si.is_proc {
                                println!("Error: var {} at {} is not defined.", name, pos);
                                std::process::exit(1); 
                            }
                            continue;
                        }

                        // println!("{:?}", self.current_scope);
                        println!("Error: var {} at {} is not defined.", name, pos);
                        std::process::exit(1); 
                    },
                    Terminal::Call => self.call_found = true,
                    Terminal::Proc => self.proc_found = true,
                    Terminal::Return => self.return_found = true,
                    Terminal::RBrace => {
                        if !self.return_found { continue; }

                        self.return_found = false;
                        self.exit();
                    }
                    _ => ()
                },
                Grammer::NonTerminal(_) => self.check_scope(Rc::clone(c)),
            }
        }
    }

    fn analysis(&mut self, node: LNode) {
        for c in &node.borrow().children {
            let symbol = c.borrow().symbol;
            match symbol {
                Grammer::Terminal(t) => match t {
                    Terminal::Num | Terminal::Boolean | Terminal::String => self.type_found = true,
                    Terminal::Array => self.array_found = true,
                    Terminal::UserDefined => {
                        // name/key
                        let name = c.borrow().str_value.as_ref().unwrap().clone();

                        // adding data types
                        let mut node: ScopeInfo = Default::default();

                        if self.proc_found {
                            if let Some(si) = self.exist_up(&name) {
                                if si.is_proc {
                                    println!("Error: proc {} at {} already defined.", name, c.borrow().pos.unwrap());
                                    std::process::exit(1);
                                }
                            }

                            if let Some(si) = self.exist_down(&name) {
                                if si.is_proc {
                                    println!("Error: proc {} at {} already defined in same scope.", name, c.borrow().pos.unwrap());
                                    std::process::exit(1);
                                }
                            }

                            node.is_proc = true;
                            self.bind(name, node);

                            self.enter_new();
                        } else if self.type_found {
                            if self.contains(&name) {
                                println!("Error: var {} at {} already defined in.", name, c.borrow().pos.unwrap());
                                std::process::exit(1);
                            }

                            if self.array_found {
                                node.is_array = true;
                            }

                            self.bind(name, node);
                        }

                        self.array_found = false;
                        self.type_found = false;
                        self.proc_found = false;
                    },
                    Terminal::Proc => self.proc_found = true,
                    Terminal::Return => self.return_found = true,
                    Terminal::RBrace => {
                        if !self.return_found { continue; }

                        self.return_found = false;
                        self.exit();
                    }
                    _ => (),
                },
                Grammer::NonTerminal(_) => self.analysis(Rc::clone(c)),
            }
        }
    }

    fn enter(&mut self) {
        let pos = self.current_scope.borrow_mut().next_scope();
        let child = Rc::clone(&self.current_scope.borrow().children[pos]); // useless clone
        self.current_scope = child;
    }

    fn enter_new(&mut self) {
        let child = Scope::new(self.next_id(), Rc::clone(&self.current_scope));
        let child = Rc::new(RefCell::new(child));
        self.current_scope.borrow_mut().children.push(Rc::clone(&child));
        self.current_scope = child;
    }

    fn exit(&mut self) {
        let parent = Rc::clone(&self.current_scope.borrow().parent.as_ref().unwrap());
        self.current_scope = parent;
    }

    fn bind(&self, name: String, node: ScopeInfo) {
        self.current_scope.borrow_mut().bind(name, node);
    }

    fn lookup(&self, name: &str) -> Option<ScopeInfo> {
        self.scope.borrow().lookup(name)
    }

    fn next_id(&mut self) -> usize {
        self.current_id += 1;
        self.current_id
    }
    
    fn exist_down(&self, name: &str) -> Option<ScopeInfo> {
        self.current_scope.borrow().exist_down(name)
    }

    fn exist_up(&self, name: &str) -> Option<ScopeInfo> {
        self.current_scope.borrow().exist_up(name)
    }

    fn contains(&self, name: &str) -> bool {
        self.current_scope.borrow().contains(name)
    }
}

type ScopeNode = Rc<RefCell<Scope>>;

pub struct Scope {
    scope_id: usize,
    scope_pos: usize,
    vtable: HashMap<String, ScopeInfo>,
    parent: Option<ScopeNode>,
    children: Vec<ScopeNode>,
}

impl Scope {
    fn new(scope_id: usize, parent: ScopeNode) -> Self {
        let parent = Some(parent);
        Self {
            scope_id,
            parent,
            ..Default::default()
        }
    }

    fn bind(&mut self, name: String, node: ScopeInfo) {
        self.vtable.insert(name, node);
    }

    fn lookup(&self, name: &str) -> Option<ScopeInfo> {
        let mut node = self.vtable.get(name).map(|opt| *opt);

        if node.is_none() {
            node = if let Some(p) = &self.parent { p.borrow().lookup(name) } else { None }
        }

        node
    }

    fn exist_down(&self, name: &str) -> Option<ScopeInfo> {
        if self.vtable.contains_key(name) {
            return self.vtable.get(name).map(|opt| *opt);
        }

        for c in &self.children {
            if let Some(si) = c.borrow().exist_down(name) {
                return Some(si);
            }
        }

        None
    }

    fn exist_up(&self, name: &str) -> Option<ScopeInfo> {
        if self.vtable.contains_key(name) {
            return self.vtable.get(name).map(|opt| *opt);
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().exist_up(name);
        }

        None
    }

    fn contains(&self, name: &str) -> bool {
        self.vtable.contains_key(name)
    }

    fn next_scope(&mut self) -> usize {
        self.scope_pos += 1;
        self.scope_pos - 1
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            scope_id: 0,
            scope_pos: 0,
            vtable: HashMap::new(),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scope").field("scope_id", &self.scope_id).field("vtable", &self.vtable).field("children", &self.children).finish()
    }
}

#[derive(Debug, Clone, Copy)]
struct ScopeInfo {
    is_array: bool,
    is_proc: bool,
}

impl Default for ScopeInfo {
    fn default() -> Self {
        Self {
            is_array: false,
            is_proc: false,
        }
    }
}