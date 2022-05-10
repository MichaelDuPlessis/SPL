use std::fmt::{self, Debug};
use crate::grammer::Terminal;

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

// pub trait Token: fmt::Debug {
    // fn row(&self) -> usize;

    // fn col(&self) -> usize;

    // fn pos(&self) -> Pos;

    // fn token(&self) -> Terminal;

    // fn num_value(&self) -> Option<isize> {
    //     None
    // }

    // fn str_value(&self) -> Option<String> {
    //     None
    // }
// }

// #[derive(Debug)]
// pub struct StructureToken {
//     token: Terminal,
//     pos: Pos,
// }

// impl StructureToken {
    // pub fn new(token: Terminal, pos: Pos) -> Self {
    //     Self {
    //         token,
    //         pos,
    //     }
    // }

//     pub fn new_box(token: Terminal, pos: Pos) -> Box<dyn Token> {
//         Box::new(Self::new(token, pos))
//     }
// }

// impl Token for StructureToken {
//     // gets tokens row
//     fn row(&self) -> usize {
//         self.pos.row
//     }

//     // gets tokens col
//     fn col(&self) -> usize {
//         self.pos.col
//     }

//     // gets tokens pos object
//     fn pos(&self) -> Pos {
//         self.pos
//     }

//     fn token(&self) -> Terminal {
//         self.token
//     }
// }

// #[derive(Debug)]
// pub struct NumToken {
    // token: Terminal,
    // value: isize,
    // pos: Pos,
// }

// impl NumToken {
//     pub fn new(token: Terminal, value: isize, pos: Pos) -> Self {
//         Self {
//             token,
//             value,
//             pos,
//         }
//     }

//     pub fn new_box(token: Terminal, value: isize, pos: Pos) -> Box<dyn Token> {
//         Box::new(Self::new(token, value, pos))
//     }
// }

// impl Token for NumToken {
//     // gets tokens row
//     fn row(&self) -> usize {
//         self.pos.row
//     }

//     // gets tokens col
//     fn col(&self) -> usize {
//         self.pos.col
//     }

//     // gets tokens pos object
//     fn pos(&self) -> Pos {
//         self.pos
//     }

//     fn token(&self) -> Terminal {
//         self.token
//     }

//     fn num_value(&self) -> Option<isize> {
//         Some(self.value)
//     }
// }

// #[derive(Debug)]
// pub struct StringToken {
//     token: Terminal,
//     value: String,
//     pos: Pos,
// }

// impl StringToken {
//     pub fn new(token: Terminal, value: String, pos: Pos) -> Self {
//         Self {
//             token,
//             value,
//             pos,
//         }
//     }

//     pub fn new_box(token: Terminal, value: String, pos: Pos) -> Box<dyn Token> {
//         Box::new(Self::new(token, value, pos))
//     }
// }

// impl Token for StringToken {
//     // gets tokens row
//     fn row(&self) -> usize {
//         self.pos.row
//     }

//     // gets tokens col
//     fn col(&self) -> usize {
//         self.pos.col
//     }

//     // gets tokens pos object
//     fn pos(&self) -> Pos {
//         self.pos
//     }

//     fn token(&self) -> Terminal {
//         self.token
//     }

//     fn str_value(&self) -> Option<String> {
//         Some(self.value.clone())
//     }
// }

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