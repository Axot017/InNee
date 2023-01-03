use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;
extern crate proc_macro2;

#[derive(FromDeriveInput)]
#[darling(attributes(with_id))]
struct WithIdOpts {
    id_type: Option<String>,
}

#[proc_macro_derive(WithId, attributes(with_id))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = WithIdOpts::from_derive_input(&input).expect("Invalid opts");
    let ident = input.ident;
    let with_id_ident = format_ident!("{}WithId", ident);
    let id_type = format_ident!("{}", opts.id_type.unwrap_or_else(|| "String".to_string()));

    let output = quote! {
        #[derive(Clone, Debug)]
        pub struct #with_id_ident(#id_type, #ident);
    };
    output.into()
}
