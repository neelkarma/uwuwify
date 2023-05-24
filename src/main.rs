use std::{
    env::args,
    io::{stdin, Read},
};

use atty::Stream;
use uwuwify::uwuwify;

/// returns piped input or command-line args if present
fn get_immediate_input() -> Option<String> {
    if atty::isnt(Stream::Stdin) {
        let mut input = String::new();
        stdin()
            .read_to_string(&mut input)
            .expect("Error reading from stdin");
        Some(input)
    } else {
        args().nth(1)
    }
}

fn main() {
    // handle immediate input
    if let Some(input) = get_immediate_input() {
        print!("{}", uwuwify(&input));
        return;
    }

    // repl
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        print!("{}", uwuwify(&input));
    }
}
