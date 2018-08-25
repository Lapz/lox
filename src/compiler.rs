// TODO: add an error reporter
// Pretty printing of errors
use chunks::Chunk;
use error::Reporter;
use opcode;
use pos::{Span, Spanned, EMPTYSPAN};
use pratt::{BinaryParselet, InfixParser, LiteralParselet, PrefixParser, UnaryParselet,GroupingParselet};

use std::collections::{HashMap, VecDeque};

use token::{RuleToken, Token, TokenType};
use value::Value;

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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UnaryOperator {
    Negate,
    Bang,
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

        compiler.prefix(RuleToken::Literal, &LiteralParselet);
        compiler.prefix(RuleToken::Minus, &UnaryParselet);
        compiler.prefix(RuleToken::LParen, &GroupingParselet);
        compiler.infix(RuleToken::Plus, &BinaryParselet(Precedence::Term));
        compiler.infix(RuleToken::Minus, &BinaryParselet(Precedence::Term));
        compiler.infix(RuleToken::Slash, &BinaryParselet(Precedence::Factor));
        compiler.infix(RuleToken::Star, &BinaryParselet(Precedence::Factor));

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

    pub fn compile(&mut self) -> Result<(), ()> {
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

    pub fn emit_constant(&mut self, constant: Value) -> Result<(), ()> {
        let value = self.make_constant(constant)?;
        self.emit_bytes(opcode::CONSTANT, value);
        Ok(())
    }

    pub fn make_constant(&mut self, value: Value) -> Result<u8, ()> {
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

    pub fn advance(&mut self) -> Result<Spanned<Token<'a>>, ()> {
        match self.current_token.take() {
            Some(token) => {
                self.line = token.span.start.line;
                self.current_token = self.tokens.pop_front();

                Ok(token)
            }
            None => eof_error!(self),
        }
    }

    pub fn check(&mut self, ty: TokenType<'a>, msg: &str) -> Result<(), ()> {
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

    pub fn current_token(&self) -> Result<&Spanned<Token<'a>>, ()> {
        let token = self.current_token.as_ref();

        if token.is_none() {
            eof_error!(self)
        } else {
            Ok(token.unwrap())
        }
    }

    // ========== PARSING ===========

    pub fn expression(&mut self, precedence: Precedence) -> Result<(), ()> {
        let token = self.current()?;
        let mut rule = token.rule();

        let parser = self.prefix.get(&rule);

        if parser.is_none() {
            let token = self.current_token()?;

            let msg = format!("Expected an expression instead found `{}` ", token.value.ty);
            self.reporter.error(msg, token.span);
            return Err(());
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

    pub fn get_op_ty(&self) -> Result<Operator, ()> {
        match self.current()? {
            &TokenType::Minus => Ok(Operator::Negate),
            &TokenType::Bang => Ok(Operator::Bang),
            &TokenType::Plus => Ok(Operator::Plus),
            &TokenType::Star => Ok(Operator::Star),
            &TokenType::Slash => Ok(Operator::Slash),
            _ => Err(()),
        }
    }

    pub fn get_un_op(&self) -> Result<UnaryOperator, ()> {
        match self.current()? {
            &TokenType::Minus => Ok(UnaryOperator::Negate),
            &TokenType::Bang => Ok(UnaryOperator::Bang),
            ref other => {
                let token = self.current_token()?;
                self.reporter
                    .error(format!("Expected `!` or `-` found {}", other), token.span);
                Err(())
            }
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
