use libc::{c_char, c_void, malloc, strcpy};
use std::fmt::{self, Display};
use std::mem;
use std::ops::Deref;
use util::reallocate;

pub type RawObject = *mut Object;

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(C)]
pub enum ObjectType {
    String,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Object {
    pub ty: ObjectType,
    pub next: RawObject,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct StringObject<'a> {
    pub obj: Object,
    pub chars: ObjectValue<'a>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum ObjectValue<'a> {
    Str(&'a str),
    String(String),
}

impl Object {
    pub fn new(ty: ObjectType, next: RawObject) -> Self {
        Object { ty, next }
    }
}

impl<'a> StringObject<'a> {
    /// Create a new string Object that dosen't take ownership of the string passed in
    /// Conserveatly copies the string from the pointer
    pub fn new(string: &'a str,next: RawObject) -> RawObject {
        let s = StringObject {
            obj: Object::new(ObjectType::String, next),
            chars: ObjectValue::Str(string),
        };

        Box::into_raw(Box::new(s)) as RawObject
    }

    /// Creates a new String Object that takes ownership of the string passed in
    pub fn from_owned(chars: String,next: RawObject) -> RawObject {
        let s = StringObject {
            obj: Object::new(ObjectType::String, next),
            chars: ObjectValue::String(chars),
        };

        Box::into_raw(Box::new(s)) as RawObject
    }
}


impl <'a> ObjectValue<'a> {

    pub fn string(&self) -> &str {
        match *self {
            ObjectValue::Str(ref string) => string,
            ObjectValue::String(ref string) => string, 
        }
    }
}

impl <'a> Deref for StringObject<'a> {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

// impl Drop for Object {
//     fn drop(&mut self) {
//         match self.ty {
//             ObjectType::String => unsafe {
//                 let mut string: &StringObject = mem::transmute(self);
//                 // Frees the string
//                 // ::std::mem::drop(string.chars);
//                 free!(StringObject, &mut string as *mut _ as *mut c_void);
//             },
//         }
//     }
// }

impl<'a> Display for ObjectValue<'a> {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectValue::Str(ref string) => write!(f,"'static: {}",string)?,
            ObjectValue::String(ref string) => write!(f,"new: {}",string)?,
        }
        Ok(())
    }
}

impl<'a> Display for StringObject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "{}", self.chars)?;
        }
        Ok(())
    }
}

// impl Debug for StringObject {
//     fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {

//         unsafe {
//             write!(f, "{:?}",::std::ffi::CStr::from_ptr(self.chars).to_str().unwrap())?;
//         }
//         Ok(())
//     }
// }
