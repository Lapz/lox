use value::Value;
use std::ops::Deref;

#[derive(PartialEq)]
#[repr(C)]
pub enum ObjectType {
    String,
}

#[repr(C)]
pub struct Object {
    ty:ObjectType,
}

#[repr(C)]
pub struct StringObject {
    pub obj: Object,
    pub length: usize,
    pub chars: *mut u8
}


impl Object {
    pub fn is_string(&self,val:Value,ty:ObjectType)  -> bool {
        val.is_object() && self.ty == ty
    }
}


impl Deref for StringObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}