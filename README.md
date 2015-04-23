## gccjit.rs - libgccjit bindings for Rust
This repository contains the basics for some high-level bindings
to libgccjit. The raw bindings themselves are in gccjit_sys, within this
repository.

This library is in its very early stages and very rough around the edges.
The API is not as good as it could be and there's more work to be done.
However, it does work!

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

There are two examples right now living in the `examples/` directory. One is a
simple function that squares its integer argument, and the other is a function
that calls a Rust function pointer to print "hello, world!".

Right now, if you call the APIs incorrectly, gccjit will print angry messages
to standard error. It may be worth encoding this into the API. Right now there's
no penalty to the APIs returning null (Rust never dereferences the opaque pointers
and gccjit doesn't dereference them if they are null), but there's no indication
to the user other than the message on standard error that something went wrong.
