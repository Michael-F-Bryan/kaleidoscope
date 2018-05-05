use lalrpop_util::Lexer;
use void::Void;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    Identifier(&'input str),
    Number(f64),
    Def,
    Extern,
}

pub fn construct_lexer(src: &str) -> Lexer<Token, Void> {
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
