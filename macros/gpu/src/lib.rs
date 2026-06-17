use proc_macro::TokenStream;
use quote::quote;

/// WIP - DO NOT USE
#[proc_macro_derive(Gpu_Cross)]
pub fn gpu_fn_cross_derive(input: TokenStream) -> TokenStream {
    // let mut input = parse_macro_input!(input as ItemFn);
    //
    // input.block = Box::new(syn::parse_quote!({
    //     objc2
    // }));
    //
    TokenStream::from(input)
}

