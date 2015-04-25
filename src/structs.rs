use gccjit_sys;

use std::marker::PhantomData;
use std::fmt;
use std::ptr;

use context::Context;
use field::Field;
use field;
use types::Type;
use types;
use location::Location;
use location;
use object::{ToObject, Object};

/// A Struct is gccjit's representation of a composite type. Despite the name,
/// Struct can represent either a struct, an union, or an opaque named type.
#[derive(Copy, Clone)]
pub struct Struct<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_struct
}

impl<'ctx> Struct<'ctx> {
    pub fn as_type(&self) -> Type<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_struct_as_type(self.ptr);
            types::from_ptr(ptr)
        }
    }

    pub fn set_fields(&self,
                      location: Option<Location<'ctx>>,
                      fields: &[Field<'ctx>]) {
        let loc_ptr = match location {
                Some(loc) => unsafe { location::get_ptr(&loc) },
                None => ptr::null_mut()
        };
        let num_fields = fields.len() as i32;
        let mut fields_ptrs : Vec<_> = fields.iter()
            .map(|x| unsafe { field::get_ptr(&x) })
            .collect();
        unsafe {
            gccjit_sys::gcc_jit_struct_set_fields(self.ptr,
                                                  loc_ptr,
                                                  num_fields,
                                                  fields_ptrs.as_mut_ptr());
        }
    }
}

impl<'ctx> ToObject<'ctx> for Struct<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        let ty = self.as_type();
        ty.to_object()
    }
}

impl<'ctx> fmt::Debug for Struct<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.as_type();
        obj.fmt(fmt)
    }
}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_struct) -> Struct<'ctx> {
    Struct {
        marker: PhantomData,
        ptr: ptr
    }
}




