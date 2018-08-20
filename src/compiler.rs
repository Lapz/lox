use scanner::Lexer;
use token::{TokenIter,Token};
use std::iter::Peekable;
use pos::Spanned;

pub struct Compiler<'a> {
    tokens:Peekable<TokenIter<'a>>,
    next:Option<Spanned<Token<'a>>>,
}



impl <'a> Compiler<'a> {
    pub fn new(tokens:Vec<Spanned<Token<'a>>>) -> Self {
        let mut iter = TokenIter {
            iter:tokens.into_iter()
        };

        Compiler {
            next:None,
            tokens:iter.peekable(),
        }
    }

    pub fn compile(&mut self, source:&str) {
        loop {
            self.advance();
            let token =&self.next;

            println!("{:?}",token);

            if self.next.is_none() {
                break;
            }
        }
    }

    pub fn advance(&mut self) {
       self.next = self.tokens.next();
    }

    // pub fn consume(ty:TokenType<'a>,msg:&str) {

    // }
}

pub fn compile(source:&str) {
    let mut scanner = Lexer::new(source);

    loop {
        let token = scanner.next();
    }
}
