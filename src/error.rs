use std::collections::VecDeque;
use pos::Span;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{self,Display};

#[derive(Debug,Default,Clone)]
pub struct Reporter {
    diagnostics:Rc<RefCell<Vec<Diagnostic>>>,
}
#[derive(Debug)]
pub struct Diagnostic {
    msg:String,
    span:Span,
    level:Level
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
        Default::default()
    }

    pub fn has_error(&self) -> bool {
        self.diagnostics.borrow().is_empty()
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
}