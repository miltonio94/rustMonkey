use std::vec;

use crate::token::{self, Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a [char],
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examimination
}

impl Lexer<'_> {
    pub fn new<'a>(input: &'a [char]) -> Self {
        Self {
            ch: input[0],
            input,
            position: 0,
            read_position: 1,
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Ok(Token::new(TokenType::Eq, &[ch, self.ch]))
                } else {
                    Ok(Token::new(TokenType::Assign, &[self.ch]))
                }
            }
            '+' => Ok(Token::new(TokenType::Plus, &[self.ch])),
            '-' => Ok(Token::new(TokenType::Minus, &[self.ch])),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Ok(Token::new(TokenType::NotEq, &[ch, self.ch]))
                } else {
                    Ok(Token::new(TokenType::Bang, &[self.ch]))
                }
            }
            '*' => Ok(Token::new(TokenType::Asterisk, &[self.ch])),
            '/' => Ok(Token::new(TokenType::Slash, &[self.ch])),
            '<' => Ok(Token::new(TokenType::Lt, &[self.ch])),
            '>' => Ok(Token::new(TokenType::Gt, &[self.ch])),
            ';' => Ok(Token::new(TokenType::Semicolon, &[self.ch])),
            '(' => Ok(Token::new(TokenType::LParen, &[self.ch])),
            ')' => Ok(Token::new(TokenType::RParen, &[self.ch])),
            ',' => Ok(Token::new(TokenType::Comma, &[self.ch])),
            '{' => Ok(Token::new(TokenType::LBrace, &[self.ch])),
            '}' => Ok(Token::new(TokenType::RBrace, &[self.ch])),
            '\0' => Ok(Token::new(TokenType::EOF, &[self.ch])),
            _ => {
                if is_letter(self.ch) {
                    let chunck = self.read_identifier();
                    if token::is_keyword(chunck) {
                        return Ok(Token::new(token::lookup_keyword(chunck)?, chunck));
                    } else {
                        return Ok(Token::new(TokenType::Ident, chunck));
                    }
                } else if is_digit(self.ch) {
                    return Ok(Token::new(TokenType::Int, self.read_number()));
                } else {
                    return Err("Could not parse token".to_string());
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> &[char] {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &[char] {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        &self.input[position..(self.position)]
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic()
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}
