extern crate gccjit;

use gccjit::Context;
use gccjit::FunctionType;
use gccjit::BinaryOp;
use gccjit::ToRValue;
use gccjit::OptimizationLevel;

use std::default::Default;
use std::mem;


fn main() {
    let context = Context::default();
    context.set_dump_code_on_compile(true);
    context.set_optimization_level(OptimizationLevel::Aggressive);
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
    let binop = context.new_binary_op(None,
                                      BinaryOp::Mult,
                                      int_ty,
                                      parm,
                                      parm);
    block.end_with_return(None, binop);
    let result = context.compile();
    let func = result.get_function("square");
    let jit_compiled_fun : extern "C" fn(i32) -> i32 = match func {
        Some(x) => unsafe { mem::transmute(x) },
        None => panic!("failed to retrieve function")
    };
    println!("the square of 2 is: {}", jit_compiled_fun(2));
    println!("the square of 10 is: {}", jit_compiled_fun(10));
    println!("the square of -2 is: {}", jit_compiled_fun(-2));
}
