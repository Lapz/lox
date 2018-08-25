use compiler::{Compiler, Operator, Precedence};
use op::opcode;
use std::fmt::Debug;
use token::{RuleToken, TokenType};
type ParseResult<T> = Result<T, ()>;

pub trait PrefixParser: Debug {
    fn parse(&self, parser: &mut Compiler) -> ParseResult<()>;
}

pub trait InfixParser: Debug {
    fn parse(&self, parser: &mut Compiler) -> ParseResult<()>;
    fn pred(&self) -> Precedence;
}

#[derive(Debug)]
pub struct LiteralParselet;

impl PrefixParser for LiteralParselet {
    fn parse(&self, parser: &mut Compiler) -> ParseResult<()> {
        // let token = parser.advance().expect("No Token");
        match parser.current()? {
            &TokenType::NUMBER(ref number) => parser.emit_constant(*number)?,
            _ => {
                let msg = format!("Expected a literal");

                // parser.error(msg,token.span);
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct UnaryParser;

impl PrefixParser for UnaryParser {
    fn parse(&self, parser: &mut Compiler) -> ParseResult<()> {
        // parser.advance()?;

        let op = parser.get_op_ty()?;
        parser.advance().expect("Token Gone");
        parser.expression(Precedence::Unary)?;

        match op {
            Operator::Negate => {
                parser.emit_byte(opcode::NEGATE);

                Ok(())
            }

            Operator::Bang => Ok(()),

            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct BinaryParselet(pub Precedence);

impl InfixParser for BinaryParselet {
    fn pred(&self) -> Precedence {
        self.0
    }

    fn parse(&self, parser: &mut Compiler) -> ParseResult<()> {
        parser.advance()?;

        let op = parser.get_op_ty()?;

        parser.advance()?;

        parser.expression(self.pred().higher())?; // Compile the rhs

        match op {
            Operator::Plus => parser.emit_byte(opcode::ADD),
            Operator::Negate => parser.emit_byte(opcode::SUB),
            Operator::Slash => parser.emit_byte(opcode::DIV),
            Operator::Star => parser.emit_byte(opcode::MUL),
            ref e => unreachable!("Parsing a binary op and found {:?}", e),
        }

        Ok(())
    }
}
