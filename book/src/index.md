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

## Expectations of the Reader

This guide makes a couple assumptions so as to avoid re-teaching the entire Rust
programming language. As such, you'll want to be familiar with things like 
`cargo`, generics, structs, enums, and traits.

We try to avoid doing tricky things with lifetimes, mainly because they can add
unnecessary complexity to code and bending over backwards to skip a copy is
often premature optimization. That said, it's logical for the `Token` type in
our first chapter (lexical analysis) to borrow from the source code instead
of creating loads of intermediate strings. Tokens are quite small, short
lived objects so it can be beneficial for them to borrow from the original 
source code.

Think of this document as a general guide to creating a compiler with LLVM
and not a step-by-step recipe where everything is spelled out in excruciating
detail. So we'll occasionally elide bits and pieces for brevity (simple
constructors, `From` impls, etc), leaving them as an exercise for the reader.

## Contributing

If there are any concepts or steps which you feel could do with extra
explanation or links to background material, feel free to create an issue on the
repository's [issue tracker] or send in a PR. 

Any help with proofreading or chapter writing is also most welcome.

[tut]: https://llvm.org/docs/tutorial/index.html
[inkwell]: https://github.com/TheDan64/inkwell
[lalrpop]: https://github.com/lalrpop/lalrpop
[syntax]: https://llvm.org/docs/tutorial/LangImpl01.html#id2
[issue tracker]: https://github.com/Michael-F-Bryan/kaleidoscope/issues