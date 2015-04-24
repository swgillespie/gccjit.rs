#![allow(dead_code, unused_variables, raw_pointer_derive)]
#![feature(optin_builtin_traits)]

extern crate gccjit_sys;

mod types;
mod context;
mod object;
mod location;
mod field;
mod structs;
mod lvalue;
mod rvalue;
mod parameter;
mod function;
mod block;

pub use context::Context;
pub use context::OptimizationLevel;
pub use context::CompileResult;
pub use location::Location;
pub use object::Object;
pub use object::ToObject;
pub use types::Type;
pub use types::Typeable;
pub use field::Field;
pub use structs::Struct;
pub use lvalue::{LValue, ToLValue};
pub use rvalue::{RValue, ToRValue};
pub use parameter::Parameter;
pub use function::{Function, FunctionType};
pub use block::{Block, BinaryOp, UnaryOp, ComparisonOp};
