use gccjit_sys;

use std::marker::PhantomData;
use std::fmt;

use context::Context;
use object::{ToObject, Object};
use object;

/// Field represents a field that composes structs or unions. A number of fields
/// can be combined to create either a struct or a union.
#[derive(Copy, Clone)]
pub struct Field<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_field
}

impl<'ctx> ToObject<'ctx> for Field<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            object::from_ptr(gccjit_sys::gcc_jit_field_as_object(self.ptr))
        }
    }
}

impl<'ctx> fmt::Debug for Field<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_field) -> Field<'ctx> {
    Field {
        marker: PhantomData,
        ptr: ptr
    }
}

pub unsafe fn get_ptr<'ctx>(f: &Field<'ctx>) -> *mut gccjit_sys::gcc_jit_field {
    f.ptr
}
