// TODO: add an error reporter
// Pretty printing of errors
use chunks::Chunk;
use error::Reporter;
use opcode;
use pos::{Span, Spanned};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use token::{RuleToken, Token, TokenType};
use value::Value;

type ParseResult<T> = Result<T, ()>;
#[derive(Debug)]
pub struct Compiler<'a> {
    chunk: Option<Chunk>,
    pub chunks: Vec<Chunk>,
    current_token: Option<Spanned<Token<'a>>>,
    tokens: VecDeque<Spanned<Token<'a>>>,
    pub reporter: Reporter,
    prefix: HashMap<RuleToken, &'a PrefixParser>,
    infix: HashMap<RuleToken, &'a InfixParser>,
    line: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Operator {
    Negate,
    Bang,
    Plus,
    Star,
    Slash,
}
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl<'a> Compiler<'a> {
    pub fn new(reporter: Reporter, tokens: Vec<Spanned<Token<'a>>>) -> Self {
        let mut tokens = tokens.into_iter().collect::<VecDeque<_>>();
        let current_token = tokens.pop_front();

        let mut compiler = Compiler {
            chunk: Some(Chunk::new()),
            chunks: vec![],
            tokens,
            current_token,
            reporter,

            prefix: HashMap::new(),
            infix: HashMap::new(),
            line: 0,
        };

        compiler.prefix(RuleToken::NUMBER, &LiteralParselet);
        compiler.prefix(RuleToken::MINUS, &UnaryParser);
        compiler.infix(RuleToken::PLUS, &BinaryParselet(Precedence::Term));
        compiler.infix(RuleToken::MINUS, &BinaryParselet(Precedence::Term));
        compiler.infix(RuleToken::SLASH, &BinaryParselet(Precedence::Factor));
        compiler.infix(RuleToken::STAR, &BinaryParselet(Precedence::Factor));

        compiler
    }

    pub fn prefix<T: PrefixParser + 'a>(&mut self, ty: RuleToken, parser: &'a T) {
        self.prefix.insert(ty, parser);
    }

    #[cfg(feature = "debug")]
    pub fn disassemble(&self) {
        for chunk in self.chunks.iter() {
            chunk.disassemble("chunk")
        }
    }

    pub fn infix<T: InfixParser + 'a>(&mut self, ty: RuleToken, parser: &'a T) {
        self.infix.insert(ty, parser);
    }

    pub fn error(&mut self, msg: String, span: Span) {
        self.reporter.error(msg, span)
    }

    pub fn compile(&mut self) -> ParseResult<()> {
        self.expression(Precedence::Assignment)?;
        self.check(TokenType::EOF, "Expected EOF")?;
        self.end_chunk();
        Ok(())
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.chunk.as_mut().unwrap().write(byte, self.line)
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        // println!("{:?}",self.chunk);
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn emit_constant(&mut self, constant: Value) -> ParseResult<()> {
        let value = self.make_constant(constant)?;
        self.emit_bytes(opcode::CONSTANT, value);
        Ok(())
    }

    pub fn make_constant(&mut self, value: Value) -> ParseResult<u8> {
        let index = self.chunk.as_mut().unwrap().add_constant(value);

        if index > 256 {
            self.reporter.error(
                "Too many constants in one chunk",
                self.current_token.as_ref().unwrap().span,
            );
            Err(())
        } else {
            Ok(index as u8)
        }
    }

    pub fn end_chunk(&mut self) {
        let mut current_chunk = self.chunk.take().unwrap();

        current_chunk.write(opcode::RETURN, self.line);

        self.chunks.push(current_chunk);
        self.chunk = Some(Chunk::new());
    }

    pub fn advance(&mut self) -> ParseResult<Spanned<Token<'a>>> {
        match self.current_token.take() {
            Some(token) => {
                self.line = token.span.start.line;
                self.current_token = self.tokens.pop_front();

                Ok(token)
            }
            None => eof_error!(self),
        }
    }

    pub fn check(&mut self, ty: TokenType<'a>, msg: &str) -> ParseResult<()> {
        if self.peek() == Some(&ty) {
            self.advance()?;
            Ok(())
        } else {
            self.reporter.error(
                msg,
                *self
                    .tokens
                    .front()
                    .map(|spanned| &spanned.span)
                    .unwrap_or(&self.reporter.end()),
            );
            Err(())
        }
    }

    pub fn peek(&self) -> Option<&TokenType<'a>> {
        self.tokens.front().map(|spanned| &spanned.value.ty)
    }

    pub fn current(&self) -> Result<&TokenType<'a>, ()> {
        let token = self.current_token.as_ref();

        match &token {
            Some(&Spanned {
                value: Token { ref ty },
                ..
            }) => Ok(ty),
            None => eof_error!(self),
        }
    }

    pub fn current_token(&self) -> Option<&Spanned<Token<'a>>> {
        self.current_token.as_ref()
    }

    // ========== PARSING ===========

    pub fn expression(&mut self, precedence: Precedence) -> Result<(), ()> {
        let token = self.current()?;
        let mut rule = token.rule();

        let parser = self.prefix.get(&rule);

        if parser.is_none() {
            let token = self.current_token();

            if token.is_none() {
                return eof_error!(self);
            } else {
                let token = token.unwrap();
                let span = token.span;
                let msg = format!("Expected an expression instead found `{}` ", token.value.ty);
                self.reporter.error(msg, span);
                return Err(());
            }

            // panic!("Parser for {:?} not found", token);
        }

        let parser = parser.unwrap();

        parser.parse(self)?;

        while precedence <= self.get_precedence() {
            {
                let token = self.peek().expect("Expected a token");

                rule = token.rule();
            }
            let parser = self.infix.get(&rule);

            let parser = if parser.is_some() {
                parser.unwrap()
            } else {
                return Ok(());
            };

            parser.parse(self)?;
        }

        Ok(())
    }

    pub fn get_precedence(&self) -> Precedence {
        let token = self.peek().expect("Expected a token");

        let rule = token.rule();

        let parser = self.infix.get(&rule);

        let parser = if parser.is_some() {
            parser.unwrap()
        } else {
            return Precedence::None;
        };

        parser.pred()
    }

    pub fn get_op_ty(&self) -> ParseResult<Operator> {
        match self.current()? {
            &TokenType::MINUS => Ok(Operator::Negate),
            &TokenType::BANG => Ok(Operator::Bang),
            &TokenType::PLUS => Ok(Operator::Plus),
            &TokenType::STAR => Ok(Operator::Star),
            &TokenType::SLASH => Ok(Operator::Slash),
            _ => Err(()),
        }
    }
}

impl Precedence {
    pub fn higher(&self) -> Precedence {
        match *self {
            Precedence::None | Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

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

        match parser.current_token() {
            Some(&Spanned {
                value: Token {
                    ty: TokenType::NUMBER(ref num),
                },
                ..
            }) => {
                parser.emit_constant(*num)?;
                Ok(())
            }
            Some(ref e) => {
                let msg = format!(
                    "Expected `{{int}}` or `{{nil}}` or `{{true|false}}` or `{{ident}} found `{}` ",
                    e.value.ty
                );
                parser.error(msg, e.span);
                Err(())
            }
            None => eof_error!(parser),
        }
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
