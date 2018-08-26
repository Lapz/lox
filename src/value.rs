use std::fmt::{self, Debug};

pub type Value = f32;

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Bool,
    Nil,
    Number,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union As {
    boolean: bool,
    number: f32,
}

#[derive(Debug,Clone, Copy)]
pub struct Val {
    val: As,
    ty: ValueType,
}

impl Val {
    pub fn bool(value: bool) -> Val {
        Val {
            val: As { boolean: value },
            ty: ValueType::Bool,
        }
    }

    pub fn nil() -> Val {
        Val {
            val: As { number:  0.0 },
            ty: ValueType::Nil,
        }
    }

    pub fn number(number: f32) -> Val {
        Val {
            val: As { number },
            ty: ValueType::Number,
        }
    }
}

impl Debug for As {
    fn fmt(&self, fmt:&mut fmt::Formatter) -> fmt::Result {
        write!(fmt,"{{")?;
        unsafe {
            write!(fmt,"{},",self.boolean)?;
            write!(fmt," {}",self.number)?;
        }
         write!(fmt,"}}")?;

        Ok(())
    }
}
