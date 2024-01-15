mod helper;

use crate::ast::expression;
use crate::ast::statement;
use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use helper::*;

#[derive(Debug)]
pub struct Parser<'a> {
    l: Lexer,

    cur_token: Token<'a>,
    peek_token: Token<'a>,
    errors: Vec<String>,
}

impl Parser<'_> {
    pub fn new<'a>(mut lex: Lexer) -> ParserError<Parser<'a>> {
        let l = lex;
        let cur_token = l.next_token()?;
        let peek_token = l.next_token()?;
        Ok(Self {
            cur_token,
            peek_token,
            l: lex,
            errors: vec![],
        })
    }

    fn next_token(&mut self) -> ParserError<()> {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token()?;

        Ok(())
    }

    pub fn parse_program(&mut self) -> ParserError<Program> {
        let mut program = Program { statements: vec![] };

        while self.cur_token.token_type != TokenType::EOF {
            program.statements.push(self.parse_statement()?);
            self.next_token()?;
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> ParserError<statement::Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self
                .parse_let_statement()
                .map(|stmt| statement::Statement::Let(stmt)),
            TokenType::Return => self
                .parse_return_statement()
                .map(|stmt| statement::Statement::Return(stmt)),
            _ => self
                .parse_expression_statement()
                .map(|stmt| statement::Statement::Expression(stmt)),
        }
    }

    fn parse_let_statement(&mut self) -> ParserError<statement::Let> {
        let token = self.cur_token.clone();

        if !self.expect_peek(&TokenType::Ident)? {
            return Err("Expected Ident".to_string());
        }

        let name = expression::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.to_string(),
        };

        if !self.expect_peek(&TokenType::Assign)? {
            return Err("Expected Assign".to_string());
        }

        self.next_token()?;

        let value = self.parse_expression(Precedence::Lowest).ok();

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token()?;
        }

        Ok(statement::Let { token, name, value })
    }

    fn cur_token_is(&self, t: &TokenType) -> bool {
        self.cur_token.token_type == *t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn expect_peek(&mut self, t: &TokenType) -> ParserError<bool> {
        if self.peek_token_is(t) {
            self.next_token()?;
            Ok(true)
        } else {
            self.peek_error(t);
            Ok(false)
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, t: &TokenType) {
        self.errors.push(format!(
            "expected next token to be {} but got {} instead",
            t, self.peek_token.token_type
        ));
    }

    fn parse_return_statement(&mut self) -> ParserError<statement::Return> {
        let token = self.cur_token.clone();

        self.next_token()?;

        let return_value = self.parse_expression(Precedence::Lowest).ok();

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token()?;
        }

        Ok(statement::Return {
            token,
            return_value,
        })
    }

    fn parse_expression_statement(&mut self) -> ParserError<statement::Expression> {
        let peek_token_is_semi = self.peek_token_is(&TokenType::Semicolon);

        let stmt = statement::Expression {
            token: self.cur_token.clone(),
            expression: self.parse_expression(Precedence::Lowest)?,
        };

        if peek_token_is_semi {
            self.next_token()?;
        }

        Ok(stmt)
    }

    fn prefix_parse_fns(&self, token: &TokenType) -> Option<PrefixParseFn> {
        match token {
            TokenType::Ident => Some(parse_identifier),
            TokenType::Int => Some(parse_integer_literal),
            TokenType::Bang => Some(parse_prefix_expression),
            TokenType::Minus => Some(parse_prefix_expression),
            TokenType::True => Some(parse_boolean),
            TokenType::False => Some(parse_boolean),
            TokenType::LParen => Some(parse_grouped_expression),
            TokenType::If => Some(parse_if_expression),
            TokenType::Function => Some(parse_function_literal),

            _ => None,
        }
    }

    fn infix_parse_fns(&self, token: &TokenType) -> Option<InfixParseFn> {
        match token {
            TokenType::Plus => Some(parse_infix_expression),
            TokenType::Minus => Some(parse_infix_expression),
            TokenType::Slash => Some(parse_infix_expression),
            TokenType::Asterisk => Some(parse_infix_expression),
            TokenType::Eq => Some(parse_infix_expression),
            TokenType::NotEq => Some(parse_infix_expression),
            TokenType::Lt => Some(parse_infix_expression),
            TokenType::Gt => Some(parse_infix_expression),
            TokenType::LParen => Some(parse_call_expression),
            _ => None,
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParserError<expression::Expression> {
        let prefix = self
            .prefix_parse_fns(&self.cur_token.token_type)
            .ok_or(&format!("couldn't find function for {:#?}", self.cur_token))?;

        let mut left_exp = Box::new(prefix(self)?);

        while !self.peek_token_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = match self.infix_parse_fns(&self.peek_token.token_type) {
                Some(fun) => fun,
                None => return Ok(*left_exp),
            };

            self.next_token()?;

            left_exp = Box::new(infix(self, left_exp)?);
        }

        Ok(*left_exp)
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::precedences(&self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        Precedence::precedences(&self.cur_token.token_type)
    }
}
