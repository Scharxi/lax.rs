use std::io::{self, stdout, BufRead, Write};

use clap::{arg, Command};

use ast_generator::ast;
use interpreter::error::LaxError;
use interpreter::parser::Parser;
use interpreter::printer::AstPrinter;
use interpreter::scanner::Scanner;

fn main() -> io::Result<()> {
    let matches = Command::new("Lax Interpreter")
        .version("1.0.0")
        .author("BufferOverflow")
        .about("Console Interface for the Lox Interpreter")
        .arg(arg!([file_path] "File to interpret"))
        .subcommand(
            Command::new("ast")
                .about("Used to generate the ast")
                .arg(arg!(-o --out <PATH> "Specify the output directory")),
        )
        .get_matches();

    if let Some(path) = matches.value_of("file_path") {
        run_file(&path.to_owned()).expect("Could not run file!")
    } else if let Some(("ast", sub_m)) = matches.subcommand() {
        let output = sub_m.value_of("out").unwrap();
        create_ast(output)?;
    } else {
        run_prompt()
    }

    Ok(())
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
    let mut parser = Parser::new(tokens.clone());
    match parser.parse() {
        None => {}
        Some(expr) => {
            println!("AST Printer: \n{}", AstPrinter::new().print(&expr)?);
        }
    }

    Ok(())
}

fn create_ast(outdir: &str) -> io::Result<()> {
    println!("Generating ast...");
    ast::expressions::define_ast(
        outdir,
        "Expr",
        &vec![
            "Binary    : Box<Expr> left, Token operator, Box<Expr> right",
            "Grouping  : Box<Expr> expression",
            "Literal   : Object value",
            "Unary     : Token operator,Box<Expr> right",
        ],
    )?;

    Ok(())
}
