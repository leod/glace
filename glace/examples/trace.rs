use quote::quote;

fn main() {
    let rustish = quote! {
        {
            let x = 5;

            let y = if x < 5 {
                42
            } else {
                x * 3
            } + 2;

            let z = {
                let mut x = 300;
                x *= 3;
                x
            };

            return y * y - z;
        }
    };
    let rustish_tree: syn::Block = syn::parse2(rustish).unwrap();

    let trace = {
        {
            let mut glace_block_ = Vec::new();

            let x = 5;
            glace_block_.append(TypedExpr(Type::type_specifier(x)))
        }
    };

    println!("{:?}", rustish_tree);
}
