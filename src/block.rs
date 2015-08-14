use std::marker::PhantomData;
use std::ffi::CString;
use std::fmt;
use std::ptr;
use std::mem;
use context::Context;
use gccjit_sys;
use object::{self, ToObject, Object};
use function::{self, Function};
use location::{self, Location};
use rvalue::{self, ToRValue};
use lvalue::{self, ToLValue};

/// BinaryOp is a enum representing the various binary operations
/// that gccjit knows how to codegen.
#[repr(C)]
pub enum BinaryOp {
    Plus,
    Minus,
    Mult,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    LShift,
    RShift
}

/// UnaryOp is an enum representing the various unary operations
/// that gccjit knows how to codegen.
#[repr(C)]
pub enum UnaryOp {
    Minus,
    BitwiseNegate,
    LogicalNegate,
    Abs
}

/// ComparisonOp is an enum representing the various comparisons that
/// gccjit is capable of doing.
#[repr(C)]
pub enum ComparisonOp {
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals
}

/// Block represents a basic block in gccjit. Blocks are created by functions.
/// A basic block consists of a series of instructions terminated by a terminator
/// instruction, which can be either a jump to one block, a conditional branch to
/// two blocks (true/false branches), a return, or a void return.
#[derive(Copy, Clone)]
pub struct Block<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_block
}

impl<'ctx> ToObject<'ctx> for Block<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_block_as_object(self.ptr);
            object::from_ptr(ptr)
        }
    }
}

impl<'ctx> fmt::Debug for Block<'ctx> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

impl<'ctx> Block<'ctx> {
    pub fn get_function(&self) -> Function<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_block_get_function(self.ptr);
            function::from_ptr(ptr)
        }
    }

    /// Evaluates the rvalue parameter and discards its result. Equivalent
    /// to (void)<expr> in C.
    pub fn add_eval<T: ToRValue<'ctx>>(&self,
                                       loc: Option<Location<'ctx>>,
                                       value: T) {
        let rvalue = value.to_rvalue();
        let loc_ptr = match loc {
                Some(loc) => unsafe { location::get_ptr(&loc) },
                None => ptr::null_mut()
            };
        unsafe {
            gccjit_sys::gcc_jit_block_add_eval(self.ptr,
                                               loc_ptr,
                                               rvalue::get_ptr(&rvalue));
        }
    }

    /// Assigns the value of an rvalue to an lvalue directly. Equivalent
    /// to <lvalue> = <rvalue> in C.
    pub fn add_assignment<L: ToLValue<'ctx>, R: ToRValue<'ctx>>(&self,
                                                                loc: Option<Location<'ctx>>,
                                                                assign_target: L,
                                                                value: R) {
        let lvalue = assign_target.to_lvalue();
        let rvalue = value.to_rvalue();
        let loc_ptr = match loc {
                Some(loc) => unsafe { location::get_ptr(&loc) },
                None => ptr::null_mut()
            };
        unsafe {
            gccjit_sys::gcc_jit_block_add_assignment(self.ptr,
                                                     loc_ptr,
                                                     lvalue::get_ptr(&lvalue),
                                                     rvalue::get_ptr(&rvalue));
        }
    }

    /// Performs a binary operation on an LValue and an RValue, assigning
    /// the result of the binary operation to the LValue upon completion.
    /// Equivalent to the *=, +=, -=, etc. operator family in C.
    pub fn add_assignment_op<L: ToLValue<'ctx>, R: ToRValue<'ctx>>(&self,
                                                                   loc: Option<Location<'ctx>>,
                                                                   assign_target: L,
                                                                   op: BinaryOp,
                                                                   value: R) {
        let lvalue = assign_target.to_lvalue();
        let rvalue = value.to_rvalue();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            gccjit_sys::gcc_jit_block_add_assignment_op(self.ptr,
                                                        loc_ptr,
                                                        lvalue::get_ptr(&lvalue),
                                                        mem::transmute(op),
                                                        rvalue::get_ptr(&rvalue));
        }
    }

    /// Adds a comment to a block. It's unclear from the documentation what
    /// this actually means.
    pub fn add_comment<S: AsRef<str>>(&self,
                       loc: Option<Location<'ctx>>,
                       message: S) {
        let message_ref = message.as_ref();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let cstr = CString::new(message_ref).unwrap();
            gccjit_sys::gcc_jit_block_add_comment(self.ptr,
                                                  loc_ptr,
                                                  cstr.as_ptr());
        }
    }

    /// Terminates a block by branching to one of two blocks, depending
    /// on the value of a conditional RValue.
    pub fn end_with_conditional<T: ToRValue<'ctx>>(&self,
                                loc: Option<Location<'ctx>>,
                                cond: T,
                                on_true: Block<'ctx>,
                                on_false: Block<'ctx>) {
        let cond_rvalue = cond.to_rvalue();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            gccjit_sys::gcc_jit_block_end_with_conditional(self.ptr,
                                                           loc_ptr,
                                                           rvalue::get_ptr(&cond_rvalue),
                                                           on_true.ptr,
                                                           on_false.ptr);
        }
    }

    /// Terminates a block by unconditionally jumping to another block.
    pub fn end_with_jump(&self,
                         loc: Option<Location<'ctx>>,
                         target: Block<'ctx>) {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            gccjit_sys::gcc_jit_block_end_with_jump(self.ptr,
                                                    loc_ptr,
                                                    target.ptr);
        }
    }

    /// Terminates a block by returning from the containing function, setting
    /// the rvalue to be the return value of the function. This is equivalent
    /// to C's "return <expr>". This function can only be used to terminate
    /// a block within a function whose return type is not void.
    pub fn end_with_return<T: ToRValue<'ctx>>(&self,
                                              loc: Option<Location<'ctx>>,
                                              ret: T) {
        let ret_rvalue = ret.to_rvalue();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            gccjit_sys::gcc_jit_block_end_with_return(self.ptr,
                                                      loc_ptr,
                                                      rvalue::get_ptr(&ret_rvalue));
        }
    }

    /// Terminates a block by returning from the containing function, returning
    /// no value. This is equivalent to C's bare "return" with no expression.
    /// This function can only be used to terminate a block within a function
    /// that returns void.
    pub fn end_with_void_return(&self, loc: Option<Location<'ctx>>) {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            gccjit_sys::gcc_jit_block_end_with_void_return(self.ptr,
                                                           loc_ptr);
        }
    }
}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_block) -> Block<'ctx> {
    Block {
        marker: PhantomData,
        ptr: ptr
    }
}
