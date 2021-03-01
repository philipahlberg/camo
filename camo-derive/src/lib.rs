use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Generics};

#[proc_macro_derive(Camo)]
pub fn derive_camo(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    if has_generics(&input.generics) {
        let span = input.generics.span();
        return Error::new(span, "`camo` does not support generics")
            .to_compile_error()
            .into();
    }

    let fields = build_trailing_expression(&input.data);
    let expr = quote! {
        use ::camo::IntoType;
        camo::Struct::new(stringify!(#name))
            #fields
    };

    let expanded = quote! {
        impl camo::Camo for #name {
            fn camo() -> camo::Struct {
                #expr
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn has_generics(generics: &Generics) -> bool {
    generics.params.iter().next().is_some()
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
