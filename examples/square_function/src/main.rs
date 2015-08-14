extern crate gccjit;

use gccjit::Context;
use gccjit::FunctionType;
use gccjit::ToRValue;
use gccjit::OptimizationLevel;

use std::default::Default;
use std::mem;


fn main() {
    let context = Context::default();
    context.set_dump_code_on_compile(true);
    context.set_optimization_level(OptimizationLevel::Standard);
    let int_ty = context.new_type::<i32>();
    let parameter = context.new_parameter(None, int_ty, "x");
    let fun = context.new_function(None,
                                   FunctionType::Exported,
                                   int_ty,
                                   &[parameter],
                                   "square",
                                   false);
    let block = fun.new_block("main_block");
    let parm = fun.get_param(0).to_rvalue();
    let square = parm * parm;
    block.end_with_return(None, square);
    let result = context.compile();
    let func = result.get_function("square");
    let jit_compiled_fun : extern "C" fn(i32) -> i32 =
        if !func.is_null() {
            unsafe { mem::transmute(func) }
        } else {
            panic!("failed to retrieve function")
        };
    println!("the square of 2 is: {}", jit_compiled_fun(2));
    println!("the square of 10 is: {}", jit_compiled_fun(10));
    println!("the square of -2 is: {}", jit_compiled_fun(-2));
}
