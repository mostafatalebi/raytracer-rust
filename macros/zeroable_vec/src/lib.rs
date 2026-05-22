use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Zeroable)]
pub fn zeroable_vector_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl ZeroableVector for #name {
            fn zero() -> Self {
                let mut v = Self::default();
                for i in 0..v.size() {
                    v[i] = 0.0;
                }
                v
            }
        }
    };

    TokenStream::from(expanded)
}

