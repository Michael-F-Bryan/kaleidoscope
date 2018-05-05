# The Lexer

The first step in creating a Kaleidoscope compiler is to turn the raw source 
code into a stream of tokens to make the parsing process easier.

This will be done using a simple regex-based lexer. The idea is you'll register
several patterns and "Token Constructor" functions then the lexer will try to 
match each pattern in turn, using the token constructor to turn a successful
match into a `Token`.

I originally wrote out the process of creating the lexer in an earlier version
of this guide but felt it was useful enough to be a part of `lalrpop` itself.

To use this lexer you'll need to add `lalrpop-util` to your `Cargo.toml` file.

```toml
[dependencies]
lalrpop-util = { git = "https://github.com/Michael-F-Bryan/lalrpop", 
                 features = ["lexer"], 
                 branch = "custom-lexer" }
```

> **TODO:** Update to use the official crate instead of the `custom-lexer` 
> branch when [#373][pr] is merged.

## Using The Lexer

We start the lexing process (also called *tokenizing*) by declaring a `Token`
type to represent each token in the Kaleidoscope language.

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    Identifier(&'input str),
    Def,
}
```

We also need a function which can construct a `Lexer` and configure it so it 
knows how to skip comments (Kaleidoscope uses `#` line comments like most
scripting languages) and teach it how to generate `Token`s.

```rust
pub fn construct_lexer(src: &str) -> Lexer<Token> {
    let mut lexer = Lexer::new(src).skipping(r"^\s+|#.*$");

    // keywords
    lexer.register_pattern(r"^def", |_| Ok(Token::Def));
    lexer.register_pattern(r"^extern", |_| Ok(Token::Extern));

    // literals
    lexer.register_pattern(r"^\d+\.\d+", |s| {
        Ok(Token::Number(s.parse().expect("always valid")))
    });
    lexer.register_pattern(r"^\d+", |s| {
        Ok(Token::Number(s.parse().expect("always valid")))
    });

    // identifiers
    lexer.register_pattern(r"^[\w][\w\d_]*", |s| Ok(Token::Identifier(s)));

    lexer
}
```

And... We're done. That's all we need to do before `lalrpop` can use our 
`Lexer`.

[pr]: https://github.com/lalrpop/lalrpop/pull/373