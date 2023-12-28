use crate::token::Token;
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

#[derive(Debug)]
pub enum Expression {
    // TODO: Remove None from here. We should be using Option instead
    None,
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    BooleanExpression(Boolean),
    IfExpression(IfExpression),
}

impl Expression {
    fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

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

    pub fn prefix_expression(&self) -> Option<&PrefixExpression> {
        match self {
            Self::PrefixExpression(prefix) => Some(&prefix),
            _ => None,
        }
    }

    pub fn infix_expression(&self) -> Option<&InfixExpression> {
        match self {
            Self::InfixExpression(infix) => Some(&infix),
            _ => None,
        }
    }

    pub fn boolean_expression(&self) -> Option<&Boolean> {
        match self {
            Self::BooleanExpression(boolean) => Some(boolean),
            _ => None,
        }
    }

    pub fn if_expression(&self) -> Option<&IfExpression> {
        match self {
            Self::IfExpression(if_exp) => Some(if_exp),
            _ => None,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::Identifier(idnt) => write!(f, "{}", idnt.to_string()),
            Self::IntegerLiteral(int) => write!(f, "{}", int.to_string()),
            Self::PrefixExpression(prefix) => write!(f, "{}", prefix.to_string()),
            Self::InfixExpression(infix) => write!(f, "{}", infix.to_string()),
            Self::BooleanExpression(boolean) => write!(f, "{}", boolean.to_string()),
            Self::IfExpression(if_exp) => write!(f, "{}", if_exp.to_string()),
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
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right.to_string())
    }
}

impl NodeInterface for PrefixExpression {
    fn token_literal(&self) -> String {
        format!("{}", self.token.literal)
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl NodeInterface for InfixExpression {
    fn token_literal(&self) -> String {
        format!("{}", self.token.literal)
    }
}

impl Display for InfixExpression {
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
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl NodeInterface for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for IfExpression {
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
