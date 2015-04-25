use std::marker::PhantomData;
use std::fmt;
use std::ptr;
use gccjit_sys;
use context::Context;
use rvalue::{RValue, ToRValue};
use rvalue;
use object::{ToObject, Object};
use object;
use field::Field;
use field;
use location::Location;
use location;

/// An LValue in gccjit represents a value that has a concrete
/// location in memory. A LValue can be converted into an RValue
/// through the ToRValue trait.
/// It is also possible to get the address of an LValue.
#[derive(Copy, Clone)]
pub struct LValue<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_lvalue
}

/// ToLValue is a trait implemented by types that can be converted (or treated
/// as) LValues.
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
    /// Given an LValue x and a Field f, gets an LValue for the field
    /// access x.f.
    pub fn access_field(&self,
                        loc: Option<Location<'ctx>>,
                        field: Field<'ctx>) -> LValue<'ctx> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_lvalue_access_field(self.ptr,
                                                              loc_ptr,
                                                              field::get_ptr(&field));
            from_ptr(ptr)
        }
    }

    /// Given an LValue x, returns the RValue address of x, akin to C's &x.
    pub fn get_address(&self,
                       loc: Option<Location<'ctx>>) -> RValue<'ctx> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_lvalue_get_address(self.ptr,
                                                             loc_ptr);
            rvalue::from_ptr(ptr)
        }
    }
}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_lvalue) -> LValue<'ctx> {
    LValue {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(lvalue: &LValue<'ctx>) -> *mut gccjit_sys::gcc_jit_lvalue {
    lvalue.ptr
}
