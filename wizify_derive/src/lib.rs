use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field};

#[proc_macro_derive(Wizard)]
pub fn derive_wizard(input: TokenStream) -> TokenStream {
    println!("Input TokenStream: {}", input);
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        Data::Struct(s) => {
            println!("{:?}", s.fields.into_iter().map(|field| field).collect::<Vec<_>>());
        }
        _ => todo!(),
    }

    quote!{
        impl Wizard for #name {
            fn wizard() {
                ()
            }
        }
    }.into()
}

