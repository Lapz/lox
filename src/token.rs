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
    IDENT(&'a str),
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    STRING(String),
    NUMBER(f32),
    COMMENT,
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    ERROR,
    EOF,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum RuleToken {
    LPAREN,
    DOT,
    MINUS,
    PLUS,
    SLASH,
    STAR,
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    IDENT,
    STRING,
    NUMBER,
    FALSE,
    TRUE,
    THIS,
    NIL,
    AND,
    OR,
    EOF,
}

impl<'a> Display for TokenType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::EOF => write!(f, "\0"),
            TokenType::IDENT(s) => write!(f, "{}", s),
            TokenType::NUMBER(ref i) => write!(f, "{}", i),
            TokenType::EQUAL => write!(f, "="),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::BANG => write!(f, "!"),
            TokenType::STAR => write!(f, "*"),
            TokenType::SLASH => write!(f, "\\"),
            TokenType::DOT => write!(f, "."),
            TokenType::LESS => write!(f, "<"),          // <
            TokenType::GREATER => write!(f, ">"),       // >
            TokenType::EQUALEQUAL => write!(f, "=="),   // ==
            TokenType::BANGEQUAL => write!(f, "!="),    // !=
            TokenType::LESSEQUAL => write!(f, "<="),    // <=
            TokenType::GREATEREQUAL => write!(f, "=>"), // =>
            TokenType::STRING(ref s) => write!(f, "{:?}", s),
            TokenType::COMMA => write!(f, ","),     // ,
            TokenType::COMMENT => write!(f, "//"),  // //
            TokenType::SEMICOLON => write!(f, ";"), //
            TokenType::LPAREN => write!(f, "("),    // (
            TokenType::RPAREN => write!(f, ")"),    // )
            TokenType::LBRACE => write!(f, "{{"),   // {
            TokenType::RBRACE => write!(f, "}}"),   // }
            // Keywords,
            TokenType::FUN => write!(f, "fun"),
            TokenType::PRINT => write!(f, "print"),
            TokenType::VAR => write!(f, "var"),
            TokenType::IF => write!(f, "if"),
            TokenType::ELSE => write!(f, "else"),
            TokenType::RETURN => write!(f, "return"),
            TokenType::TRUE => write!(f, "true"),
            TokenType::FALSE => write!(f, "false"),
            TokenType::THIS => write!(f, "this"),
            TokenType::CLASS => write!(f, "class"),
            TokenType::FOR => write!(f, "for"),
            TokenType::WHILE => write!(f, "while"),
            TokenType::SUPER => write!(f, "super"),
            TokenType::ERROR => write!(f, "error"),
            TokenType::AND => write!(f, "and"),
            TokenType::OR => write!(f, "or"),
            TokenType::NIL => write!(f, "nil"),
        }
    }
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType<'a>) -> Token<'a> {
        Self { ty }
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
            TokenType::NUMBER(ref float) => {
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
            TokenType::NUMBER(_) => RuleToken::NUMBER,
            TokenType::MINUS => RuleToken::MINUS,
            TokenType::PLUS => RuleToken::PLUS,
            TokenType::EOF => RuleToken::EOF,
            ref e => unimplemented!("{:?}", e),
        }
    }
}
