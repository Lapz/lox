use scanner::Lexer;
use token::{TokenType,Token};
use std::iter::Peekable;
use pos::Spanned;
use chunks::Chunk;
use opcode;
use std::collections::{VecDeque};
use value::Value;

#[derive(Debug)]
pub struct Compiler<'a> {
    chunk:Option<Chunk>,
    chunks:Vec<Chunk>,
    current_token:Option<Spanned<Token<'a>>>,
    tokens:VecDeque<Spanned<Token<'a>>>,
    line:u32,
}



impl <'a> Compiler<'a> {
    pub fn new(tokens:Vec<Spanned<Token<'a>>>) -> Self {
        let mut tokens = tokens.into_iter().collect::<VecDeque<_>>();
        let current_token = tokens.pop_front();
        
        Compiler {
            chunk:Some(Chunk::new()),
            chunks:vec![],
            tokens,
            current_token,
            line:0
        }
    }

    pub fn compile(&mut self) -> Result<(),()> {

       
        self.expression().unwrap();

        self.check(TokenType::EOF, "Expected EOF")?;
        self.end_chunk();
        Ok(())

    }

    pub fn emit_byte(&mut self,byte:u8) {
        self.chunk.as_mut().unwrap().write(byte,self.line)
    }

    pub fn emit_bytes(&mut self,byte1:u8,byte2:u8) {
        // println!("{:?}",self.chunk);
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn emit_constant(&mut self,constant:Value) -> Result<(),()> {
        let value = self.make_constant(constant)?;
        self.emit_bytes(opcode::CONSTANT,value);
        Ok(())
    }

    pub fn make_constant(&mut self,value:Value) -> Result<u8,()> {
        let index = self.chunk.as_mut().unwrap().add_constant(value);

        if index > 256 {
            Err(())
        }else {
           Ok(index as u8)
        }
        
    }

    pub fn end_chunk(&mut self) {
        let mut current_chunk = self.chunk.take().unwrap();

       

        current_chunk.write(opcode::RETURN, self.line);

        self.chunks.push(current_chunk);
        self.chunk = Some(Chunk::new());
    }

    pub fn advance(&mut self) -> Result<Spanned<Token<'a>>,()> {

       match self.current_token.take() {
           Some(token) => {
               self.line = token.span.start.line;
               self.current_token = self.tokens.pop_front();

               Ok(token)
           },
           None => Err(())
       }
    }

    pub fn check(&mut self,ty:TokenType<'a>,msg:&str) -> Result<(),()> {
        if self.peek() == Some(&ty) {
            self.advance()?;
            Ok(())

        }else {
            Err(())
        }
    }

    pub fn peek(&self) -> Option<&TokenType<'a>> {
        self.tokens.front().map(|spanned|&spanned.value.ty)
    }


    // ========== PARSING ===========

    pub fn expression(&mut self)  -> Result<(),()> {
        self.number()
    }

    pub fn number(&mut self) -> Result<(),()> {
       
        let token = self.current_token.as_ref().unwrap();

        match &token.value.ty {
            &TokenType::NUMBER(ref num) => {
                self.emit_constant(*num)?;
                Ok(())
            },
            _ => Err(())
        }
    }

    
}


