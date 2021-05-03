use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Fields, Generics};

pub fn derive_camo(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    if has_generics(&input.generics) {
        let span = input.generics.span();
        return Error::new(span, "`camo` does not support generics")
            .to_compile_error();
    }

    let fields = build_trailing_expression(&input.data);
    let expr = quote! {
        use ::camo::IntoType;
        camo::Struct::new(stringify!(#name))
            #fields
    };

    quote! {
        impl camo::Camo for #name {
            fn camo() -> camo::Struct {
                #expr
            }
        }
    }
}

fn has_generics(generics: &Generics) -> bool {
    generics.params.iter().len() != 0
}

fn build_trailing_expression(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {f.span()=>
                        .field(camo::Field::new(stringify!(#name), #ty::into_type()))
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(..) => todo!(),
            Fields::Unit => todo!(),
        },
        Data::Enum(..) => todo!(),
        Data::Union(..) => unimplemented!(),
    }
}
