use std::vec;

pub enum TokenType {
    Illegal,
    Eof,

    // Identifier + literals
    Ident(vec::Vec<u8>),
    Int(vec::Vec<u8>),

    //Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Eq,
    NotEq,

    Lt,
    Gt,

    // Delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub struct Token {
    Type: TokenType,
    Literal: String,
}

impl Token {
    pub fn new(tokenType: TokenType, ch: &[u8]) -> Self {
        //
    }
}
