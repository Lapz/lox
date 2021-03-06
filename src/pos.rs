use std::fmt::{self, Display};
use std::str::Chars;
#[derive(Debug, Clone)]
pub struct CharPosition<'a> {
    pub pos: Position,
    pub chars: Chars<'a>,
}

/// Represents a Span in the source file along with its value
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

pub const EMPTYSPAN: Span = Span {
    start: Position {
        line: 1,
        column: 0,
        absolute: 1,
    },
    end: Position {
        line: 1,
        column: 0,
        absolute: 1,
    },
};

/// A span between two locations in a source file
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Copy, PartialOrd, Clone, PartialEq, Eq, Ord)]
pub struct Position {
    pub line: u32,
    pub column: u32,
    pub absolute: usize,
}

impl<'a> CharPosition<'a> {
    pub fn new(input: &'a str) -> Self {
        CharPosition {
            pos: Position {
                line: 1,
                column: 1,
                absolute: 0,
            },
            chars: input.chars(),
        }
    }
}

impl<'a> Iterator for CharPosition<'a> {
    type Item = (Position, char);

    fn next(&mut self) -> Option<(Position, char)> {
        self.chars.next().map(|ch| {
            let pos = self.pos;
            self.pos = self.pos.shift(ch);
            (pos, ch)
        })
    }
}

impl Position {
    pub fn shift(mut self, ch: char) -> Self {
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else if ch == '\t' {
            self.column += 4;
        } else {
            self.column += 1;
        }

        self.absolute += ch.len_utf8();
        self
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.line, self.column)
    }
}
