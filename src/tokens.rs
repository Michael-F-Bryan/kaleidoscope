use codespan::ByteIndex;
use lalrpop_util::{Lexer, ParseError};
use void::Void;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    Identifier(&'input str),
    Number(f64),
    Def,
    Extern,
    OpenParen,
    CloseParen,
}

impl<'input> Token<'input> {
    pub fn as_ident(&self) -> Option<&'input str> {
        match *self {
            Token::Identifier(id) => Some(id),
            _ => None,
        }
    }
}

pub type Spanned<'input> =
    Result<(ByteIndex, Token<'input>, ByteIndex), ParseError<ByteIndex, Token<'input>, Void>>;

pub fn construct_lexer(src: &str) -> impl Iterator<Item = Spanned> {
    let mut lexer = Lexer::new(src).skipping(r"^\s+|#.*$");

    // punctuation
    lexer.register_pattern(r"^\(", |_| Ok(Token::OpenParen));
    lexer.register_pattern(r"^\)", |_| Ok(Token::CloseParen));

    // keywords
    lexer.register_pattern(r"^def", |_| Ok(Token::Def));
    lexer.register_pattern(r"^extern", |_| Ok(Token::Extern));

    // literals
    lexer.register_pattern(r"^\d+(\.\d+)?", |s| {
        Ok(Token::Number(s.parse().expect("parse never fails")))
    });

    // identifiers
    lexer.register_pattern(r"^[\w][\w\d_]*", |s| Ok(Token::Identifier(s)));

    lexer.map(|sp| {
        sp.map(|(l, tok, r)| (ByteIndex(l as u32), tok, ByteIndex(r as u32)))
            .map_err(|e| e.map_location(|loc| ByteIndex(loc as u32)))
    })
}

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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! lexer_test {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let src = $src;
                let should_be = Token::from($should_be);

                let (_, got, _) = construct_lexer(src)
                    .next()
                    .expect("Unexpected EOF")
                    .expect("Unexpected error");

                assert_eq!(got, should_be);
            }
        };
    }

    lexer_test!(recognise_an_integer, "123" => 123);
    lexer_test!(recognise_a_decimal, "3.1415" => 3.1415);
    lexer_test!(recognise_an_ident, "foo" => "foo");
    lexer_test!(recognise_def, "def" => Token::Def);
    lexer_test!(recognise_extern, "extern" => Token::Extern);
    lexer_test!(recognise_open_paren, "(" => Token::OpenParen);
    lexer_test!(recognise_close_paren, ")" => Token::CloseParen);
}
