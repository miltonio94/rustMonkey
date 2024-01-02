use super::expression;
use crate::ast::NodeInterface;
use crate::token::Token;
use std::fmt::Display;

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(Expression),
    Block(Block),
}

impl Statement {
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

impl NodeInterface for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::Let(let_statement) => let_statement.token_literal(),
            Self::Return(return_statement) => return_statement.token_literal(),
            _ => "".to_string(),
        }
    }
}

impl Display for Statement {
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
pub struct Let {
    pub token: Token,
    pub name: expression::Identifier,
    pub value: Option<expression::Expression>,
}

impl NodeInterface for Let {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for Let {
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
pub struct Return {
    pub token: Token,
    pub return_value: Option<expression::Expression>,
}

impl NodeInterface for Return {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for Return {
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
pub struct Expression {
    pub token: Token,
    pub expression: expression::Expression,
}

impl NodeInterface for Expression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression.to_string())
    }
}

#[derive(Debug)]
pub struct Block {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl NodeInterface for Block {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for s in self.statements.iter() {
            out.push_str(&s.to_string())
        }

        write!(f, "{}", out)
    }
}
