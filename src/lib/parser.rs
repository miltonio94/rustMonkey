use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser {
    l: Box<Lexer>,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lex: Box<Lexer>) -> Self {
        let l = lex.as_mut();
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Self {
            cur_token,
            peek_token,
            l: lex,
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

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self
                .parse_let_statement()
                .map(|stmt| Statement::LetStatement(stmt)),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.to_string(),
        };

        if !self.expect_peek(TokenType::Assign) {
            println!("expect assign");
            return None;
        }

        // TODO: Skipping expression until we encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(LetStatement {
            token,
            name,
            value: vec![],
        })
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
