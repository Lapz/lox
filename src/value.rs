use std::fmt::{self, Debug, Display};

// pub type Value = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy)]
pub struct Value {
    val: As,
    ty: ValueType,
}

impl Value {
    pub fn bool(value: bool) -> Value {
        Value {
            val: As { boolean: value },
            ty: ValueType::Bool,
        }
    }

    pub fn nil() -> Value {
        Value {
            val: As { number: 0.0 },
            ty: ValueType::Nil,
        }
    }

    pub fn number(number: f32) -> Value {
        Value {
            val: As { number },
            ty: ValueType::Number,
        }
    }

    pub fn as_bool(&self) -> bool {
        if self.ty != ValueType::Bool {
            panic!(
                "Value is type `{:?}` instead of {:?}",
                self.ty,
                ValueType::Bool
            );
        }

        unsafe { self.val.boolean }
    }

    pub fn as_number(&self) -> f32 {
        if self.ty != ValueType::Number {
            panic!(
                "Value is type `{:?}` instead of {:?}",
                self.ty,
                ValueType::Bool
            );
        }

        unsafe { self.val.number }
    }

    pub fn is_number(&self) -> bool {
        self.ty == ValueType::Number
    }

    pub fn is_bool(&self) -> bool {
        self.ty == ValueType::Bool
    }

    pub fn is_nil(&self) -> bool {
        self.ty == ValueType::Nil
    }

    pub fn is_falsey(&self) -> bool {
        self.is_nil() || self.is_bool() && !self.as_bool()
    }

    pub fn is_equal(&self,other:&Value) -> bool {
       
       
        if self.ty != other.ty {
            false
        }else {
            match self.ty {
                ValueType::Bool => self.as_bool() == other.as_bool(),
                ValueType::Nil => true,
                ValueType::Number => self.as_number() == other.as_number(),
            }
        }
    }
}

impl Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Value {{")?;
        unsafe {
            if self.ty == ValueType::Number || self.ty == ValueType::Nil {
                write!(fmt, "val:{},", self.val.number)?;
            } else {
                write!(fmt, "val:{},", self.val.boolean)?;
            }
        }
        write!(fmt, " ty:{:?}", self.ty)?;
        write!(fmt, "}}")?;
        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            if self.ty == ValueType::Number || self.ty == ValueType::Nil {
                write!(fmt, "{}", self.val.number)?;
            } else if self.ty == ValueType::Nil {
                write!(fmt, "nil")?;
            } else {
                write!(fmt, "{}", self.val.boolean)?;
            }
        }

        Ok(())
    }
}
