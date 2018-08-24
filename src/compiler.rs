// TODO: add an error reporter
// Pretty printing of errors
use chunks::Chunk;
use error::Reporter;
use opcode;
use pos::{Spanned, EMPTYSPAN,Span};
use scanner::Lexer;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::iter::Peekable;
use token::{Token, TokenType,RuleToken};
use value::Value;
use std::hash::Hash;

#[derive(Debug)]
pub struct Compiler<'a> {
    chunk: Option<Chunk>,
    chunks: Vec<Chunk>,
    current_token: Option<Spanned<Token<'a>>>,
    tokens: VecDeque<Spanned<Token<'a>>>,
    reporter: Reporter,
    rules: HashMap<RuleToken, ParseRule<'a>>,
    infix_parsers: HashMap<TokenType<'a>, &'a InfixParser>,
    prefix: HashMap<TokenType<'a>, &'a PrefixParser>,
    line: u32,
}

trait PrefixParser: Debug {
    fn parse(&self, compiler: &mut Compiler) -> Result<(), ()>;
}

trait InfixParser: Debug {
    // add code here
    fn parse(&self,compiler:&mut Compiler) -> Result<(), ()>;
    fn precedence(&self) -> Precedence;
}

#[derive(Debug, Clone, Copy,Eq,PartialEq,Hash)]
pub enum Operator {
    Negate,
    Bang,
    PLUS,
    Star,
    Slash,
}
#[derive(Debug,Clone,Copy)]
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


#[derive(Debug)]
pub struct ParseRule<'a> {
    prefix: Option<&'a PrefixParser>,
    infix: Option<&'a InfixParser>,
    precedence: Precedence,
}

impl<'a> Compiler<'a> {
    pub fn new(reporter: Reporter, tokens: Vec<Spanned<Token<'a>>>) -> Self {
        let mut tokens = tokens.into_iter().collect::<VecDeque<_>>();
        let current_token = tokens.pop_front();

        let compiler = Compiler {
            chunk: Some(Chunk::new()),
            chunks: vec![],
            tokens,
            current_token,
            reporter,
            infix_parsers: HashMap::new(),
            prefix: HashMap::new(),
            rules: HashMap::new(),
            line: 0,
        };

        let mut rules = HashMap::new();

        // rules.insert(RuleToken::LPAREN,ParseRule {
        //     infix:None,
        //     prefix:Some(&Grouping),
        //     precedence:Precedence::Call,
        // });

        // rules.insert(RuleToken::DOT, ParseRule{
        //     infix:None,
        //     prefix:None,
        //     precedence:Precedence::None,
        // });

        // rules.insert(RuleToken::MINUS, ParseRule{
        //     infix:Some(&Unary(Precedence::Term)),
        //     prefix:Some(&Binary),
        //     precedence:Precedence::Term,
        // });

        // rules.insert(RuleToken::PLUS, ParseRule{
        //     infix:None,
        //     prefix:Some(&Binary),
        //     precedence:Precedence::Term,
        // });


        // rules.insert(RuleToken::SLASH, ParseRule{
        //     infix:None,
        //     prefix:Some(&Binary),
        //     precedence:Precedence::Factor,
        // });

        // rules.insert(RuleToken::STAR, ParseRule{
        //     infix:None,
        //     prefix:Some(&Binary),
        //     precedence:Precedence::Factor,
        // });

        // rules.insert(RuleToken::BANG, ParseRule{
        //     infix:None,
        //     prefix:Some(&Binary),
        //     precedence:Precedence::None,
        // });

        // rules.insert(RuleToken::BANGEQUAL,ParseRule {
        //     infix:None,
        //     prefix:None,
        //     precedence:Precedence::Equality,
        // });

        // rules.insert(RuleToken::EQUAL, ParseRule{
        //     infix:None,
        //     prefix:None,
        //     precedence:Precedence::None,
        // });

        // rules.insert(RuleToken::EQUALEQUAL,ParseRule {
        //     infix:None,
        //     prefix:None,
        //     precedence:Precedence::Equality,
        // });

        // rules.insert(RuleToken::NUMBER,ParseRule {
        //     infix:Some(&LiteralParser(Precedence::None)),
        //     prefix:None,
        //     precedence:Precedence::None,
        // });

        
/*                     

  ParseRule {None,     None,    PREC_COMPARISON }); // TOKEN_GREATER         
  ParseRule {None,     None,    PREC_COMPARISON }); // TOKEN_GREATER_EQUAL   
  ParseRule {None,     None,    PREC_COMPARISON }); // TOKEN_LESS            
  ParseRule {None,     None,    PREC_COMPARISON }); // TOKEN_LESS_EQUAL      
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_IDENTIFIER      
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_STRING          
  ParseRule {number,   None,    PREC_NONE });       // TOKEN_NUMBER          
  ParseRule {None,     None,    PREC_AND });        // TOKEN_AND             
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_CLASS           
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_ELSE            
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_FALSE           
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_FUN             
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_FOR             
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_IF              
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_NIL             
  ParseRule {None,     None,    PREC_OR });         // TOKEN_OR              
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_PRINT           
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_RETURN          
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_SUPER           
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_THIS            
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_TRUE            
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_VAR             
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_WHILE           
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_ERROR           
  ParseRule {None,     None,    PREC_NONE });       // TOKEN_EOF                                                                        
 */
        // compiler.prefix(&)

        // rules.insert(RuleToken::LPAREN, ParseRule::new(Precedence::Call,Some(&compiler.grouping),None));

        compiler
    }

    pub fn prefix<T: PrefixParser + 'a>(&mut self, parser: &'a T, ty: TokenType<'a>) {
        self.prefix.insert(ty, parser);
    }

    pub fn error(&mut self,msg:String,span:Span) {
        self.reporter.error(msg, span)
    }

    pub fn compile(&mut self) -> Result<(), ()> {
        self.unary()?;
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

    // ========== PARSING ===========

    pub fn precedence(&mut self, pred: Precedence) -> Result<(), ()> {
        self.advance()?;


        // let op_type = self.get_op_ty()?;

        // let rule = self.get_rule(op_type).expect("Expected an expression");

        // if let Some(ref prefix)  = rule.prefix {
        //     prefix.parse(&mut self)?;
        // }

        // while pred <= self.get_rule(rtoken: RuleToken) {

        // }


        

        // let parser = self.prefix.get();

       Ok(())
    }

    pub fn expression(&mut self) -> Result<(), ()> {
        let token = self.peek()?;

        let parser = self.
        
        self.precedence(Precedence::Assignment)
    }

    pub fn number(&mut self) -> Result<(), ()> {
        let token = self.current_token.as_ref();

        match &token {
            Some(&Spanned {
                value: Token {
                    ty: TokenType::NUMBER(ref num),
                },
                ..
            }) => {
                self.emit_constant(*num)?;
                Ok(())
            }
            Some(ref e) => {
                let msg = format!("Expected `{{int}}` found `{}` ", e.value.ty);
                self.reporter.error(msg, e.span);
                Err(())
            }
            None => eof_error!(self),
        }
    }

    pub fn grouping(&mut self) -> Result<(), ()> {
        self.expression()?;
        self.check(TokenType::RPAREN, "Expeceted '(' ")
    }

    pub fn unary(&mut self) -> Result<(), ()> {
        let op_type = self.get_op_ty()?;
        self.advance()?; // Eat the - or !
        self.precedence(Precedence::Unary);
        match op_type {
            RuleToken::MINUS => {
                self.emit_byte(opcode::NEGATE);
                Ok(())
            }

            _ => unreachable!(),
        }
    }

    pub fn binary(&mut self) -> Result<(), ()> {
        let op_type = self.get_op_ty()?;
        self.advance()?;

        let rule = self.get_rule(op_type).expect("Expected an expression");

        self.precedence(rule.precedence.higher());

        match op_type {
            RuleToken::PLUS => self.emit_byte(opcode::ADD),
            RuleToken::MINUS=> self.emit_byte(opcode::SUB),
            RuleToken::SLASH=> self.emit_byte(opcode::DIV),
            RuleToken::STAR=> self.emit_byte(opcode::MUL),
            ref e => unreachable!("Parsing a binary op and found {:?}", e),
        }

        Ok(())
    }

    pub fn get_op_ty(&self) -> Result<RuleToken, ()> {
        match &self.current_token.as_ref().unwrap().value.ty {
            &TokenType::MINUS => Ok(RuleToken::MINUS),
            &TokenType::BANG => Ok(RuleToken::BANG),
            &TokenType::PLUS => Ok(RuleToken::PLUS),
            &TokenType::STAR => Ok(RuleToken::STAR),
            &TokenType::SLASH => Ok(RuleToken::SLASH),
            _ => Err(()),
        }
    }

    pub fn get_rule(&self, rtoken:RuleToken ) -> Option<&ParseRule<'a>> {
        self.rules.get(&rtoken)
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



#[derive(Debug)]
pub struct Grouping;

impl PrefixParser for Grouping {
    fn parse(&self,compiler:&mut Compiler) -> Result<(),()> {
        compiler.grouping()?;
        Ok(())
    }
}



#[derive(Debug)]
pub struct Binary;

impl PrefixParser for Binary {
    fn parse(&self,compiler:&mut Compiler) -> Result<(),()> {
        compiler.binary()?;
        Ok(())
    }
}



#[derive(Debug)]
pub struct Unary(pub Precedence);

impl InfixParser for Unary {
    fn parse(&self,compiler:&mut Compiler) -> Result<(),()> {
        compiler.unary()?;
        Ok(())
    }

    fn precedence(&self) -> Precedence {
        self.0
    }

}


#[derive(Debug)]
pub struct LiteralParser(pub Precedence);

impl InfixParser for LiteralParser {
    fn parse(&self,compiler:&mut Compiler) -> Result<(),()> {
        compiler.number()?;
        Ok(())
    }

    fn precedence(&self) -> Precedence {
        self.0
    }

}
