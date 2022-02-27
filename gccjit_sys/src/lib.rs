#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_int, FILE, c_void, c_long, c_double, size_t, ssize_t};

// opaque pointers
pub enum gcc_jit_context {}
pub enum gcc_jit_result {}
pub enum gcc_jit_object {}
pub enum gcc_jit_location {}
pub enum gcc_jit_type {}
pub enum gcc_jit_field {}
pub enum gcc_jit_struct {}
pub enum gcc_jit_function {}
pub enum gcc_jit_block {}
pub enum gcc_jit_rvalue {}
pub enum gcc_jit_lvalue {}
pub enum gcc_jit_param {}
pub enum gcc_jit_case {}
pub enum gcc_jit_function_type {}
pub enum gcc_jit_vector_type {}
pub enum gcc_jit_extended_asm {}

#[repr(C)]
pub enum gcc_jit_tls_model {
    GCC_JIT_TLS_MODEL_NONE,
    GCC_JIT_TLS_MODEL_GLOBAL_DYNAMIC,
    GCC_JIT_TLS_MODEL_LOCAL_DYNAMIC,
    GCC_JIT_TLS_MODEL_INITIAL_EXEC,
    GCC_JIT_TLS_MODEL_LOCAL_EXEC,
}

#[repr(C)]
pub enum gcc_jit_str_option {
    GCC_JIT_STR_OPTION_PROGNAME,
    GCC_JIT_NUM_STR_OPTIONS
}

#[repr(C)]
pub enum gcc_jit_int_option {
    GCC_JIT_INT_OPTION_OPTIMIZATION_LEVEL,
    GCC_JIT_NUM_INT_OPTIONS
}

#[repr(C)]
pub enum gcc_jit_bool_option {
    GCC_JIT_BOOL_OPTION_DEBUGINFO,
    GCC_JIT_BOOL_OPTION_DUMP_INITIAL_TREE,
    GCC_JIT_BOOL_OPTION_DUMP_INITIAL_GIMPLE,
    GCC_JIT_BOOL_OPTION_DUMP_GENERATED_CODE,
    GCC_JIT_BOOL_OPTION_DUMP_SUMMARY,
    GCC_JIT_BOOL_OPTION_DUMP_EVERYTHING,
    GCC_JIT_BOOL_OPTION_SELFCHECK_GC,
    GCC_JIT_BOOL_OPTION_KEEP_INTERMEDIATES,
    GCC_JIT_NUM_BOOL_OPTIONS
}

#[repr(C)]
pub enum gcc_jit_output_kind {
    GCC_JIT_OUTPUT_KIND_ASSEMBLER,
    GCC_JIT_OUTPUT_KIND_OBJECT_FILE,
    GCC_JIT_OUTPUT_KIND_DYNAMIC_LIBRARY,
    GCC_JIT_OUTPUT_KIND_EXECUTABLE
}

#[repr(C)]
pub enum gcc_jit_types {
    GCC_JIT_TYPE_VOID,
    /* "void *". */
    GCC_JIT_TYPE_VOID_PTR,
    /* C++'s bool type; also C99's "_Bool" type, aka "bool" if using
    stdbool.h. */
    GCC_JIT_TYPE_BOOL,
    /* Various integer types. */
    /* C's "char" (of some signedness) and the variants where the
    signedness is specified. */
    GCC_JIT_TYPE_CHAR,
    GCC_JIT_TYPE_SIGNED_CHAR,
    GCC_JIT_TYPE_UNSIGNED_CHAR,
    /* C's "short" and "unsigned short". */
    GCC_JIT_TYPE_SHORT, /* signed */
    GCC_JIT_TYPE_UNSIGNED_SHORT,
    /* C's "int" and "unsigned int". */
    GCC_JIT_TYPE_INT, /* signed */
    GCC_JIT_TYPE_UNSIGNED_INT,
    /* C's "long" and "unsigned long". */
    GCC_JIT_TYPE_LONG, /* signed */
    GCC_JIT_TYPE_UNSIGNED_LONG,
    /* C99's "long long" and "unsigned long long". */
    GCC_JIT_TYPE_LONG_LONG, /* signed */
    GCC_JIT_TYPE_UNSIGNED_LONG_LONG,

    /* Floating-point types */
    GCC_JIT_TYPE_FLOAT,
    GCC_JIT_TYPE_DOUBLE,
    GCC_JIT_TYPE_LONG_DOUBLE,
    /* C type: (const char *). */
    GCC_JIT_TYPE_CONST_CHAR_PTR,
    /* The C "size_t" type. */
    GCC_JIT_TYPE_SIZE_T,
    /* C type: (FILE *) */
    GCC_JIT_TYPE_FILE_PTR,
    /* Complex numbers. */
    GCC_JIT_TYPE_COMPLEX_FLOAT,
    GCC_JIT_TYPE_COMPLEX_DOUBLE,
    GCC_JIT_TYPE_COMPLEX_LONG_DOUBLE,

    GCC_JIT_TYPE_UINT8_T,
    GCC_JIT_TYPE_UINT16_T,
    GCC_JIT_TYPE_UINT32_T,
    GCC_JIT_TYPE_UINT64_T,
    GCC_JIT_TYPE_UINT128_T,
    GCC_JIT_TYPE_INT8_T,
    GCC_JIT_TYPE_INT16_T,
    GCC_JIT_TYPE_INT32_T,
    GCC_JIT_TYPE_INT64_T,
    GCC_JIT_TYPE_INT128_T,
}

#[repr(C)]
pub enum gcc_jit_function_kind {
    /* Function is defined by the client code and visible
       by name outside of the JIT. */
    GCC_JIT_FUNCTION_EXPORTED,
    /* Function is defined by the client code, but is invisible
       outside of the JIT. Analogous to a "static" function. */
    GCC_JIT_FUNCTION_INTERNAL,
    /* Function is not defined by the client code; we're merely
       referring to it. Analogous to using an "extern" function from a
       header file. */
    GCC_JIT_FUNCTION_IMPORTED,
    /* Function is only ever inlined into other functions, and is
       invisible outside of the JIT.
       Analogous to prefixing with "inline" and adding
       __attribute__((always_inline)).
       Inlining will only occur when the optimization level is
       above 0; when optimization is off, this is essentially the
       same as GCC_JIT_FUNCTION_INTERNAL. */
    GCC_JIT_FUNCTION_ALWAYS_INLINE
}

#[repr(C)]
pub enum gcc_jit_global_kind
{
    /* Global is defined by the client code and visible
       by name outside of this JIT context via gcc_jit_result_get_global. */
    GCC_JIT_GLOBAL_EXPORTED,
    /* Global is defined by the client code, but is invisible
       outside of this JIT context. Analogous to a "static" global. */
    GCC_JIT_GLOBAL_INTERNAL,
    /* Global is not defined by the client code; we're merely
       referring to it. Analogous to using an "extern" global from a
       header file. */
    GCC_JIT_GLOBAL_IMPORTED
}

#[repr(C)]
pub enum gcc_jit_unary_op
{
    /* Negate an arithmetic value; analogous to:
       -(EXPR)
       in C. */
    GCC_JIT_UNARY_OP_MINUS,
    /* Bitwise negation of an integer value (one's complement); analogous
       to:
       ~(EXPR)
       in C. */
    GCC_JIT_UNARY_OP_BITWISE_NEGATE,
    /* Logical negation of an arithmetic or pointer value; analogous to:
       !(EXPR)
       in C. */
    GCC_JIT_UNARY_OP_LOGICAL_NEGATE,
    /* Absolute value of an arithmetic expression; analogous to:
       abs (EXPR)
       in C. */
    GCC_JIT_UNARY_OP_ABS
}

#[repr(C)]
pub enum gcc_jit_binary_op
{
    /* Addition of arithmetic values; analogous to:
    (EXPR_A) + (EXPR_B)
    in C.
    For pointer addition, use gcc_jit_context_new_array_access. */
    GCC_JIT_BINARY_OP_PLUS,
    /* Subtraction of arithmetic values; analogous to:
    (EXPR_A) - (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_MINUS,
    /* Multiplication of a pair of arithmetic values; analogous to:
    (EXPR_A) * (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_MULT,
    /* Quotient of division of arithmetic values; analogous to:
    (EXPR_A) / (EXPR_B)
    in C.
    The result type affects the kind of division: if the result type is
    integer-based, then the result is truncated towards zero, whereas
    a floating-point result type indicates floating-point division. */
    GCC_JIT_BINARY_OP_DIVIDE,
    /* Remainder of division of arithmetic values; analogous to:
    (EXPR_A) % (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_MODULO,
    /* Bitwise AND; analogous to:
    (EXPR_A) & (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_BITWISE_AND,
    /* Bitwise exclusive OR; analogous to:
    (EXPR_A) ^ (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_BITWISE_XOR,
    /* Bitwise inclusive OR; analogous to:
    (EXPR_A) | (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_BITWISE_OR,
    /* Logical AND; analogous to:
    (EXPR_A) && (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_LOGICAL_AND,
    /* Logical OR; analogous to:
    (EXPR_A) || (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_LOGICAL_OR,
    /* Left shift; analogous to:
    (EXPR_A) << (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_LSHIFT,
    /* Right shift; analogous to:
    (EXPR_A) >> (EXPR_B)
    in C. */
    GCC_JIT_BINARY_OP_RSHIFT
}

#[repr(C)]
pub enum gcc_jit_comparison
{
    /* (EXPR_A) == (EXPR_B). */
    GCC_JIT_COMPARISON_EQ,
    /* (EXPR_A) != (EXPR_B). */
    GCC_JIT_COMPARISON_NE,
    /* (EXPR_A) < (EXPR_B). */
    GCC_JIT_COMPARISON_LT,
    /* (EXPR_A) <=(EXPR_B). */
    GCC_JIT_COMPARISON_LE,
    /* (EXPR_A) > (EXPR_B). */
    GCC_JIT_COMPARISON_GT,
    /* (EXPR_A) >= (EXPR_B). */
    GCC_JIT_COMPARISON_GE
}

#[repr(C)]
pub enum gcc_jit_inline_mode
{
    GCC_JIT_INLINE_MODE_DEFAULT,
    GCC_JIT_INLINE_MODE_ALWAYS_INLINE,
    GCC_JIT_INLINE_MODE_NO_INLINE,
    GCC_JIT_INLINE_MODE_INLINE,
}

#[link(name = "gccjit")]
extern {
    // context operations
    pub fn gcc_jit_context_acquire() -> *mut gcc_jit_context;
    pub fn gcc_jit_context_release(ctx: *mut gcc_jit_context);
    pub fn gcc_jit_context_set_str_option(ctx: *mut gcc_jit_context,
                                          option: gcc_jit_str_option,
                                          value: *const c_char);
    pub fn gcc_jit_context_set_int_option(ctx: *mut gcc_jit_context,
                                          option: gcc_jit_int_option,
                                          value: c_int);
    pub fn gcc_jit_context_set_bool_option(ctx: *mut gcc_jit_context,
                                           option: gcc_jit_bool_option,
                                           value: c_int);
    pub fn gcc_jit_context_compile(ctx: *mut gcc_jit_context) -> *mut gcc_jit_result;
    pub fn gcc_jit_context_compile_to_file(ctx: *mut gcc_jit_context,
                                           kind: gcc_jit_output_kind,
                                           path: *const c_char);
    pub fn gcc_jit_context_dump_to_file(ctx: *mut gcc_jit_context,
                                        path: *const c_char,
                                        update_locations: c_int);
    pub fn gcc_jit_context_set_logfile(ctx: *mut gcc_jit_context,
                                       file: *mut FILE,
                                       flags: c_int,
                                       verbosity: c_int);
    pub fn gcc_jit_context_get_first_error(ctx: *mut gcc_jit_context) -> *const c_char;
    pub fn gcc_jit_context_get_last_error(ctx: *mut gcc_jit_context) -> *const c_char;

    // result operations
    pub fn gcc_jit_result_get_code(result: *mut gcc_jit_result,
                                   funcname: *const c_char) -> *mut c_void;
    pub fn gcc_jit_result_get_global(result: *mut gcc_jit_result,
                                     globalname: *const c_char) ->  *mut c_void;
    pub fn gcc_jit_result_release(result: *mut gcc_jit_result);

    // object operations. gcc_jit_object is the root of a C++ inheritence
    // hierarchy, but this is a C API.
    pub fn gcc_jit_object_get_context(obj: *mut gcc_jit_object) -> *mut gcc_jit_context;
    pub fn gcc_jit_object_get_debug_string(obj: *mut gcc_jit_object) -> *const c_char;

    pub fn gcc_jit_context_new_location(ctx: *mut gcc_jit_context,
                                        filename: *const c_char,
                                        line: c_int,
                                        col: c_int) -> *mut gcc_jit_location;

    // upcast operator for location
    pub fn gcc_jit_location_as_object(loc: *mut gcc_jit_location) -> *mut gcc_jit_object;

    pub fn gcc_jit_type_as_object(ty: *mut gcc_jit_type) -> *mut gcc_jit_object;

    pub fn gcc_jit_context_get_type(ctx: *mut gcc_jit_context,
                                    ty: gcc_jit_types) -> *mut gcc_jit_type;
    pub fn gcc_jit_context_get_int_type(ctx: *mut gcc_jit_context,
                                        num_bytes: c_int,
                                        is_signed: c_int) -> *mut gcc_jit_type;
    pub fn gcc_jit_type_get_pointer(ty: *mut gcc_jit_type) -> *mut gcc_jit_type;
    pub fn gcc_jit_type_get_const(ty: *mut gcc_jit_type) -> *mut gcc_jit_type;
    pub fn gcc_jit_type_get_volatile(ty: *mut gcc_jit_type) -> *mut gcc_jit_type;
    pub fn gcc_jit_context_new_array_type(ctx: *mut gcc_jit_context,
                                          loc: *mut gcc_jit_location,
                                          ty: *mut gcc_jit_type,
                                          num_elements: c_int) -> *mut gcc_jit_type;
    // struct handling
    pub fn gcc_jit_context_new_field(ctx: *mut gcc_jit_context,
                                     loc: *mut gcc_jit_location,
                                     ty: *mut gcc_jit_type,
                                     name: *const c_char) -> *mut gcc_jit_field;
    pub fn gcc_jit_field_as_object(field: *mut gcc_jit_field) -> *mut gcc_jit_object;
    pub fn gcc_jit_context_new_struct_type(ctx: *mut gcc_jit_context,
                                           loc: *mut gcc_jit_location,
                                           name: *const c_char,
                                           num_fields: c_int,
                                           fields: *mut *mut gcc_jit_field) -> *mut gcc_jit_struct;

    pub fn gcc_jit_context_new_opaque_struct(ctx: *mut gcc_jit_context,
                                             loc: *mut gcc_jit_location,
                                             name: *const c_char) -> *mut gcc_jit_struct;

    pub fn gcc_jit_struct_as_type(struct_: *mut gcc_jit_struct) -> *mut gcc_jit_type;

    pub fn gcc_jit_struct_set_fields(struct_: *mut gcc_jit_struct,
                                     loc: *mut gcc_jit_location,
                                     num_fields: c_int,
                                     fields: *mut *mut gcc_jit_field);
    pub fn gcc_jit_context_new_union_type(ctx: *mut gcc_jit_context,
                                          loc: *mut gcc_jit_location,
                                          name: *const c_char,
                                          num_fields: c_int,
                                          fields: *mut *mut gcc_jit_field) -> *mut gcc_jit_type;

    pub fn gcc_jit_context_new_function_ptr_type(ctx: *mut gcc_jit_context,
                                                 loc: *mut gcc_jit_location,
                                                 ret_ty: *mut gcc_jit_type,
                                                 num_params: c_int,
                                                 param_tys: *mut *mut gcc_jit_type,
                                                 is_variadic: c_int) -> *mut gcc_jit_type;

    // constructing functions
    pub fn gcc_jit_context_new_param(ctx: *mut gcc_jit_context,
                                     loc: *mut gcc_jit_location,
                                     ty: *mut gcc_jit_type,
                                     name: *const c_char) -> *mut gcc_jit_param;
    pub fn gcc_jit_param_as_object(param: *mut gcc_jit_param) -> *mut gcc_jit_object;
    pub fn gcc_jit_param_as_lvalue(param: *mut gcc_jit_param) -> *mut gcc_jit_lvalue;
    pub fn gcc_jit_param_as_rvalue(param: *mut gcc_jit_param) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_function(ctx: *mut gcc_jit_context,
                                        loc: *mut gcc_jit_location,
                                        kind: gcc_jit_function_kind,
                                        return_ty: *mut gcc_jit_type,
                                        name: *const c_char,
                                        num_params: c_int,
                                        param: *mut *mut gcc_jit_param,
                                        is_variadic: c_int) -> *mut gcc_jit_function;
    pub fn gcc_jit_context_get_builtin_function(ctx: *mut gcc_jit_context,
                                                name: *const c_char) -> *mut gcc_jit_function;
    pub fn gcc_jit_function_as_object(func: *mut gcc_jit_function) -> *mut gcc_jit_object;

    pub fn gcc_jit_function_get_param(func: *mut gcc_jit_function,
                                      idx: c_int) -> *mut gcc_jit_param;
    pub fn gcc_jit_function_dump_to_dot(func: *mut gcc_jit_function,
                                        path: *const c_char);
    pub fn gcc_jit_function_new_block(func: *mut gcc_jit_function,
                                      name: *const c_char) -> *mut gcc_jit_block;
    pub fn gcc_jit_block_as_object(block: *mut gcc_jit_block) -> *mut gcc_jit_object;
    pub fn gcc_jit_block_get_function(block: *mut gcc_jit_block) -> *mut gcc_jit_function;

    pub fn gcc_jit_context_new_global(ctx: *mut gcc_jit_context,
                                      loc: *mut gcc_jit_location,
                                      kind: gcc_jit_global_kind,
                                      ty: *mut gcc_jit_type,
                                      name: *const c_char) -> *mut gcc_jit_lvalue;
    pub fn gcc_jit_lvalue_as_object(lvalue: *mut gcc_jit_lvalue) -> *mut gcc_jit_object;
    pub fn gcc_jit_lvalue_as_rvalue(lvalue: *mut gcc_jit_lvalue) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_rvalue_as_object(rvalue: *mut gcc_jit_rvalue) -> *mut gcc_jit_object;
    pub fn gcc_jit_rvalue_get_type(rvalue: *mut gcc_jit_rvalue) -> *mut gcc_jit_type;

    pub fn gcc_jit_context_new_rvalue_from_int(ctx: *mut gcc_jit_context,
                                               ty: *mut gcc_jit_type,
                                               value: c_int) ->  *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_rvalue_from_long(ctx: *mut gcc_jit_context,
                                                ty: *mut gcc_jit_type,
                                                value: c_long) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_zero(ctx: *mut gcc_jit_context,
                                ty: *mut gcc_jit_type) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_one(ctx: *mut gcc_jit_context,
                               ty: *mut gcc_jit_type) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_rvalue_from_double(ctx: *mut gcc_jit_context,
                                                  ty: *mut gcc_jit_type,
                                                  value: c_double) -> *mut gcc_jit_rvalue;

    pub fn gcc_jit_context_new_rvalue_from_ptr(ctx: *mut gcc_jit_context,
                                               ty: *mut gcc_jit_type,
                                               value: *mut c_void) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_null(ctx: *mut gcc_jit_context,
                                ty: *mut gcc_jit_type) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_string_literal(ctx: *mut gcc_jit_context,
                                              value: *const c_char) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_unary_op(ctx: *mut gcc_jit_context,
                                        loc: *mut gcc_jit_location,
                                        op: gcc_jit_unary_op,
                                        ty: *mut gcc_jit_type,
                                        rvalue: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_binary_op(ctx: *mut gcc_jit_context,
                                         loc: *mut gcc_jit_location,
                                         op: gcc_jit_binary_op,
                                         ty: *mut gcc_jit_type,
                                         left: *mut gcc_jit_rvalue,
                                         right: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_comparison(ctx: *mut gcc_jit_context,
                                          loc: *mut gcc_jit_location,
                                          op: gcc_jit_comparison,
                                          left: *mut gcc_jit_rvalue,
                                          right: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;

    pub fn gcc_jit_context_new_call(ctx: *mut gcc_jit_context,
                                    loc: *mut gcc_jit_location,
                                    func: *mut gcc_jit_function,
                                    num_args: c_int,
                                    args: *mut *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_call_through_ptr(ctx: *mut gcc_jit_context,
                                                loc: *mut gcc_jit_location,
                                                fun_ptr: *mut gcc_jit_rvalue,
                                                num_args: c_int,
                                                args: *mut *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;

    pub fn gcc_jit_context_new_cast(ctx: *mut gcc_jit_context,
                                    loc: *mut gcc_jit_location,
                                    rvalue: *mut gcc_jit_rvalue,
                                    ty: *mut gcc_jit_type) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_array_access(ctx: *mut gcc_jit_context,
                                            loc: *mut gcc_jit_location,
                                            ptr: *mut gcc_jit_rvalue,
                                            idx: *mut gcc_jit_rvalue) -> *mut gcc_jit_lvalue;

    pub fn gcc_jit_lvalue_access_field(struct_or_union: *mut gcc_jit_lvalue,
                                       loc: *mut gcc_jit_location,
                                       field: *mut gcc_jit_field) -> *mut gcc_jit_lvalue;

    pub fn gcc_jit_rvalue_access_field(struct_or_union: *mut gcc_jit_rvalue,
                                       loc: *mut gcc_jit_location,
                                       field: *mut gcc_jit_field) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_rvalue_dereference_field(ptr: *mut gcc_jit_rvalue,
                                            loc: *mut gcc_jit_location,
                                            field: *mut gcc_jit_field) -> *mut gcc_jit_lvalue;
    pub fn gcc_jit_rvalue_dereference(ptr: *mut gcc_jit_rvalue,
                                      loc: *mut gcc_jit_location) -> *mut gcc_jit_lvalue;
    pub fn gcc_jit_lvalue_get_address(lvalue: *mut gcc_jit_lvalue,
                                      loc: *mut gcc_jit_location) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_function_new_local(func: *mut gcc_jit_function,
                                      loc: *mut gcc_jit_location,
                                      ty: *mut gcc_jit_type,
                                      name: *const c_char) -> *mut gcc_jit_lvalue;
    pub fn gcc_jit_block_add_eval(block: *mut gcc_jit_block,
                                  loc: *mut gcc_jit_location,
                                  rvalue: *mut gcc_jit_rvalue);
    pub fn gcc_jit_block_add_assignment(block: *mut gcc_jit_block,
                                        loc: *mut gcc_jit_location,
                                        lvalue: *mut gcc_jit_lvalue,
                                        rvalue: *mut gcc_jit_rvalue);
    pub fn gcc_jit_block_add_assignment_op(block: *mut gcc_jit_block,
                                           loc: *mut gcc_jit_location,
                                           lvalue: *mut gcc_jit_lvalue,
                                           op: gcc_jit_binary_op,
                                           rvalue: *mut gcc_jit_rvalue);
    pub fn gcc_jit_block_add_comment(block: *mut gcc_jit_block,
                                     loc: *mut gcc_jit_location,
                                     msg: *const c_char);
    pub fn gcc_jit_block_end_with_conditional(block: *mut gcc_jit_block,
                                              loc: *mut gcc_jit_location,
                                              cond: *mut gcc_jit_rvalue,
                                              on_true: *mut gcc_jit_block,
                                              on_false: *mut gcc_jit_block);
    pub fn gcc_jit_block_end_with_jump(block: *mut gcc_jit_block,
                                       loc: *mut gcc_jit_location,
                                       target: *mut gcc_jit_block);
    pub fn gcc_jit_block_end_with_return(block: *mut gcc_jit_block,
                                         loc: *mut gcc_jit_location,
                                         ret: *mut gcc_jit_rvalue);
    pub fn gcc_jit_block_end_with_void_return(block: *mut gcc_jit_block,
                                              loc: *mut gcc_jit_location);
    pub fn gcc_jit_context_new_child_context(parent: *mut gcc_jit_context) -> *mut gcc_jit_context;
    pub fn gcc_jit_context_dump_reproducer_to_file(parent: *mut gcc_jit_context,
                                                   path: *const c_char);

    pub fn gcc_jit_context_new_case(ctxt: *mut gcc_jit_context, min_value: *mut gcc_jit_rvalue, max_value: *mut gcc_jit_rvalue, dest_block: *mut gcc_jit_block) -> *mut gcc_jit_case;
    pub fn gcc_jit_block_end_with_switch(block: *mut gcc_jit_block, loc: *mut gcc_jit_location, expr: *mut gcc_jit_rvalue, default_block: *mut gcc_jit_block, num_cases: c_int, cases: *mut *mut gcc_jit_case);
    pub fn gcc_jit_case_as_object(case_: *mut gcc_jit_case) -> *mut gcc_jit_object;

    pub fn gcc_jit_function_get_address(fun: *mut gcc_jit_function, loc: *mut gcc_jit_location) ->  *mut gcc_jit_rvalue;

    pub fn gcc_jit_type_get_vector(typ: *mut gcc_jit_type, num_units:  size_t) -> *mut gcc_jit_type;
    pub fn gcc_jit_context_new_rvalue_from_vector(ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location, vec_type: *mut gcc_jit_type, num_elements: size_t, elements: *mut *mut gcc_jit_rvalue) ->  *mut gcc_jit_rvalue;

    pub fn gcc_jit_context_add_command_line_option(ctxt: *mut gcc_jit_context, optname: *const c_char);
    pub fn gcc_jit_context_add_driver_option(ctxt: *mut gcc_jit_context, optname: *const c_char);

    pub fn gcc_jit_type_get_aligned(typ: *mut gcc_jit_type, alignment_in_bytes: size_t) ->  *mut gcc_jit_type;

    pub fn gcc_jit_function_get_return_type(func: *mut gcc_jit_function) -> *mut gcc_jit_type;
    pub fn gcc_jit_function_get_param_count(func: *mut gcc_jit_function) -> ssize_t;

    pub fn gcc_jit_type_dyncast_array(typ: *mut gcc_jit_type) -> *mut gcc_jit_type;
    pub fn gcc_jit_type_is_bool(typ: *mut gcc_jit_type) -> c_int;
    pub fn gcc_jit_type_is_integral(typ: *mut gcc_jit_type) -> c_int;
    pub fn gcc_jit_type_unqualified(typ: *mut gcc_jit_type) -> *mut gcc_jit_type;
    pub fn gcc_jit_type_is_pointer(typ: *mut gcc_jit_type) -> *mut gcc_jit_type;
    pub fn gcc_jit_type_dyncast_function_ptr_type(typ: *mut gcc_jit_type) -> *mut gcc_jit_function_type;
    pub fn gcc_jit_function_type_get_return_type(function_type: *mut gcc_jit_function_type) -> *mut gcc_jit_type;
    pub fn gcc_jit_function_type_get_param_count(function_type: *mut gcc_jit_function_type) -> ssize_t;
    pub fn gcc_jit_type_dyncast_vector(typ: *mut gcc_jit_type) -> *mut gcc_jit_vector_type;
    pub fn gcc_jit_function_type_get_param_type(function_type: *mut gcc_jit_function_type, index: c_int) -> *mut gcc_jit_type;
    pub fn gcc_jit_vector_type_get_num_units(vector_type: *mut gcc_jit_vector_type) -> ssize_t;
    pub fn gcc_jit_vector_type_get_element_type(vector_type: *mut gcc_jit_vector_type) -> *mut gcc_jit_type;
    pub fn gcc_jit_struct_get_field(struct_type: *mut gcc_jit_struct, index: c_int) -> *mut gcc_jit_field;
    pub fn gcc_jit_type_is_struct(typ: *mut gcc_jit_type) -> *mut gcc_jit_struct;
    pub fn gcc_jit_struct_get_field_count(struct_type: *mut gcc_jit_struct) -> ssize_t;

    pub fn gcc_jit_global_set_initializer(global: *mut gcc_jit_lvalue, blob: *const c_void, num_bytes: size_t) -> *mut gcc_jit_lvalue;

    pub fn gcc_jit_block_end_with_extended_asm_goto(block: *mut gcc_jit_block, loc: *mut gcc_jit_location, asm_template: *const c_char, num_goto_blocks: c_int, goto_blocks: *mut *mut gcc_jit_block, fallthrough_block: *mut gcc_jit_block) -> *mut gcc_jit_extended_asm;
    pub fn gcc_jit_extended_asm_as_object(ext_asm: *mut gcc_jit_extended_asm) -> *mut gcc_jit_object;
    pub fn gcc_jit_extended_asm_set_volatile_flag(ext_asm: *mut gcc_jit_extended_asm, flag: c_int);
    pub fn gcc_jit_extended_asm_set_inline_flag(ext_asm: *mut gcc_jit_extended_asm, flag: c_int);
    pub fn gcc_jit_extended_asm_add_output_operand(ext_asm: *mut gcc_jit_extended_asm, asm_symbolic_name: *const c_char, constraint: *const c_char, dest: *mut gcc_jit_lvalue);
    pub fn gcc_jit_extended_asm_add_input_operand(ext_asm: *mut gcc_jit_extended_asm, asm_symbolic_name: *const c_char, constraint: *const c_char, src: *mut gcc_jit_rvalue);
    pub fn gcc_jit_extended_asm_add_clobber(ext_asm: *mut gcc_jit_extended_asm, victim: *const c_char);
    pub fn gcc_jit_context_add_top_level_asm(ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location, asm_stmts: *const c_char);
    pub fn gcc_jit_block_add_extended_asm(block: *mut gcc_jit_block, loc: *mut gcc_jit_location, asm_template: *const c_char) -> *mut gcc_jit_extended_asm;

    pub fn gcc_jit_lvalue_set_tls_model(lvalue: *mut gcc_jit_lvalue, model: gcc_jit_tls_model);
    pub fn gcc_jit_lvalue_set_link_section(lvalue: *mut gcc_jit_lvalue, name: *const c_char);

    /*pub fn gcc_jit_function_set_personality_function(func: *mut gcc_jit_function, personality_func: *mut gcc_jit_function);
    pub fn gcc_jit_block_add_try_finally(block: *mut gcc_jit_block, loc: *mut gcc_jit_location, try_block: *mut gcc_jit_block, finally_block: *mut gcc_jit_block);*/

    pub fn gcc_jit_context_new_bitcast(ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location, rvalue: *mut gcc_jit_rvalue, type_: *mut gcc_jit_type) -> *mut gcc_jit_rvalue;

    //pub fn gcc_jit_function_set_inline_mode(func: *mut gcc_jit_function, inline_mode: gcc_jit_inline_mode);

    pub fn gcc_jit_lvalue_set_register_name(lvalue: *mut gcc_jit_lvalue, reg_name: *const c_char);

    pub fn gcc_jit_context_new_struct_constructor(ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location, typ: *mut gcc_jit_type, arr_length: c_int, fields: *mut *mut gcc_jit_field, values: *mut *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_union_constructor(ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location, typ: *mut gcc_jit_type, field: *mut gcc_jit_field, value: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_array_constructor(ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location, typ: *mut gcc_jit_type, arr_length: c_int, values: *mut *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_global_set_initializer_rvalue(global: *mut gcc_jit_lvalue, init_value: *mut gcc_jit_rvalue) -> *mut gcc_jit_lvalue;

    pub fn gcc_jit_type_get_size(typ: *mut gcc_jit_type) -> ssize_t;
    pub fn gcc_jit_compatible_types(ltype: *mut gcc_jit_type, rtype: *mut gcc_jit_type) -> bool;

    pub fn gcc_jit_context_set_bool_print_errors_to_stderr(ctxt: *mut gcc_jit_context, enabled: c_int);

    pub fn gcc_jit_lvalue_set_alignment(lvalue: *mut gcc_jit_lvalue, alignment: c_int);
    pub fn gcc_jit_lvalue_get_alignment(lvalue: *mut gcc_jit_lvalue) -> c_int;


    pub fn gcc_jit_context_get_target_builtin_function(ctxt: *mut gcc_jit_context, name: *const c_char) -> *mut gcc_jit_function;

    pub fn gcc_jit_context_new_rvalue_vector_perm(ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location, elements1: *mut gcc_jit_rvalue, elements2: *mut gcc_jit_rvalue, mask: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
    pub fn gcc_jit_context_new_vector_constructor(ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location, typ: *mut gcc_jit_type, num_values: size_t, values: *mut *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
}
