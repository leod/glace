use glsl::syntax::TypeSpecifier;

trait Type {
    const TYPE_SPECIFIER: TypeSpecifier;
}

enum Block<T: Type> {
    
}

enum Stmt {

}

pub struct ExprIf<T: Type> {
    pub cond: Box<Expr<bool>>,
    pub then_branch: Block<T>,
    pub else_branch: Option<Box<Expr<T>>>,
}

enum Expr<T: Type> {
    If {
        cond:
        pub then_branch: 
    }
}