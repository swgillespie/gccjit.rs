use std::default::Default;
use std::ops::Drop;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::str::Utf8Error;

use gccjit_sys;
use gccjit_sys::gcc_jit_int_option::*;
use gccjit_sys::gcc_jit_str_option::*;
use gccjit_sys::gcc_jit_bool_option::*;

use block::{self, BinaryOp, Block, UnaryOp, ComparisonOp};
use field::{self, Field};
use function::{self, Function, FunctionType};
use location::{self, Location};
use lvalue::{self, LValue};
use object::{self, Object, ToObject};
use parameter::{self, Parameter};
use rvalue::{self, RValue, ToRValue};
use structs::{self, Struct};
use Type;
use types;

#[repr(C)]
#[derive(Debug)]
pub enum GlobalKind {
    Exported,
    Internal,
    Imported,
}

/// Represents an optimization level that the JIT compiler
/// will use when compiling your code.
#[repr(C)]
#[derive(Debug)]
pub enum OptimizationLevel {
    /// No optimizations are applied.
    None,
    /// Optimizies for both speed and code size, but doesn't apply
    /// any optimizations that take extended periods of time.
    Limited,
    /// Performs all optimizations that do not involve a tradeoff
    /// of code size for speed.
    Standard,
    /// Performs all optimizations at the Standard level, as well
    /// as function inlining, loop vectorization, some loop unrolling,
    /// and various other optimizations.
    Aggressive
}

/// This enum indicates to gccjit the format of the output
/// code that is written out by compile_to_file.
#[repr(C)]
pub enum OutputKind {
    /// Outputs an assembly file (.S)
    Assembler,
    /// Outputs an object file (.o)
    ObjectFile,
    /// Outputs a dynamic library (.so)
    DynamicLibrary,
    /// Outputs an executable
    Executable
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
    /// CompileResult), this function returns a null pointer.
    ///
    /// It is the caller's responsibility to ensure that this pointer is not used
    /// past the lifetime of the CompileResult object. Second, it is
    /// the caller's responsibility to check whether or not the pointer
    /// is null. It is also expected that the caller of this function
    /// will transmute this pointer to a function pointer type.
    pub fn get_function<S: AsRef<str>>(&self, name: S) -> *mut () {
        let c_str = CString::new(name.as_ref()).unwrap();
        unsafe {
            let func = gccjit_sys::gcc_jit_result_get_code(self.ptr,
                                                           c_str.as_ptr());
            mem::transmute(func)
        }
    }

    /// Gets a pointer to a global variable that lives on the JIT heap.
    ///
    /// It is the caller's responsibility
    /// to ensure that the pointer is not used past the lifetime of the
    /// CompileResult object. It is also the caller's responsibility to
    /// check whether or not the returned pointer is null.
    pub fn get_global<S: AsRef<str>>(&self, name: S) -> *mut () {
        let c_str = CString::new(name.as_ref()).unwrap();
        unsafe {
            let ptr = gccjit_sys::gcc_jit_result_get_global(self.ptr, c_str.as_ptr());
            mem::transmute(ptr)
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

pub struct Case<'ctx> {
    marker: PhantomData<&'ctx Case<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_case,
}

impl<'ctx> ToObject<'ctx> for Case<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_case_as_object(self.ptr);
            object::from_ptr(ptr)
        }
    }
}

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

impl<'ctx> Context<'ctx> {
    /// Sets the program name reported by the JIT.
    pub fn set_program_name<S: AsRef<str>>(&self, name: S) {
        let name_ref = name.as_ref();
        let c_str = CString::new(name_ref).unwrap();
        unsafe {
            gccjit_sys::gcc_jit_context_set_str_option(self.ptr,
                                                       GCC_JIT_STR_OPTION_PROGNAME,
                                                       c_str.as_ptr());
        }
    }

    pub fn add_command_line_option<S: AsRef<str>>(&self, name: S) {
        let c_str = CString::new(name.as_ref()).unwrap();
        unsafe {
            gccjit_sys::gcc_jit_context_add_command_line_option(self.ptr, c_str.as_ptr())
        }
    }

    pub fn add_driver_option<S: AsRef<str>>(&self, name: S) {
        let c_str = CString::new(name.as_ref()).unwrap();
        unsafe {
            gccjit_sys::gcc_jit_context_add_driver_option(self.ptr, c_str.as_ptr())
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

    pub fn set_debug_info(&self, value: bool) {
        unsafe {
            gccjit_sys::gcc_jit_context_set_bool_option(self.ptr,
                                                        GCC_JIT_BOOL_OPTION_DEBUGINFO,
                                                        value as i32);
        }
    }

    pub fn set_keep_intermediates(&self, value: bool) {
        unsafe {
            gccjit_sys::gcc_jit_context_set_bool_option(self.ptr,
                                                        GCC_JIT_BOOL_OPTION_KEEP_INTERMEDIATES,
                                                        value as i32);
        }
    }

    pub fn set_dump_everything(&self, value: bool) {
        unsafe {
            gccjit_sys::gcc_jit_context_set_bool_option(self.ptr,
                                                        GCC_JIT_BOOL_OPTION_DUMP_EVERYTHING,
                                                        value as i32);
        }
    }

    pub fn set_dump_initial_gimple(&self, value: bool) {
        unsafe {
            gccjit_sys::gcc_jit_context_set_bool_option(self.ptr,
                                                        GCC_JIT_BOOL_OPTION_DUMP_INITIAL_GIMPLE,
                                                        value as i32);
        }
    }

    /// When set to true, dumps the code that the JIT generates to standard
    /// out during compilation.
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

    /// Compiles the context and saves the result to a file. The
    /// type of the file is controlled by the OutputKind parameter.
    pub fn compile_to_file<S: AsRef<str>>(&self, kind: OutputKind, file: S) {
        unsafe {
            let file_ref = file.as_ref();
            let cstr = CString::new(file_ref).unwrap();
            gccjit_sys::gcc_jit_context_compile_to_file(self.ptr,
                                                        mem::transmute(kind),
                                                        cstr.as_ptr());
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

    pub fn new_case<S: ToRValue<'ctx>, T: ToRValue<'ctx>>(&self, min_value: S, max_value: T, dest_block: Block<'ctx>) -> Case {
        let min_value = min_value.to_rvalue();
        let max_value = max_value.to_rvalue();
        unsafe {
            Case {
                marker: PhantomData,
                ptr: gccjit_sys::gcc_jit_context_new_case(self.ptr, rvalue::get_ptr(&min_value), rvalue::get_ptr(&max_value),
                    block::get_ptr(&dest_block)),
            }
        }
    }

    /// Creates a new location for use by gdb when debugging a JIT compiled
    /// program. The filename, line, and col are used by gdb to "show" your
    /// source when in a debugger.
    pub fn new_location<'a, S: AsRef<str>>(&'a self,
                                           filename: S,
                                           line: i32,
                                           col: i32) -> Location<'a> {
        unsafe {
            let filename_ref = filename.as_ref();
            let cstr = CString::new(filename_ref).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_location(self.ptr,
                                                               cstr.as_ptr(),
                                                               line,
                                                               col);
            location::from_ptr(ptr)
        }
    }

    pub fn new_global<'a, S: AsRef<str>>(&self, loc: Option<Location<'a>>, kind: GlobalKind, ty: Type<'a>, name: S) -> LValue<'a> {
        unsafe {
            let loc_ptr = match loc {
                Some(loc) => location::get_ptr(&loc),
                None => ptr::null_mut()
            };
            let cstr = CString::new(name.as_ref()).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_global(
                self.ptr,
                loc_ptr,
                mem::transmute(kind),
                types::get_ptr(&ty),
                cstr.as_ptr());
            lvalue::from_ptr(ptr)
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

    pub fn new_c_type<'a>(&'a self, c_type: CType) -> types::Type<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_get_type(get_ptr(self), c_type.to_sys());
            types::from_ptr(ptr)
        }
    }

    pub fn new_int_type<'a>(&'a self, num_bytes: i32, signed: bool) -> types::Type<'a> {
        unsafe {
            let ctx_ptr = get_ptr(self);
            let ptr = gccjit_sys::gcc_jit_context_get_int_type(ctx_ptr, num_bytes, signed as i32);
            types::from_ptr(ptr)
        }
    }

    /// Constructs a new field with an optional source location, type, and name.
    /// This field can be used to compose unions or structs.
    pub fn new_field<'a, S: AsRef<str>>(&'a self,
                         loc: Option<Location<'a>>,
                         ty: types::Type<'a>,
                         name: S) -> Field<'a> {
        let name_ref = name.as_ref();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let cstr = CString::new(name_ref).unwrap();
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

    pub fn new_vector_type<'a>(&'a self, ty: types::Type<'a>, num_units: u64) -> types::Type<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_type_get_vector(types::get_ptr(&ty), num_units);
            types::from_ptr(ptr)
        }
    }

    /// Constructs a new struct type with the given name, optional source location,
    /// and a list of fields. The returned struct is concrete and new fields cannot
    /// be added to it.
    pub fn new_struct_type<'a, S: AsRef<str>>(&'a self,
                                              loc: Option<Location<'a>>,
                                              name: S,
                                              fields: &[Field<'a>]) -> Struct<'a> {
        let name_ref = name.as_ref();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_fields = fields.len() as i32;
        let mut fields_ptrs : Vec<_> = fields.iter()
            .map(|x| unsafe { field::get_ptr(&x) })
            .collect();
        unsafe {
            let cname = CString::new(name_ref).unwrap();
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
    pub fn new_opaque_struct_type<'a, S: AsRef<str>>(&'a self,
                                                     loc: Option<Location<'a>>,
                                                     name: S) -> Struct<'a> {
        let name_ref = name.as_ref();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let cstr = CString::new(name_ref).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_opaque_struct(self.ptr,
                                                                    loc_ptr,
                                                                    cstr.as_ptr());
            structs::from_ptr(ptr)
        }
    }

    /// Creates a new union type from a set of fields.
    pub fn new_union_type<'a, S: AsRef<str>>(&'a self,
                                             loc: Option<Location<'a>>,
                                             name: S,
                                             fields: &[Field<'a>]) -> types::Type<'a> {
        let name_ref = name.as_ref();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_fields = fields.len() as i32;
        let mut fields_ptrs : Vec<_> = fields.iter()
            .map(|x| unsafe { field::get_ptr(&x) })
            .collect();
        unsafe {
            let cname = CString::new(name_ref).unwrap();
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
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_types = param_types.len() as i32;
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
    /// and whether or not the function is variadic.
    pub fn new_function<'a, S: AsRef<str>>(&'a self,
                                           loc: Option<Location<'a>>,
                                           kind: FunctionType,
                                           return_ty: types::Type<'a>,
                                           params: &[Parameter<'a>],
                                           name: S,
                                           is_variadic: bool) -> Function<'a> {
        let name_ref = name.as_ref();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_params = params.len() as i32;
        let mut params_ptrs : Vec<_> = params.iter()
            .map(|x| unsafe { parameter::get_ptr(&x) })
            .collect();
        unsafe {
            let cstr = CString::new(name_ref).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_function(self.ptr,
                                                               loc_ptr,
                                                               mem::transmute(kind),
                                                               types::get_ptr(&return_ty),
                                                               cstr.as_ptr(),
                                                               num_params,
                                                               params_ptrs.as_mut_ptr(),
                                                               is_variadic as i32);
            function::from_ptr(ptr)
        }
    }

    /// Creates a new binary operation between two RValues and produces a new RValue.
    pub fn new_binary_op<'a, L: ToRValue<'a>, R: ToRValue<'a>>(&'a self,
                                                               loc: Option<Location<'a>>,
                                                               op: BinaryOp,
                                                               ty: types::Type<'a>,
                                                               left: L,
                                                               right: R) -> RValue<'a> {
        let left_rvalue = left.to_rvalue();
        let right_rvalue = right.to_rvalue();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_binary_op(self.ptr,
                                                                loc_ptr,
                                                                mem::transmute(op),
                                                                types::get_ptr(&ty),
                                                                rvalue::get_ptr(&left_rvalue),
                                                                rvalue::get_ptr(&right_rvalue));
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = self.get_last_error() {
                panic!("{}", error);
            }
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a unary operation on one RValue and produces a result RValue.
    pub fn new_unary_op<'a, T: ToRValue<'a>>(&'a self,
                                             loc: Option<Location<'a>>,
                                             op: UnaryOp,
                                             ty: types::Type<'a>,
                                             target: T) -> RValue<'a> {
        let rvalue = target.to_rvalue();
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

    pub fn new_comparison<'a, L: ToRValue<'a>, R: ToRValue<'a>>(&'a self,
                                                                loc: Option<Location<'a>>,
                                                                op: ComparisonOp,
                                                                left: L,
                                                                right: R) -> RValue<'a> {
        let left_rvalue = left.to_rvalue();
        let right_rvalue = right.to_rvalue();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_comparison(self.ptr,
                                                                 loc_ptr,
                                                                 mem::transmute(op),
                                                                 rvalue::get_ptr(&left_rvalue),
                                                                 rvalue::get_ptr(&right_rvalue));
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = self.get_last_error() {
                panic!("{}", error);
            }
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a function call to a function object with a given number of parameters.
    /// The RValue that is returned is the result of the function call.
    /// Note that due to the way that Rust's generics work, it is currently
    /// not possible to be generic over different types of arguments (RValues
    /// together with LValues and Parameters, for example), so in order to
    /// mix the types of the arguments it may be necessary to call to_rvalue()
    /// before calling this function.
    pub fn new_call<'a>(&'a self,
                        loc: Option<Location<'a>>,
                        func: Function<'a>,
                        args: &[RValue<'a>]) -> RValue<'a> {
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_params = args.len() as i32;
        let mut params_ptrs : Vec<_> = args.iter()
            .map(|x| unsafe { rvalue::get_ptr(&x) })
            .collect();
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_call(self.ptr,
                                                           loc_ptr,
                                                           function::get_ptr(&func),
                                                           num_params,
                                                           params_ptrs.as_mut_ptr());
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = self.get_last_error() {
                panic!("{}", error);
            }
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates an indirect function call that dereferences a function pointer and
    /// attempts to invoke it with the given arguments. The RValue that is returned
    /// is the result of the function call.
    pub fn new_call_through_ptr<'a, F: ToRValue<'a>>(&'a self,
                                                     loc: Option<Location<'a>>,
                                                     fun_ptr: F,
                                                     args: &[RValue<'a>]) -> RValue<'a> {
        let fun_ptr_rvalue = fun_ptr.to_rvalue();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        let num_params = args.len() as i32;
        let mut params_ptrs : Vec<_> = args.iter()
            .map(|x| x.to_rvalue())
            .map(|x| unsafe { rvalue::get_ptr(&x) })
            .collect();
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_call_through_ptr(self.ptr,
                                                           loc_ptr,
                                                           rvalue::get_ptr(&fun_ptr_rvalue),
                                                           num_params,
                                                           params_ptrs.as_mut_ptr());
            rvalue::from_ptr(ptr)
        }
    }

    /// Cast an RValue to a specific type. I don't know what happens when the cast fails yet.
    pub fn new_cast<'a, T: ToRValue<'a>>(&'a self,
                                         loc: Option<Location<'a>>,
                                         value: T,
                                         dest_type: types::Type<'a>) -> RValue<'a> {
        let rvalue = value.to_rvalue();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_cast(self.ptr,
                                                           loc_ptr,
                                                           rvalue::get_ptr(&rvalue),
                                                           types::get_ptr(&dest_type));
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = self.get_last_error() {
                panic!("{}", error);
            }
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates an LValue from an array pointer and an offset. The LValue can be the target
    /// of an assignment, or it can be converted into an RValue (i.e. loaded).
    pub fn new_array_access<'a, A: ToRValue<'a>, I: ToRValue<'a>>(&'a self,
                                                                  loc: Option<Location<'a>>,
                                                                  array_ptr: A,
                                                                  index: I) -> LValue<'a> {
        let array_rvalue = array_ptr.to_rvalue();
        let idx_rvalue = index.to_rvalue();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_array_access(self.ptr,
                                                                   loc_ptr,
                                                                   rvalue::get_ptr(&array_rvalue),
                                                                   rvalue::get_ptr(&idx_rvalue));
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
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = self.get_last_error() {
                panic!("{}", error);
            }
            rvalue::from_ptr(ptr)
        }
    }

    pub fn new_rvalue_from_vector<'a>(&'a self, loc: Option<Location<'a>>, vec_type: types::Type<'a>, elements: &[RValue<'a>]) -> RValue<'a> {
        unsafe {
            let loc_ptr = match loc {
                Some(loc) => location::get_ptr(&loc),
                None => ptr::null_mut()
            };
            let ptr = gccjit_sys::gcc_jit_context_new_rvalue_from_vector(self.ptr, loc_ptr, types::get_ptr(&vec_type), elements.len() as _, elements.as_ptr() as *mut *mut _);
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

    /// Creates an RValue for a raw pointer. This function
    /// requires that the lifetime of the pointer be greater
    /// than that of the jitted program.
    pub fn new_rvalue_from_ptr<'a>(&'a self,
                                   ty: types::Type<'a>,
                                   value: *mut ()) -> RValue<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_new_rvalue_from_ptr(self.ptr,
                                                                      types::get_ptr(&ty),
                                                                      mem::transmute(value));
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a null RValue.
    pub fn new_null<'a>(&'a self,
                        ty: types::Type<'a>) -> RValue<'a> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_context_null(self.ptr,
                                                       types::get_ptr(&ty));
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = self.get_last_error() {
                panic!("{}", error);
            }
            rvalue::from_ptr(ptr)
        }
    }

    /// Creates a string literal RValue.
    pub fn new_string_literal<'a, S: AsRef<str>>(&'a self,
                                  value: S) -> RValue<'a> {
        unsafe {
            let cstr = CString::new(value.as_ref()).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_string_literal(self.ptr,
                                                                     cstr.as_ptr());
            rvalue::from_ptr(ptr)
        }
    }

    /// Dumps a small C file to the path that can be used to reproduce a series
    /// of API calls. You should only ever need to call this if you are debugging
    /// an issue in gccjit itself or this library.
    pub fn dump_reproducer_to_file<S: AsRef<str>>(&self,
                                                  path: S) {
        unsafe {
            let path_ref = path.as_ref();
            let cstr = CString::new(path_ref).unwrap();
            gccjit_sys::gcc_jit_context_dump_reproducer_to_file(self.ptr,
                                                                cstr.as_ptr());
        }
    }

    /// Creates a new parameter with a given type, name, and location.
    pub fn new_parameter<'a, S: AsRef<str>>(&'a self,
                                            loc: Option<Location<'a>>,
                                            ty: types::Type<'a>,
                                            name: S) -> Parameter<'a> {
        let name_ref = name.as_ref();
        let loc_ptr = match loc {
            Some(loc) => unsafe { location::get_ptr(&loc) },
            None => ptr::null_mut()
        };
        unsafe {
            let cstr = CString::new(name_ref).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_new_param(self.ptr,
                                                            loc_ptr,
                                                            types::get_ptr(&ty),
                                                            cstr.as_ptr());
            parameter::from_ptr(ptr)
        }
    }

    /// Get a builtin function from gcc. It's not clear what functions are
    /// builtin and you'll likely need to consult the GCC internal docs
    /// for a full list.
    pub fn get_builtin_function<'a, S: AsRef<str>>(&'a self, name: S) -> Function<'a> {
        let name_ref = name.as_ref();
        unsafe {
            let cstr = CString::new(name_ref).unwrap();
            let ptr = gccjit_sys::gcc_jit_context_get_builtin_function(self.ptr,
                                                                       cstr.as_ptr());
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = self.get_last_error() {
                panic!("{}", error);
            }
            function::from_ptr(ptr)
        }
    }

    pub fn get_first_error(&self) -> Result<Option<&'ctx str>, Utf8Error> {
        unsafe {
            let str = gccjit_sys::gcc_jit_context_get_first_error(self.ptr);
            if str.is_null() {
                Ok(None)
            }
            else {
                Ok(Some(CStr::from_ptr(str).to_str()?))
            }
        }
    }

    pub fn get_last_error(&self) -> Result<Option<&'ctx str>, Utf8Error> {
        unsafe {
            let str = gccjit_sys::gcc_jit_context_get_last_error(self.ptr);
            if str.is_null() {
                Ok(None)
            }
            else {
                Ok(Some(CStr::from_ptr(str).to_str()?))
            }
        }
    }

    pub fn set_logfile<S: AsRef<str>>(&self, logfile: S) {
        use std::os::raw::c_void;

        extern {
            static stderr: *mut c_void;
        }

        unsafe {
            gccjit_sys::gcc_jit_context_set_logfile(self.ptr, stderr as *mut _, 0, 0);
        }
    }

    pub fn add_top_level_asm(&self, loc: Option<Location<'ctx>>, asm_stmts: &str) {
        let asm_stmts = CStr::from_bytes_with_nul(asm_stmts.as_bytes()).expect("asm_stmts to cstring");
        let loc_ptr =
            match loc {
                Some(loc) => unsafe { location::get_ptr(&loc) },
                None => ptr::null_mut(),
            };
        unsafe {
            gccjit_sys::gcc_jit_context_add_top_level_asm(self.ptr, loc_ptr, asm_stmts.as_ptr());
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

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_context) -> Context<'ctx> {
    Context {
        marker: PhantomData,
        ptr: ptr
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::default::Default;
    use std::mem;

    #[test]
    fn create_context() {
        let _ctx = Context::default();
    }

    #[test]
    fn create_child_context() {
        let ctx = Context::default();
        let _child = ctx.new_child_context();
    }

    #[test]
    fn create_location() {
        let ctx = Context::default();
        let _location = ctx.new_location("hello.rs", 1, 32);
    }

    #[test]
    fn create_type() {
        let ctx = Context::default();
        let _int_type = ctx.new_type::<i32>();
    }

    #[test]
    fn create_field() {
        let ctx = Context::default();
        let int_type = ctx.new_type::<i32>();
        let _int_field = ctx.new_field(None, int_type, "x");
    }

    #[test]
    fn basic_function() {
        let context = Context::default();
        let int_ty = context.new_type::<i32>();
        let parameter = context.new_parameter(None, int_ty, "x");
        let fun = context.new_function(None, FunctionType::Exported, int_ty, &[parameter], "square", false);
        let block = fun.new_block("main_block");
        let parm = fun.get_param(0).to_rvalue();
        let square = parm * parm;
        block.end_with_return(None, square);

        let result = context.compile();
        unsafe {
            let func_ptr = result.get_function("square");
            assert!(!func_ptr.is_null());
            let func : extern "C" fn(i32) -> i32 = mem::transmute(func_ptr);
            assert_eq!(func(4), 16);
            assert_eq!(func(9), 81);
            assert_eq!(func(-2), 4);
        }
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

pub enum CType {
    Bool,
    Char,
    UChar,
    SChar,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    LongLong,
    ULongLong,
    SizeT,
}

impl CType {
    fn to_sys(&self) -> gccjit_sys::gcc_jit_types {
        use gccjit_sys::gcc_jit_types::*;
        use self::CType::*;

        match *self {
            Bool => GCC_JIT_TYPE_BOOL,
            Char => GCC_JIT_TYPE_CHAR,
            UChar => GCC_JIT_TYPE_UNSIGNED_CHAR,
            SChar => GCC_JIT_TYPE_SIGNED_CHAR,
            Short => GCC_JIT_TYPE_SHORT,
            UShort => GCC_JIT_TYPE_UNSIGNED_SHORT,
            Int => GCC_JIT_TYPE_INT,
            UInt => GCC_JIT_TYPE_UNSIGNED_INT,
            Long => GCC_JIT_TYPE_LONG,
            ULong => GCC_JIT_TYPE_UNSIGNED_LONG,
            LongLong => GCC_JIT_TYPE_LONG_LONG,
            ULongLong => GCC_JIT_TYPE_UNSIGNED_LONG_LONG,
            SizeT => GCC_JIT_TYPE_SIZE_T,
        }
    }
}
