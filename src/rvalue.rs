use std::marker::{Send, PhantomData};
use std::fmt;
use gccjit_sys;
use context::Context;
use object::{ToObject, Object};
use object;
use types::Type;
use types;
use location::Location;
use field::Field;

#[derive(Copy, Clone)]
pub struct RValue<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_rvalue
}

pub trait ToRValue<'ctx> {
    fn to_rvalue(&self) -> RValue<'ctx>;
}

impl<'ctx> ToObject<'ctx> for RValue<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            object::from_ptr(gccjit_sys::gcc_jit_rvalue_as_object(self.ptr))
        }
    }
}

impl<'ctx> fmt::Debug for RValue<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

impl<'ctx> ToRValue<'ctx> for RValue<'ctx> {
    fn to_rvalue(&self) -> RValue<'ctx> {
        unsafe { from_ptr(self.ptr) }
    }
}

impl<'ctx> !Send for RValue<'ctx> {}

impl<'ctx> RValue<'ctx> {
    pub fn get_type(&self) -> Type<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_rvalue_get_type(self.ptr);
            types::from_ptr(ptr)
        }
    }

    pub fn access_field(&self,
                        loc: Option<Location<'ctx>>,
                        field: Field<'ctx>) -> RValue<'ctx> {
        unimplemented!()
    }

    pub fn dereference_field(&self,
                             loc: Option<Location<'ctx>>,
                             field: Field<'ctx>) -> RValue<'ctx> {
        unimplemented!()
    }

    pub fn dereference(&self,
                       loc: Option<Location<'ctx>>,
                       field: Field<'ctx>) -> RValue<'ctx> {
        unimplemented!()
    }
}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_rvalue) -> RValue<'ctx> {
    RValue {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(rvalue: &RValue<'ctx>) -> *mut gccjit_sys::gcc_jit_rvalue {
    rvalue.ptr
}

