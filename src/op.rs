pub mod opcode {
    pub const RETURN: u8 = 0x0;
    pub const CONSTANT: u8 = 0x1;
    pub const NEGATE: u8 = 0x2;
    pub const ADD: u8 = 0x3;
    pub const SUB: u8 = 0x4;
    pub const MUL: u8 = 0x5;
    pub const DIV: u8 = 0x6;
    pub const NIL: u8 = 0x7;
    pub const TRUE: u8 = 0x8;
    pub const FALSE: u8 = 0x9;
    pub const NOT: u8 = 0x10;
    pub const EQUAL: u8 = 0x11;
    pub const GREATER: u8 = 0x12;
    pub const LESS: u8 = 0x13;
}
