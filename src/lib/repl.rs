use crate::lexer;
use crate::token;
use std::io::Write;
use std::io::{self, BufRead};

static PROMPT: &str = ">> ";

pub fn start(io_in: io::Stdin, mut io_out: io::Stdout) -> io::Result<()> {
    loop {
        io_out.lock();
        io_out.write_all(PROMPT.as_bytes())?;
        io_out.flush()?;

        let mut scanner = io_in.lock();
        let mut line = String::new();
        scanner.read_line(&mut line)?;

        let mut l = lexer::Lexer::new(line);

        let mut tok = l.next_token();
        while tok.token_type != token::TokenType::EOF {
            let mut out = Vec::new();
            write!(out, "{:?}\n", tok)?;
            io_out.write_all(out.as_slice())?;
            out.flush()?;

            tok = l.next_token();
        }
    }
}
