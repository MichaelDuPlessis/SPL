use std::{cell::RefCell, rc::Rc, collections::{HashMap, LinkedList}, fmt::{Debug}};
use crate::{token::{LNode}, grammer::{Terminal, Grammer, Type, Number, Boolean}};

pub struct ScopeAnalysis {
    head: LNode,
    scope: ScopeNode,
    current_scope: ScopeNode,
    current_id: usize,
    proc_found: bool,
    array_found: bool,
    return_found: bool,
    type_found: Option<Type>,
    call_found: bool,
    halt_found: bool,
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
            type_found: None,
            call_found: false,
            halt_found: false,
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

        if !self.all_used() {
            println!("Not all variables and procedures used");
            std::process::exit(1);
        }

        Rc::clone(&self.current_scope)
    }

    fn check_scope(&mut self, node: LNode) {
        for (i, c) in node.borrow().children.iter().enumerate() {
            match c.borrow().symbol {
                Grammer::Terminal(t) => match t {
                    Terminal::UserDefined => {
                        let name = c.borrow();
                        let name = name.str_value.as_ref().unwrap();
                        let pos = c.borrow().pos.unwrap();

                        if self.proc_found {
                            self.enter(name);
                            self.proc_found = false;
                            continue;
                        }

                        if self.call_found {
                            if self.exist_up(name).is_none() {
                                println!("Error: proc call {} at {} is not defined.", name, pos);
                                std::process::exit(1);
                            }

                            self.used(name, true, false);
                            self.call_found = false;
                            continue;
                        }

                        if let Some(si) = self.exist_up(name) {
                            if si.is_proc {
                                println!("Error: var {} at {} is not defined.", name, pos);
                                std::process::exit(1); 
                            }
                            if !self.return_found && !self.halt_found {
                                if node.borrow().children[i+1].borrow().children.is_empty() {
                                    self.used(name, false, false);
                                } else {
                                    self.used(name, false, true);
                                }
                            }

                            continue;
                        }

                        println!("Error: var {} at {} is not defined.", name, pos);
                        std::process::exit(1); 
                    },
                    Terminal::Call => self.call_found = true,
                    Terminal::Proc => self.proc_found = true,
                    Terminal::Return => self.return_found = true,
                    Terminal::Halt => self.halt_found = true,
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
                    Terminal::Num | Terminal::Boolean | Terminal::String => {
                        if t == Terminal::Num {
                            self.type_found = Some(Type::Number(Number::NN));
                        } else if t == Terminal::Boolean {
                            self.type_found = Some(Type::Boolean(Boolean::Unknown));
                        } else {
                            self.type_found = Some(Type::String);
                        }
                    },
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
                            self.bind(name.clone(), node);

                            self.enter_new(name);
                        } else if let Some(typ) = self.type_found {
                            if let Some(si) = self.find_in_scope(&name) {
                                if self.array_found == si.is_array {
                                    println!("Error: var {} at {} already defined.", name, c.borrow().pos.unwrap());
                                    std::process::exit(1);
                                }
                            }

                            node.is_array = self.array_found;

                            node.data_type = typ;

                            self.bind(name, node);
                        }

                        self.array_found = false;
                        self.type_found = None;
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

    fn enter(&mut self, name: &str) {
        let pos = self.current_scope.borrow().scope_pos(name);
        let child = Rc::clone(&self.current_scope.borrow().children[pos]); // useless clone
        self.current_scope = child;
    }

    fn enter_new(&mut self, name: String) {
        let child = Scope::new(self.next_id(), Rc::clone(&self.current_scope));
        let child = Rc::new(RefCell::new(child));
        RefCell::borrow_mut(&self.current_scope).add_scope(Rc::clone(&child), name);
        self.current_scope = child;
    }

    fn exit(&mut self) {
        let parent = Rc::clone(&self.current_scope.borrow().parent.as_ref().unwrap());
        self.current_scope = parent;
    }

    fn bind(&self, name: String, node: ScopeInfo) {
        RefCell::borrow_mut(&self.current_scope).bind(name, node);
    }

    // fn lookup(&self, name: &str) -> Option<ScopeInfo> {
    //     self.scope.borrow().lookup(name)
    // }

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

    fn find_in_scope(&self, name: &str) -> Option<ScopeInfo> {
        self.current_scope.borrow().find_in_scope(name)
    }

    fn used(&self, name: &str, call: bool, arr: bool) {
        self.current_scope.borrow_mut().used(name, call, arr);
    }

    fn all_used(&self) -> bool {
        self.scope.borrow().all_used()
    }
}

pub type ScopeNode = Rc<RefCell<Scope>>;

pub struct Scope {
    scope_id: usize,
    scope_map: HashMap<String, usize>,
    vtable: LinkedList<(String, ScopeInfo)>,
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
        self.vtable.push_front((name, node));
    }

    // fn lookup(&self, name: &str) -> Option<ScopeInfo> {
    //     // let mut node = self.vtable.get(name).map(|opt| *opt);
    //     let mut node = self.vtable.iter().find(|i| i.0 == name);

    //     if node.is_none() {
    //         node = if let Some(p) = &self.parent { p.borrow().lookup(name) } else { None }
    //     }

    //     node
    // }

    fn exist_down(&self, name: &str) -> Option<ScopeInfo> {
        if let Some(si) = self.vtable.iter().find(|i| i.0 == name) {
            return Some(si.1);
        }

        for c in &self.children {
            if let Some(si) = c.borrow().exist_down(name) {
                return Some(si);
            }
        }

        None
    }

    pub fn exist_up(&self, name: &str) -> Option<ScopeInfo> {
        if let Some(si) = self.vtable.iter().find(|i| i.0 == name) {
            return Some(si.1);
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().exist_up(name);
        }

        None
    }

    fn find_in_scope(&self, name: &str) -> Option<ScopeInfo> {
        if let Some(si) = self.vtable.iter().find(|i| i.0 == name) {
            Some(si.1)
        } else {
            None
        }
    }

    fn scope_pos(&self, name: &str) -> usize {
        *self.scope_map.get(name).unwrap()
    }

    pub fn child_scope(&self, name: &str) -> ScopeNode {
        let pos = *self.scope_map.get(name).unwrap();
        Rc::clone(&self.children[pos])
    }

    fn add_scope(&mut self, child: ScopeNode, name: String) {
        self.scope_map.insert(name, self.children.len());
        self.children.push(Rc::clone(&child));
    }

    pub fn parent(&self) -> ScopeNode {
        Rc::clone(self.parent.as_ref().unwrap())
    }

    fn used(&mut self, name: &str, call: bool, arr: bool) {
        if let Some((_, si)) = self.vtable.iter_mut().find(|i|
            i.0 == name && i.1.is_proc == call && i.1.is_array == arr) {
            si.is_used = true;
            return;
        }

        if let Some(parent) = &self.parent {
            parent.borrow_mut().used(name, call, arr);
        }
    }

    fn all_used(&self) -> bool {
        for (_, si) in &self.vtable {
            if !si.is_used {
                return false;
            }
        }

        for c in &self.children {
            if !c.borrow().all_used() {
                return false;
            }
        }

        return true;
    }

    pub fn add_type(&mut self, name: &str, t: Type) {
        // check if type does not conflict
        if let Some((_, si)) = self.vtable.iter_mut().find(|i| i.0 == name) {
            if si.data_type == Type::Unknown {
                si.is_defined = false;
            } else if si.data_type == t {
                si.is_defined = true;
            }

            if si.data_type != t {
                println!("Error: Cannot assign {} to {}", t, si.data_type);
                std::process::exit(1);
            }

            si.data_type = t;
            return;
        }

        let mut curr = Rc::clone(self.parent.as_ref().unwrap());
        loop {
            if let Some((_, si)) = RefCell::borrow_mut(&curr).vtable.iter_mut().find(|i| i.0 == name) {
                si.data_type = t;
                si.is_defined = true;
                break;
            }

            let parent = Rc::clone(curr.borrow().parent.as_ref().unwrap());
            curr = parent;
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            scope_id: 0,
            scope_map: HashMap::new(),
            vtable: LinkedList::new(),
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
pub struct ScopeInfo {
    pub data_type: Type,
    pub is_array: bool,
    pub is_proc: bool,
    pub is_defined: bool,
    pub is_used: bool,
}

impl ScopeInfo {
    fn new(data_type: Type) -> Self {
        Self {
            data_type,
            ..Default::default()
        }
    }
}

impl Default for ScopeInfo {
    fn default() -> Self {
        Self {
            data_type: Type::Unknown,
            is_array: false,
            is_proc: false,
            is_defined: false,
            is_used: false,
        }
    }
}