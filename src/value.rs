use object::{Object, ObjectType, RawObject, StringObject};
use std::fmt::{self, Debug, Display};
use std::mem;

/// Represents that types that are used in lox
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Bool,
    Nil,
    Number,
    Object,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union As {
    boolean: bool,
    number: f32,
    /// A values whos state is stored on the heap
    object: RawObject,
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

    pub fn object(object: RawObject) -> Value {
        Value {
            val: As { object },
            ty: ValueType::Object,
        }
    }

    pub fn as_bool(&self) -> bool {
        debug_assert_eq!(
            self.ty,
            ValueType::Bool,
            "Value is type `{:?}` instead of {:?}",
            self.ty,
            ValueType::Bool
        );

        unsafe { self.val.boolean }
    }

    pub fn as_number(&self) -> f32 {
        debug_assert_eq!(
            self.ty,
            ValueType::Number,
            "Value is type `{:?}` instead of {:?}",
            self.ty,
            ValueType::Number
        );

        unsafe { self.val.number }
    }

    pub fn as_object(&self) -> RawObject {
        debug_assert_eq!(
            self.ty,
            ValueType::Object,
            "Value is type `{:?}` instead of {:?}",
            self.ty,
            ValueType::Object
        );

        unsafe { self.val.object }
    }

    pub fn as_string<'a>(&self) -> &StringObject<'a> {
        let ptr = self.as_object();

        unsafe { mem::transmute(ptr) }
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

    pub fn is_object(&self) -> bool {
        self.ty == ValueType::Object
    }

    pub fn is_string(&self) -> bool {
        unsafe {
            self.is_object()
                && mem::transmute::<RawObject, &Object>(self.as_object()).ty == ObjectType::String
        }
    }

    pub fn is_equal(&self, other: &Value) -> bool {
        if self.ty != other.ty {
            false
        } else {
            match self.ty {
                ValueType::Bool => self.as_bool() == other.as_bool(),
                ValueType::Nil => true,
                ValueType::Number => self.as_number() == other.as_number(),
                ValueType::Object => {
                    if self.is_string() && other.is_string() {
                        let a_string = self.as_string();
                        let b_string = other.as_string();

                        // Refractor to check if strings

                        a_string.chars.string() == b_string.chars.string()
                    } else {
                        false
                    }
                }
            }
        }
    }
}

impl Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Value {{")?;
        unsafe {
            if self.ty == ValueType::Number || self.ty == ValueType::Nil {
                write!(fmt, "val:{:?},", self.val.number)?;
            } else if self.ty == ValueType::Object {
                write!(fmt, "{}", self.as_string())?;
            } else {
                write!(fmt, "val:{:?},", self.val.boolean)?;
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
            } else if self.ty == ValueType::Object {
                let obj: &Object = mem::transmute(self.as_object());

                match obj.ty {
                    ObjectType::String => write!(fmt, "{}", self.as_string())?,
                }
            } else {
                write!(fmt, "{}", self.val.boolean)?;
            }
        }

        Ok(())
    }
}
