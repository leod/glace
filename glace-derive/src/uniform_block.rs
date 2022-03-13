use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_quote, Data, DataStruct, DeriveInput, Error, Fields, Path, Result};

pub fn derive(input: DeriveInput) -> Result<TokenStream2> {
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields,
        _ => return Err(Error::new_spanned(
            input.ident,
            "derive(UniformInput) does not support tuple structs, unit structs, enums, or unions",
        )),
    };

    let trait_path: Path = parse_quote!(::glace::UniformBlock);

    let name = input.ident;
    let name_str = Literal::string(&name.to_string());

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = fields.named.iter().map(|field| {
        let field_name_str = Literal::string(&field.ident.as_ref().unwrap().to_string());
        let field_ty = &field.ty;

        quote! {
            ::glace::Field {
                name: #field_name_str,
                ty: <#field_ty as ::glace::UniformType>::TYPE_SPECIFIER,
            }
        }
    });

    Ok(quote! {
        unsafe impl #impl_generics #trait_path for #name #ty_generics #where_clause {
            const NAME: &'static str = #name_str;
            const FIELDS: &'static [::glace::Field] = &[
                #( #fields, )*
            ];
        }
    })
}
