use std::vec;

use crate::token::{self, Token};

pub struct Lexer {
    input: vec::Vec<u8>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examimination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.as_bytes().to_vec();
        Self {
            ch: input[0],
            input,
            position: 0,
            read_position: 1,
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0'
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'\0' => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    let chunck = self.read_identifier();
                    if token::is_keyword(chunck) {
                        return token::lookup_keyword(chunck);
                    } else {
                        return Token::Ident(chunck.to_vec());
                    }
                } else if is_digit(self.ch) {
                    return Token::Int(self.read_number().to_vec());
                } else {
                    return Token::Illegal;
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> &[u8] {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &[u8] {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        &self.input[position..(self.position)]
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}
