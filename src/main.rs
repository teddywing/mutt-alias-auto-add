//! alias-auto-add
//!
//! Adds unique aliases to a Mutt alias file.
//!
//! Reads an email from STDIN and tries to add an alias for the from address
//! listed. If the given alias already exists, a new unique alias is generated
//! and used instead. This allows us to always capture an alias even if a
//! person has multiple email addresses.

use std::env;
use std::io::{self, BufRead, Write};

mod alias;

#[cfg(test)]
mod tests;

use alias::*;

fn print_usage(program: &str) {
    println!("Usage: {} FILE", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let file = if args.len() > 1 {
        &args[1]
    } else {
        print_usage(&program);
        return;
    };

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Error reading from STDIN");

        // Write the message to STDOUT so that it can be properly read by Mutt
        println!("{}", line);

        if line.starts_with("From: ") {
            let mut alias = Alias::new(&line);
            match alias.write_to_file(&file) {
                Ok(_)  => continue,
                Err(e @ AliasSearchError::NotFound) | Err(e @ AliasSearchError::EmailExists) =>
                    io::stderr().write(e.to_string().as_bytes()).ok(),
                Err(e) => io::stderr().write(e.to_string().as_bytes()).ok(),
            };
        }
    }
}
