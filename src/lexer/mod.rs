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
        let input = input.as_bytes().to_vec();
        Self {
            ch: input[0],
            input,
            position: 0,
            readPosition: 1,
        }
    }

    pub fn read_char(&mut self) {
        if self.readPosition >= self.input.len() {
            self.ch = b'\0'
        } else {
            self.ch = self.input[self.readPosition]
        }
        self.position = self.readPosition;
        self.readPosition += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = Token::new(self.ch);
        self.read_char();
        tok
    }
}
