use error::Reporter;
use pos::{CharPosition, Position, Span, Spanned};
use std::fmt::{self, Display};
use std::str::Chars;
use token::{Token, TokenType};

#[derive(Debug)]
pub enum LexerError {
    UnclosedString,
    UnclosedBlockComment,
    EOF,
    Unexpected(char, Position),
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    // A lexer instance
    input: &'a str,
    reporter: Reporter,
    chars: CharPosition<'a>,
    lookahead: Option<(Position, char)>,
    end: Position,
}
impl<'a> Lexer<'a> {
    /// Returns a new Lexer
    pub fn new(input: &'a str, reporter: Reporter) -> Lexer {
        let mut chars = CharPosition::new(input);
        let end = chars.pos;
        Lexer {
            input,
            end,
            reporter,
            lookahead: chars.next(),
            chars,
        }
    }

    fn advance(&mut self) -> Option<(Position, char)> {
        match self.lookahead {
            Some((pos, ch)) => {
                self.end = self.end.shift(ch);
                self.lookahead = self.chars.next();
                Some((pos, ch))
            }

            None => None,
        }
    }

    fn span_error<T: Into<String>>(&mut self, msg: T, start: Position, end: Position) {
        self.reporter.error(msg.into(), Span { start, end })
    }

    fn error<T: Into<String>>(&mut self, msg: T, pos: Position) {
        self.reporter.error(
            msg.into(),
            Span {
                start: pos,
                end: pos,
            },
        )
    }

    fn slice(&self, start: Position, end: Position) -> &'a str {
        &self.input[start.absolute..end.absolute]
    }

    fn take_whilst<F>(&mut self, start: Position, mut terminate: F) -> (Position, &'a str)
    where
        F: FnMut(char) -> bool,
    {
        while let Some((end, ch)) = self.lookahead {
            if !terminate(ch) {
                return (end, self.slice(start, end));
            }
            self.advance();
        }

        (self.end, self.slice(start, self.end))
    }

    fn peek<F>(&mut self, mut check: F) -> bool
    where
        F: FnMut(char) -> bool,
    {
        self.lookahead.map_or(false, |(_, ch)| check(ch))
    }

    fn line_comment(&mut self, start: Position) {
        let (_, _) = self.take_whilst(start, |ch| ch != '\n');
    }

    fn block_comment(&mut self, start: Position) -> Result<(), ()> {
        self.advance(); // Eats the '*'
        loop {
            self.advance(); // Eats the '*'

            match self.lookahead {
                Some((_, '/')) => {
                    self.advance();
                    return Ok(());
                }
                Some((_, _)) => continue,

                None => {
                    let msg: String = LexerError::UnclosedBlockComment.into();

                    self.span_error(msg, start, self.end);
                    return Err(());
                }
            }
        }
    }

    fn string_literal(&mut self, start: Position) -> Result<Spanned<Token<'a>>, ()> {
        let mut string = String::new();

        while let Some((next, ch)) = self.advance() {
            match ch {
                '"' => {
                    let end = next.shift(ch);

                    return Ok(spans(TokenType::STRING(string), start, end));
                }

                ch => string.push(ch),
            }
        }

        let msg: String = LexerError::UnclosedString.into();

        self.span_error(msg, start, self.end);

        Err(())
    }

    fn number(&mut self, start: Position) -> Result<Spanned<Token<'a>>, ()> {
        let (end, int) = self.take_whilst(start, |c| c.is_numeric());

        let (token, start, end) = match self.lookahead {
            Some((_, '.')) => {
                self.advance();

                let (end, float) = self.take_whilst(start, |c| c.is_numeric());

                match self.lookahead {
                    Some((_, ch)) if ch.is_alphabetic() => {
                        let msg: String = LexerError::Unexpected(ch, start).into();

                        self.error(msg, start);

                        return Err(()); // Rejects floats like 10.k
                    }

                    _ => (
                        TokenType::NUMBER(float.parse().expect("An invalid float was used")),
                        start,
                        end,
                    ),
                }
            }

            Some((_, ch)) if ch.is_alphabetic() => {
                let msg: String = LexerError::Unexpected(ch, start).into();
                self.error(msg, start);

                return Err(());
            }
            Some(_) | None => {
                
                (TokenType::NUMBER(int.parse().expect("Coundln't parse the float")), start, end)
                
            },
        };

        Ok(spans(token, start, end))
    }

    fn identifier(&mut self, start: Position) -> Spanned<Token<'a>> {
        let (end, ident) = self.take_whilst(start, is_letter_ch);
        spans(look_up_identifier(ident), start, end)
    }

    pub fn next(&mut self) -> Result<Spanned<Token<'a>>, ()> {
        while let Some((start, ch)) = self.advance() {
            return match ch {
                '.' => Ok(span(TokenType::DOT, start)),
                // '?' => Ok(span(TokenType::QUESTION, start)),
                ';' => Ok(span(TokenType::SEMICOLON, start)),
                '{' => Ok(span(TokenType::LBRACE, start)),
                '}' => Ok(span(TokenType::RBRACE, start)),
                // '[' => Ok(span(TokenType::LBRACKET, start)),
                // ']' => Ok(span(TokenType::RBRACKET, start)),
                '(' => Ok(span(TokenType::LPAREN, start)),
                ')' => Ok(span(TokenType::RPAREN, start)),
                ',' => Ok(span(TokenType::COMMA, start)),
                // ':' => Ok(span(TokenType::COLON, start)),
                // '^' => Ok(span(TokenType::EXPONENTIAL, start)),
                // '%' => Ok(span(TokenType::MODULO, start)),
                '"' => match self.string_literal(start) {
                    Ok(token) => Ok(token),
                    Err(_) => continue,
                },

                '=' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        Ok(spans(TokenType::EQUALEQUAL, start, start.shift('=')))
                    } else {
                        Ok(span(TokenType::EQUAL, start))
                    }
                }

                '+' => {
                    Ok(span(TokenType::PLUS, start))
                    // if self.peek(|ch| ch == '=') {
                    //     self.advance();
                    //     Ok(spans(TokenType::PLUSASSIGN, start, start.shift('=')))
                    // } else {
                    //     Ok(span(TokenType::PLUS, start))
                    // }
                }

                '-' => {
                    // if self.peek(|ch| ch == '=') {
                    //     self.advance();
                    //     Ok(spans(TokenType::MINUSASSIGN, start, start.shift('=')))
                    // } else if self.peek(|ch| ch == '>') {
                    //     self.advance();
                    //     Ok(spans(TokenType::FRETURN, start, start.shift('>')))
                    // } else {
                    Ok(span(TokenType::MINUS, start))
                    // }
                }

                '*' => {
                    // if self.peek(|ch| ch == '=') {
                    //     self.advance();
                    //     Ok(spans(TokenType::STARASSIGN, start, start.shift('=')))
                    // } else {
                    Ok(span(TokenType::STAR, start))
                    // }
                }

                '/' => {
                    if self.peek(|ch| ch == '/') {
                        self.advance();
                        self.line_comment(start);
                        continue;
                    } else if self.peek(|ch| ch == '*') {
                        if let Err(_) =  self.block_comment(start) {
                        }
                        continue;
                    } else {
                        Ok(span(TokenType::SLASH, start))
                    }
                }

                '!' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        Ok(spans(TokenType::BANGEQUAL, start, start.shift('=')))
                    } else {
                        Ok(span(TokenType::BANG, start))
                    }
                }

                '>' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        Ok(spans(TokenType::GREATEREQUAL, start, start.shift('=')))
                    } else {
                        Ok(span(TokenType::GREATER, start))
                    }
                }
                '<' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        Ok(spans(TokenType::LESSEQUAL, start, start.shift('=')))
                    } else {
                        Ok(span(TokenType::LESS, start))
                    }
                }

                ch if ch.is_numeric() => self.number(start),
                ch if is_letter_ch(ch) => Ok(self.identifier(start)),
                ch if ch.is_whitespace() => continue,
                ch => {
                    let msg: String = LexerError::Unexpected(ch, start).into();
                    self.error(msg, start);
                   
                    continue;
                }
            };
        }

        Ok(spans(TokenType::EOF, self.end, self.end))
    }

    pub fn lex(&mut self) -> Result<Vec<Spanned<Token<'a>>>, ()> {
        let mut tokens = vec![];

        while self.lookahead.is_some() {
            match self.next() {
                Ok(token) => tokens.push(token),
                Err(_) => (),
            }
        }
        
        tokens.push(span(TokenType::EOF, self.end));

        tokens.retain(|t| t.value.ty != TokenType::COMMENT);

        self.reporter.set_end(Span {
            start: self.end,
            end: self.end,
        });

        if self.reporter.has_error() {
            Err(())
        } else {
            Ok(tokens)
        }
    }
}

#[inline]
fn is_letter_ch(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

#[inline]
fn span(token: TokenType, start: Position) -> Spanned<Token> {
    Spanned {
        value: token_with_info(token),
        span: Span { start, end: start },
    }
}

#[inline]
fn spans(token: TokenType, start: Position, end: Position) -> Spanned<Token> {
    Spanned {
        value: token_with_info(token),
        span: Span { start, end },
    }
}

#[inline]
fn token_with_info(ty: TokenType) -> Token {
    Token { ty }
}

#[inline]
fn look_up_identifier(id: &str) -> TokenType {
    match id {
        // Class
        "class" => TokenType::CLASS,
        "print" => TokenType::PRINT,
        "this" => TokenType::THIS,
        // Functions and vars
        "fun" => TokenType::FUN,
        "var" => TokenType::VAR,
        // Control Flow
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "for" => TokenType::FOR,
        "while" => TokenType::WHILE,
        "return" => TokenType::RETURN,
        // "break" => TokenType::BREAK,
        // "continue" => TokenType::CONTINUE,
        // "do" => TokenType::DO,
        // Booleans
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "or" => TokenType::OR,
        "and" => TokenType::AND,
        "nil" => TokenType::NIL,
        // "int" => TokenType::IntType,
        // "float" => TokenType::FloatType,
        _ => TokenType::IDENT(id),
    }
}

impl Into<String> for LexerError {
    fn into(self) -> String {
        match self {
            LexerError::UnclosedString => "Unclosed string".into(),
            LexerError::EOF => "Unexpected EOF".into(),
            LexerError::UnclosedBlockComment => "Unclosed block comment".into(),
            LexerError::Unexpected(ref c, _) => format!("Unexpected char '{}' ", c),
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LexerError::UnclosedString => write!(f, "unclosed string"),
            LexerError::EOF => write!(f, "Unexpected EOf"),
            LexerError::UnclosedBlockComment => write!(f, "unclosed block comment"),
            LexerError::Unexpected(ref c, ref p) => write!(f, "Unexpected char {} on {}", c, p),
        }
    }
}

#[cfg(test)]
mod test{
    use super::Lexer;
    use error::Reporter;
    #[test] 

    fn it_work() {
        let source = "-10 a () {} ,.-+;/*!!= ";
    }
}