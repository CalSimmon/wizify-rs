mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Wizard, attributes(wizard))]
pub fn derive_wizard(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut prefix = "".to_string();
    let mut begin_msg = "".to_string();
    let mut closing_msg = "".to_string();

    let _ = for attr in &input.attrs {
        if attr.path().is_ident("wizard") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    meta.value()?;
                    let pre: syn::LitStr = meta.input.parse()?;
                    prefix = pre.value();
                }
                if meta.path.is_ident("begin_msg") {
                    meta.value()?;
                    let begin: syn::LitStr = meta.input.parse()?;
                    begin_msg = begin.value();
                }
                if meta.path.is_ident("closing_msg") {
                    meta.value()?;
                    let close: syn::LitStr = meta.input.parse()?;
                    closing_msg = close.value();
                }
                Ok(())
            });
        }
    };

    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => {
                    return TokenStream::from(quote! {
                        compile_error!("Wizard only supports structs with named fields...");
                    });
                }
            }
        }
        _ => {
            return TokenStream::from(quote!{
                compile_error!("Wizard only works with structs...");
            });
        }
    };

    let field_inits = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let mut prompt = ident.as_ref().unwrap().to_string();
        let mut expression: Option<syn::Expr> = None;

        for attr in &f.attrs {
            if attr.path().is_ident("wizard") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("prompt") {
                        meta.value()?;
                        let lit: syn::LitStr = meta.input.parse()?;
                        prompt = lit.value();
                    }
                    if meta.path.is_ident("validate") {
                        meta.value()?;
                        let expr: syn::Expr = meta.input.parse()?;
                        expression = Some(expr)
                    }
                    Ok(())
                }).unwrap();
            }
        }

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
            },
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

    let expanded = quote!{
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

