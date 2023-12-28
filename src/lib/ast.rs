pub mod expression;
pub mod statement;

use expression::Expression;
use statement::Statement;
use std::fmt::Display;

pub trait NodeInterface: Display {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

impl NodeInterface for Node {
    fn token_literal(&self) -> String {
        match self {
            Self::Statement(statement) => statement.token_literal(),
            Self::Expression(expression) => expression.to_string(), // TODO: remove this,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(expression) => write!(f, "{}", expression.to_string()),
            Self::Statement(stmt) => write!(f, "{}", stmt.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl NodeInterface for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for s in self.statements.iter() {
            out.push_str(&s.to_string());
        }

        write!(f, "{}", out)
    }
}
