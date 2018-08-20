use std::str::Chars;
use std::iter::Peekable;

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    line:u32
}

pub struct Token<'a> {
    ty:TokenType<'a>,
    line:u32
}

pub enum LexerError {
    Unexepected,
    UnterminatedString
}

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
    EOF
}

impl <'a> Token <'a> {
    pub fn new(ty:TokenType<'a>,line:u32) -> Token<'a> {
        Self {
            ty,
            line
        }
    }
}
impl <'a> Scanner<'a> {
    pub fn new(source:&'a str) -> Self {
        let mut chars = source.chars().peekable();
        Scanner {
            source,
            chars,
            line:0
        }
        
    }
   
    pub fn scan(&mut self) -> Result<Token<'a>,LexerError> {
        self.skip_whitespace();
        Ok(match self.chars.next() {
            Some('(') => Token::new(TokenType::LPAREN,self.line),
            Some(')') => Token::new(TokenType::RPAREN,self.line),
            Some('{') => Token::new(TokenType::LBRACE,self.line),
            Some('}') => Token::new(TokenType::RBRACE,self.line),
            Some(';') => Token::new(TokenType::SEMICOLON,self.line),
            Some(',') => Token::new(TokenType::COMMA,self.line),
            Some('.') => Token::new(TokenType::DOT,self.line),
            Some('-') => Token::new(TokenType::MINUS,self.line),
            Some('+') => Token::new(TokenType::PLUS,self.line),
            Some('/') => Token::new(TokenType::SLASH,self.line),
            Some('*') => Token::new(TokenType::STAR,self.line),
            Some('!') => if self.peek('=') {
                self.chars.next();
                Token::new(TokenType::BANGEQUAL,self.line)

            }else {
                Token::new(TokenType::BANG,self.line)
            },
            Some('=') => if self.peek('=') {
                self.chars.next();
                Token::new(TokenType::EQUALEQUAL,self.line)

            }else {
                Token::new(TokenType::EQUAL,self.line)
            },
            Some('<') => if self.peek('=') {
                self.chars.next();
                Token::new(TokenType::LESSEQUAL,self.line)

            }else {
                Token::new(TokenType::LESS,self.line)
            },
            Some('>') => if self.peek('=') {
                self.chars.next();
                Token::new(TokenType::GREATEREQUAL,self.line)

            }else {
                Token::new(TokenType::GREATER,self.line)
            },
            Some('"') => return self.string(),
            Some(ch) => if ch.is_numeric() {
                self.number()
            }else {
                self.identifier()
            }
            None => Token::new(TokenType::EOF,self.line),
            _ =>return Err(LexerError::Unexepected),
        })
    }


    pub fn peek(&mut self,expected:char) -> bool {
        match self.chars.peek() {
            Some(&ch) => ch == expected,
            None => false,
        }
    }

    pub fn string(&mut self) -> Result<Token<'a>,LexerError> {
        let mut string = String::new();
        while !self.peek('"') {
            if self.peek('\n') {
                self.line +=1;
            }

            string.push(self.chars.next().unwrap());
        }
        if self.chars.peek().is_none() {
            return Err(LexerError::UnterminatedString)
        }

        self.chars.next();// Eat the closing "

        Ok(Token::new(TokenType::STRING(string), self.line))
    }

    pub fn number(&mut self) -> Token<'a> {
        let mut string = String::new();
        while let Some(ref ch) = self.chars.peek() {
            if ch.is_numeric() {
                string.push(self.chars.next().unwrap());
            }
            break;
        }

        if self.chars.peek() == Some(&'.') {
            string.push(self.chars.next().unwrap());
        }

        while let Some(ref ch) = self.chars.peek() {
            if ch.is_numeric() {
                string.push(self.chars.next().unwrap());
            }
            break;
        }

        Token::new(TokenType::NUMBER(string.parse::<f32>().expect("Invalid Float")), self.line)
    }
    pub fn skip_whitespace(&mut self) {
        loop {
            let c = self.chars.peek();

            match c {
                Some(&' ') | Some(&'\r') | Some(&'\t') => {
                    self.chars.next();
                    break;
                },
                Some(&'\n') => {
                    self.line += 1;
                    self.chars.next();
                    break;
                },
                Some(&'/') => {
                    if self.peek('/') {
                        self.chars.next();
                        while !self.peek('\n') {
                            self.chars.next();
                        }
                    }else {
                        return;
                    }

                    break;
                }
                _ => return
            }
        }
    }
}