use std::collections::HashMap;
use std::{fmt::Display, vec};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifier + literals
    // TODO: Change from Vec<u8> to &[u8]
    Ident,
    Int,

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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Illegal => write!(f, ""),
            Self::EOF => write!(f, ""),

            Self::Ident => write!(f, "{}", ""),
            Self::Int => write!(f, "{}", ""),
            Self::Assign => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Eq => write!(f, "=="),
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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::Illegal => write!(f, ""),
            TokenType::EOF => write!(f, ""),

            TokenType::Ident => write!(f, "{}", self.literal),
            TokenType::Int => write!(f, "{}", self.literal),
            TokenType::Assign => write!(f, "{}", self.token_type.to_string()),
            TokenType::Plus => write!(f, "{}", self.token_type.to_string()),
            TokenType::Minus => write!(f, "{}", self.token_type.to_string()),
            TokenType::Bang => write!(f, "{}", self.token_type.to_string()),
            TokenType::Asterisk => write!(f, "{}", self.token_type.to_string()),
            TokenType::Slash => write!(f, "{}", self.token_type.to_string()),
            TokenType::Eq => write!(f, "{}", self.token_type.to_string()),
            TokenType::NotEq => write!(f, "{}", self.token_type.to_string()),

            TokenType::Lt => write!(f, "{}", self.token_type.to_string()),
            TokenType::Gt => write!(f, "{}", self.token_type.to_string()),

            TokenType::Comma => write!(f, "{}", self.token_type.to_string()),
            TokenType::Semicolon => write!(f, "{}", self.token_type.to_string()),

            TokenType::LParen => write!(f, "{}", self.token_type.to_string()),
            TokenType::RParen => write!(f, "{}", self.token_type.to_string()),
            TokenType::LBrace => write!(f, "{}", self.token_type.to_string()),
            TokenType::RBrace => write!(f, "{}", self.token_type.to_string()),

            TokenType::Function => write!(f, "{}", self.token_type.to_string()),
            TokenType::Let => write!(f, "{}", self.token_type.to_string()),
            TokenType::True => write!(f, "{}", self.token_type.to_string()),
            TokenType::False => write!(f, "{}", self.token_type.to_string()),
            TokenType::If => write!(f, "{}", self.token_type.to_string()),
            TokenType::Else => write!(f, "{}", self.token_type.to_string()),
            TokenType::Return => write!(f, "{}", self.token_type.to_string()),
        }
    }
}

// TODO: refactor this function to not use a hash and return Some<Token>
pub fn lookup_keyword(ident: &[u8]) -> TokenType {
    let ident = String::from_utf8(ident.to_vec()).unwrap_or_default();
    let keywords = HashMap::from([
        ("fn".to_string(), TokenType::Function),
        ("let".to_string(), TokenType::Let),
        ("true".to_string(), TokenType::True),
        ("false".to_string(), TokenType::False),
        ("if".to_string(), TokenType::If),
        ("else".to_string(), TokenType::Else),
        ("return".to_string(), TokenType::Return),
    ]);

    keywords.get(&ident).unwrap_or(&TokenType::Illegal).clone()
}

// TODO: once the above refactor is done we can remove this function
pub fn is_keyword(ident: &[u8]) -> bool {
    let ident = String::from_utf8(ident.to_vec()).unwrap_or_default();
    let keywords = HashMap::from([
        ("fn".to_string(), TokenType::Function),
        ("let".to_string(), TokenType::Let),
        ("true".to_string(), TokenType::True),
        ("false".to_string(), TokenType::False),
        ("if".to_string(), TokenType::If),
        ("else".to_string(), TokenType::Else),
        ("return".to_string(), TokenType::Return),
    ]);

    keywords.get(&ident).is_some()
}
