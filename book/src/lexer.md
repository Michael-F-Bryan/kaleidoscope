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
> branch when [lalrpop#373][pr] is merged.

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
    lexer.register_pattern(r"^\d+(\.\d+)?", |s| {
        Ok(Token::Number(s.parse().expect("parse never fails")))
    });

    // identifiers
    lexer.register_pattern(r"^[\w][\w\d_]*", |s| Ok(Token::Identifier(s)));

    lexer
}
```

And... We're done. That's everything we need to do so `lalrpop` can parse our
language using a custom tokenizer.

## Testing The Lexer

As a sanity check, lets add some tests to make sure we can tokenize things 
correctly. These tests are fairly simple, you construct an input string, run it
through the lexer, then make sure it returns *exactly* what you expect.

For example, here's a simple test which makes sure we can recognise a basic 
integer and convert it to a 64-bit floating point number.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognise_a_number() {
        let src = "123";
        let should_be = Token::Number(123.0);

        let (_start, got, _end) = construct_lexer(src).next().unwrap().unwrap();

        assert_eq!(got, should_be);
    }
}
```

This works pretty well, although ideally we'd have to write (at least) one test
per token type, which requires a lot of copy-pasta. It'd be much nicer to use
a macro for generating all the boilerplate associated with these tests.

```rust
macro_rules! lexer_test {
    ($name:ident, $src:expr => $should_be:expr) => {
        #[test]
        fn $name() {
            let src = $src;
            let should_be = Token::from($should_be);

            let (_, got, _) = construct_lexer(src).next()
                .expect("Unexpected EOF")
                .expect("Unexpected error");

            assert_eq!(got, should_be);
        } 
    };
}
```

While we're at it, we'll add a couple `From` implementations to make
constructing a `Token` easier in our tests (notice the `Token::from(...)` in the
macro).

```rust
impl<'a> From<i32> for Token<'a> {
    fn from(other: i32) -> Token<'a> {
        Token::Number(other as f64)
    }
}

impl<'a> From<f64> for Token<'a> {
    fn from(other: f64) -> Token<'a> {
        Token::Number(other)
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(other: &'a str) -> Token<'a> {
        Token::Identifier(other)
    }
}
```

Now we've set things up so adding new lexer tests is almost trivial.

```rust
lexer_test!(recognise_an_integer, "123" => 123);
lexer_test!(recognise_a_decimal, "3.1415" => 3.1415);
lexer_test!(recognise_an_ident, "foo" => "foo");
lexer_test!(recognise_def, "def" => Token::Def);
lexer_test!(recognise_extern, "extern" => Token::Extern);
```

[pr]: https://github.com/lalrpop/lalrpop/pull/373