use crate::ast::{
    Boolean, Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral,
    LetStatement, PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    l: Box<Lexer>,

    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

// TODO: Explore making parse functions return a result
// errors maybe should be a list of Errors and maybe we should have our own error type

type PrefixParseFn = fn(&mut Parser) -> Expression;
type InfixParseFn = fn(&mut Parser, Box<Expression>) -> Expression;

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

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self
                .parse_let_statement()
                .map(|stmt| Statement::LetStatement(stmt)),
            TokenType::Return => self
                .parse_return_statement()
                .map(|stmt| Statement::ReturnStatement(stmt)),
            _ => self
                .parse_expression_statement()
                .map(|stmt| Statement::ExpressionStatement(stmt)),
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
            value: Expression::None,
        })
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

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            return_value: Expression::None,
        };

        self.next_token();

        Some(stmt)
    }

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let stmt = ExpressionStatement {
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
            _ => None,
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
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

fn parse_identifier(parser: &mut Parser) -> Expression {
    Expression::Identifier(Identifier {
        token: parser.cur_token.clone(),
        value: parser.cur_token.literal.clone(),
    })
}

fn parse_integer_literal(parser: &mut Parser) -> Expression {
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

    Expression::IntegerLiteral(IntegerLiteral {
        token: parser.cur_token.clone(),
        value,
    })
}

fn parse_prefix_expression(parser: &mut Parser) -> Expression {
    let token = parser.cur_token.clone();
    let operator = parser.cur_token.literal.clone();

    parser.next_token();

    let right = Box::new(match parser.parse_expression(Precedence::Prefix) {
        Some(exp) => exp,
        None => return Expression::None,
    });

    Expression::PrefixExpression(PrefixExpression {
        token,
        operator,
        right,
    })
}

fn parse_infix_expression(parser: &mut Parser, left: Box<Expression>) -> Expression {
    let token = parser.cur_token.clone();
    let operator = parser.cur_token.literal.clone();

    let precedence = parser.cur_precedence();
    parser.next_token();

    let right = Box::new(match parser.parse_expression(precedence) {
        Some(exp) => exp,
        None => Expression::None,
    });

    return Expression::InfixExpression(InfixExpression {
        token,
        operator,
        right,
        left,
    });
}

fn parse_boolean(parser: &mut Parser) -> Expression {
    Expression::BooleanExpression(Boolean {
        token: parser.cur_token.clone(),
        value: parser.cur_token_is(TokenType::True),
    })
}

fn parse_grouped_expression(parser: &mut Parser) -> Expression {
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

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    fn precedences(token: &TokenType) -> Precedence {
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
