#![feature(nll)]
extern crate libc;
#[macro_use]
mod value;
#[macro_use]
mod macros;
mod chunks;
mod compiler;
mod error;
mod object;
mod op;
mod pos;
mod scanner;
mod token;
mod vm;

use compiler::Compiler;
use error::Reporter;
use object::StringObject;
use op::opcode;
use scanner::Lexer;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use vm::{VMResult, VM};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),

        _ => println!("Usage: rlox [path]"),
    }
}

fn repl() {
    loop {
        let _ = io::stdout().write(b"lexer>> ");
        let _ = io::stdout().flush();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read the input");
    }
}

fn run_file(path: &str) {
    let mut file = File::open(path).expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let input = contents.trim();

    if contents.is_empty() {
        ::std::process::exit(0)
    }

    let reporter = Reporter::new();

    let mut lex = Lexer::new(&input, reporter.clone());

    let tokens = match lex.lex() {
        Ok(tokens) => tokens,
        Err(_) => {
            reporter.emit(&input);
            ::std::process::exit(64)
        }
    };

    let mut compiler = Compiler::new(reporter.clone(), tokens);

    if let Err(_) = compiler.compile() {
        reporter.emit(&input);
        println!("{:#?}", compiler);
    }

    let mut vm = VM::new(&compiler.chunks[0]);

    vm.interpret();

    println!(
        "{:?}",
        StringObject::new("hello world\0".as_ptr() as *const ::libc::c_char, 12)
    )
}
