use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;
use std::rc::Rc;

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

    fn next_token(mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(self) -> Option<ast::Program> {
        None
    }
}
