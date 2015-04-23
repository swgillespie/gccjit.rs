use std::default::Default;
use std::ops::Drop;
use std::ffi::CString;
use std::marker::{PhantomData, Send};
use std::mem;
use std::ptr;
use std::num::FromPrimitive;

use location::Location;
use location;

use structs::Struct;
use structs;

use types;
use field::Field;
use field;

use rvalue::RValue;
use rvalue;

use function::{Function, FunctionType};
use function;

use block::{BinaryOp, UnaryOp, ComparisonOp};

use parameter::Parameter;
use parameter;

use lvalue::LValue;
use lvalue;

use gccjit_sys;

use gccjit_sys::gcc_jit_int_option::*;
use gccjit_sys::gcc_jit_str_option::*;
use gccjit_sys::gcc_jit_bool_option::*;

/// Represents an optimization level that the JIT compiler
/// will use when compiling your code.
#[repr(C)]
pub enum OptimizationLevel {
    None,
    Limited,
    Standard,
    Aggressive
}

/// Represents a successful compilation of a context. This type
/// provides the means to access compiled functions and globals.
/// JIT compiled functions are exposted to Rust as an extern "C" function
/// pointer.
pub struct CompileResult {
    ptr: *mut gccjit_sys::gcc_jit_result
}

impl CompileResult {
    /// Gets a function pointer to a JIT compiled function. If the function
    /// does not exist (wasn't compiled by the Context that produced this
    /// CompileResult), this function returns None.
    /// It is THE RESPONSIBILITY OF THE CALLER of this function to ensure
    /// that this pointer does not outlive the CompileResult object. This
    /// pointer must be transmuted to a function pointer in order to be
    /// called.
    pub fn get_function(&self, name: &str) -> Option<*mut u8> {
        let c_str = CString::new(name).unwrap();
        unsafe {
            let func = gccjit_sys::gcc_jit_result_get_code(self.ptr,
                                                           c_str.as_ptr());
            if func.is_null() {
                None
            } else {
                Some(mem::transmute(func))
            }
        }
    }

    /// Gets a pointer to a global variable that lives on the JIT heap.
    /// It is similarly the caller's responsibility to ensure that this
    /// value stays valid.
    pub fn get_global(&self, name: &str) -> Option<*mut u8> {
        let c_str = CString::new(name).unwrap();
        unsafe {
            let ptr = gccjit_sys::gcc_jit_result_get_global(self.ptr, c_str.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(mem::transmute(ptr))
            }
        }
    }
}

impl Drop for CompileResult {
    fn drop(&mut self) {
        unsafe {
            gccjit_sys::gcc_jit_result_release(self.ptr);
        }
    }
}

impl !Send for CompileResult {}

/// Wrapper around a GCC JIT context object that keeps
/// the state of the JIT compiler. In GCCJIT, this object
/// is responsible for all memory management of JIT data
/// structures, and as such anything made from this context
/// must have a lifetime strictly less than this object.
///
/// It's possible to create a child context from a parent context.
/// In that case, the child context must have a lifetime strictly
/// less than the parent context.
pub struct Context<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_context
}

impl Default for Context<'static> {
    fn default() -> Context<'static> {
        unsafe {
            Context {
                marker: PhantomData,
                ptr: gccjit_sys::gcc_jit_context_acquire()
            }
        }
    }
}

impl<'ctx> !Send for Context<'ctx> {}

impl<'ctx> Context<'ctx> {
    /// Sets the program name reported by the JIT.
    pub fn set_program_name(&self, name: &str) {
        let c_str = CString::new(name).unwrap();
        unsafe {
            gccjit_sys::gcc_jit_context_set_str_option(self.ptr,
                                                       GCC_JIT_STR_OPTION_PROGNAME,
                                                       c_str.as_ptr());
        }
    }

    /// Sets the optimization level that the JIT compiler will use.
    /// The higher the optimization level, the longer compilation will
    /// take.
    pub fn set_optimization_level(&self, level: OptimizationLevel) {
        unsafe {
            gccjit_sys::gcc_jit_context_set_int_option(self.ptr,
                                                       GCC_JIT_INT_OPTION_OPTIMIZATION_LEVEL,
                                                       level as i32);
        }
    }

    pub fn set_dump_code_on_compile(&self, value: bool) {
        unsafe {
            gccjit_sys::gcc_jit_context_set_bool_option(self.ptr,
                                                        GCC_JIT_BOOL_OPTION_DUMP_GENERATED_CODE,
                                                        value as i32);
        }
    }

    /// Compiles the context and returns a CompileResult that contains
    /// the means to access functions and globals that have currently
    /// been JIT compiled.
    pub fn compile(&self) -> CompileResult {
        unsafe {
            CompileResult {
                ptr: gccjit_sys::gcc_jit_context_compile(self.ptr)
            }
        }
    }

    /// Creates a new child context from this context. The child context
    /// is a fully-featured context, but it has a lifetime that is strictly
    /// less than the lifetime that spawned it.
    pub fn new_child_context<'b>(&'b self) -> Context<'b> {
        unsafe {
            Context {
                marker: PhantomData,
                ptr: gccjit_sys::gcc_jit_context_new_child_context(self.ptr)
            }
        }
    }

    /// Creates a new location for use by gdb when debugging a JIT compiled
    /// program. The filename, line, and col are used by gdb to "show" your
    /// source when in a debugger.
    pub fn new_location<'a>(&'a self,
                        filename: &str,
                        line: i32,
                        col: i32) -> Location<'a> {
        unsafe {
            let cstr = CString::new(filename).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_location(self.ptr,
                                                               cstr.as_ptr(),
                                                               line,
                                                               col);
            location::from_ptr(ptr)
        }
    }

    /// Constructs a new type for any type that implements the Typeable trait.
    /// This library only provides a handful of implementations of Typeable
    /// for some primitive types - utilizers of this library are encouraged
    /// to provide their own types that implement Typeable for ease of type
    /// creation.
    pub fn new_type<'a, T: types::Typeable>(&'a self) -> types::Type<'a> {
        <T as types::Typeable>::get_type(self)
    }

    /// Constructs a new field with an optional source location, type, and name.
    /// This field can be used to compose unions or structs. 
    pub fn new_field<'a>(&'a self,
                     loc: Option<Location<'a>>,
                     ty: types::Type<'a>,
                     name: &str) -> Field<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let cstr = CString::new(name).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_field(self.ptr,
                                                            loc_ptr,
                                                            types::get_ptr(&ty),
                                                            cstr.as_ptr());
            field::from_ptr(ptr)
        }
    }

    /// Constructs a new array type with a given base element type and a
    /// size.
    pub fn new_array_type<'a>(&'a self,
                          loc: Option<Location<'a>>,
                          ty: types::Type<'a>,
                          num_elements: i32) -> types::Type<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_array_type(self.ptr,
                                                                 loc_ptr,
                                                                 types::get_ptr(&ty),
                                                                 num_elements);
            types::from_ptr(ptr)
        }
    }

    /// Constructs a new struct type with the given name, optional source location,
    /// and a list of fields. The returned struct is concrete and new fields cannot
    /// be added to it.
    pub fn new_struct_type<'a>(&'a self,
                           loc: Option<Location<'a>>,
                           name: &str,
                           fields: &[Field<'a>]) -> Struct<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_fields = i32::from_usize(fields.len()).unwrap();
        let mut fields_ptrs : Vec<_> = fields.iter()
            .map(|x| unsafe { field::get_ptr(&x) })
            .collect();
        unsafe {
            let cname = CString::new(name).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_struct_type(self.ptr,
                                                                  loc_ptr,
                                                                  cname.as_ptr(),
                                                                  num_fields,
                                                                  fields_ptrs.as_mut_ptr());
            structs::from_ptr(ptr)
        }
    }

    /// Constructs a new struct type whose fields are not known. Fields can
    /// be added to this struct later, but only once.
    pub fn new_opaque_struct_type<'a>(&'a self,
                                  loc: Option<Location<'a>>,
                                  name: &str) -> Struct<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let cstr = CString::new(name).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_opaque_struct(self.ptr,
                                                                    loc_ptr,
                                                                    cstr.as_ptr());
            structs::from_ptr(ptr)
        }
    }

    /// Creates a new union type from a set of fields.
    pub fn new_union_type<'a>(&'a self,
                              loc: Option<Location<'a>>,
                              name: &str,
                              fields: &[Field<'a>]) -> types::Type<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_fields = i32::from_usize(fields.len()).unwrap();
        let mut fields_ptrs : Vec<_> = fields.iter()
            .map(|x| unsafe { field::get_ptr(&x) })
            .collect();
        unsafe {
            let cname = CString::new(name).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_union_type(self.ptr,
                                                                 loc_ptr,
                                                                 cname.as_ptr(),
                                                                 num_fields,
                                                                 fields_ptrs.as_mut_ptr());
            types::from_ptr(ptr)
        }
    }

    /// Creates a new function pointer type with the given return type
    /// parameter types, and an optional location. The last flag can
    /// make the function variadic, although Rust can't really handle
    /// the varargs calling convention.
    pub fn new_function_pointer_type<'a>(&'a self,
                                         loc: Option<Location<'a>>,
                                         return_type: types::Type<'a>,
                                         param_types: &[types::Type<'a>],
                                         is_variadic: bool) -> types::Type<'a> {
        assert!(!is_variadic, "Rust can't call variadic C methods");
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_types = i32::from_usize(param_types.len()).unwrap();
        let mut types_ptrs : Vec<_> = param_types.iter()
            .map(|x| unsafe { types::get_ptr(&x) })
            .collect();
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_function_ptr_type(self.ptr,
                                                                        loc_ptr,
                                                                        types::get_ptr(&return_type),
                                                                        num_types,
                                                                        types_ptrs.as_mut_ptr(),
                                                                        is_variadic as i32);
            types::from_ptr(ptr)
        }
    }

    /// Creates a new function with the given function kind, return type, parameters, name,
    /// and whether or not the function is variadic. It's not currently possible to call
    /// variadic functions from Rust right now, so that option is turned off for now.
    pub fn new_function<'a>(&'a self,
                            loc: Option<Location<'a>>,
                            kind: FunctionType,
                            return_ty: types::Type<'a>,
                            params: &[Parameter<'a>],
                            name: &str,
                            is_variadic: bool) -> Function<'a> {
        assert!(!is_variadic, "don't support variadic functions yet");
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_params = i32::from_usize(params.len()).unwrap();
        let mut params_ptrs : Vec<_> = params.iter()
            .map(|x| unsafe { parameter::get_ptr(&x) })
            .collect();
        unsafe {
            let cstr = CString::new(name).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_function(self.ptr,
                                                               loc_ptr,
                                                               mem::transmute(kind),
                                                               types::get_ptr(&return_ty),
                                                               cstr.as_ptr(),
                                                               num_params,
                                                               params_ptrs.as_mut_ptr(),
                                                               0);
            function::from_ptr(ptr)
        }
    }

    /// Creates a new binary operation between two RValues and produces a new RValue.
    pub fn new_binary_op<'a>(&'a self,
                        loc: Option<Location<'a>>,
                        op: BinaryOp,
                        ty: types::Type<'a>,
                        left: RValue<'a>,
                        right: RValue<'a>) -> RValue<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_binary_op(self.ptr,
                                                                loc_ptr,
                                                                mem::transmute(op),
                                                                types::get_ptr(&ty),
                                                                rvalue::get_ptr(&left),
                                                                rvalue::get_ptr(&right));
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a unary operation on one RValue and produces a result RValue.
    pub fn new_unary_op<'a>(&'a self,
                        loc: Option<Location<'a>>,
                        op: UnaryOp,
                        ty: types::Type<'a>,
                        rvalue: RValue<'a>) -> RValue<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_unary_op(self.ptr,
                                                               loc_ptr,
                                                               mem::transmute(op),
                                                               types::get_ptr(&ty),
                                                               rvalue::get_ptr(&rvalue));
            rvalue::from_ptr(ptr)
        }
    }

    pub fn new_comparison<'a>(&'a self,
                              loc: Option<Location<'a>>,
                              op: ComparisonOp,
                              left: RValue<'a>,
                              right: RValue<'a>) -> RValue<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_comparison(self.ptr,
                                                                 loc_ptr,
                                                                 mem::transmute(op),
                                                                 rvalue::get_ptr(&left),
                                                                 rvalue::get_ptr(&right));
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a function call to a function object with a given number of parameters.
    /// The RValue that is returned is the result of the function call.
    pub fn new_call<'a>(&'a self,
                    loc: Option<Location<'a>>,
                    func: Function<'a>,
                    args: &[RValue<'a>]) -> RValue<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_params = i32::from_usize(args.len()).unwrap();
        let mut params_ptrs : Vec<_> = args.iter()
            .map(|x| unsafe { rvalue::get_ptr(&x) })
            .collect();
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_call(self.ptr,
                                                           loc_ptr,
                                                           function::get_ptr(&func),
                                                           num_params,
                                                           params_ptrs.as_mut_ptr());
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates an indirect function call that dereferences a function pointer and
    /// attempts to invoke it with the given arguments. The RValue that is returned
    /// is the result of the function call.
    pub fn new_call_through_ptr<'a>(&'a self,
                                    loc: Option<Location<'a>>,
                                    fun_ptr: RValue<'a>,
                                    args: &[RValue<'a>]) -> RValue<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_params = i32::from_usize(args.len()).unwrap();
        let mut params_ptrs : Vec<_> = args.iter()
            .map(|x| unsafe { rvalue::get_ptr(&x) })
            .collect();
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_call_through_ptr(self.ptr,
                                                           loc_ptr,
                                                           rvalue::get_ptr(&fun_ptr),
                                                           num_params,
                                                           params_ptrs.as_mut_ptr());
            rvalue::from_ptr(ptr)
        }
    }

    /// Cast an RValue to a specific type. I don't know what happens when the cast fails yet.
    pub fn new_cast<'a>(&'a self,
                        loc: Option<Location<'a>>,
                        rvalue: RValue<'a>,
                        dest_type: types::Type<'a>) -> RValue<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_cast(self.ptr,
                                                           loc_ptr,
                                                           rvalue::get_ptr(&rvalue),
                                                           types::get_ptr(&dest_type));
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates an LValue from an array pointer and an offset. The LValue can be the target
    /// of an assignment, or it can be converted into an RValue (i.e. loaded).
    pub fn new_array_access<'a>(&'a self,
                            loc: Option<Location<'a>>,
                            array_ptr: RValue<'a>,
                            index: RValue<'a>) -> LValue<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_array_access(self.ptr,
                                                                   loc_ptr,
                                                                   rvalue::get_ptr(&array_ptr),
                                                                   rvalue::get_ptr(&index));
            lvalue::from_ptr(ptr)
        }
    }

    /// Creates a new RValue from a given long value.
    pub fn new_rvalue_from_long<'a>(&'a self,
                                    ty: types::Type<'a>,
                                    value: i64) -> RValue<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_rvalue_from_long(self.ptr,
                                                                       types::get_ptr(&ty),
                                                                       value);
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a new RValue from a given int value.
    pub fn new_rvalue_from_int<'a>(&'a self,
                                   ty: types::Type<'a>,
                                   value: i32) -> RValue<'a> {

        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_rvalue_from_int(self.ptr,
                                                                      types::get_ptr(&ty),
                                                                      value);
            rvalue::from_ptr(ptr)           
        }
    }

    /// Creates a new RValue from a given double value.
    pub fn new_rvalue_from_double<'a>(&'a self,
                                      ty: types::Type<'a>,
                                      value: f64) -> RValue<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_rvalue_from_double(self.ptr,
                                                                       types::get_ptr(&ty),
                                                                       value);
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a zero element for a given type.
    pub fn new_rvalue_zero<'a>(&'a self,
                               ty: types::Type<'a>) -> RValue<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_zero(self.ptr,
                                                       types::get_ptr(&ty));
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a one element for a given type.
    pub fn new_rvalue_one<'a>(&'a self,
                              ty: types::Type<'a>) -> RValue<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_one(self.ptr,
                                                      types::get_ptr(&ty));
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates an RValue for a raw pointer.
    pub unsafe fn new_rvalue_from_ptr<'a>(&'a self,
                                          ty: types::Type<'a>,
                                          value: *mut u8) -> RValue<'a> {
        let ptr = gccjit_sys::gcc_jit_context_new_rvalue_from_ptr(self.ptr,
                                                          types::get_ptr(&ty),
                                                          mem::transmute(value));
        rvalue::from_ptr(ptr)
    }

    /// Creates a null RValue.
    pub fn new_null<'a>(&'a self,
                    ty: types::Type<'a>) -> RValue<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_null(self.ptr,
                                                       types::get_ptr(&ty));
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a string literal RValue.
    pub fn new_string_literal<'a>(&'a self,
                              value: &str) -> RValue<'a> {
        unsafe {
            // FIXME - is this safe?
            let cstr = CString::new(value).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_string_literal(self.ptr,
                                                                     cstr.as_ptr());
            rvalue::from_ptr(ptr)
        }
    }

    /// Dumps a small C file to the path that can be used to reproduce a series
    /// of API calls. You should only ever need to call this if you are debugging
    /// a segfault in gccjit or this library.
    pub fn dump_reproducer_to_file(&self,
                                   path: &str) {
        unsafe {
            let cstr = CString::new(path).unwrap();
            gccjit_sys::gcc_jit_context_dump_reproducer_to_file(self.ptr,
                                                                cstr.as_ptr());
        }
    }

    /// Creates a new parameter with a given type, name, and location.
    pub fn new_parameter<'a>(&'a self,
                         loc: Option<Location<'a>>,
                         ty: types::Type<'a>,
                         name: &str) -> Parameter<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let cstr = CString::new(name).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_param(self.ptr,
                                                            loc_ptr,
                                                            types::get_ptr(&ty),
                                                            cstr.as_ptr());
            parameter::from_ptr(ptr)
        }
    }
}

impl<'ctx> Drop for Context<'ctx> {
    fn drop(&mut self) {
        unsafe {
            gccjit_sys::gcc_jit_context_release(self.ptr);
        }
    }
}

#[doc(hidden)]
pub unsafe fn get_ptr<'ctx>(ctx: &'ctx Context<'ctx>) -> *mut gccjit_sys::gcc_jit_context {
    ctx.ptr
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;
    
    #[test]
    fn create_context() {
        let ctx = Context::default();
    }

    #[test]
    fn create_child_context() {
        let ctx = Context::default();
        let child = ctx.new_child_context();
    }

    #[test]
    fn create_location() {
        let ctx = Context::default();
        let location = ctx.new_location("hello.rs", 1, 32);
    }

    #[test]
    fn create_type() {
        let ctx = Context::default();
        let int_type = ctx.new_type::<i32>();
    }

    #[test]
    fn create_field() {
        let ctx = Context::default();
        let int_type = ctx.new_type::<i32>();
        let int_field = ctx.new_field(None, int_type, "x");
    }

    /* Uncomment these tests periodically to remind yourself of
     * 1) why rust is awesome and 2) make sure that you've set up
     * lifetimes correctly so that these invariant violations are
     * caught at compile time.
    #[test]
    fn invalid_type_lifetime() {
        panic!("this shouldn't compile!");
        let ty = {
            let ctx = Context::default();
            ctx.new_type::<i32>()
        };
    }

    #[test]
    fn create_incorrect_child_context() {
        let child = {
            let mut ctx = Context::default();
            ctx.new_child_context()
        };
    }*/
}
