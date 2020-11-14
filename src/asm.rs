use std::ffi::CStr;
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
    pub fn gcc_jit_extended_asm_set_volatile_flag(&self, flag: bool) {
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_set_volatile_flag(self.ptr, flag as c_int);
        }
    }

    pub fn set_inline_flag(&self, flag: bool) {
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_set_inline_flag(self.ptr, flag as c_int);
        }
    }

    pub fn add_output_operand(&self, asm_symbolic_name: &str, constraint: &str, dest: LValue<'ctx>) {
        let asm_symbolic_name = CStr::from_bytes_with_nul(asm_symbolic_name.as_bytes()).expect("asm symbolic name to cstring");
        let constraint = CStr::from_bytes_with_nul(constraint.as_bytes()).expect("constraint to cstring");
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_add_output_operand(self.ptr, asm_symbolic_name.as_ptr(), constraint.as_ptr(), lvalue::get_ptr(&dest));
        }
    }

    pub fn add_input_operand(&self, asm_symbolic_name: &str, constraint: &str, src: RValue<'ctx>) {
        let asm_symbolic_name = CStr::from_bytes_with_nul(asm_symbolic_name.as_bytes()).expect("asm symbolic name to cstring");
        let constraint = CStr::from_bytes_with_nul(constraint.as_bytes()).expect("constraint to cstring");
        unsafe {
            gccjit_sys::gcc_jit_extended_asm_add_input_operand(self.ptr, asm_symbolic_name.as_ptr(), constraint.as_ptr(), rvalue::get_ptr(&src));
        }
    }

    pub fn add_clobber(&self, victim: &str) {
        let victim = CStr::from_bytes_with_nul(victim.as_bytes()).expect("victim to cstring");
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
