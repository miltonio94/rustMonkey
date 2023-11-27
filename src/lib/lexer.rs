use std::vec;

use crate::token::{self, Token, TokenType};

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
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(
                        TokenType::Eq,
                        String::from_utf8(vec![ch, self.ch]).unwrap_or_default(),
                    )
                } else {
                    Token::new(
                        TokenType::Assign,
                        String::from_utf8(vec![self.ch]).unwrap_or_default(),
                    )
                }
            }
            b'+' => Token::new(
                TokenType::Plus,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b'-' => Token::new(
                TokenType::Minus,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(
                        TokenType::NotEq,
                        String::from_utf8(vec![ch, self.ch]).unwrap_or_default(),
                    )
                } else {
                    Token::new(
                        TokenType::Bang,
                        String::from_utf8(vec![self.ch]).unwrap_or_default(),
                    )
                }
            }
            b'*' => Token::new(
                TokenType::Asterisk,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b'/' => Token::new(
                TokenType::Slash,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b'<' => Token::new(
                TokenType::Lt,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b'>' => Token::new(
                TokenType::Gt,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b';' => Token::new(
                TokenType::Semicolon,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b'(' => Token::new(
                TokenType::LParen,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b')' => Token::new(
                TokenType::RParen,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b',' => Token::new(
                TokenType::Comma,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b'{' => Token::new(
                TokenType::LBrace,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            b'}' => Token::new(
                TokenType::RBrace,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),

            b'\0' => Token::new(
                TokenType::EOF,
                String::from_utf8(vec![self.ch]).unwrap_or_default(),
            ),
            _ => {
                if is_letter(self.ch) {
                    let chunck = self.read_identifier();
                    if token::is_keyword(chunck) {
                        return Token::new(
                            token::lookup_keyword(chunck),
                            String::from_utf8(chunck.into()).unwrap_or_default(),
                        );
                    } else {
                        return Token::new(
                            TokenType::Ident,
                            String::from_utf8(chunck.into()).unwrap_or_default(),
                        );
                    }
                } else if is_digit(self.ch) {
                    return Token::new(
                        TokenType::Int,
                        String::from_utf8(self.read_number().into()).unwrap_or_default(),
                    );
                } else {
                    return Token::new(
                        TokenType::Illegal,
                        String::from_utf8(vec![self.ch]).unwrap_or_default(),
                    );
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

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            b'\0'
        } else {
            self.input[self.read_position]
        }
    }
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}
