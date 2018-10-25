use libc::{c_char, c_void, malloc, strcpy};

use std::ops::Deref;
use value::Value;

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum ObjectType {
    String,
}

#[derive(Debug)]
#[repr(C)]
pub struct Object {
    pub ty: ObjectType,
}

#[derive(Debug)]
#[repr(C)]
pub struct StringObject {
    pub obj: Object,
    pub length: usize,
    pub chars: *mut c_char,
}

impl Object {
    pub fn new(ty: ObjectType) -> Self {
        Object { ty }
    }

    // pub fn is_string(&self, val: Value) -> bool {
    //     is_object_type(val, ObjectType::String)
    // }

    
}

// pub fn is_object_type(val: Value, ty: ObjectType) -> bool {

//         val.is_object() && ::std::mem::transmute::<*mut c_void,&Object>(val.as_object()).ty == ty
// }

impl StringObject {
    /// Create a new string Object with the null char added allready
    pub fn new(string: *const c_char, length: usize) -> *mut c_void {
        unsafe {
            // Allocate the memory needed for the string
            // Copy the string to a buffer
            let mut buf = malloc(length * 4) as *mut c_char;

            let chars = strcpy(buf, string) as *mut c_char;

            let s = StringObject {
                obj: Object::new(ObjectType::String),
                length,
                chars,
            };

            Box::into_raw(Box::new(s)) as *mut c_void
        }

        // StringObject {

        // }
    }
}

impl Deref for StringObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
