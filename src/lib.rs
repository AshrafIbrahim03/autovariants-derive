use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Variants)]
pub fn variants_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let enum_data = match input.data {
        syn::Data::Struct(_) => panic!("This macro only works for enums"),
        syn::Data::Enum(data_enum) => data_enum,
        syn::Data::Union(_) => panic!("This macro only works for enums"),
    };
    let vars: Vec<proc_macro2::TokenStream> = enum_data
        .variants
        .iter()
        .map(|v| quote! {Self::#v})
        .collect();
    let num_variants = enum_data.variants.len();
    let expanded = quote! {
        impl auto_variants::Variants<#num_variants> for #name {
            const VARIANTS:[Self;#num_variants] =[
                #(#vars),*
            ];
        fn variants() -> &'static [Self]{
                &Self::VARIANTS
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
