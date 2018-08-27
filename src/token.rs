use pos::Spanned;
use std::fmt::{self, Display};
use std::hash::{self, Hash};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub ty: TokenType<'a>,
}

#[derive(Debug, Clone)]
pub struct TokenIter<'a> {
    pub iter: IntoIter<Spanned<Token<'a>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType<'a> {
    Ident(&'a str),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    String(String),
    Number(f32),
    Comment,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum RuleToken {
    LParen,
    Minus,
    Plus,
    Slash,
    Star,
    Literal,
    None,
    Bang,
    Comparison,
    Equality,
    This,
    And,
    Or,
}

impl<'a> Display for TokenType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::EOF => write!(f, "\0"),
            TokenType::Ident(s) => write!(f, "{}", s),
            TokenType::Number(ref i) => write!(f, "{}", i),
            TokenType::Equal => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "\\"),
            TokenType::Dot => write!(f, "."),
            TokenType::Less => write!(f, "<"),          // <
            TokenType::Greater => write!(f, ">"),       // >
            TokenType::EqualEqual => write!(f, "=="),   // ==
            TokenType::BangEqual => write!(f, "!="),    // !=
            TokenType::LessEqual => write!(f, "<="),    // <=
            TokenType::GreaterEqual => write!(f, "=>"), // =>
            TokenType::String(ref s) => write!(f, "{:?}", s),
            TokenType::Comma => write!(f, ","),     // ,
            TokenType::Comment => write!(f, "//"),  // //
            TokenType::Semicolon => write!(f, ";"), //
            TokenType::LParen => write!(f, "("),    // (
            TokenType::RParen => write!(f, ")"),    // )
            TokenType::LBrace => write!(f, "{{"),   // {
            TokenType::RBrace => write!(f, "}}"),   // }
            // Keywords,
            TokenType::Fun => write!(f, "fun"),
            TokenType::Print => write!(f, "print"),
            TokenType::Var => write!(f, "var"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Return => write!(f, "return"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::This => write!(f, "this"),
            TokenType::Class => write!(f, "class"),
            TokenType::For => write!(f, "for"),
            TokenType::While => write!(f, "while"),
            TokenType::Super => write!(f, "super"),
            TokenType::And => write!(f, "and"),
            TokenType::Or => write!(f, "or"),
            TokenType::Nil => write!(f, "nil"),
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Spanned<Token<'a>>;

    fn next(&mut self) -> Option<Spanned<Token<'a>>> {
        self.iter.next()
    }
}

impl<'a> Eq for TokenType<'a> {}

impl<'a> Hash for TokenType<'a> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match *self {
            TokenType::Number(ref float) => {
                let int = *float as i32;
                int.hash(state);
            }
            ref o => o.hash(state),
        }
    }
}

impl<'a> TokenType<'a> {
    pub fn rule(&self) -> RuleToken {
        match *self {
            TokenType::Number(_) => RuleToken::Literal,
            TokenType::False => RuleToken::Literal,
            TokenType::True => RuleToken::Literal,
            TokenType::Minus => RuleToken::Minus,
            TokenType::Plus => RuleToken::Plus,
            TokenType::Slash => RuleToken::Slash,
            TokenType::Star => RuleToken::Star,
            TokenType::EOF => RuleToken::None,
            TokenType::LParen => RuleToken::LParen,
            TokenType::RParen => RuleToken::None,
            TokenType::Bang => RuleToken::Bang,
            TokenType::Less
            | TokenType::Greater
            | TokenType::BangEqual
            | TokenType::LessEqual
            | TokenType::GreaterEqual => RuleToken::Comparison,
            TokenType::Equal | TokenType::EqualEqual => RuleToken::Equality,
            TokenType::Nil => RuleToken::Literal,
            ref e => unimplemented!("{:?}", e),
        }
    }
}
