# The Lexer

The first step in creating a Kaleidoscope compiler is to turn the raw source 
code into a stream of tokens to make the parsing process easier.

This will be done using a simple regex-based lexer. The idea is you'll register
several patterns and "Token Constructor" functions then the lexer will try to 
match each pattern in turn, using the token constructor to turn a successful
match into a `Token`.

## The Lexer Type

> **Note:** I'm hoping to get this merged into the `lalrpop` repository some
> time, so we'll put the lexer definition and use in separate files. Hopefully
> you can skip the `Lexer` definition in the future and just import it from the
> `lalrpop_util` crate!

First you'll need to add the `regex` crate to your `Cargo.toml` and the 
corresponding `pub mod lexer` and `extern crate regex` line to `lib.rs`. We'll
be using the [failure] crate for error handling, so make sure to add that too.

```console
$ cargo add regex failure
$ cat src/lib.rs
extern crate failure;
extern crate regex;

pub mod lexer;
```

Next comes the definition of our `Lexer`. The `Lexer` contains a reference to
the source string, a list of regex patterns and boxed token constructors, a 
regex used for automatically skipping chunks of code (e.g. whitespace or 
comments), and an index to keep track of where we're up to in the source code.

I've also added a simple `LexError` struct which lets us generate error messages
like "*unknown character `%` at line 5, column 10*".

```rust
use failure::Error;
use regex::Regex;

pub struct Lexer<'input, Token: 'input> {
    src: &'input str,
    patterns: Vec<(Regex, Box<Fn(&'input str) -> Result<Token, Error>>)>,
    skips: Regex,
    ix: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LexError {
    pub next_char: char,
}
```

The extra `: 'input` bound on `Token` means our `Token` can borrow directly from
the source code instead of needing to create extra string copies for every 
identifier or other string in the program.

Once we've got a definition of the `Lexer` struct it's time to add a constructor
and some methods for configuring the `Lexer`.

```rust
impl<'input, Token: 'input> Lexer<'input, Token> {
    /// Create a new `Lexer` with an empty pattern table and which ignores all
    /// whitespace by default.
    pub fn new(src: &'input str) -> Lexer<'input, Token> {
        Lexer {
            src: src,
            patterns: Vec::new(),
            skips: Regex::new(r"^\s+").unwrap(),
            ix: 0,
        }
    }

    /// Register a token pattern and a function for turning the matched text
    /// into its corresponding `Token`.
    pub fn register_pattern<F>(&mut self, pattern: &str, constructor: F)
    where
        F: Fn(&'input str) -> Result<Token, Error> + 'static,
    {
        assert!(
            pattern.starts_with("^"),
            "All patterns must match the beginning of the text"
        );

        let re = Regex::new(pattern).expect("Invalid regex");
        let constructor = Box::new(constructor);

        self.patterns.push((re, constructor));
    }

    /// Set a custom skip pattern.
    pub fn skipping(self, pattern: &str) -> Lexer<'input, Token> {
        assert!(
            pattern.starts_with("^"),
            "All patterns must match the beginning of the text"
        );
        let skips = Regex::new(pattern).expect("Invalid regex");

        Lexer { skips, ..self }
    }
```

Our lexer also needs a couple helper functions for retrieving the remaining 
source text, checking if we're finished, and skipping past text the user doesn't
care about (our `skips`).

```rust
    fn skip(&mut self) {
        while let Some(skipped) = self.skips.find(self.remaining()) {
            self.ix += skipped.as_str().len();
        }
    }

    fn remaining(&self) -> &'input str {
        &self.src[self.ix..]
    }

    fn is_finished(&self) -> bool {
        self.src.len() <= self.ix
    }
}
```

Lalrpop requires our lexer to be an iterator of something looking like
`Result<(Loc, Tok, Loc), Error>`. In english this translates to a fallible
stream of tokens (`Tok`) that also lets us know the location (`Loc`)
immediately before and after the token.

```rust
impl<'input, Token: 'input> Iterator for Lexer<'input, Token> {
    type Item = Result<(usize, Token, usize), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip();

        if self.is_finished() {
            return None;
        }

        let start = self.ix;

        for &(ref pattern, ref constructor) in &self.patterns {
            if let Some(found) = pattern.find(self.remaining()) {
                self.ix += found.end();

                let ret = constructor(found.as_str()).map(|t| (start, t, self.ix));
                return Some(ret);
            }
        }

        let err = LexError {
            index: self.ix,
            next_char: self.remaining()
                .chars()
                .next()
                .expect("We've already checked for EOF"),
        };

        Some(Err(err.into()))
    }
}
```

Although this `next()` method looks a little intimidating, all we're really
doing is:

1. Skip over any characters we don't care about
2. Stop if we've reached the end of input
3. Check each registered pattern in turn. If we find something that matches 
   turn that into a `Token`, update the current location, and return the result
   to the user (making sure to attach the start and end locations)
4. If we didn't find anything, construct a `LexError` to let the user know we've
   hit an unknown character and can't continue

Again, you shouldn't need to write out any of this in the future. Hopefully 
it'll be built into `lalrpop`.

## Using The Lexer

Now we've got a `Lexer` we can define the various tokens Kaleidoscope will have.
This is just a simple `enum`.

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    Identifier(&'input str),
    Def,
}
```

We also need a function which can construct a `Lexer` and configure it so it 
knows how to skip comments (Kaleidoscope uses `#` like most scripting languages)
and can generate our `Token`s.

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