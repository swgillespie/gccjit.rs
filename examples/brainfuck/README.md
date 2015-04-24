## An optimizing Brainfuck compiler, powered by libgccjit!

This example shows how to use libgccjit as an ahead-of-time compiler backend.
Generating code in a JIT scenario vs an AOT scenario is exactly the same - the only
difference is a call to `compile_to_file` instead of `compile`.

This example can be built with `cargo build` and run by piping in a brainfuck program
to standard in. `hello.bf` contains a brainfuck Hello World program. It can be invoked using

```
cargo build
cargo run < hello.bf
```

The compiler generates an executable named `bf_out`.
