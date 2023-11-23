use rust_monkey::repl;
use std::io;
use whoami;

fn main() {
    let user = whoami::username();
    println!("Hello {}! This is the monkey programing language!", user);
    println!("Feel free to type in some commands");
    match repl::start(io::stdin(), io::stdout()) {
        io::Result::Ok(_) => (),
        io::Result::Err(_err) => (),
    }
}
