mod helper;

use crate::ast::expression;
use crate::ast::statement;
use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use helper::*;

#[derive(Debug)]
pub struct Parser {
    l: Box<Lexer>,

    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

// TODO: Explore making parse functions return a result
// errors maybe should be a list of Errors and maybe we should have our own error type

impl Parser {
    pub fn new(mut lex: Box<Lexer>) -> Self {
        let l = lex.as_mut();
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Self {
            cur_token,
            peek_token,
            l: lex,
            errors: vec![],
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program { statements: vec![] };

        while self.cur_token.token_type != TokenType::EOF {
            match self.parse_statement() {
                Some(stmt) => program.statements.push(stmt),
                None => (),
            };
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<statement::Statement> {
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

    fn parse_let_statement(&mut self) -> Option<statement::Let> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = expression::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.to_string(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = match self.parse_expression(Precedence::Lowest) {
            Some(exp) => exp,
            None => panic!("was expecting Some(Expression) got None"),
        };

        Some(statement::Let { token, name, value })
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, t: TokenType) {
        self.errors.push(format!(
            "expected next token to be {} but got {} instead",
            t, self.peek_token.token_type
        ));
    }

    fn parse_return_statement(&mut self) -> Option<statement::Return> {
        let token = self.cur_token.clone();

        self.next_token();

        let return_value = match self.parse_expression(Precedence::Lowest) {
            Some(exp) => exp,
            None => panic!("Was expecting Some(Expression) got None"),
        };

        Some(statement::Return {
            token,
            return_value,
        })
    }

    fn parse_expression_statement(&mut self) -> Option<statement::Expression> {
        let stmt = statement::Expression {
            token: self.cur_token.clone(),
            expression: self.parse_expression(Precedence::Lowest)?,
        };
        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    fn prefix_parse_fns(&self, token: TokenType) -> Option<PrefixParseFn> {
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

    fn infix_parse_fns(&self, token: TokenType) -> Option<InfixParseFn> {
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

    fn parse_expression(&mut self, precedence: Precedence) -> Option<expression::Expression> {
        let prefix = self.prefix_parse_fns(self.cur_token.token_type.clone())?;

        let mut left_exp = Box::new(prefix(self));

        while !self.peek_token_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = match self.infix_parse_fns(self.peek_token.token_type.clone()) {
                Some(fun) => fun,
                None => return Some(*left_exp),
            };

            self.next_token();

            left_exp = Box::new(infix(self, left_exp));
        }

        Some(*left_exp)
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::precedences(&self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        Precedence::precedences(&self.cur_token.token_type)
    }
}
