use gccjit_sys;
use context::Context;
use std::marker::{PhantomData, Send};
use std::fmt;
use std::ffi::CStr;
use std::str;

#[derive(Copy, Clone)]
pub struct Object<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_object
}

impl<'ctx> fmt::Debug for Object<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_object_get_debug_string(self.ptr);
            let cstr = CStr::from_ptr(ptr);
            let rust_str = str::from_utf8_unchecked(cstr.to_bytes());
            fmt.write_str(rust_str)
        }
    }
}

pub trait ToObject<'ctx> {
    fn to_object(&self) -> Object<'ctx>;
}

impl<'ctx> ToObject<'ctx> for Object<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe { from_ptr(self.ptr) }
    }
}

impl<'ctx> !Send for Object<'ctx> {}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_object) -> Object<'ctx> {
    Object {
        marker: PhantomData,
        ptr: ptr
    }
}

