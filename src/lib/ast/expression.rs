use super::statement;
use crate::ast::NodeInterface;
use crate::token::Token;
use std::fmt::Display;

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    IntegerLiteral(IntegerLiteral<'a>),
    Prefix(Prefix<'a>),
    Infix(Infix<'a>),
    Boolean(Boolean<'a>),
    If(If<'a>),
    FunctionLiteral(FunctionLiteral<'a>),
    Call(Call<'a>),
}

impl Expression<'_> {
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

impl Display for Expression<'_> {
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
pub struct Identifier<'a> {
    pub token: Token<'a>,
    pub value: String,
}

impl NodeInterface for Identifier<'_> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct IntegerLiteral<'a> {
    pub token: Token<'a>,
    pub value: i64,
}

impl Display for IntegerLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal.iter().collect::<String>())
    }
}

impl NodeInterface for IntegerLiteral<'_> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug)]
pub struct Prefix<'a> {
    pub token: Token<'a>,
    pub operator: String,
    pub right: Box<Expression<'a>>,
}

impl Display for Prefix<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right.to_string())
    }
}

impl NodeInterface for Prefix<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.iter().collect()
    }
}

#[derive(Debug)]
pub struct Infix<'a> {
    pub token: Token<'a>,
    pub left: Box<Expression<'a>>,
    pub operator: String,
    pub right: Box<Expression<'a>>,
}

impl NodeInterface for Infix<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.iter().collect()
    }
}

impl Display for Infix<'_> {
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
pub struct Boolean<'a> {
    pub token: Token<'a>,
    pub value: bool,
}

impl NodeInterface for Boolean<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.iter().collect()
    }
}

impl Display for Boolean<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal.iter().collect::<String>())
    }
}

#[derive(Debug)]
pub struct If<'a> {
    pub token: Token<'a>,
    pub condition: Box<Expression<'a>>,
    pub consequence: statement::Block<'a>,
    pub alternative: Option<statement::Block<'a>>,
}

impl NodeInterface for If<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.iter().collect()
    }
}

impl Display for If<'_> {
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
pub struct FunctionLiteral<'a> {
    pub token: Token<'a>,
    pub parameters: Vec<Identifier<'a>>,
    pub body: statement::Block<'a>,
}

impl NodeInterface for FunctionLiteral<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.iter().collect()
    }
}

impl Display for FunctionLiteral<'_> {
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
pub struct Call<'a> {
    pub token: Token<'a>,
    pub function: Box<Expression<'a>>,
    pub arguments: Vec<Expression<'a>>,
}

impl NodeInterface for Call<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.iter().collect()
    }
}

impl Display for Call<'_> {
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
