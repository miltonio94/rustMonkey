use super::expression::{Expression, Identifier};
use crate::ast::NodeInterface;
use crate::token::Token;
use std::fmt::Display;

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
    BlockStatement(BlockStatement),
}

impl Statement {
    pub fn let_statement(&self) -> Option<&LetStatement> {
        match self {
            Self::LetStatement(let_statement) => Some(let_statement),
            _ => None,
        }
    }

    pub fn return_statement(&self) -> Option<&ReturnStatement> {
        match self {
            Self::ReturnStatement(return_statement) => Some(return_statement),
            _ => None,
        }
    }

    pub fn expression_statement(&self) -> Option<&ExpressionStatement> {
        match self {
            Self::ExpressionStatement(exp) => Some(exp),
            _ => None,
        }
    }

    pub fn block_statement(&self) -> Option<&BlockStatement> {
        match self {
            Self::BlockStatement(exp) => Some(exp),
            _ => None,
        }
    }
}

impl NodeInterface for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::LetStatement(let_statement) => let_statement.token_literal(),
            Self::ReturnStatement(return_statement) => return_statement.token_literal(),
            _ => "".to_string(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpressionStatement(expr) => write!(f, "{}", expr.to_string()),
            Self::LetStatement(stmt) => write!(f, "{}", stmt.to_string()),
            Self::ReturnStatement(stmt) => write!(f, "{}", stmt.to_string()),
            Self::BlockStatement(stmt) => write!(f, "{}", stmt.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl NodeInterface for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        out.push_str(&format!(
            "{} {} = ",
            self.token_literal(),
            self.name.to_string()
        ));

        if self.value.is_none() {
            out.push_str(&self.value.to_string());
        };

        out.push(';');

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl NodeInterface for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        out.push_str(&format!("{} ", self.token_literal()));

        if self.return_value.is_none() {
            out.push_str(&self.return_value.to_string());
        }

        out.push(';');

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl NodeInterface for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression.to_string())
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl NodeInterface for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for s in self.statements.iter() {
            out.push_str(&s.to_string())
        }

        write!(f, "{}", out)
    }
}
