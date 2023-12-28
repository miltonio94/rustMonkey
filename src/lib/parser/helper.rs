use crate::ast::expression::{self, Expression};
use crate::ast::statement::{self, Statement};
use crate::parser::Parser;
use crate::token::TokenType;

pub type PrefixParseFn = fn(&mut Parser) -> Expression;
pub type InfixParseFn = fn(&mut Parser, Box<Expression>) -> Expression;

pub fn parse_identifier(parser: &mut Parser) -> Expression {
    Expression::Identifier(expression::Identifier {
        token: parser.cur_token.clone(),
        value: parser.cur_token.literal.clone(),
    })
}

pub fn parse_integer_literal(parser: &mut Parser) -> Expression {
    let value: i64 = match parser.cur_token.literal.parse() {
        Ok(val) => val,
        Err(err) => {
            //
            parser.errors.push(format!(
                "could not parse {} as integer. Err: {}",
                parser.cur_token.literal.clone(),
                err.to_string()
            ));
            return Expression::None;
        }
    };

    Expression::IntegerLiteral(expression::IntegerLiteral {
        token: parser.cur_token.clone(),
        value,
    })
}

pub fn parse_prefix_expression(parser: &mut Parser) -> Expression {
    let token = parser.cur_token.clone();
    let operator = parser.cur_token.literal.clone();

    parser.next_token();

    let right = Box::new(match parser.parse_expression(Precedence::Prefix) {
        Some(exp) => exp,
        None => return Expression::None,
    });

    Expression::PrefixExpression(expression::PrefixExpression {
        token,
        operator,
        right,
    })
}

pub fn parse_infix_expression(parser: &mut Parser, left: Box<Expression>) -> Expression {
    let token = parser.cur_token.clone();
    let operator = parser.cur_token.literal.clone();

    let precedence = parser.cur_precedence();
    parser.next_token();

    let right = Box::new(match parser.parse_expression(precedence) {
        Some(exp) => exp,
        None => Expression::None,
    });

    return Expression::InfixExpression(expression::InfixExpression {
        token,
        operator,
        right,
        left,
    });
}

pub fn parse_boolean(parser: &mut Parser) -> Expression {
    Expression::BooleanExpression(expression::Boolean {
        token: parser.cur_token.clone(),
        value: parser.cur_token_is(TokenType::True),
    })
}

pub fn parse_grouped_expression(parser: &mut Parser) -> Expression {
    parser.next_token();

    let exp = match parser.parse_expression(Precedence::Lowest) {
        Some(exp) => exp,
        None => return Expression::None,
    };

    if !parser.expect_peek(TokenType::RParen) {
        return Expression::None;
    }

    exp
}

pub fn parse_if_expression(parser: &mut Parser) -> Expression {
    let token = parser.cur_token.clone();

    if !parser.expect_peek(TokenType::LParen) {
        return Expression::None;
    };

    parser.next_token();
    let condition = match parser.parse_expression(Precedence::Lowest) {
        Some(exp) => Box::new(exp),
        None => Box::new(Expression::None),
    };

    if !parser.expect_peek(TokenType::RParen) {
        return Expression::None;
    };

    if !parser.expect_peek(TokenType::LBrace) {
        return Expression::None;
    };

    let consequence = parse_block_statement(parser);

    let mut if_exp = expression::IfExpression {
        token,
        condition,
        consequence,
        alternative: None,
    };

    if parser.peek_token_is(&TokenType::Else) {
        parser.next_token();

        if !parser.peek_token_is(&TokenType::LBrace) {
            return Expression::None;
        }

        if_exp.alternative = Some(parse_block_statement(parser));
    }

    Expression::IfExpression(if_exp)
}

pub fn parse_block_statement(parser: &mut Parser) -> statement::BlockStatement {
    let token = parser.cur_token.clone();

    let mut statements: Vec<Statement> = Vec::new();

    parser.next_token();

    while !parser.cur_token_is(TokenType::RBrace) && !parser.cur_token_is(TokenType::EOF) {
        let stmt = parser.parse_statement();

        if let Some(stmt) = stmt {
            statements.push(stmt);
        }

        parser.next_token();
    }

    statement::BlockStatement { token, statements }
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn precedences(token: &TokenType) -> Precedence {
        match token {
            TokenType::Eq => Precedence::Equals,
            TokenType::NotEq => Precedence::Equals,
            TokenType::Lt => Precedence::LessGreater,
            TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Slash => Precedence::Product,
            TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}
