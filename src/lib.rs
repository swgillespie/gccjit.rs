//! # gccjit.rs - Idiomatic Rust bindings to gccjit
//!
//! This library aims to provide idiomatic Rust bindings to gccjit,
//! the embeddable shared library that provides JIT compilation utilizing
//! GCC's backend. See https://gcc.gnu.org/wiki/JIT for more information
//! and for documentation of gccjit itself.
//!
//! Each one of the types provided in this crate corresponds to a pointer
//! type provided by the libgccjit C API. Type conversions are handled by
//! the ToRValue and ToLValue types, which represent values that can be
//! rvalues and values that can be lvalues, respectively.
//!
//! In addition, these types are all statically verified by the Rust compiler to
//! never outlive the Context object from which they came, a requirement
//! to using libgccjit correctly.

#![allow(raw_pointer_derive)]

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
pub use context::OutputKind;
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
