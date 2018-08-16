use chunks::Chunk;

pub struct VM {
    chunk:Chunk
    ip:u8
}

pub enum VMResult {
    CompileError,
    RuntimeError,
    Ok
}
impl VM {
    pub fn new() -> Self {
        VM {
            chunk:Chunk::new(),
            ip:0
        }
    }

    pub fn interpret(&mut self,chunk:Chunk) -> VMResult {
        self.chunk = chunk;
        self.ip = chunk.code.len();
        self.run()
        VMResult::Ok
    }
}