use std::io::{self, BufRead, BufReader, Read, stdout, Write};

use error::*;
use scanner::*;
use token::*;

mod scanner;
mod token;
mod error;
mod lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: jlax [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    if run(buf.as_str()).is_err() {
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(&line.as_str());
        } else {
            break;
        }
        print!("> ");
        let _ = stdout().flush();
    }
}

fn run(source: &str) -> Result<(), LaxError> {
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
