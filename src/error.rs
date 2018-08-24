use pos::Span;
use pos::EMPTYSPAN;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Reporter {
    diagnostics: Rc<RefCell<Vec<Diagnostic>>>,
    end: Span,
}
#[derive(Debug)]
pub struct Diagnostic {
    msg: String,
    span: Span,
    level: Level,
}

#[derive(Debug, PartialEq)]
pub enum Level {
    Warn,
    Error,
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Level::Warn => write!(f, "warning"),
            Level::Error => write!(f, "error"),
        }
    }
}

impl Reporter {
    pub fn new() -> Reporter {
        Self {
            diagnostics: Rc::new(RefCell::new(Vec::new())),
            end: EMPTYSPAN,
        }
    }

    pub fn has_error(&self) -> bool {
        !self.diagnostics.borrow().is_empty()
    }

    pub fn set_end(&mut self, span: Span) {
        self.end = span;
    }

    pub fn end(&self) -> Span {
        self.end
    }

    pub fn remove_error(&mut self) {
        self.diagnostics.borrow_mut().pop();
    }

    pub fn error<T: Into<String>>(&self, msg: T, span: Span) {
        self.diagnostics.borrow_mut().push(Diagnostic {
            msg: msg.into(),
            span,
            level: Level::Error,
        })
    }

    pub fn warn<T: Into<String>>(&self, msg: T, span: Span) {
        self.diagnostics.borrow_mut().push(Diagnostic {
            msg: msg.into(),
            span,
            level: Level::Warn,
        })
    }

    pub fn emit(&self, input: &str) {
        for diagnostic in self.diagnostics.borrow().iter() {
            print(input, diagnostic)
        }
    }
}

pub fn print(input: &str, d: &Diagnostic) {
    let prefix = "| ";

    println!("{}: {}", d.level, d.msg);

    let span = d.span;

    let start_line = if span.start.line >= 4 {
        span.start.line - 4
    } else {
        0
    };

    for (idx, line) in input.lines().enumerate().skip(start_line as usize) {
        let line = line;
        let line_idx = idx + 1;
        println!("{:>4} {}{}", line_idx, prefix, line);
        if line_idx == span.start.line as usize {
            let end = if line_idx == span.end.line as usize {
                span.end.column as usize
            } else {
                line.len()
            };
            let carets = repeat_string("^", end - span.start.column as usize + 1);

            let carets = match d.level {
                Level::Warn => carets,
                Level::Error => carets,
            };

            let whitespace = repeat_string(" ", span.start.column as usize - 1);
            println!("     {}{}{}", prefix, whitespace, carets);
        } else if line_idx == span.end.line as usize {
            let carets = repeat_string("^", span.end.column as usize);
            let carets = match d.level {
                Level::Warn => carets,
                Level::Error => carets,
            };
            println!("     {}{}", prefix, carets);
        } else if line_idx > span.start.line as usize
            && line_idx < span.end.line as usize
            && !line.is_empty()
        {
            let carets = repeat_string("^", line.len());
            let carets = match d.level {
                Level::Warn => carets,
                Level::Error => carets,
            };
            println!("     {}{}", prefix, carets);
        }

        if line_idx >= span.end.line as usize + 3 {
            break;
        }
    }
}

fn repeat_string(s: &str, count: usize) -> String {
    ::std::iter::repeat(s).take(count).collect()
}
