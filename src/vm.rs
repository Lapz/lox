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
            stack: [Value::nil(); 256],
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
                for byte in self.stack[1..self.stack_top].iter() {
                    print!("[{}]", byte);
                }

                print!("\n")
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
                opcode::NIL => self.push(Value::nil()),
                opcode::TRUE => self.push(Value::bool(true)),
                opcode::FALSE => self.push(Value::bool(false)),
                opcode::NEGATE => {
                    if !self.peek(1).is_number() {
                        return self.runtime_error("Unary `-` operand must be a number.");
                    }

                    let v = Value::number(-self.pop().as_number());
                    self.push(v);
                }
                opcode::ADD => binary_op!(+,number,self),
                opcode::SUB => binary_op!(-,number,self),
                opcode::MUL => binary_op!(*,number,self),
                opcode::DIV => binary_op!(/,number,self),
                opcode::NOT => {
                    let value = Value::bool(self.pop().is_falsey());
                    self.push(value);
                }
                opcode::EQUAL => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(Value::bool(a.is_equal(&b)))
                }
                opcode::GREATER => binary_op!(>,bool,self),
                opcode::LESS => binary_op!(<,bool,self),
                _ => return VMResult::RuntimeError,
            }
        }
    }

    fn concat(&mut self) {
        let b = self.pop().as_string();
        let a = self.pop().as_string();

        let length = a.length + b.length;
        


    }

    fn runtime_error(&self, msg: &str) -> VMResult {
        let instructon = self.chunk.code.len() - self.ip;

        eprintln!("[line {}] error: {}", self.chunk.lines[instructon], msg);

        VMResult::RuntimeError
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

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack_top - distance]
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
