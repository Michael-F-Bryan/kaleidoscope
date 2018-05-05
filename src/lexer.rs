use failure::Error;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Fail)]
#[fail(display = "Unrecognised token, {}", next_char)]
pub struct LexError {
    pub index: usize,
    pub next_char: char,
}

/// A generic table-based lexer.
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
