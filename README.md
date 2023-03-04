## gccjit.rs - libgccjit bindings for Rust
This repository contains the basics for some high-level bindings
to libgccjit. The raw bindings themselves are in gccjit_sys, within this
repository.

## Building and running

This project requires you to have libgccjit.so already installed on your machine.
You might be able to obtain it from your distro's package manager. I'm on Ubuntu
and it didn't look like I could get it with apt-get, so I built it from source.
YMMV.

Once you've got libgccjit.so, a simple
```
cargo build
```
should suffice. There aren't many unit tests right now, but they can be run
using `cargo test`.

There are four examples right now living in the `examples/` directory:
* `square_function` - A square function, as a simple example for code generation,
* `factorial` - A factorial function, as a more complicated example involving recursion and conditional jumps. gcc removes all recursion at O3.
* `hello_world` - An example that invokes a function written in Rust from JIT-compiled code.
* `brainfuck` - An ahead-of-time compiler for brainfuck. The speed is very impressive given how easy it was to setup with libgccjit.

## Some benchmarks, my compiler vs a naive interpreter I wrote in Haskell:
```
sierpinski_triangle, haskell:
   real     0m0.052s
   user     0m0.026s
   sys      0m0.004s
sierpinski_triangle, libgccjit AOT:
   real     0m0.001s
   user     0m0.000s
   sys      0m0.001s
sierpinski_triangle, libgccjit JIT:
   real     0m0.140s
   user     0m0.106s
   sys      0m0.028s
   
mandlebrot_set, haskell:
   real     16m0.317s
   user     15m53.721s
   sys      0m6.291s
mandlebrot_set, libgccjit AOT:
   real     0m1.392s
   user     0m1.374s
   sys      0m0.004s
mandlebrot_set, libgccjit JIT
   real     0m5.498s
   user     0m5.446s
   sys      0m0.041s
```
The interpreter beats the JIT on the sierpinski triangle benchmark but the JIT blows the interpreter out of
the water (170x faster!) on the mandlebrot set benchmark

## Error handling
Right now, if you call the APIs incorrectly, gccjit will print angry messages
to standard error. It may be worth encoding this into the API. Right now there's
no penalty to the APIs returning null (Rust never dereferences the opaque pointers
and gccjit doesn't dereference them if they are null), but there's no indication
to the user other than the message on standard error that something went wrong.
