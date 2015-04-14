use gccjit_sys;

use std::marker::{PhantomData, Send};
use std::fmt;

use context::Context;
use field::Field;

use types::Type;
use types;

use object::{ToObject, Object};

pub enum Opaque {}
pub enum Concrete {}

#[derive(Copy, Clone)]
pub struct Struct<'ctx, T> {
    opacity: PhantomData<T>,
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_struct
}

impl<'ctx> Struct<'ctx, Opaque> {
    pub fn set_fields(self, fields: &[Field<'ctx>]) -> Struct<'ctx, Concrete> {
        unimplemented!()
    }
}

impl<'ctx, T> Struct<'ctx, T> {
    pub fn as_type(&self) -> Type<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_struct_as_type(self.ptr);
            types::from_ptr(ptr)
        }
    }
}

impl<'ctx, T> ToObject<'ctx> for Struct<'ctx, T> {
    fn to_object(&self) -> Object<'ctx> {
        let ty = self.as_type();
        ty.to_object()
    }
}

impl<'ctx, T> fmt::Debug for Struct<'ctx, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.as_type();
        obj.fmt(fmt)
    }
}

impl<'ctx, T> !Send for Struct<'ctx, T> {}

pub unsafe fn get_ptr<'ctx, T>(s: &Struct<'ctx, T>) -> *mut gccjit_sys::gcc_jit_struct {
    s.ptr
}

pub unsafe fn from_ptr<'ctx, T>(ptr: *mut gccjit_sys::gcc_jit_struct) -> Struct<'ctx, T> {
    Struct {
        opacity: PhantomData,
        marker: PhantomData,
        ptr: ptr
    }
}




