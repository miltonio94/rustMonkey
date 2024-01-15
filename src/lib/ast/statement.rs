use super::expression;
use crate::ast::NodeInterface;
use crate::token::Token;
use std::fmt::Display;

#[derive(Debug)]
pub enum Statement<'a> {
    Let(Let<'a>),
    Return(Return<'a>),
    Expression(Expression<'a>),
    Block(Block<'a>),
}

impl Statement<'_> {
    pub fn let_statement(&self) -> Option<&Let> {
        match self {
            Self::Let(let_statement) => Some(let_statement),
            _ => None,
        }
    }

    pub fn return_statement(&self) -> Option<&Return> {
        match self {
            Self::Return(return_statement) => Some(return_statement),
            _ => None,
        }
    }

    pub fn expression_statement(&self) -> Option<&Expression> {
        match self {
            Self::Expression(exp) => Some(exp),
            _ => None,
        }
    }

    pub fn block_statement(&self) -> Option<&Block> {
        match self {
            Self::Block(exp) => Some(exp),
            _ => None,
        }
    }
}

impl NodeInterface for Statement<'_> {
    fn token_literal(&self) -> String {
        match self {
            Self::Let(let_statement) => let_statement.token_literal(),
            Self::Return(return_statement) => return_statement.token_literal(),
            _ => "".to_string(),
        }
    }
}

impl Display for Statement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(expr) => write!(f, "{}", expr.to_string()),
            Self::Let(stmt) => write!(f, "{}", stmt.to_string()),
            Self::Return(stmt) => write!(f, "{}", stmt.to_string()),
            Self::Block(stmt) => write!(f, "{}", stmt.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Let<'a> {
    pub token: Token<'a>,
    pub name: expression::Identifier<'a>,
    pub value: Option<expression::Expression<'a>>,
}

impl NodeInterface for Let<'_> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for Let<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        out.push_str(&format!(
            "{} {} = ",
            self.token_literal(),
            self.name.to_string()
        ));

        if let Some(val) = &self.value {
            out.push_str(&val.to_string());
        }

        out.push(';');

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct Return<'a> {
    pub token: Token<'a>,
    pub return_value: Option<expression::Expression<'a>>,
}

impl NodeInterface for Return<'_> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for Return<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        out.push_str(&format!("{} ", self.token_literal()));

        if let Some(value) = &self.return_value {
            out.push_str(&value.to_string());
        }

        out.push(';');

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct Expression<'a> {
    pub token: Token<'a>,
    pub expression: expression::Expression<'a>,
}

impl NodeInterface for Expression<'_> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression.to_string())
    }
}

#[derive(Debug)]
pub struct Block<'a> {
    pub token: Token<'a>,
    pub statements: Vec<Statement<'a>>,
}

impl NodeInterface for Block<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.iter().collect()
    }
}

impl Display for Block<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for s in self.statements.iter() {
            out.push_str(&s.to_string())
        }

        write!(f, "{}", out)
    }
}
