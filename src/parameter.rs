use std::marker::PhantomData;
use std::fmt;
use gccjit_sys;
use context::Context;
use object::{ToObject, Object};
use object;
use rvalue::{RValue, ToRValue};
use rvalue;
use lvalue::{LValue, ToLValue};
use lvalue;

/// Parameter represents a parameter to a function. A series of parameteres
/// can be combined to form a function signature.
#[derive(Copy, Clone)]
pub struct Parameter<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_param
}

impl<'ctx> ToObject<'ctx> for Parameter<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            object::from_ptr(gccjit_sys::gcc_jit_param_as_object(self.ptr))
        }
    }
}

impl<'ctx> fmt::Debug for Parameter<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

impl<'ctx> ToRValue<'ctx> for Parameter<'ctx> {
    fn to_rvalue(&self) -> RValue<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_param_as_rvalue(self.ptr);
            rvalue::from_ptr(ptr)
        }
    }
}

impl<'ctx> ToLValue<'ctx> for Parameter<'ctx> {
    fn to_lvalue(&self) -> LValue<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_param_as_lvalue(self.ptr);
            lvalue::from_ptr(ptr)
        }
    }
}


pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_param) -> Parameter<'ctx> {
    Parameter {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(loc: &Parameter<'ctx>) -> *mut gccjit_sys::gcc_jit_param {
    loc.ptr
}

