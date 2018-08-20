#![feature(nll)]
#[macro_use]
mod macros;
mod chunks;
mod op;
mod pos;
mod scanner;
mod token;
mod util;
mod value;
mod vm;
mod compiler;
mod error;

use chunks::Chunk;
use scanner::Lexer;
use op::opcode;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use vm::{VMResult, VM};
use compiler::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            repl()
        }
        2 => {
            run_file(&args[1])

        }

        _ => println!("Usage: rlox [path]"),
    }

    // let mut chunk = Chunk::new();

    // let constant = chunk.add_constant(1.2);
    // chunk.write(opcode::CONSTANT, 123);
    // chunk.write(constant as u8, 123);
    // chunk.write(opcode::NEGATE,123);
    // chunk.write(opcode::RETURN, 123);

    // let mut vm = VM::new(&chunk);

    // vm.interpret();
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

    let mut lex = Lexer::new(&input);

    let tokens = lex.lex().expect("Couldn't complete the lexing");

    let mut compiler = Compiler::new(tokens);

    compiler.compile().expect("Compilation Succedded");

    println!("{:?}",compiler);




}

fn interpret(file: &str) -> VMResult {
    VMResult::Ok
}
