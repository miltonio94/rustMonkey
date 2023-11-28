use crate::token::Token;

pub trait NodeInterface {
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
            Self::Expression(_expression) => String::new(), // TODO: remove this,
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
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

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Vec<Expression>,
}

impl NodeInterface for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl NodeInterface for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
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
#[derive(Debug)]
pub enum Expression {
    None, //TODO: remove this
}
