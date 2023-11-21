use std::vec;

use crate::token::Token;

pub struct Lexer {
    input: vec::Vec<u8>,
    position: usize,     // current position in input (points to current char)
    readPosition: usize, // current reading position in input (after current char)
    ch: u8,              // current char under examimination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        // FIXME: error handling for the string when it contains non ASCII chars
        Self {
            input: input.as_bytes().to_vec(),
            position: 0,
            readPosition: 0,
            ch: b'\0',
        }
    }
    pub fn read_char(&mut self) {
        if self.readPosition >= self.input.len() {
            self.ch = b'\0'
        } else {
            self.ch = self.input[l.readPosition]
        }
        self.position = self.readPosition;
        self.readPosition += 1;
    }
    pub fn nextToken(&mut self) -> Token {
        //
    }
}
