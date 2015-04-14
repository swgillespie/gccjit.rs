## gccjit.rs - libgccjit bindings for Rust
This repository contains the basics for some high-level bindings
to libgccjit. The raw bindings themselves are in gccjit_sys, within this
repository.

Right now this doesn't work. In the example directory, I've got what
will become a brainfuck interpreter in the future, but right now is
attempting to JIT compile a square function. This unfortunately seems to stack
overflow on my machine and I'm not sure why.

At any rate, hopefully this becomes functional soon!

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

In the `examples` directory, there's a project called `brainfuck` that was
going to become a brainfuck interpreter. Right now it's a smoke test for this library
that currently crashes. You can build it with `cargo build` or build/run with
`cargo run` - although Cargo does not like it when the program it runs segfaults.

The docs can be built with `cargo doc` - I started out being good with documentation,
but I slipped a little as I continued. I'll get better, I promise!
