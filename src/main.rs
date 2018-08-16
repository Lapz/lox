mod chunks;
mod value;

use chunks::{Chunk,TryFrom,opcode};

fn main() {

    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(opcode::CONSTANT,123);
    chunk.write(constant as u8,123);
    chunk.write(opcode::RETURN,123);

    chunk.disassemble("TEST CHUNK");
}
