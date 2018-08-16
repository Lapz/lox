/// Bytecode Operands
pub enum OpCode {
    Return,
    Constant,
    Negate,
    Add,
    Sub,
    Div,
    Mul
}

pub mod opcode {
    use super::{OpCode};
    use util::TryFrom;

    pub const RETURN: u8 = 0x0;
    pub const CONSTANT: u8 = 0x1;
    pub const NEGATE:u8 = 0x2;
    pub const ADD:u8 = 0x3;
    pub const SUB:u8 = 0x4;
    pub const MUL:u8 = 0x5;
    pub const DIV:u8 = 0x6;

    

}
