use std::{fmt::{self, Debug}, cell::RefCell, rc::Rc};
use crate::grammer::{Terminal, Grammer};

pub type TokenList = Vec<Token>;

pub struct Token {
    token: Terminal,
    num_value: Option<isize>,
    str_value: Option<String>,
    pos: Pos,
}

impl Token {
    pub fn new_struct_token(token: Terminal, pos: Pos) -> Self {
        Self {
            token,
            pos,
            num_value: None,
            str_value: None,
        }
    }

    pub fn new_num_token(token: Terminal, pos: Pos, value: isize) -> Self {
        Self {
            token,
            pos,
            num_value: Some(value),
            str_value: None,
        }
    }

    pub fn new_str_token(token: Terminal, pos: Pos, value: String) -> Self {
        Self {
            token,
            pos,
            num_value: None,
            str_value: Some(value),
        }
    }

    pub fn row(&self) -> usize {
        self.pos.row
    }

    pub fn col(&self) -> usize {
        self.pos.col
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn token(&self) -> Terminal {
        self.token
    }

    pub fn num_value(&self) -> Option<isize> {
        self.num_value
    }

    pub fn str_value(&self) -> Option<String> {
        self.str_value.clone()
    }
}

// struct reprenting position of a token
#[derive(Clone, Copy, Debug)]
pub struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    // create new Pos
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
        }
    }

    // increases col
    pub fn next_col(&mut self) {
        self.col += 1;
    }

    // increases row and sets col to 0
    pub fn next_row(&mut self) {
        self.row += 1;
        self.col = 1;
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ln {}, Col {}", self.row, self.col)
    }
}

// node used to build tree
pub type LNode = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub symbol: Grammer,
    pub children: Vec<LNode>,
    pub pos: Option<Pos>,
    pub num_value: Option<isize>,
    pub str_value: Option<String>,
}

impl Node {
    pub fn new(symbol: Grammer, id: usize) -> Self {
        Self {
            id,
            symbol,
            children: Vec::new(),
            pos: None,
            num_value: None,
            str_value: None,
        }
    } 
    
    pub fn add_children(&mut self, children: &Vec<LNode>) {
        for c in children {
            self.children.push(Rc::clone(c));
        }
    }

    pub fn find(&self, id: usize, node: &LNode) -> Option<LNode> {
        if node.borrow().id == id {
            return Some(Rc::clone(node));
        }

        for c in &node.borrow().children {
            if let Some(n) = self.find(id, c) {
                if n.borrow().id == id {
                    return Some(Rc::clone(&n));
                }
            }
        }

        None
    }
}