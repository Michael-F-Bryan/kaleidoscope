# Parser and AST

The next step in our quest to create a Kaleidoscope compiler is parse a stream
of tokens into a more computer-friendly form, an [*Abstract Syntax Tree*][ast].
An abstract syntax tree (AST) is a tree data structure with node types for each
construct in the Kaleidoscope language.

## The Language Grammar

Before we can start writing a parser we'll need to formalize exactly what form
we expect Kaleidoscope to take. This is often referred to as the language's
[grammar].

At the top level a file is composed of zero or more *items*, where an item can
either be a function definition (`def foo() ...`) or an `extern` declaration.
Or stated more concisely:

```text
file := item*
item := function_def
      | extern
```

An `extern` declaration is just the word "*extern*" followed by a function 
signature.

```text
extern       := "extern" function_sig
function_sig := IDENT "(" IDENT* ")"
```

A function definition (our `function_def` rule) is just a function signature 
followed by an expression.

```text
function_def := "def" function_sig expr
```

For now an expression can be either a literal, a variable name, or a function
call. Like most other languages, the arguments to a function call can themselves
be expressions. We use the symbol "*ε*" (epsilon) to indicate the case when
nothing is a valid pattern (i.e. there are no arguments).

```text
expr          := LITERAL
               | IDENT
               | function_call
function_call := IDENT "(" args ")"
args          := expr
               | expr ("," expr)*
               | ε
```

The notation we've been using so far is similar to the more formal 
[Backus-Naur Form][bnf] used to describe programming language syntax. Upper-case
names (e.g. `IDENT`) are used to refer to literal tokens (sometimes called 
*terminals*), with lower-case names signifying a particular rule/syntactic 
construct in the language (*non-terminal*) that may be itself composed of other
rules or tokens.

## Translating To Lalrpop

The reason we took the time to write down Kaleidoscope's grammar using formal
notation is twofold. Having a formal spec for the language lets you definitively
say whether a particular program is syntactically correct and also helps direct
the development of the parser itself.

The second reason is `lalrpop` will generate a parser based upon a *grammar file*
which lets the parser generator know what patterns are valid and what to do when
a valid pattern is encountered.

Because `lalrpop` will generate the Rust code for parsing from our grammar file
we need to execute a build step before compiling the main crate. This is done
by writing a [build.rs] file that invokes `lalrpop`.

```rust
extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
```

By simply invoking the `process_root()` function `lalrpop` will traverse our
entire `src/` directory looking for any `*.lalrpop` files, and generate the 
corresponding Rust parser code.

We also need to add `lalrpop` as a [*build dependency*][build].

```console
$ cargo add --build lalrpop
$ cat Cargo.toml
...
[build-dependencies]
lalrpop = "0.15.2"
```

[ast]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[grammar]: https://en.wikibooks.org/wiki/Introduction_to_Programming_Languages/Grammars
[bnf]: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form
[build.rs]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[build]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#build-dependencies