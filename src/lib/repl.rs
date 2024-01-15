use crate::lexer;
use crate::parser::Parser;
use std::io::Write;
use std::io::{self, BufRead};

static PROMPT: &str = ">> ";

pub fn start(io_in: io::Stdin, mut io_out: io::Stdout) -> io::Result<()> {
    loop {
        let _ = io_out.lock();
        io_out.write_all(PROMPT.as_bytes())?;
        io_out.flush()?;

        let mut scanner = io_in.lock();
        let mut line = String::new();
        scanner.read_line(&mut line)?;

        let l = lexer::Lexer::new(&line);
        let mut p = match Parser::new(l) {
            Ok(p) => p,
            Err(_e) => {
                io_out.write_all("Could not create a parser".as_bytes())?;
                continue;
            }
        };

        if let Ok(program) = p.parse_program() {
            io_out.write_all(program.to_string().as_bytes())?;
            io_out.write_all("\n".as_bytes())?;
        } else {
            let errors = p.errors();
            print_parse_errors(&io_out, &errors)?;
            continue;
        }
    }
}

fn print_parse_errors(mut io_out: &io::Stdout, errors: &Vec<String>) -> io::Result<()> {
    for msg in errors.iter() {
        io_out.write_all(msg.as_bytes())?;
        io_out.write_all("\n".as_bytes())?;
    }
    Ok(())
}
