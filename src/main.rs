#[macro_use]
mod macros;
mod util;
mod op;
mod chunks;
mod value;
mod vm;



use chunks::{ Chunk,};
use op::{opcode};
use vm::VM;

fn main() {

    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.2);
    chunk.write(opcode::CONSTANT, 123);
    chunk.write(constant as u8, 123);
    chunk.write(opcode::NEGATE,123);
    chunk.write(opcode::RETURN, 123);


    let mut vm = VM::new(&chunk);

    vm.interpret();

}
