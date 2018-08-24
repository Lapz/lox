use compiler::Compiler;
use token::{TokenType,RuleToken};
use op::opcode;
use std::fmt::Debug;
type ParseResult<T> = Result<T,()>;

pub trait PrefixParser:Debug {
    fn parse(&self,parser:&mut Compiler,rule:RuleToken) -> ParseResult<()>;
}

#[derive(Debug)]
pub struct NameParser;

impl PrefixParser for NameParser {
    fn parse(&self,parser:&mut Compiler,token:RuleToken) -> ParseResult<()> {
        
        // let token = parser.advance().expect("No Token");
        match parser.current()? {
            &TokenType::NUMBER(ref number) => parser.emit_constant(*number)?,
            _ => {
                let msg = format!("Expected a literal");

                // parser.error(msg,token.span);
            }
        }

        println!("{:?}",parser);

        Ok(())

    }
}

#[derive(Debug)]
pub struct UnaryParser;

impl PrefixParser for UnaryParser {
    fn parse(&self,parser:&mut Compiler,token:RuleToken) -> ParseResult<()> {
        // parser.advance()?;

        let op = parser.get_op_ty()?;
        parser.advance().expect("Token Gone");

        match op {
            RuleToken::MINUS => {
                parser.emit_byte(opcode::NEGATE);
                parser.expression()?;
               
                Ok(())
            },

            _ => unreachable!()
        }

    }
}