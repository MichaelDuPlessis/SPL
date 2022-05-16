use std::{cell::RefCell, rc::Rc, collections::HashMap, fmt::{Debug, Display}};
use crate::{token::{LNode, Type}, grammer::{Terminal, Grammer}};

pub struct ScopeAnalysis {
    head: LNode,
    scope: ScopeNode,
    current_id: usize,
    data_type: Option<Terminal>,
    proc_found: bool,
    var_found: bool,
    array_found: bool,
    return_found: bool,
}

impl ScopeAnalysis {
    pub fn new(head: LNode) -> Self {
        Self {
            head,
            scope: Rc::new(RefCell::new(Default::default())),
            current_id: 0,
            data_type: None,
            var_found: false,
            array_found: false,
            proc_found: false,
            return_found: false,
        }
    }

    pub fn scope(&mut self) {
        // builds the scope tree
        self.analysis(Rc::clone(&self.head));

        println!("{:?}", self.scope);
        // self.return_found = false;
        // self.proc_found = false;

        // // checks the scope tree
        // self.check_scope(Rc::clone(&self.head));

    }

    fn check_scope(&mut self, node: LNode) {
        for c in &node.borrow().children {
            if self.proc_found {
                let mut new_scope = None;
                for s in &self.scope.borrow().children {
                    if s.borrow().lookup(c.borrow().str_value.as_ref().unwrap()).is_some() {
                        new_scope = Some(Rc::clone(s));
                        break;
                    }
                }

                if let Some(ns) = new_scope {
                    self.enter(ns);
                }

                self.proc_found = false;
            }

            match &c.borrow().symbol {
                Grammer::Terminal(t) => match t {
                    Terminal::UserDefined => {
                        let node = c.borrow();
                        let name = node.str_value.as_ref().unwrap();
                        let pos = node.pos.unwrap();
                        if self.lookup(name).is_none() {
                            println!("Token {} at {} not found in scope or parent scope.", name, pos);
                            std::process::exit(1);
                        }
                    },
                    Terminal::Proc => self.proc_found = true,
                    Terminal::Return => {
                        // self.exit();
                        self.return_found = true;
                        // return;
                    },
                    Terminal::RBrace => {
                        if !self.return_found { continue; }

                        self.return_found = false;
                        self.exit();
                    }
                    _ => ()
                },
                Grammer::NonTerminal(_) =>  {
                    self.check_scope(Rc::clone(c));
                },        
            }
        }
    }

    fn analysis(&mut self, node: LNode) {
        for c in &node.borrow().children {
            let symbol = c.borrow().symbol;
            match symbol {
                Grammer::Terminal(t) => match t {
                    Terminal::Num | Terminal::Boolean | Terminal::String => {
                        self.data_type = t;
                        self.var_found = true;
                    },
                    Terminal::Array => self.array_found = true,
                    Terminal::UserDefined => {
                        if self.var_found {
                            // name/key
                            let name = c.borrow().str_value.as_ref().unwrap().clone();
    
                            // adding data types
                            let node = Rc::clone(c);
                            node.borrow_mut().data_type = self.data_type;
                            node.borrow_mut().is_array = self.array_found;

                            self.vbind(name, node);
    
                            self.var_found = false;
                            self.array_found = false;
                        } else if self.proc_found {
                            // name/key
                            let name = c.borrow().str_value.as_ref().unwrap().clone();

                            if self.exists_down(&name) || self.exists_up(&name) {
                                println!("Redeclaration of {} at {}", name, c.borrow().pos.unwrap());
                                std::process::exit(1);
                            }

                            let node = Rc::clone(c);

                            self.enter_new();
                            self.fbind(name, node);
    
                            self.proc_found = false;
                        }
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

    fn enter(&mut self, scope: ScopeNode) {
        self.scope.borrow_mut().children.push(Rc::clone(&scope));
        self.scope = scope;
    }

    fn enter_new(&mut self) {
        self.new_id();
        let child = Scope::new(self.current_id, Rc::clone(&self.scope));
        let child = Rc::new(RefCell::new(child));
        self.scope.borrow_mut().children.push(Rc::clone(&child));
        self.scope = child;
    }

    fn exit(&mut self) {
        let parent = Rc::clone(&self.scope.borrow().parent.as_ref().unwrap());
        self.scope = parent;
    }

    fn vbind(&self, name: String, node: LNode) {
        self.scope.borrow_mut().vbind(name, node);
    }

    fn fbind(&self, name: String, node: LNode) {
        self.scope.borrow_mut().fbind(name, node);
    }

    fn lookup(&self, name: &str) -> Option<LNode> {
        self.scope.borrow().lookup(name)
    }

    fn exists_down(&self, name: &str) -> bool {
        self.scope.borrow().exists_down(name)
    }

    fn exists_up(&self, name: &str) -> bool {
        self.scope.borrow().exists_up(name)
    }

    fn new_id(&mut self) {
        self.current_id += 1;
    }
}

type ScopeNode = Rc<RefCell<Scope>>;

struct Scope {
    scope_id: usize,
    vtable: HashMap<String, LNode>,
    ftable: HashMap<String, LNode>,
    parent: Option<ScopeNode>,
    children: Vec<ScopeNode>,
}

impl Scope {
    fn new(scope_id: usize, parent: ScopeNode) -> Self {
        let parent = Some(parent);
        Self {
            parent,
            scope_id,
            ..Default::default()
        }
    }

    fn vbind(&mut self, name: String, node: LNode) {
        self.vtable.insert(name, node);
    }

    fn fbind(&mut self, name: String, node: LNode) {
        self.ftable.insert(name, node);
    }

    fn lookup(&self, name: &str) -> Option<LNode> {
        let func = |opt| Rc::clone(opt);

        let mut node = self.vtable.get(name).map(func);
        
        if node.is_none() {
            node = self.ftable.get(name).map(func);
        }

        if node.is_none() {
            node = if let Some(p) = &self.parent { p.borrow().lookup(name) } else { None }
        }

        node
    }

    fn exists_down(&self, name: &str) -> bool {
        self.ftable.contains_key(name) || self.children.iter().fold(false, |acc, x| acc || x.borrow().exists_down(name))
    }

    fn exists_up(&self, name: &str) -> bool {
        if self.ftable.contains_key(name) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().exists_up(name);
        }

        return false;
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            scope_id: 0,
            vtable: HashMap::new(),
            ftable: HashMap::new(),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scope").field("scope_id", &self.scope_id).field("vtable", &self.vtable).field("ftable", &self.ftable).field("children", &self.children).finish()
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "scope_id: {}, vtable: {:?}", self.scope_id, self.vtable)?;
        for s in &self.children {
            write!(f, "{}", s.borrow())?;
        }

        Ok(())
    }
}