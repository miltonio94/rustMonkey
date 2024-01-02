use super::statement;
use crate::ast::NodeInterface;
use crate::token::Token;
use std::fmt::Display;

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(Prefix),
    Infix(Infix),
    Boolean(Boolean),
    If(If),
    FunctionLiteral(FunctionLiteral),
    Call(Call),
}

impl Expression {
    pub fn identifier(&self) -> Option<&Identifier> {
        match self {
            Self::Identifier(idnt) => Some(idnt),
            _ => None,
        }
    }

    pub fn integer_literal(&self) -> Option<&IntegerLiteral> {
        match self {
            Self::IntegerLiteral(int) => Some(int),
            _ => None,
        }
    }

    pub fn prefix_expression(&self) -> Option<&Prefix> {
        match self {
            Self::Prefix(prefix) => Some(&prefix),
            _ => None,
        }
    }

    pub fn infix_expression(&self) -> Option<&Infix> {
        match self {
            Self::Infix(infix) => Some(&infix),
            _ => None,
        }
    }

    pub fn boolean_expression(&self) -> Option<&Boolean> {
        match self {
            Self::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    pub fn if_expression(&self) -> Option<&If> {
        match self {
            Self::If(if_exp) => Some(if_exp),
            _ => None,
        }
    }

    pub fn function_literal(&self) -> Option<&FunctionLiteral> {
        match self {
            Self::FunctionLiteral(function) => Some(function),
            _ => None,
        }
    }

    pub fn call_expression(&self) -> Option<&Call> {
        match self {
            Self::Call(call) => Some(call),
            _ => None,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(idnt) => write!(f, "{}", idnt.to_string()),
            Self::IntegerLiteral(int) => write!(f, "{}", int.to_string()),
            Self::Prefix(prefix) => write!(f, "{}", prefix.to_string()),
            Self::Infix(infix) => write!(f, "{}", infix.to_string()),
            Self::Boolean(boolean) => write!(f, "{}", boolean.to_string()),
            Self::If(if_exp) => write!(f, "{}", if_exp.to_string()),
            Self::FunctionLiteral(function) => write!(f, "{}", function.to_string()),
            Self::Call(call) => write!(f, "{}", call.to_string()),
        }
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

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

impl NodeInterface for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug)]
pub struct Prefix {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right.to_string())
    }
}

impl NodeInterface for Prefix {
    fn token_literal(&self) -> String {
        format!("{}", self.token.literal)
    }
}

#[derive(Debug)]
pub struct Infix {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl NodeInterface for Infix {
    fn token_literal(&self) -> String {
        format!("{}", self.token.literal)
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        )
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl NodeInterface for Boolean {
    fn token_literal(&self) -> String {
        format!("{}", self.token.literal)
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

#[derive(Debug)]
pub struct If {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: statement::Block,
    pub alternative: Option<statement::Block>,
}

impl NodeInterface for If {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("if");

        out.push_str(&self.condition.to_string());
        out.push(' ');
        out.push_str(&self.consequence.to_string());

        if let Some(alternative) = &self.alternative {
            out.push_str("else ");
            out.push_str(&alternative.to_string());
        }

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: statement::Block,
}

impl NodeInterface for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.token_literal();

        out.push('(');

        let mut params: Vec<String> = Vec::new();
        for param in self.parameters.iter() {
            params.push(param.to_string());
        }

        out.push_str(&params.join(", "));
        out.push(')');
        out.push_str(&self.body.to_string());

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct Call {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl NodeInterface for Call {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.function.to_string();

        let mut args: Vec<String> = Vec::new();

        for arg in self.arguments.iter() {
            args.push(arg.to_string());
        }

        out.push('(');
        out.push_str(&args.join(", "));
        out.push(')');

        write!(f, "{}", out)
    }
}
