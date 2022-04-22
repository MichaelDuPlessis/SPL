use core::fmt;
use std::{str::Chars, iter::Peekable, process::exit};
use crate::grammer::Nonterminal;

const FILLER: [char; 4] = ['\n', ' ', '\r', '\t']; // holds all chars that can be ignored

// Lexer for the language
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>, // to allow us to look ahead
    tokens: Vec<Token>,
    current_pos: Pos,
    current_token: String,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input.chars().peekable(),
            tokens: Vec::new(),
            current_pos: Pos::new(1, 1),
            current_token: String::new(),
        }
    }

    // method to created tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        // loop until error or until complete and tokens are returned
        loop {
            if let Some(character) = self.peek() { // still chars to check
                let character = *character;
                self.current_token.clear();

                // starting dfa checking
                match character {
                    // sperator
                    ' '|'\r'|'\t' => { self.next(); },
                    // new line
                    '\n' => {
                        self.next();
                        self.current_pos.next_row();
                    },
                    // main/mult
                    'm' => self.token_double(("main", Nonterminal::Main), ("mult", Nonterminal::Mult), self.current_pos),
                    // halt
                    'h' => self.token("halt", Nonterminal::Halt, self.current_pos),
                    // if/input
                    'i' => self.token_double(("if", Nonterminal::If), ("input", Nonterminal::Input), self.current_pos),
                    // else/equal
                    'e' => self.token_double(("else", Nonterminal::Else), ("eq", Nonterminal::Equal), self.current_pos),
                    // then/true
                    't' => self.token_double(("then", Nonterminal::Then), ("true", Nonterminal::True), self.current_pos),
                    // return
                    'r' => self.token("return", Nonterminal::Return, self.current_pos),
                    // proc
                    'p' => self.token("proc", Nonterminal::Proc, self.current_pos),
                    // do
                    'd' => self.token("do", Nonterminal::Do, self.current_pos),
                    // until
                    'u' => self.token("until", Nonterminal::Until, self.current_pos),
                    // while
                    'w' => self.token("while", Nonterminal::While, self.current_pos),
                    // output/or
                    'o' => self.token_double(("output", Nonterminal::Out), ("or", Nonterminal::Or), self.current_pos),
                    // call
                    'c' => self.token("call", Nonterminal::Call, self.current_pos),
                    // false
                    'f' => self.token("false", Nonterminal::False, self.current_pos),
                    // not/num
                    'n' => self.token_double(("not", Nonterminal::Not), ("num", Nonterminal::Num), self.current_pos),
                    // and
                    'a' => self.token_and(self.current_pos),
                    // larger
                    'l' => self.token("larger", Nonterminal::Larger, self.current_pos),
                    // sub/string
                    's' => self.token_double(("sub", Nonterminal::Sub), ("string", Nonterminal::String), self.current_pos),
                    // bool
                    'b' => self.token("bool", Nonterminal::Boolean, self.current_pos),
                    // user defined
                    'g'|'j'|'k'|'q'|'v'|'x'|'y'|'z' => self.token_user_defined(self.current_pos),
                    // number
                    '0'|'-'|'1'..='9' => self.token_number(self.current_pos),
                    // short string
                    '\"' => self.token_short_string(self.current_pos),
                    // assignment
                    ':' => self.token_assignment(self.current_pos),
                    // basic token
                    '('|')'|'['|']'|'{'|'}'|','|';' => {
                        self.tokens.push(Token::new(Nonterminal::basic_token(character, self.current_pos), self.current_pos));
                        self.next();
                    },
                    // Invalid token
                    _ => {
                        println!("Invalid character {} at {}", character, self.current_pos);
                        exit(1);
                    },
                }
            } else { // if no more tokens than return what has been found
                return self.tokens.clone();
            }
        }
    }

    // checking if assignent
    fn token_assignment(&mut self, token_pos: Pos) {
        // no need to add to current_token string as if next char is no = then invalid
        self.next().unwrap();

        if let Some(current_char) = self.next() {
            if current_char != '=' {
                println!("Invalid character {} at {}", current_char, self.current_pos);
                exit(1);
            }

            self.tokens.push(Token::new(Nonterminal::Assignment, token_pos));
        } else {
            println!("End of file reached before token could be completed");
            exit(1);
        }
    }

    // checking if short string
    fn token_short_string(&mut self, token_pos: Pos) {
        let mut length: u8 = 0; // making sure length of string is <= 15

        self.next();

        let valid_chars = ('A'..'Z').chain('0'..'9').chain([' ']).collect::<Vec<char>>();

        while let Some(current_char) = self.next() {
            if current_char == '\"' {
                self.tokens.push(Token::new(Nonterminal::ShortString(self.current_token.clone()), token_pos));
                return;
            }

            if length >= 15 {
                println!("Max length of short string exceeded at {}", self.current_pos);
                exit(1);
            }

            if valid_chars.contains(&current_char) {
                length += 1;
                self.current_token.push(current_char);
            } else {
                println!("Invalid character {} at {}", current_char, self.current_pos);
                exit(1);
            }
        }

        println!("Short string started here {} but never closed", token_pos);
        exit(1);
    }

    // checking if number
    fn token_number(&mut self, token_pos: Pos) {
        let character = self.next().unwrap();
        self.current_token.push(character); // safe to unwrap cause caller checked to see if it should call num

        if self.current_token == "0" {
            if let Some(next_char) = self.peek() {
                let next_char = *next_char;
                if !FILLER.contains(&next_char) {
                    println!("Invalid token {} at {}", next_char, self.current_pos);
                    exit(1);
                }
            }
            self.tokens.push(Token::new(Nonterminal::Number(self.current_token.parse().unwrap()), token_pos)); // can unwrap cause we know its just zero
            return;
        }

        if self.current_token == "-" {
            match self.next() {
                Some(current_char) => {
                    if !('1'..'9').contains(&current_char) {
                        println!("Invalid symbol {} afer - at {}", current_char, token_pos);
                        exit(1);
                    }

                    
                    self.current_token.push(current_char);
                },
                None => {
                    println!("Missing number afer - at {}", token_pos);
                    exit(1);
                },
            }
        }

        while let Some(current_char) = self.peek() {
            let current_char = *current_char;

            if "()[]{};,".contains(current_char) || FILLER.contains(&current_char) {
                self.tokens.push(Token::new(Nonterminal::Number(self.current_token.parse().unwrap()), token_pos)); // can unwrap cause we know its just zeros
                return;
            }

            if ('0'..'9').contains(&current_char) {
                self.current_token.push(current_char); // can unwrap cause we know its just zeros
            } else {
                println!("Invalid symbol {} after - at {}", current_char, token_pos);
                exit(1);
            }

            self.next();
        }

        self.tokens.push(Token::new(Nonterminal::Number(self.current_token.parse().unwrap()), token_pos));
    }

    // to check and
    fn token_and(&mut self, token_pos: Pos) {
        let character = self.next().unwrap();
        self.current_token.push(character);

        if let Some(current_char) = self.peek() {
            if *current_char != 'n' {
                self.token_double(("add", Nonterminal::Add), ("arr", Nonterminal::Array), token_pos);
                return;
            } else {
                let character = self.next().unwrap();
                self.current_token.push(character);

                if let Some(current_char) = self.peek() {
                    if *current_char == 'd' {
                        let character = self.next().unwrap(); // can unwrap because know there will be a next
                        self.current_token.push(character); 
                        self.tokens.push(Token::new(Nonterminal::And, token_pos))
                    } else {
                        self.token_user_defined(token_pos);
                    }
                } else {
                    self.token_user_defined(token_pos);
                }
            }
        } else {
            self.token_user_defined(token_pos);
        }
    }

    // check if first token is found otherwise search for second
    fn token_double(&mut self, first: (&str, Nonterminal), second: (&str, Nonterminal), token_pos: Pos) {
        // first check if token can be formed else go to user defeined
        for c in first.0[self.current_token.len()..].chars() {
            if let Some(current_char) = self.peek() {

                if *current_char != c {
                    self.token(&second.0[self.current_token.len()..], second.1, token_pos);
                    return;
                }

                let character = self.next().unwrap(); // can unwrap because know there will be a next
                self.current_token.push(character); 
            } else {
                self.token_user_defined(token_pos);
                return;
            }
        }

        self.check_user_defined(first.1, token_pos);
    }  

    // method to check if token is main
    fn token(&mut self, token_string: &str, token_type: Nonterminal, token_pos: Pos) {
        // first check if token can be formed else go to user defeined
        for c in token_string.chars() {
            if let Some(current_char) = self.peek() {

                if *current_char != c {
                    self.token_user_defined(token_pos);
                    return;
                }
                
                let character = self.next().unwrap(); // can unwrap because know there will be a next
                self.current_token.push(character); 
            } else {
                self.token_user_defined(token_pos);
                return;
            }
        }

        self.check_user_defined(token_type, token_pos);
    }

    // check if user defined variable
    fn check_user_defined(&mut self, token_type: Nonterminal, token_pos: Pos) {
        // check if any valid user defined characters follow token if so make it user defined
        if let Some(current_char) = self.peek() {
            if ('a'..'z').contains(current_char) || ('0'..'9').contains(current_char) {
                self.token_user_defined(token_pos)
            } else {
                self.tokens.push(Token::new(token_type, token_pos));
            }
        } else {
            self.tokens.push(Token::new(token_type, token_pos));
            return;
        }
    }

    // creating user defined token
    fn token_user_defined(&mut self, token_pos: Pos) {
        while let Some(current_char) = self.peek() {
            let current_char = *current_char;
            match current_char {
                'a'..='z'|'0'..='9' => {
                    self.current_token.push(current_char);
                    self.next();
                }, // safe to unwrap because know there is a next
                _ => break,
            }
        }

        self.tokens.push(Token::new(Nonterminal::UserDefined(self.current_token.clone()), token_pos))
    }

    // wrapper around iterators next
    fn next(&mut self) -> Option<char> {
        self.current_pos.next_col();
        self.source.next()
    }

    // wrapper around iterators peek
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
}

// a token of the language
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Token {
    token: Nonterminal,
    pos: Pos,
}

#[allow(dead_code)]
impl Token {
    // creates a new token
    fn new(token: Nonterminal, pos: Pos) -> Self {
        Self {
            token,
            pos,
        }
    }

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