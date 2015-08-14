extern crate gccjit;

use gccjit::Context;
use gccjit::FunctionType;
use gccjit::ToRValue;
use gccjit::OptimizationLevel;
use gccjit::ComparisonOp;

use std::default::Default;
use std::mem;


fn main() {
    let context = Context::default();
    context.set_dump_code_on_compile(true);
    context.set_optimization_level(OptimizationLevel::Standard);
    let int_ty = context.new_type::<i32>();
    let parameter = context.new_parameter(None, int_ty, "n");
    let factorial = context.new_function(None,
                                         FunctionType::Exported,
                                         int_ty,
                                         &[parameter],
                                         "factorial",
                                         false);
    let block = factorial.new_block("entry");
    let parm = factorial.get_param(0).to_rvalue();
    // if (n == 0) goto recurse_block else goto ret_block
    let cond = context.new_comparison(None,
                                      ComparisonOp::Equals,
                                      parm,
                                      context.new_rvalue_zero(int_ty));
    let false_branch = factorial.new_block("recurse_block");
    let true_branch = factorial.new_block("ret_block");
    block.end_with_conditional(None, cond, true_branch, false_branch);
    // ret_block: return 1
    true_branch.end_with_return(None, context.new_rvalue_one(int_ty));
    // recurse_block: return n * fact(n-1)
    let n_minus_one = parm - context.new_rvalue_one(int_ty);
    let call = context.new_call(None, factorial, &[n_minus_one]);
    let mul = parm * call;
    false_branch.end_with_return(None, mul);
    let result = context.compile();
    factorial.dump_to_dot("factorial.dot");
    let fact_ptr = result.get_function("factorial");
    let fact_fn : extern "C" fn(i32) -> i32 =
        if !fact_ptr.is_null() {
            unsafe { mem::transmute(fact_ptr) }
        } else {
            panic!("failed to find factorial function")
        };
    println!("fact(5) = {}", fact_fn(5));
    println!("fact(10) = {}", fact_fn(10));
}
