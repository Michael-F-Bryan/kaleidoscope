use codespan::{self, ByteIndex};

pub type Span = codespan::Span<ByteIndex>;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub items: Vec<Item>,
    pub span: Span,
}

impl File {
    pub fn new(items: Vec<Item>, span: Span) -> File {
        File { items, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Extern(FunctionDecl),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub args: Vec<Ident>,
    pub span: Span,
}

impl FunctionDecl {
    pub fn new(ident: Ident, args: Vec<Ident>, span: Span) -> FunctionDecl {
        FunctionDecl { ident, args, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub decl: FunctionDecl,
    pub body: Expr,
    pub span: Span,
}

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

impl Ident {
    pub fn new<S: Into<String>>(name: S, span: Span) -> Ident {
        Ident {
            name: name.into(),
            span,
        }
    }
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
