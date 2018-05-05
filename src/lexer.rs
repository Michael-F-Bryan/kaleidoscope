use failure::Error;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Fail)]
#[fail(display = "Unrecognised token, {} at index {}", next_char, index)]
pub struct LexError {
    pub index: usize,
    pub next_char: char,
}

/// A generic table-based lexer.
///
/// This works by registering a bunch of token patterns and *Token Constructors*
/// which let you convert the matched text into a custom `Token` type. When
/// tokenizing, the lexer will try each pattern in turn until it finds a match
/// and use the corresponding token constructor to create a token and return it.
///
/// # Examples
///
/// Here's how you would tokenize a trivial language consisting of just integers
/// and words, where all whitespace is ignored.
///
/// ```rust
/// # extern crate failure;
/// # extern crate kaleidoscope;
/// # use kaleidoscope::lexer::Lexer;
/// #[derive(Debug, PartialEq)]
/// pub enum Token {
///   Integer(i64),
///   Word(String),
/// }
///
/// # fn run() -> Result<(), failure::Error> {
/// let src = "Hello 5 world";
///
/// // create the lexer
/// let mut lexer = Lexer::new(src);
///
/// // register a bunch of patterns so it knows how to create tokens
/// lexer.register_pattern(r"^\d+", |s| Ok(Token::Integer(s.parse().unwrap())));
/// lexer.register_pattern(r"^\w+", |s| Ok(Token::Word(s.to_string())));
///
/// // then run the lexer to completion, bailing on the first error
/// let got = lexer.collect::<Result<Vec<_>, _>>()?;
///
/// // Extract just the tokens out of the resulting list of positions and tokens
/// let tokens: Vec<Token> = got.into_iter()
///                             .map(|(_start, tok, _end)| tok)
///                             .collect();
///
/// let should_be = vec![Token::Word(String::from("Hello")),
///                      Token::Integer(5),
///                      Token::Word(String::from("world"))];
///
/// // as a sanity check, make sure we got back what we expected
/// assert_eq!(tokens, should_be);
/// # Ok(())
/// # }
/// # fn main() { run().unwrap() }
/// ```
///
/// If you want to avoid unnecessary copies, the `Token` type can contain
/// references to the original source code.
///
/// ```rust
/// # extern crate failure;
/// # extern crate kaleidoscope;
/// # use kaleidoscope::lexer::Lexer;
/// #[derive(Debug, PartialEq)]
/// pub enum Token<'input> {
///   Integer(i64),
///   Word(&'input str), // <-- borrowing part of the original string
/// }
///
/// # fn run() -> Result<(), failure::Error> {
/// let src = "Hello 5 world";
///
/// let mut lexer = Lexer::new(src);
///
/// lexer.register_pattern(r"^\d+", |s| Ok(Token::Integer(s.parse().unwrap())));
/// lexer.register_pattern(r"^\w+", |s| Ok(Token::Word(s)));  // <-- no "to_string()"!
///
/// let got = lexer.collect::<Result<Vec<_>, _>>()?;
/// # Ok(())
/// # }
/// # fn main() { run().unwrap() }
/// ```
///
/// This is completely safe because the `Token: 'input` lifetime on `Lexer` will
/// ensure tokens can never outlive their source code.
pub struct Lexer<'input, Token: 'input> {
    src: &'input str,
    patterns: Vec<(Regex, Box<Fn(&'input str) -> Result<Token, Error>>)>,
    skips: Regex,
    ix: usize,
}

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
    ///
    /// # Note
    ///
    /// The order in which you register patterns **is important**. Patterns
    /// registered earlier will take precedence over later patterns.
    ///
    /// # Panics
    ///
    /// All patterns must begin with a `^` to ensure they match the start of a
    /// string.
    ///
    /// This function will also `panic!()` if an invalid regex pattern is passed
    /// in.
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
    ///
    /// # Examples
    ///
    /// A common desire is the ability to have a lexer which skips whitespace
    /// and ignores the rest of the line when encountering a specific character
    /// (e.g. `#`).
    ///
    /// ```rust
    /// # extern crate kaleidoscope;
    /// # use kaleidoscope::lexer::Lexer;
    /// # fn make_lexer() -> Lexer<'static, &'static str> {
    /// # let some_source_text = "# this is a comment\ntext";
    /// let lexer = Lexer::new(some_source_text).skipping(r"^\s+|(?m)#.*$");
    /// # lexer
    /// # }
    /// # // make sure our pattern actually does what it says it does
    /// # fn main() {
    /// #  let mut l = make_lexer();
    /// #  l.register_pattern(r"^\w+", |s| Ok(s));
    /// #  assert_eq!(l.next().unwrap().unwrap().1, "text");
    /// # }
    /// ```
    ///
    /// Note that you need to enable multiline regex patterns (`(?m)`) when
    /// skipping to the end of a line.
    pub fn skipping(self, pattern: &str) -> Lexer<'input, Token> {
        assert!(
            pattern.starts_with("^"),
            "All patterns must match the beginning of the text"
        );
        let skips = Regex::new(pattern).expect("Invalid regex");

        Lexer { skips, ..self }
    }

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
