use libc::{c_void, free, realloc};
use std::ptr;

pub unsafe fn reallocate(previous: *mut c_void, old_size: usize, new_size: usize) -> *mut c_void {
    if new_size == 0 {
        free(previous);
        return ptr::null::<c_void>() as *mut c_void;
    }

    return realloc(previous, new_size);
}

macro_rules! free {
    ($ty:ty,$pointer:expr) => {{
        reallocate($pointer, ::std::mem::size_of::<$ty>(), 0)
    }};
}

macro_rules! free_array {
    ($ty:ty,$pointer:expr, $old_count:expr) => {{
        reallocate($pointer, ::std::mem::size_of::<$ty>() * $old_count, 0)
    }};
}
