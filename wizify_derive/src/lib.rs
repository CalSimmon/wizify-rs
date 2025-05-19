mod attributes;
mod parse;
mod error;

use attributes::{fields::FieldAttributes, types::TypeAttributes};
use error::Error;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Field, Fields};

#[proc_macro_derive(Wizard, attributes(wizard))]
pub fn derive_wizard(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ast = impl_wizard(&input);
    proc_macro::TokenStream::from(ast)
}

fn impl_wizard(ast: &DeriveInput) -> TokenStream {
    let attrs = TypeAttributes::parse(&ast.attrs)
        .map_err(error::abort)
        .unwrap();

    let field_inits = collect_info(ast, &attrs)
        .map_err(error::abort)
        .unwrap();

    // let mut prefix = "".to_string();
    // let mut begin_msg = "".to_string();
    // let mut closing_msg = "".to_string();

    // let _ = for attr in &input.attrs {
    //     if attr.path().is_ident("wizard") {
    //         let _ = attr.parse_nested_meta(|meta| {
    //             if meta.path.is_ident("prefix") {
    //                 meta.value()?;
    //                 let pre: syn::LitStr = meta.input.parse()?;
    //                 prefix = pre.value();
    //             }
    //             if meta.path.is_ident("begin_msg") {
    //                 meta.value()?;
    //                 let begin: syn::LitStr = meta.input.parse()?;
    //                 begin_msg = begin.value();
    //             }
    //             if meta.path.is_ident("closing_msg") {
    //                 meta.value()?;
    //                 let close: syn::LitStr = meta.input.parse()?;
    //                 closing_msg = close.value();
    //             }
    //             Ok(())
    //         });
    //     }
    // };

    // let fields = match &input.data {
    //     Data::Struct(data) => match &data.fields {
    //         Fields::Named(fields) => &fields.named,
    //         _ => {
    //             return TokenStream::from(quote! {
    //                 compile_error!("Wizard only supports structs with named fields...");
    //             });
    //         }
    //     },
    //     _ => {
    //         return TokenStream::from(quote! {
    //             compile_error!("Wizard only works with structs...");
    //         });
    //     }
    // };

    let field_inits = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        // let mut prompt = ident.as_ref().unwrap().to_string();
        // let mut expression: Option<syn::Expr> = None;

        // for attr in &f.attrs {
        //     if attr.path().is_ident("wizard") {
        //         attr.parse_nested_meta(|meta| {
        //             if meta.path.is_ident("prompt") {
        //                 meta.value()?;
        //                 let lit: syn::LitStr = meta.input.parse()?;
        //                 prompt = lit.value();
        //             }
        //             if meta.path.is_ident("validate") {
        //                 meta.value()?;
        //                 let expr: syn::Expr = meta.input.parse()?;
        //                 expression = Some(expr)
        //             }
        //             Ok(())
        //         })
        //         .unwrap();
        //     }
        // }

        if prefix != "" {
            prompt = format!("{}{}", prefix, prompt);
        };

        let mut is_option: bool = false;
        let mut option_type: Option<&syn::Type> = None;

        match &f.ty {
            syn::Type::Path(type_path) => {
                is_option = type_path
                    .path
                    .segments
                    .last()
                    .map(|s| s.ident == "Option")
                    .unwrap_or(false);
                if is_option {
                    option_type = parse::options::extract_type_from_option(&ty);
                };
            }
            _ => (),
        };

        let validation = if let Some(expr) = expression {
            quote! {
                .validate_with(|i: &#ty| -> Result<(), &str> {
                    let input = *i;
                    if #expr {
                        Ok(())
                    } else {
                        Err("This input is not valid...")
                    }
                })
            }
        } else {
            quote! {}
        };

        if is_option {
            quote! {
                #ident: {
                    Some(
                        dialoguer::Input::<#option_type>::new()
                            .with_prompt(#prompt)
                            #validation
                            .allow_empty(true)
                            .interact()
                            .unwrap()
                    )
                }
            }
        } else {
            quote! {
                #ident: {
                    dialoguer::Input::<#ty>::new()
                        .with_prompt(#prompt)
                        #validation
                        .interact()
                        .unwrap()
                }
            }
        }
    });

    let expanded = quote! {
        impl Wizard for #name {
            fn wizard() -> Self {
                print!("{}", #begin_msg);
                let output = Self {
                    #(#field_inits,)*
                };
                print!("{}", #closing_msg);

                output
            }
        }
    };

    TokenStream::from(expanded)
}

fn collect_info(
    ast: &DeriveInput,
    attrs: &TypeAttributes,
) -> Result<TokenStream, Error> {
    match &ast.data {
        Data::Struct(data) => collect_info_struct(data, attrs),
        Data::Enum(data) => collect_info_enum(data, attrs),
        Data::Union(_) => Err(Error::message("Union type is not supported...")),
    }
}

// TODO - Implement struct info gathering
fn collect_info_struct(
    ast: &DataStruct,
    attrs: &TypeAttributes,
) ->Result<TokenStream, Error> {
    // Get all fields
    
    // If they are an option, get the internal type
    //
    // Return the entire token stream
    Err(Error::message("Function has not been implemented yet..."))
}

fn prompt_from_fields(
    fields: &Fields,
    attrs: &TypeAttributes,
) -> Result<TokenStream, Error> {
    let field_attrs = fields
        .into_iter()
        .map(|field| -> Result<_, Error> {
            let field_name = &field.ident.unwrap();
            let mut attributes = FieldAttributes::parse(&field.attrs, field_name);
            
            if attrs.prefix.is_some() {
                attributes.unwrap().add_prefix(&attrs.prefix.unwrap());
            }

            Ok(attributes)
        });
}

fn generate_prompt(
    field: &Field,
    attrs: &TypeAttributes,
) -> Return<TokenStream, Error> {

}

// TODO - Implement enum info gathering
fn collect_info_enum(
    ast: &DataEnum,
    attrs: &TypeAttributes,
) ->Result<TokenStream, Error> {
    Err(Error::message("Function has not been implemented yet..."))
}
