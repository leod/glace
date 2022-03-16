use glsl::syntax as glsl;
use syn::spanned::Spanned as _;

use super::GenError;

use GenError::Unsupported;

pub fn bin_op(o: &syn::BinOp) -> Result<glsl::AssignmentOp, GenError> {
    Ok(match o {
        syn::BinOp::AddEq(_) => glsl::AssignmentOp::Add,
        syn::BinOp::SubEq(_) => glsl::AssignmentOp::Sub,
        syn::BinOp::MulEq(_) => glsl::AssignmentOp::Mult,
        syn::BinOp::DivEq(_) => glsl::AssignmentOp::Div,
        syn::BinOp::RemEq(_) => glsl::AssignmentOp::Mod,
        syn::BinOp::BitXorEq(_) => glsl::AssignmentOp::Xor,
        syn::BinOp::BitAndEq(_) => glsl::AssignmentOp::And,
        syn::BinOp::BitOrEq(_) => glsl::AssignmentOp::Or,
        syn::BinOp::ShlEq(_) => glsl::AssignmentOp::LShift,
        syn::BinOp::ShrEq(_) => glsl::AssignmentOp::RShift,
        o => {
            return Err(Unsupported(
                o.span(),
                format!("unsupported binary assigment op: {:?}", o),
            ))
        }
    })
}

pub fn expr(e: &syn::Expr) -> Result<glsl::Expr, GenError> {
    match e {
        syn::Expr::Array(e) => Err(Unsupported(e.span(), "array".into())),
        syn::Expr::Assign(e) => {
            if !e.attrs.is_empty() {
                return Err(Unsupported(e.span(), "attributes".into()));
            }
            Ok(glsl::Expr::Assignment(
                Box::new(expr(&e.left)?),
                glsl::AssignmentOp::Equal,
                Box::new(expr(&e.right)?),
            ))
        }
        syn::Expr::AssignOp(e) => {
            if !e.attrs.is_empty() {
                return Err(Unsupported(e.span(), "attributes".into()));
            }
            Ok(glsl::Expr::Assignment(
                Box::new(expr(&e.left)?),
                bin_op(&e.op)?,
                Box::new(expr(&e.right)?),
            ))
        }
        syn::Expr::Async(_) => todo!(),
        syn::Expr::Await(_) => todo!(),
        syn::Expr::Binary(_) => todo!(),
        syn::Expr::Block(_) => todo!(),
        syn::Expr::Box(_) => todo!(),
        syn::Expr::Break(_) => todo!(),
        syn::Expr::Call(_) => todo!(),
        syn::Expr::Cast(_) => todo!(),
        syn::Expr::Closure(_) => todo!(),
        syn::Expr::Continue(_) => todo!(),
        syn::Expr::Field(_) => todo!(),
        syn::Expr::ForLoop(_) => todo!(),
        syn::Expr::Group(_) => todo!(),
        syn::Expr::If(_) => todo!(),
        syn::Expr::Index(_) => todo!(),
        syn::Expr::Let(_) => todo!(),
        syn::Expr::Lit(_) => todo!(),
        syn::Expr::Loop(_) => todo!(),
        syn::Expr::Macro(_) => todo!(),
        syn::Expr::Match(_) => todo!(),
        syn::Expr::MethodCall(_) => todo!(),
        syn::Expr::Paren(_) => todo!(),
        syn::Expr::Path(_) => todo!(),
        syn::Expr::Range(_) => todo!(),
        syn::Expr::Reference(_) => todo!(),
        syn::Expr::Repeat(_) => todo!(),
        syn::Expr::Return(_) => todo!(),
        syn::Expr::Struct(_) => todo!(),
        syn::Expr::Try(_) => todo!(),
        syn::Expr::TryBlock(_) => todo!(),
        syn::Expr::Tuple(_) => todo!(),
        syn::Expr::Type(_) => todo!(),
        syn::Expr::Unary(_) => todo!(),
        syn::Expr::Unsafe(_) => todo!(),
        syn::Expr::Verbatim(_) => todo!(),
        syn::Expr::While(_) => todo!(),
        syn::Expr::Yield(_) => todo!(),
        _ => todo!(),
    }
}
