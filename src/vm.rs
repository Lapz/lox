use chunks::Chunk;
use op::opcode;
use value::Value;

const STACK_MAX: usize = 256;

pub struct VM<'a> {
    chunk: &'a Chunk,
    stack: [Value; STACK_MAX],
    stack_top: usize,
    ip: usize,
}

pub enum VMResult {
    CompileError,
    RuntimeError,
    Ok,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack_top: 1,
            stack: [0.0; 256],
        }
    }

    pub fn interpret(&mut self) -> VMResult {
        self.run();
        VMResult::Ok
    }

    pub fn run(&mut self) -> VMResult {
        loop {
            #[cfg(feature = "debug")]
            self.chunk.disassemble("test");

            if cfg!(feature = "stack") {
                println!("[");

                for (i, byte) in self.stack.iter().enumerate() {
                    if i + 1 == self.stack.len() {
                        print!("{}", byte);
                    } else {
                        print!("{},", byte);
                    }
                }

                println!("]");
            }

            match self.read_byte() {
                opcode::RETURN => {
                    println!("{}", self.pop());
                    return VMResult::Ok;
                }
                opcode::CONSTANT => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                opcode::NEGATE => {
                    let v = self.pop();
                    self.push(-v);
                }
                opcode::ADD => binary_op!(+,self),
                opcode::SUB => binary_op!(-,self),
                opcode::MUL => binary_op!(*,self),
                opcode::DIV => binary_op!(/,self),
                _ => return VMResult::RuntimeError,
            }
        }

        VMResult::Ok
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.constants[index]
    }

    fn reset_stack(&mut self) {
        self.stack_top = 0;
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;

        self.stack[self.stack_top]
    }
}
