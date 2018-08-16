use value::Value;
use util::TryFrom;


type Line = u32;

#[derive(Debug)]
/// Awrapper around an array of bytesc
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<Line>,
}

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


impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn write(&mut self, byte: u8, line: Line) {
        self.code.push(byte);
        self.lines.push(line)
    }

    #[cfg(feature = "debug")]
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==\n", name);

        let mut i = 0;

        while i < self.code.len() {
            i = self.disassemble_instruction(i);
        }
    }

    #[cfg(feature = "debug")]
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04}", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ")
        } else {
            print!("{:4} ", self.lines[offset])
        }

        let instruction = self.code[offset];

        match OpCode::try_from(instruction) {
            Ok(OpCode::Return) => simple_instruction("OPCODE::RETURN", offset),
            Ok(OpCode::Constant) => self.constant_instruction("OPCODE::CONSTANT", offset),
            Ok(OpCode::Negate) => simple_instruction("OPCODE::NEGATE", offset),
            Ok(OpCode::Add) =>simple_instruction("OPCODE::ADD", offset),
            Ok(OpCode::Sub) => simple_instruction("OPCODE::SUB", offset),
            Ok(OpCode::Div) => simple_instruction("OPCODE::DIV", offset),
            Ok(OpCode::Mul) => simple_instruction("OPCODE::MUL", offset),
            Err(ref unknown) => {
                println!("UNKOWN OPCODE {}", unknown);
                offset + 1
            }
        }
    }
    pub fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!(
            "{:16}{:4} '{}'",
            name, constant, self.constants[constant as usize]
        );
        offset + 2
    }
}

pub fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}


impl TryFrom<u8> for OpCode {
        type Error = u8;

        fn try_from(original: u8) -> Result<OpCode, Self::Error> {
            use opcode::*;
            match original {
                RETURN => Ok(OpCode::Return),
                CONSTANT => Ok(OpCode::Constant),
                NEGATE => Ok(OpCode::Negate),
                ADD => Ok(OpCode::Add),
                SUB => Ok(OpCode::Sub),
                MUL => Ok(OpCode::Mul),
                _ => Err(original),
            }
        }
    }