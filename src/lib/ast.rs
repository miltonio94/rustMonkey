use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}
trait Statement: Node {}
trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
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
    pub value: Vec<Box<dyn Expression>>,
}

pub struct Identifier {
    pub token: Token,
    Value: Vec<u8>,
}

impl Expression for Identifier {}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}
