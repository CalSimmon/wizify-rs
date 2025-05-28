mod attributes;
mod parse;
mod error;

use attributes::{fields::FieldAttributes, types::TypeAttributes};
use error::Error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Wizard, attributes(wizard))]
pub fn derive_wizard(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ast = impl_wizard(&input);
    proc_macro::TokenStream::from(ast)
}

fn impl_wizard(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let attrs = TypeAttributes::parse(&ast.attrs)
        .map_err(error::abort)
        .unwrap();

    let field_inits = collect_info(ast, &attrs)
        .map_err(error::abort)
        .unwrap();

    let (begin_msg, closing_msg) = (attrs.begin_msg, attrs.closing_msg);
    let begin = if let Some(msg) = begin_msg {
        quote! {
            print!("{}", #msg);
        }
    } else {
        quote! {}
    };
    let closing = if let Some(msg) = closing_msg {
        quote! {
            print!("{}", #msg);
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl Wizard for #name {
            fn wizard() -> Self {
                #begin
                let output = Self {
                    #(#field_inits),*
                };
                #closing

                output
            }
        }
    };

    expanded.into()
}

fn collect_info(
    ast: &DeriveInput,
    attrs: &TypeAttributes,
) -> Result<Vec<TokenStream>, Error> {
    match &ast.data {
        Data::Struct(data) => Ok(prompt_from_fields(&data.fields, attrs)),
        Data::Enum(_data) => Err(Error::message("Enums have not yet been implemented...")),
        Data::Union(_) => Err(Error::message("Wizard does not support unions...")),
    }
}

fn prompt_from_fields(
    fields: &Fields,
    attrs: &TypeAttributes,
) -> Vec<TokenStream> {
    let field_prompts: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let attributes = FieldAttributes::parse(&field.attrs, &field_name);
            attributes.unwrap().generate_prompt(&field, attrs).into()
        })
        .collect();

    field_prompts
}

// TODO - Implement enum info gathering
// fn collect_info_enum(
//     ast: &DataEnum,
//     attrs: &TypeAttributes,
// ) -> impl Iterator<Item = proc_macro::TokenStream> {
//     Err(Error::message("Function has not been implemented yet..."))
// }
