use std::marker::{Send, PhantomData};
use std::fmt;
use std::ptr;
use gccjit_sys;
use context::Context;
use object::{ToObject, Object};
use object;
use types::Type;
use types;
use field::Field;
use field;
use lvalue::LValue;
use lvalue;
use location::Location;
use location;

/// An RValue is a value that may or may not have a storage address in gccjit.
/// RValues can be dereferenced, used for field accesses, and are the parameters
/// given to a majority of the gccjit API calls.
#[derive(Copy, Clone)]
pub struct RValue<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_rvalue
}

/// ToRValue is a trait implemented by types that can be converted to, or
/// treated as, an RValue.
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
    /// Gets the type of this RValue.
    pub fn get_type(&self) -> Type<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_rvalue_get_type(self.ptr);
            types::from_ptr(ptr)
        }
    }

    /// Given an RValue x and a Field f, returns an RValue representing
    /// C's x.f.
    pub fn access_field(&self,
                        loc: Option<Location<'ctx>>,
                        field: Field<'ctx>) -> LValue<'ctx> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_rvalue_access_field(self.ptr,
                                                              loc_ptr,
                                                              field::get_ptr(&field));
            lvalue::from_ptr(ptr)
        }
    }

    /// Given an RValue x and a Field f, returns an LValue representing
    /// C's x->f.
    pub fn dereference_field(&self,
                             loc: Option<Location<'ctx>>,
                             field: Field<'ctx>) -> LValue<'ctx> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_rvalue_dereference_field(self.ptr,
                                                                   loc_ptr,
                                                                   field::get_ptr(&field));
            lvalue::from_ptr(ptr)
        }
    }

    /// Given a RValue x, returns an RValue that represents *x.
    pub fn dereference(&self,
                       loc: Option<Location<'ctx>>) -> LValue<'ctx> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_rvalue_dereference(self.ptr,
                                                             loc_ptr);
                                                            
            lvalue::from_ptr(ptr)
        }
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

