use std::marker::PhantomData;
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

/// FunctionType informs gccjit what sort of function a new function will be.
/// An exported function is a function that will be exported using the CompileResult
/// interface, able to be called outside of the jit. An internal function is
/// a function that cannot be called outside of jitted code. An extern function
/// is a function with external linkage, and always inline is a function that is
/// always inlined wherever it is called and cannot be accessed outside of the jit.
#[repr(C)]
pub enum FunctionType {
    /// Defines a function that is "exported" by the JIT and can be called from
    /// Rust.
    Exported,
    /// Defines a function that is internal to the JIT and cannot be called
    /// from Rust, but can be called from jitted code.
    Internal,
    /// Defines a function with external linkage.
    Extern,
    /// Defines a function that should always be inlined whenever it is called.
    /// A function with this type cannot be called from Rust. If the optimization
    /// level is None, this function will not actually be inlined, but it still
    /// can only be called from within jitted code.
    AlwaysInline
}

/// Function is gccjit's representation of a function. Functions are constructed
/// by constructing basic blocks and connecting them together. Locals are declared
/// at the function level.
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

    pub fn dump_to_dot<S: AsRef<str>>(&self, path: S) {
        unsafe {
            let cstr = CString::new(path.as_ref()).unwrap();
            gccjit_sys::gcc_jit_function_dump_to_dot(self.ptr, cstr.as_ptr());
        }
    }

    pub fn new_block<S: AsRef<str>>(&self, name: S) -> Block<'ctx> {
        unsafe {
            let cstr = CString::new(name.as_ref()).unwrap();
            let ptr = gccjit_sys::gcc_jit_function_new_block(self.ptr,
                                                             cstr.as_ptr());
            block::from_ptr(ptr)
        }
    }

    pub fn new_local<S: AsRef<str>>(&self,
                     loc: Option<Location<'ctx>>,
                     ty: Type<'ctx>,
                     name: S) -> LValue<'ctx> {
        unsafe {
            let loc_ptr = match loc {
                Some(loc) => location::get_ptr(&loc),
                None => ptr::null_mut()
            };
            let cstr = CString::new(name.as_ref()).unwrap();
            let ptr = gccjit_sys::gcc_jit_function_new_local(self.ptr,
                                                             loc_ptr,
                                                             types::get_ptr(&ty),
                                                             cstr.as_ptr());
            lvalue::from_ptr(ptr)
        }
    }
}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_function) -> Function<'ctx> {
    Function {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(loc: &Function<'ctx>) -> *mut gccjit_sys::gcc_jit_function {
    loc.ptr
}


