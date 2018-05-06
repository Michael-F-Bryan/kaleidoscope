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

impl Function {
    pub fn new(decl: FunctionDecl, body: Expr, span: Span) -> Function {
        Function { decl, body, span }
    }
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

impl Literal {
    pub fn new(value: f64, span: Span) -> Literal {
        Literal { value, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub ident: Ident,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl FunctionCall {
    pub fn new(ident: Ident, args: Vec<Expr>, span: Span) -> FunctionCall {
        FunctionCall { ident, args, span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grammar::{ExprParser, ItemParser};
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
}
