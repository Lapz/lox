use libc::{c_char, c_void,malloc, strcpy};
use std::ops::Deref;
use std::mem;
use util::reallocate;
use std::fmt::{self,Display};


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
pub struct StringObject {
    pub obj: Object,
    pub length: usize,
    pub chars: *mut c_char,
}

impl Object {
    pub fn new(ty: ObjectType, next: RawObject) -> Self {
        Object { ty, next }
    }

  
}



impl StringObject {
    /// Create a new string Object that dosen't take ownership of the string passed in
    /// Conserveatly copies the string from the pointer
    pub fn new(string: *const c_char, length: usize, next: RawObject) -> RawObject {
        unsafe {
            // Allocate the memory needed for the string
            // Copy the string to a buffer
            let buf = malloc(length * 4) as *mut c_char;

            let chars = strcpy(buf, string) as *mut c_char;

            

            let s = StringObject {
                obj: Object::new(ObjectType::String, next),
                length,
                chars,
            };

            

            Box::into_raw(Box::new(s)) as RawObject

        }
    }

    /// Creates a new String Object that takes ownership of the string passed in
    pub fn from_owned(chars: *const c_char, length: usize, next: RawObject) -> RawObject {
        let s = StringObject {
            obj: Object::new(ObjectType::String, next),
            length,
            chars: chars as *mut c_char,
        };

        Box::into_raw(Box::new(s)) as RawObject
    }
}

impl Deref for StringObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        match self.ty {
            ObjectType::String => unsafe {
                let string: &StringObject = mem::transmute(self); 
                // Frees the string 
                free_array!(char, string.chars as *mut c_void, string.length + 1);
            
            },
        }
    }
}



impl Display for StringObject {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {

        unsafe {
            write!(f, "{}",::std::ffi::CStr::from_ptr(self.chars).to_str().unwrap())?;
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