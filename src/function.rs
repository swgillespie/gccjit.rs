use std::marker::{PhantomData, Send};
use std::fmt;
use std::ptr;
use context::Context;
use gccjit_sys;
use object::{ToObject, Object};
use object;
use parameter::Parameter;
use parameter;
use std::ffi::CString;
use block::Block;
use block;
use lvalue::LValue;
use lvalue;
use location::Location;
use location;
use types::Type;
use types;

#[repr(C)]
pub enum FunctionType {
    Exported,
    Internal,
    Extern,
    AlwaysInline
}

#[derive(Copy, Clone)]
pub struct Function<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_function
}

impl<'ctx> ToObject<'ctx> for Function<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_function_as_object(self.ptr);
            object::from_ptr(ptr)
        }
    }
}

impl<'ctx> fmt::Debug for Function<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

impl<'ctx> Function<'ctx> {
    pub fn get_param(&self, idx: i32) -> Parameter<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_function_get_param(self.ptr, idx);
            parameter::from_ptr(ptr)
        }
    }

    pub fn dump_to_dot(&self, path: &str) {
        unsafe {
            let cstr = CString::new(path).unwrap();
            gccjit_sys::gcc_jit_function_dump_to_dot(self.ptr, cstr.as_ptr());
        }
    }

    pub fn new_block(&self, name: &str) -> Block<'ctx> {
        unsafe {
            let cstr = CString::new(name).unwrap();
            let ptr = gccjit_sys::gcc_jit_function_new_block(self.ptr,
                                                             cstr.as_ptr());
            block::from_ptr(ptr)
        }
    }

    pub fn new_local(&self,
                     loc: Option<Location<'ctx>>,
                     ty: Type<'ctx>,
                     name: &str) -> LValue<'ctx> {
        unsafe {
            let loc_ptr = match loc {
                Some(loc) => location::get_ptr(&loc),
                None => ptr::null_mut()
            };
            let cstr = CString::new(name).unwrap();
            let ptr = gccjit_sys::gcc_jit_function_new_local(self.ptr,
                                                             loc_ptr,
                                                             types::get_ptr(&ty),
                                                             cstr.as_ptr());
            lvalue::from_ptr(ptr)
        }
    }
}

impl<'ctx> !Send for Function<'ctx> {}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_function) -> Function<'ctx> {
    Function {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(loc: &Function<'ctx>) -> *mut gccjit_sys::gcc_jit_function {
    loc.ptr
}


