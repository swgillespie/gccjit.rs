use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_int;

use {Context, LValue, Object, RValue, ToObject, lvalue, object, rvalue};

#[derive(Copy, Clone)]
pub struct ExtendedAsm<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_extended_asm
}

impl<'ctx> ToObject<'ctx> for ExtendedAsm<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_extended_asm_as_object(self.ptr);
            object::from_ptr(ptr)
        }
    }
}

impl<'ctx> ExtendedAsm<'ctx> {
    pub fn set_volatile_flag(&self, flag: bool) {
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_set_volatile_flag(self.ptr, flag as c_int);
        }
    }

    pub fn set_inline_flag(&self, flag: bool) {
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_set_inline_flag(self.ptr, flag as c_int);
        }
    }

    pub fn add_output_operand(&self, asm_symbolic_name: Option<&str>, constraint: &str, dest: LValue<'ctx>) {
        let asm_symbolic_name = asm_symbolic_name.map(|name| CString::new(name).unwrap());
        let asm_symbolic_name =
            match asm_symbolic_name {
                Some(name) => name.as_ptr(),
                None => std::ptr::null_mut(),
            };
        let constraint = CString::new(constraint).unwrap();
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_add_output_operand(self.ptr, asm_symbolic_name, constraint.as_ptr(), lvalue::get_ptr(&dest));
        }
    }

    pub fn add_input_operand(&self, asm_symbolic_name: Option<&str>, constraint: &str, src: RValue<'ctx>) {
        let asm_symbolic_name = asm_symbolic_name.map(|name| CString::new(name).unwrap());
        let asm_symbolic_name =
            match asm_symbolic_name {
                Some(name) => name.as_ptr(),
                None => std::ptr::null_mut(),
            };
        let constraint = CString::new(constraint).unwrap();
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_add_input_operand(self.ptr, asm_symbolic_name, constraint.as_ptr(), rvalue::get_ptr(&src));
        }
    }

    pub fn add_clobber(&self, victim: &str) {
        let victim = CString::new(victim).unwrap();
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_add_clobber(self.ptr, victim.as_ptr());
        }
    }

    pub unsafe fn from_ptr(ptr: *mut gccjit_sys::gcc_jit_extended_asm) -> Self {
        Self {
            marker: PhantomData,
            ptr: ptr
        }
    }
}
