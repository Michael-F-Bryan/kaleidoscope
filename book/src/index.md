# Overview

This is yet another implementation of the [LLVM Kaleidoscope tutorial][tut],
written in Rust. 

In this tutorial we'll be using the [inkwell] crate to make using the LLVM 
bindings a little easier. Seeing as this guide is more focused on the compiler 
backend and code generation, we'll be using [lalrpop] to parse our source code.

This guide will be structured similarly to the original Kaleidoscope tutorial,
with a chapter devoted to each step in the process.


[tut]: https://llvm.org/docs/tutorial/index.html
[inkwell]: https://github.com/TheDan64/inkwell
[lalrpop]: https://github.com/lalrpop/lalrpop