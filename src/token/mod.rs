use std::collections::HashMap;
use std::{fmt::Display, vec};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,

    // Identifier + literals
    // TODO: Change from Vec<u8> to &[u8]
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

pub fn lookup_keyword(ident: &[u8]) -> Token {
    let ident = String::from_utf8(ident.to_vec()).unwrap_or_default();
    let keywords = HashMap::from([
        ("fn".to_string(), Token::Function),
        ("let".to_string(), Token::Let),
    ]);

    keywords.get(&ident).unwrap_or(&Token::Illegal).clone()
}

pub fn is_keyword(ident: &[u8]) -> bool {
    let ident = String::from_utf8(ident.to_vec()).unwrap_or_default();
    let keywords = HashMap::from([
        ("fn".to_string(), Token::Function),
        ("let".to_string(), Token::Let),
    ]);

    keywords.get(&ident).is_some()
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Illegal => write!(f, ""),
            Self::Eof => write!(f, ""),

            Self::Ident(ident) => write!(
                f,
                "{}",
                String::from_utf8(ident.to_vec())
                    .unwrap_or_default()
                    .to_string()
            ),
            Self::Int(number) => write!(
                f,
                "{}",
                String::from_utf8(number.to_vec())
                    .unwrap_or_default()
                    .to_string()
            ),
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
