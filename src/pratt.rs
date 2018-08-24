use compiler::Compiler;
use token::{TokenType,RuleToken};
use op::opcode;
type ParseResult<T> = Result<T,()>;

trait PrefixParser {
    fn parse(&self,parser:&mut Compiler,token:TokenType) -> ParseResult<()>;
}


struct NameParser {

}

impl PrefixParser for NameParser {
    fn parse(&self,parser:&mut Compiler,token:TokenType) -> ParseResult<()> {
        let token = parser.advance().expect("No Token");

        match token.value.ty {
            TokenType::NUMBER(ref number) => parser.emit_constant(*number)?,
            _ => {
                let msg = format!("Expected a literal");

                parser.error(msg,token.span);
            }
        }

        Ok(())

    }
}

struct UnaryParser {

}

impl PrefixParser for UnaryParser {
    fn parse(&self,parser:&mut Compiler,token:TokenType) -> ParseResult<()> {
        let op = parser.get_op_ty()?;
        parser.advance().expect("Token Gone");

        match op {
            RuleToken::MINUS => {
                parser.emit_byte(opcode::NEGATE);
                Ok(())
            },

            _ => unreachable!()
        }

    }
}