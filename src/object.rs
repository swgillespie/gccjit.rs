use gccjit_sys;
use context::Context;
use std::marker::PhantomData;
use std::fmt;
use std::ffi::CStr;
use std::str;

/// Object represents the root of all objects in gccjit. It is not useful
/// in and of itself, but it provides the implementation for Debug
/// used by most objects in this library.
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

/// ToObject is a trait implemented by types that can be upcast to Object.
pub trait ToObject<'ctx> {
    fn to_object(&self) -> Object<'ctx>;
}

impl<'ctx> ToObject<'ctx> for Object<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe { from_ptr(self.ptr) }
    }
}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_object) -> Object<'ctx> {
    Object {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(object: &Object<'ctx>) -> *mut gccjit_sys::gcc_jit_object {
    object.ptr
}


