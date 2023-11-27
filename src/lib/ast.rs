use crate::token::Token;

trait NodeInterface {
    fn token_literal(&self) -> String;
}

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

pub enum Statement {
    LetStatement(LetStatement),
}

impl Statement {
    pub fn let_statement(&self) -> Option<&LetStatement> {
        match self {
            Self::LetStatement(let_statement) => Some(let_statement),
            _ => None,
        }
    }
}

impl NodeInterface for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::LetStatement(let_statement) => let_statement.token_literal(),
        }
    }
}

pub enum Expression {
    //
}

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

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl NodeInterface for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}
