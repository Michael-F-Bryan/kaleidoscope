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

## The Abstract Syntax Tree

Now we are more familiar with the language we can start writing out the data 
types which will make up Kaleidoscope's AST. The code for this will be placed
in the `src/ast.rs` module.

This step is actually super easy, it's just a case of translating our grammar
from the pseudo-BNF above into types. Whenever you see alternation ("or") you
use an `enum`, and each line corresponds to a `struct`.

First up we have the definitions for a `File` and `Item`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Extern(FunctionDecl),
    Function(Function),
}
```

In turn, these are made up from either a `Function` or a `FunctionDecl` (forward
declaration).

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub args: Vec<Ident>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub decl: FunctionDecl,
    pub body: Expr,
    pub span: Span,
}
```

And finally, we have the types which make up our *expressions*.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub value: f64,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub ident: Ident,
    pub args: Vec<Ident>,
    pub span: Span,
}
```

There's nothing worse than a program which says there's an error in your program
but never bothers to tell you where, so we'll need to keep track of the location
of each AST node in the original source text. This can be done by creating a
custom `Span` struct and keeping track of the byte index, but we can avoid 
reinventing the wheel with the [codespan] crate. 

Codespan gives us nice things like a `FileMap` and `CodeMap` abstraction, as 
well as a bunch of functionality for [generating diagnostics][reporting].

To start using the `codespan` crate, add it as a dependency then make sure to
import it in `ast.rs`.

```rust
use codespan::{self, ByteIndex};

pub type Span = codespan::Span<ByteIndex>;
```

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
by writing a [build script] that invokes `lalrpop`.

```rust
// build.rs

extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
```

By simply invoking the `process_root()` function `lalrpop` will traverse our
entire `src/` directory looking for any `*.lalrpop` files, and generate the 
corresponding Rust parser code.

We also need to add `lalrpop` as a [*build dependency*][build-dep].

```console
$ cargo add --build lalrpop
$ cat Cargo.toml
...
[build-dependencies]
lalrpop = "0.15.2"
```

Now the build system is set up we can finally get to work on our grammar file.

> *Note:** If you aren't already familiar with `lalrpop` I highly suggest you
> read through [the lalrpop guide][guide]. The guide explains how lalrpop works
> in more detail and walks you through writing grammar files.

Lalrpop uses a DSL to tell the parser generator how to parse our Kaleidoscope
code and how to generate an AST from the parse results.

A grammar file is broken up into roughly two parts. At the start you can add
arbitrary Rust code which will be copied to the top of the generated document,
this is typically used to import the necessary types and functions. A special
`grammar;` line separates the Rust preable from the rest of the grammar file,
this is where the various grammar rules will go.

Our top-most rule is the `File`. This will generate a `FileParser` which should
(hopefully) result in an `ast::File` struct.

```rust
// src/grammar.lalrpop

grammar<'input>;

pub File: File = {
    <l:@L> <items:Item*> <r:@R> => File::new(items, Span::new(l, r)),
};
```

The syntax is kinda similar to a Rust `match` statement or `macro_rules` 
definition. On the left is a bunch of patterns and variable bindings, then 
there's a `=>` followed by the Rust code to be run. 

The above rule says we want to bind the match's start position (`@L`) to the `l`
variable, followed by zero or more `Item`s (bound to `items`), then we'll 
bind the end position (`@R`) to `r`. When the generated parser encounters 
something which matches that pattern, it'll execute the `File::new()` function
and pass in the appropriate information.

This notation will probably be a little confusing if you've never used `lalrpop`
before. If so, you should probably pause here and read their [guide].

> **Note:** Don't forget to add the trailing `;` at the end of a rule's closing
> curly bracket. This has tripped me up more times than I'd like to admit.

We want to start seeing results fairly quickly, so I'm going to write the code
for parsing just an `extern` statement first, then come back and fill out the 
rest of the grammar afterwards.

These are all the rules we'll need to parse a simple `extern foo()` line:

```rust
pub Item: Item = {
    <Extern> => Item::Extern(<>),
};

Extern: FunctionDecl = {
    <l:@L> "extern" <name:Ident> "(" ")" <r:@R> => FunctionDecl::new(name, Vec::new(), Span::new(l, r)),
};

Ident: Ident = {
    <l:@L> <id:"ident"> <r:@R> => Ident::new(id.as_ident().unwrap(), Span::new(l, r)),
};
```

And because we're using a custom lexer we need to tell `lalrpop` how to use it.
This is done in a special `extern` block which I usually put at the bottom of
the file.

```rust
extern {
    type Location = ByteIndex;
    type Error = ParseError<ByteIndex, Token<'input>, Void>;

    enum Token<'input> {
        "ident" => Token::Identifier(_),
        "extern" => Token::Extern,
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,
    }
}
```

> **Exercise for the Reader:** In the above snippet you might notice we've added
> two extra variants to the `Token` enum, `Token::OpenParen` and 
> `Token::CloseParen`. Try tweaking our `construct_lexer()` function until the
> following tests pass.
>
> ```rust
> // src/tokens.rs
> lexer_test!(recognise_open_paren, "(" => Token::OpenParen);
> lexer_test!(recognise_close_paren, ")" => Token::CloseParen);
> ```
>
> Hint: you may want to look at how we registered the `def` and `extern` 
> keywords.

The above tells `lalrpop` what type we're using for the `Location` 
(`codespan::ByteIndex`) as well as how to report errors (the `type Error` line).
We also need to tell it how to interpret the `Token`s returned by the lexer, so
there's an `enum Token<'input>` line as well.

This should be enough to make the following test pass!

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use grammar::ItemParser;
    use tokens;

    #[test]
    fn parse_an_extern() {
        let src = "extern foo()";
        let lexer = tokens::construct_lexer(src);

        let should_be = Item::Extern(FunctionDecl {
            ident: Ident {
                name: "foo".to_string(), 
                span: Span::new(ByteIndex(7), ByteIndex(10)),
            },
            args: Vec::new(),
            span: Span::new(ByteIndex(0), ByteIndex(src.len() as u32)),
        });

        let got = ItemParser::new().parse(lexer).unwrap();

        assert_eq!(got, should_be);
    }
}
```

If you've got this far then congratulations. You can officially tokenize and 
parse some Kaleidoscope code! 

From here on the process of adding more complex language constructs (e.g. 
function definitions, expressions) to the parser is actually quite mechanical.
You:

1. Add a test which takes a string of text and write out the type you expect it
   to parse into.
2. Throw the text at the corresponding `XXXParser` type
3. Add a rule matching the language construct to your grammar file
4. Repeat steps 2 and 3 until the test passes

## Parsing The Rest

Now we've got a better understanding of how to use lalrpop to parse code, lets
implement the rest of our grammar rules. You'll quickly realise that the code
written in `grammar.lalrpop` very closely follows the rules we wrote down at the
beginning of the chapter.

We need to define the following constructs:

- Functions (i.e. function signature + body)
- Expressions (i.e. ident or literal or function call)
    - Function calls

We also need to add support for commas to our lexer, but hopefully this should
be fairly routine for now. Just make sure the following test passes:

```rust
lexer_test!(recognise_comma, "," => Token::Comma);
```

First off, as a sanity check lets add a couple tests for parsing identifiers and
literals. This will make sure the `ExprParser` we're about to define (by adding
an `Expr` rule to `grammar.lalrpop`) works as expected.

```rust
#[test]
fn parse_a_literal() {
    let src = "123";
    let lexer = tokens::construct_lexer(src);

    let should_be = Literal::new(123.0, Span::new(ByteIndex(0), ByteIndex(3)));
    let should_be = Expr::Literal(should_be);

    let got = ExprParser::new().parse(lexer).unwrap();
    assert_eq!(got, should_be);
}

#[test]
fn parse_an_ident() {
    let src = "foo";
    let lexer = tokens::construct_lexer(src);

    let should_be = Ident::new("foo", Span::new(ByteIndex(0), ByteIndex(3)));
    let should_be = Expr::Ident(should_be);

    let got = ExprParser::new().parse(lexer).unwrap();
    assert_eq!(got, should_be);
}
```

Now we've got some tests to guide us, lets write the `Expr` rule.

```rust
// src/grammar.lalrpop

pub Expr: Expr = {
    <Ident> => Expr::Ident(<>),
    <Literal> => Expr::Literal(<>),
};

Literal: Literal = {
    <l:@L> <lit:"literal"> <r:@R> => Literal::new(lit.as_number().unwrap(), Span::new(l, r)),
};

Ident: Ident = {
    <l:@L> <id:"ident"> <r:@R> => Ident::new(id.as_ident().unwrap(), Span::new(l, r)),
};
```

> **Note:** I've added a couple helper methods to make things easier. The
> `as_number()` method will return `Some(f64)` when a we get a `Token::Number`,
> and `None` otherwise. Likewise `as_ident()` will extract the `&'input str` 
> part from an identifier token. 
>
> Lalrpop makes sure the `"ident"` pattern only matches a valid identifier
> token so we use `unwrap()` because we *know* the conversion will never fail
> (and if it does, this indicates a programming error so it should ideally
> blow up loudly).

Next we want to parse a function call into an `Expr::FunctionCall`. The test 
itself is pretty mundane, although constructing the *exact* AST we expect to
parse is starting to get rather verbose.

```rust
#[test]
fn parse_a_function_call() {
    let src = "foo(a, b, 123)";
    let lexer = tokens::construct_lexer(src);

    let name = Ident::new("foo", Span::new(ByteIndex(0), ByteIndex(3)));
    let args = vec![
        Expr::Ident(Ident::new("a", Span::new(ByteIndex(4), ByteIndex(5)))),
        Expr::Ident(Ident::new("b", Span::new(ByteIndex(7), ByteIndex(8)))),
        Expr::Literal(Literal::new(123.0, Span::new(ByteIndex(10), ByteIndex(13)))),
    ];
    let should_be = Expr::FunctionCall(FunctionCall::new(
        name,
        args,
        Span::new(ByteIndex(0), ByteIndex(14)),
    ));

    let got = ExprParser::new().parse(lexer).unwrap();
    assert_eq!(got, should_be);
}
```

Implementing the rule is quite trivial though. Don't forget to update the `Expr`
rule appropriately!

```rust
pub Expr: Expr = {
    <Ident> => Expr::Ident(<>),
    <Literal> => Expr::Literal(<>),
    <FunctionCall> => Expr::FunctionCall(<>),
};

FunctionCall: FunctionCall = {
    <l:@L> <name:Ident> "(" <args:Comma<Expr>> ")" <r:@R> => FunctionCall::new(name, args, Span::new(l, r)),
};

Comma<T>: Vec<T> = { 
    <v:(<T> ",")*> <e:T?> => match e { 
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
};
```

You'll notice we included a "macro" (`Comma<T>`) for parsing any comma-separated
`T`, where `T` can be any arbitrary rule. This is literally copy-pasted from
the *Macros*" chapter in the lalrpop book and lets us say "*parse a 
comma-separated list of `Expr`essions and bind them to the `args` variable*" in
a very concise way.

Last up is to parse a full function, which is made up of a `"def"`, the function
signature, and a function body. 

Writing out the test for this one is a massive
pain because of how nested the AST is getting. If you have a suggestion for 
making these tests easier to write, please let us know by making an [issue] on
GitHub!!

```rust
#[test]
fn parse_a_full_function() {
    let src = "def foo(a b) add(5, 3.15)";
    let lexer = tokens::construct_lexer(src);

    let name = Ident::new("foo", Span::new(ByteIndex(4), ByteIndex(7)));
    let args = vec![
        Ident::new("a", Span::new(ByteIndex(8), ByteIndex(9))),
        Ident::new("b", Span::new(ByteIndex(10), ByteIndex(11))),
    ];
    let decl = FunctionDecl::new(name, args, Span::new(ByteIndex(4), ByteIndex(12)));

    let call_name = Ident::new("add", Span::new(ByteIndex(13), ByteIndex(16)));
    let body_args = vec![
        Expr::Literal(Literal::new(5.0, Span::new(ByteIndex(17), ByteIndex(18)))),
        Expr::Literal(Literal::new(3.15, Span::new(ByteIndex(20), ByteIndex(24)))),
    ];

    let body = Expr::FunctionCall(FunctionCall::new(
        call_name,
        body_args,
        Span::new(ByteIndex(13), ByteIndex(25)),
    ));

    let should_be = Item::Function(Function::new(
        decl,
        body,
        Span::new(ByteIndex(0), ByteIndex(25)),
    ));

    let got = ItemParser::new().parse(lexer).unwrap();
    assert_eq!(got, should_be);
}
```

As I was writing the rules for parsing a function I took the time to do a little
refactoring and extract the common code between `extern` and `def` (the function
signature part) into its own `FunctionDecl` rule.

```rust
pub Item: Item = {
    <Extern> => Item::Extern(<>),
    <FunctionDef> => Item::Function(<>),
};

FunctionDef: Function = {
    <l:@L> "def" <decl:FunctionDecl> <body:Expr> <r:@R> => Function::new(decl, body, Span::new(l, r)),
};

Extern: FunctionDecl = {
    <l:@L> "extern" <decl:FunctionDecl> <r:@R> => FunctionDecl { span: Span::new(l, r), ..decl },
};

FunctionDecl: FunctionDecl = {
    <l:@L> <name:Ident> "(" <args:Ident*> ")" <r:@R> => FunctionDecl::new(name, args, Span::new(l, r)),
};
```

> **Hint:** When using tests to make sure the parser gives you *exactly* what 
> you expect, it often takes a non-trivial amount of time to figure out what's
> different between the two parse trees you received. This is especially 
> annoying when updating things like the `Span` indices.
>
> The [pretty_assertions] crate overrides the default `assert_eq!()` macro to
> show a full-colour diff, pointing out *exactly* what's wrong and how you can
> change it. You may find this useful for testing.

[ast]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[grammar]: https://en.wikibooks.org/wiki/Introduction_to_Programming_Languages/Grammars
[bnf]: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form
[build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[build-dep]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#build-dependencies
[codespan]: https://github.com/brendanzab/codespan
[reporting]: https://github.com/brendanzab/codespan/tree/master/codespan-reporting
[guide]: http://lalrpop.github.io/lalrpop/README.html
[issue]: https://github.com/Michael-F-Bryan/kaleidoscope/issues
[pretty_assertions]: https://github.com/colin-kiegel/rust-pretty-assertions