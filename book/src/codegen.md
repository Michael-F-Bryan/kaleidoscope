# Code Generation to LLVM IR

Welcome to chapter 3, code generation. This is where things start getting 
interesting and we'll begin actually using LLVM!

## How LLVM Works

First up you'll probably want to know how LLVM is structured. At the very top
level is the `Context`, an object which contains the various global variables,
interned/memoized objects (see [string interning]), and other things LLVM needs
for the lifetime of your compiler. You use this to create the other various
objects in LLVM and constrain their lifetimes.

A `Module` can be thought of as a single object file (C/C++) or crate (Rust). It
contains all the definitions for your functions and types and acts as a discrete
chunk. You can create one via the `Context::create_module()` method.

You can add functions to a `Module` via the `Module::add_function()` method. 
From there you'll populate the function's body using a `Builder`. This super 
handy class has loads of functions for creating and manipulating LLVM IR like
`build_call()`, `build_float_to_signed_int()`, `build_conditional_branch()`, and
more.

A function is composed of multiple `BasicBlock`s. This is roughly a
continuous sequence of instructions without any branching or jumps, with each
basic block in a function having a unique name (think of labels in assembly).
To add instructions to a basic block, you'll position the `Builder` at the end
of the block then call its various `build_*()` methods.

One thing to keep in mind is that all instructions, basic blocks, and functions
are owned by their parent `Module`. Presumably LLVM uses some sort of arena or
object pool. Although I haven't encountered any memory issues with the `inkwell`
bindings so far, as a Rust programmer it's still useful to know who owns what
memory and for how long.

> **Note:** Although most of the above methods and type names are from the  
> [inkwell] crate (the LLVM binding library we'll use), although there's a 
> fairly 1:1 relationship with the original C++ library.

Obviously there's a lot more to LLVM than the summary above, but this should be
enough to get us through the initial codegen step.

## Writing The Compiler

> **TODO:** Write this


[string interning]: https://en.wikipedia.org/wiki/String_interning
[inkwell]: https://github.com/TheDan64/inkwell