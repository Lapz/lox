#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub ty: TokenType<'a>,
}

#[derive(Debug, PartialEq)]
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

impl<'a> Token<'a> {
    pub fn new(ty: TokenType<'a>) -> Token<'a> {
        Self { ty }
    }
}
