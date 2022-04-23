use std::fmt;
use crate::grammer::Terminal;

pub type TokenList = Vec<Box<dyn Token>>;

pub trait Token: fmt::Debug {
    fn row(&self) -> usize;

    fn col(&self) -> usize;

    fn pos(&self) -> &Pos;

    fn token(&self) -> Terminal;

    fn value(&self) -> Option<Value> {
        None
    }
}

pub union Value<'a> {
    pub num: isize,
    pub text: &'a str,
}

#[derive(Debug)]
pub struct StructureToken {
    token: Terminal,
    pos: Pos,
}

impl StructureToken {
    pub fn new(token: Terminal, pos: Pos) -> Self {
        Self {
            token,
            pos,
        }
    }

    pub fn new_box(token: Terminal, pos: Pos) -> Box<dyn Token> {
        Box::new(Self::new(token, pos))
    }
}

impl Token for StructureToken {
    // gets tokens row
    fn row(&self) -> usize {
        self.pos.row
    }

    // gets tokens col
    fn col(&self) -> usize {
        self.pos.col
    }

    // gets tokens pos object
    fn pos(&self) -> &Pos {
        &self.pos
    }

    fn token(&self) -> Terminal {
        self.token
    }
}

#[derive(Debug)]
pub struct NumToken {
    token: Terminal,
    value: isize,
    pos: Pos,
}

impl NumToken {
    pub fn new(token: Terminal, value: isize, pos: Pos) -> Self {
        Self {
            token,
            value,
            pos,
        }
    }

    pub fn new_box(token: Terminal, value: isize, pos: Pos) -> Box<dyn Token> {
        Box::new(Self::new(token, value, pos))
    }
}

impl Token for NumToken {
    // gets tokens row
    fn row(&self) -> usize {
        self.pos.row
    }

    // gets tokens col
    fn col(&self) -> usize {
        self.pos.col
    }

    // gets tokens pos object
    fn pos(&self) -> &Pos {
        &self.pos
    }

    fn token(&self) -> Terminal {
        self.token
    }

    fn value(&self) -> Option<Value> {
        Some(Value { num: self.value })
    }
}

#[derive(Debug)]
pub struct StringToken {
    token: Terminal,
    value: String,
    pos: Pos,
}

impl StringToken {
    pub fn new(token: Terminal, value: String, pos: Pos) -> Self {
        Self {
            token,
            value,
            pos,
        }
    }

    pub fn new_box(token: Terminal, value: String, pos: Pos) -> Box<dyn Token> {
        Box::new(Self::new(token, value, pos))
    }
}

impl Token for StringToken {
    // gets tokens row
    fn row(&self) -> usize {
        self.pos.row
    }

    // gets tokens col
    fn col(&self) -> usize {
        self.pos.col
    }

    // gets tokens pos object
    fn pos(&self) -> &Pos {
        &self.pos
    }

    fn token(&self) -> Terminal {
        self.token
    }

    fn value(&self) -> Option<Value> {
        Some(Value { text: &self.value })
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