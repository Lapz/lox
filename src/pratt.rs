use compiler::{Compiler, Operator, Precedence, UnaryOperator};
use op::opcode;
use pos::Spanned;
use std::fmt::Debug;
use token::{RuleToken, Token, TokenType};
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

        match parser.current_token()? {
            &Spanned {
                value: Token {
                    ty: TokenType::Number(ref num),
                },
                ..
            } => {
                parser.emit_constant(*num)?;
                Ok(())
            }
            ref e => {
                let msg = format!(
                    "Expected `{{int}}` or `{{nil}}` or `{{true|false}}` or `{{ident}} found `{}` ",
                    e.value.ty
                );
                parser.error(msg, e.span);
                Err(())
            }
        }
    }
}

#[derive(Debug)]
pub struct UnaryParselet;

impl PrefixParser for UnaryParselet {
    fn parse(&self, parser: &mut Compiler) -> ParseResult<()> {
        // parser.advance()?;

        let op = parser.get_un_op()?;
        parser.advance().expect("Token Gone");
        parser.expression(Precedence::Unary)?;

        match op {
            UnaryOperator::Negate => {
                parser.emit_byte(opcode::NEGATE);

                Ok(())
            }

            UnaryOperator::Bang => Ok(()),
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


#[derive(Debug)]
pub struct GroupingParselet;

impl PrefixParser for GroupingParselet {
    fn parse(&self, parser: &mut Compiler) -> ParseResult<()> {
        parser.advance()?; //Eats the (
        parser.expression(Precedence::Assignment)?;
        
        parser.check(TokenType::RParen, "Expected ')'")?;
        Ok(())
    }
}