#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub ty: glsl::syntax::TypeSpecifier,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    If(ExprIf),
    Call(ExprCall),
    Binary(ExprBinary),
    Lit(ExprLit),
    Assign(ExprAssign),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprIf {
    pub cond: Box<Expr>,
    pub then_branch: Block,
    pub else_branch: Option<Box<Expr>>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprCall {
    pub func: syn::Ident,
    pub args: Vec<Box<Expr>>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprLit {
    pub lit: Lit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprAssign {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Local(Local),
    Expr(Expr),
    Semi(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Local {
    pub ident: syn::Ident,
    pub mutable: bool,
    pub ty: Type,
    pub init: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(LitInt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LitInt {
    pub value: u64,
}
