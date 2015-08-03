use std::marker::PhantomData;
use std::fmt;
use std::ptr;
use std::mem;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr};
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
use block::BinaryOp;

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

macro_rules! binary_operator_for {
    ($ty:ty, $name:ident, $op:expr) => {
        impl<'ctx> $ty for RValue<'ctx> {
            type Output = RValue<'ctx>;

            fn $name(self, rhs: RValue<'ctx>) -> RValue<'ctx> {
                unsafe {
                    let rhs_rvalue = rhs.to_rvalue();
                    let obj_ptr = object::get_ptr(&self.to_object());
                    let ctx_ptr = gccjit_sys::gcc_jit_object_get_context(obj_ptr);
                    let ty = rhs.get_type();
                    let ptr = gccjit_sys::gcc_jit_context_new_binary_op(ctx_ptr,
                                                                        ptr::null_mut(),
                                                                        mem::transmute($op),
                                                                        types::get_ptr(&ty),
                                                                        self.ptr,
                                                                        rhs_rvalue.ptr);
                    from_ptr(ptr)
                }
            }
        }
    }
}

// Operator overloads for ease of manipulation of rvalues
binary_operator_for!(Add, add, BinaryOp::Plus);
binary_operator_for!(Sub, sub, BinaryOp::Minus);
binary_operator_for!(Mul, mul, BinaryOp::Mult);
binary_operator_for!(Div, div, BinaryOp::Divide);
binary_operator_for!(Rem, rem, BinaryOp::Modulo);
binary_operator_for!(BitAnd, bitand, BinaryOp::BitwiseAnd);
binary_operator_for!(BitOr, bitor, BinaryOp::BitwiseOr);
binary_operator_for!(BitXor, bitxor, BinaryOp::BitwiseXor);
binary_operator_for!(Shl<RValue<'ctx>>, shl, BinaryOp::LShift);
binary_operator_for!(Shr<RValue<'ctx>>, shr, BinaryOp::RShift);

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

