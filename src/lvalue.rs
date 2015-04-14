use std::marker::{Send, PhantomData};
use std::fmt;
use gccjit_sys;
use context::Context;
use rvalue::{RValue, ToRValue};
use rvalue;
use object::{ToObject, Object};
use object;
use location::Location;
use field::Field;

#[derive(Copy, Clone)]
pub struct LValue<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_lvalue
}

pub trait ToLValue<'ctx> {
    fn to_lvalue(&self) -> LValue<'ctx>;
}

impl<'ctx> ToObject<'ctx> for LValue<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            object::from_ptr(gccjit_sys::gcc_jit_lvalue_as_object(self.ptr))
        }
    }
}

impl<'ctx> fmt::Debug for LValue<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

impl<'ctx> ToLValue<'ctx> for LValue<'ctx> {
    fn to_lvalue(&self) -> LValue<'ctx> {
        unsafe { from_ptr(self.ptr) }
    }
}

impl<'ctx> ToRValue<'ctx> for LValue<'ctx> {
    fn to_rvalue(&self) -> RValue<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_lvalue_as_rvalue(self.ptr);
            rvalue::from_ptr(ptr)
        }
    }
}

impl<'ctx> LValue<'ctx> {
    pub fn access_field(&self,
                        loc: Option<Location<'ctx>>,
                        field: Field<'ctx>) -> LValue<'ctx> {
        unimplemented!()
    }

    pub fn get_address(&self,
                       loc: Option<Location<'ctx>>) -> RValue<'ctx> {
        unimplemented!()
    }
}

impl<'ctx> !Send for LValue<'ctx> {}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_lvalue) -> LValue<'ctx> {
    LValue {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(lvalue: &LValue<'ctx>) -> *mut gccjit_sys::gcc_jit_lvalue {
    lvalue.ptr
}
