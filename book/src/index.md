# Overview

This is yet another implementation of the [LLVM Kaleidoscope tutorial][tut],
written in Rust. 

In this tutorial we'll be using the [inkwell] crate to make using the LLVM 
bindings a little easier. Seeing as this guide is more focused on the compiler 
backend and code generation, we'll be using [lalrpop] to parse our source code.

This guide will be structured similarly to the original Kaleidoscope tutorial,
with a chapter devoted to each step in the process.

## The Kaleidoscope Language

(from [the original tutorial][syntax])

Kaleidoscope is a procedural language that allows you to define functions,
use conditionals, math, etc. Over the course of the tutorial, we’ll extend
Kaleidoscope to support the if/then/else construct, a for loop, user defined
operators, JIT compilation with a simple command line interface, etc.

Because we want to keep things simple, the only datatype in Kaleidoscope is a
64-bit floating point type (aka ‘double’ in C parlance). As such, all values
are implicitly double precision and the language doesn’t require type
declarations. This gives the language a very nice and simple syntax.

```
# Compute the x'th fibonacci number.
def fib(x)
  if x < 3 then
    1
  else
    fib(x-1)+fib(x-2)

# This expression will compute the 40th number.
fib(40)
```

We also allow Kaleidoscope to call into standard library functions. This
means that you can use the ‘extern’ keyword to define a function before you
use it (this is also useful for mutually recursive functions).

```
extern sin(arg);
extern cos(arg);
extern atan2(arg1 arg2);

atan2(sin(.4), cos(42))
```

[tut]: https://llvm.org/docs/tutorial/index.html
[inkwell]: https://github.com/TheDan64/inkwell
[lalrpop]: https://github.com/lalrpop/lalrpop
[syntax]: https://llvm.org/docs/tutorial/LangImpl01.html#id2