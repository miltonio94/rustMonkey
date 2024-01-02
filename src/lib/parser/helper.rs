use crate::ast::expression::{self, Expression};
use crate::ast::statement::{self, Statement};
use crate::parser::Parser;
use crate::token::TokenType;

pub type ParserError<T> = Result<T, String>;

pub type PrefixParseFn = fn(&mut Parser) -> ParserError<Expression>;
pub type InfixParseFn = fn(&mut Parser, Box<Expression>) -> ParserError<Expression>;

pub fn parse_identifier(parser: &mut Parser) -> ParserError<Expression> {
    Ok(Expression::Identifier(expression::Identifier {
        token: parser.cur_token.clone(),
        value: parser.cur_token.literal.clone(),
    }))
}

pub fn parse_integer_literal(parser: &mut Parser) -> ParserError<Expression> {
    let value: i64 = match parser.cur_token.literal.parse() {
        Ok(val) => val,
        Err(err) => {
            let error = format!(
                "could not parse {} as integer. Err: {err}",
                parser.cur_token.literal.clone(),
            );
            parser.errors.push(error.clone());
            return Err(error);
        }
    };

    Ok(Expression::IntegerLiteral(expression::IntegerLiteral {
        token: parser.cur_token.clone(),
        value,
    }))
}

pub fn parse_prefix_expression(parser: &mut Parser) -> ParserError<Expression> {
    let token = parser.cur_token.clone();
    let operator = parser.cur_token.literal.clone();

    parser.next_token();

    let right = Box::new(parser.parse_expression(Precedence::Prefix)?);

    Ok(Expression::Prefix(expression::Prefix {
        token,
        operator,
        right,
    }))
}

pub fn parse_infix_expression(
    parser: &mut Parser,
    left: Box<Expression>,
) -> ParserError<Expression> {
    let token = parser.cur_token.clone();
    let operator = parser.cur_token.literal.clone();

    let precedence = parser.cur_precedence();
    parser.next_token();

    let right = Box::new(parser.parse_expression(precedence)?);

    Ok(Expression::Infix(expression::Infix {
        token,
        operator,
        right,
        left,
    }))
}

pub fn parse_boolean(parser: &mut Parser) -> ParserError<Expression> {
    Ok(Expression::Boolean(expression::Boolean {
        token: parser.cur_token.clone(),
        value: parser.cur_token_is(TokenType::True),
    }))
}

pub fn parse_grouped_expression(parser: &mut Parser) -> ParserError<Expression> {
    parser.next_token();

    let exp = parser.parse_expression(Precedence::Lowest)?;

    if !parser.expect_peek(TokenType::RParen) {
        return Err("Was expecting RParen".to_string());
    }

    Ok(exp)
}

pub fn parse_if_expression(parser: &mut Parser) -> ParserError<Expression> {
    let token = parser.cur_token.clone();

    if !parser.expect_peek(TokenType::LParen) {
        return Err("Was expecting LParen".to_string());
    };

    parser.next_token();
    let condition = Box::new(parser.parse_expression(Precedence::Lowest)?);

    if !parser.expect_peek(TokenType::RParen) {
        return Err("Was expecting RParen".to_string());
    };

    if !parser.expect_peek(TokenType::LBrace) {
        return Err("Was expecting LBrace".to_string());
    };

    let consequence = parse_block_statement(parser)?;

    let mut if_exp = expression::If {
        token,
        condition,
        consequence,
        alternative: None,
    };

    if parser.peek_token_is(&TokenType::Else) {
        parser.next_token();

        if !parser.peek_token_is(&TokenType::LBrace) {
            return Err("Was expecting LBrace".to_string());
        }

        if_exp.alternative = Some(parse_block_statement(parser)?);
    }

    Ok(Expression::If(if_exp))
}

pub fn parse_block_statement(parser: &mut Parser) -> ParserError<statement::Block> {
    let token = parser.cur_token.clone();

    let mut statements: Vec<Statement> = Vec::new();

    parser.next_token();

    while !parser.cur_token_is(TokenType::RBrace) && !parser.cur_token_is(TokenType::EOF) {
        let stmt = parser.parse_statement();

        if let Ok(stmt) = stmt {
            statements.push(stmt);
        }

        parser.next_token();
    }

    Ok(statement::Block { token, statements })
}

pub fn parse_function_literal(parser: &mut Parser) -> ParserError<expression::Expression> {
    let token = parser.cur_token.clone();

    if !parser.expect_peek(TokenType::LParen) {
        return Err("Was expecting LParen next got none".to_string());
    };

    let parameters = parse_function_parameters(parser)?;

    if !parser.expect_peek(TokenType::LBrace) {
        return Err("Was expecting LBrace next got none".to_string());
    };

    let body = parse_block_statement(parser)?;

    Ok(Expression::FunctionLiteral(expression::FunctionLiteral {
        token,
        parameters,
        body,
    }))
}

fn parse_function_parameters(parser: &mut Parser) -> ParserError<Vec<expression::Identifier>> {
    let mut identifier: Vec<expression::Identifier> = Vec::new();

    if parser.peek_token_is(&TokenType::RParen) {
        parser.next_token();
        return Ok(identifier);
    };

    parser.next_token();

    identifier.push(expression::Identifier {
        token: parser.cur_token.clone(),
        value: parser.cur_token.literal.clone(),
    });

    while parser.peek_token_is(&TokenType::Comma) {
        parser.next_token();
        parser.next_token();

        identifier.push(expression::Identifier {
            token: parser.cur_token.clone(),
            value: parser.cur_token.literal.clone(),
        });
    }

    if !parser.expect_peek(TokenType::RParen) {
        return Err("Was expecting next token to be RParen".to_string());
    }

    Ok(identifier)
}

pub fn parse_call_expression(
    parser: &mut Parser,
    function: Box<Expression>,
) -> ParserError<Expression> {
    let arguments = parse_call_arguments(parser);

    Ok(Expression::Call(expression::Call {
        token: parser.cur_token.clone(),
        function,
        arguments,
    }))
}

fn parse_call_arguments(parser: &mut Parser) -> Vec<Expression> {
    let mut args: Vec<Expression> = Vec::new();

    if parser.peek_token_is(&TokenType::RParen) {
        parser.next_token();
        return args;
    }

    parser.next_token();

    if let Ok(arg) = parser.parse_expression(Precedence::Lowest) {
        args.push(arg);
    }

    while parser.peek_token_is(&TokenType::Comma) {
        parser.next_token();
        parser.next_token();

        if let Ok(arg) = parser.parse_expression(Precedence::Lowest) {
            args.push(arg);
        }
    }

    if !parser.expect_peek(TokenType::RParen) {
        panic!("Was expecting a closing bracket");
    };

    args
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
            TokenType::Eq => Self::Equals,
            TokenType::NotEq => Self::Equals,
            TokenType::Lt => Self::LessGreater,
            TokenType::Gt => Self::LessGreater,
            TokenType::Plus => Self::Sum,
            TokenType::Minus => Self::Sum,
            TokenType::Slash => Self::Product,
            TokenType::Asterisk => Self::Product,
            TokenType::LParen => Self::Call,
            _ => Precedence::Lowest,
        }
    }
}
