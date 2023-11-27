use crate::ast::{self, Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

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

        while self.cur_token != Token::EOF {
            match self.parse_statement() {
                Some(stmt) => program.statements.push(stmt),
                None => (),
            };
            self.next_token();
        }

        None
    }

    fn parse_statement(&self) -> Option<Statement> {
        match self.cur_token {
            Token::Let => self
                .parse_let_statement()
                .map(|stmt| Statement::LetStatement(stmt)),
            _ => None,
        }
    }

    fn parse_let_statement(&self) -> Option<LetStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(Token::Ident(vec![])) {
            return None;
        }

        let name = todo!("do the thing");
        None
    }

    fn cur_token_is(&self, t: Token) -> bool {
        todo!("do the thing")
    }

    fn peek_token_is(&self, t: Token) -> bool {
        todo!("do the thing")
    }

    fn expect_peek(&self, t: Token) -> bool {
        todo!("do the thing")
    }
}
