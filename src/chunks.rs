use value::Value;

type Line = u32;
#[derive(Debug)]
/// Awrapper around an array of bytesc
pub struct Chunk {
    code:Vec<u8>,
    constants:Vec<Value>,
    lines:Vec<Line>,
}

/// Bytecode Operands
pub enum OpCode {
    Return,
    Constant
}

pub trait TryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from(T) -> Result<Self, Self::Error>;
}


impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code:Vec::new(),
            constants:Vec::new(),
            lines:Vec::new()
        }
    }


    pub fn add_constant(&mut self,value:Value) -> usize {
        self.constants.push(value);
        self.constants.len() -1
    }

    pub fn write(&mut self,byte:u8,line:Line) {
        self.code.push(byte);
        self.lines.push(line)
    }


    #[cfg(feature = "debug")]
    pub fn disassemble(&self,name:&str) {
        println!("== {} ==\n",name);

        let mut i = 0;

        while i < self.code.len() {
            i =self.disassemble_instruction(i);
        }

    }

    #[cfg(feature = "debug")]
    pub fn disassemble_instruction(&self,offset:usize) -> usize {
        print!("{:04}",offset);

        if offset > 0 && self.lines[offset] ==self.lines[offset-1] {
            print!("   | ")
        } else {
            print!("{:4} ",self.lines[offset])
        }

        let instruction = self.code[offset];

        match OpCode::try_from(instruction) {
            Ok(OpCode::Return) => simple_instruction("OPCODE::RETURN", offset),
            Ok(OpCode::Constant) => self.constant_instruction("OPCODE::CONSTANT", offset),
            Err(ref unknown) => {
                println!("UNKOWN OPCODE {}",unknown);

                offset +1
            }
        } 
    }
    pub fn constant_instruction(&self,name:&str,offset:usize) -> usize {
        let constant = self.code[offset+1];
        println!("{:16}{:4} '{}'",name,constant,self.constants[constant as usize]);
        offset +2
    }


}

pub fn simple_instruction(name:&str,offset:usize) -> usize {
    println!("{}",name);
    offset +1
}








pub mod opcode {
    use super::{TryFrom,OpCode};
    
    pub const RETURN:u8 = 0x0;
    pub const CONSTANT:u8 = 0x1;


    impl TryFrom<u8> for OpCode {
        type Error = u8;

        fn try_from(original:u8) -> Result<OpCode,Self::Error> {
            match original {
                RETURN => Ok(OpCode::Return),
                CONSTANT => Ok(OpCode::Constant),
                _ => Err(original)
            }
        }
    }

}