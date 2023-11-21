use std::{fmt::Display, vec};

#[derive(Debug, PartialEq)]
pub enum Token {
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

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    pub fn new(ch: u8) -> Self {
        match ch {
            b'=' => Self::Assign,
            b';' => Self::Semicolon,
            b'(' => Self::LParen,
            b')' => Self::RParen,
            b',' => Self::Comma,
            b'+' => Self::Plus,
            b'{' => Self::LBrace,
            b'}' => Self::RBrace,
            b'\0' => Self::Eof,
            _ => Self::Illegal,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Illegal => write!(f, ""),
            Self::Eof => write!(f, ""),

            Self::Ident(ident) => write!(f, ""),
            Self::Int(number) => write!(f, ""),

            Self::Assign => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Eq => write!(f, "="),
            Self::NotEq => write!(f, "!="),

            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),

            Self::Comma => write!(f, ","),
            Self::Semicolon => write!(f, ";"),

            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBrace => write!(f, "{}", '{'),
            Self::RBrace => write!(f, "{}", '}'),

            Self::Function => write!(f, "fn"),
            Self::Let => write!(f, "let"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Return => write!(f, "return"),
        }
    }
}
