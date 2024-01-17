pub mod expression;
pub mod statement;

use expression::Expression;
use statement::Statement;
use std::fmt::Display;

pub trait NodeInterface: Display {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Node<'a> {
    Statement(Statement<'a>),
    Expression(Expression<'a>),
}

impl NodeInterface for Node<'_> {
    fn token_literal(&self) -> String {
        match self {
            Self::Statement(statement) => statement.to_string(),
            Self::Expression(expression) => expression.to_string(),
        }
    }
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(expression) => write!(f, "{}", expression.to_string()),
            Self::Statement(stmt) => write!(f, "{}", stmt.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl NodeInterface for Program<'_> {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

impl Display for Program<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for s in self.statements.iter() {
            out.push_str(&s.to_string());
        }

        write!(f, "{}", out)
    }
}
