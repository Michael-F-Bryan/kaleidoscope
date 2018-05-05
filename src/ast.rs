use codespan::{self, ByteIndex};

pub type Span = codespan::Span<ByteIndex>;

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
