extern crate gccjit;

use gccjit::Context;
use gccjit::FunctionType;

use std::default::Default;


fn main() {
    let context = Context::default();
    let void_ty = context.new_type::<()>();
    let fun = context.new_function(None,
                                   FunctionType::Exported,
                                   void_ty,
                                   &[],
                                   "hello",
                                   false);
    let block = fun.new_block("main_block");
    let function_ptr = context.new_function_pointer_type(None,
                                                         void_ty,
                                                         &[],
                                                         false);
    let ptr = unsafe {
        context.new_rvalue_from_ptr(function_ptr, say_hello as *mut u8)
    };
    let call = context.new_call_through_ptr(None, ptr, &[]);
    block.add_eval(None, call);
    block.end_with_void_return(None);
    let result = context.compile();
    let hello = result.get_function::<(),()>("hello").unwrap();
    // this is an ugly quirk of how get_function works right now.
    // if Rust had variadic templates/traits, this wouldn't be necessary.
    // perhaps it would be better to do something other than this.
    hello(());
}

extern "C" fn say_hello() {
    println!("hello, world!");
}
