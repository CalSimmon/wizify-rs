use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, Field, Ident};

use crate::{
    error::Error,
    parse::{self, fields::{parse_field_attributes, FieldAttr, FieldAttrKind}}
};

use super::types::TypeAttributes;

#[derive(Default, Debug)]
pub struct FieldAttributes {
    pub prompt: String,
    pub validation: Option<Expr>,
}

impl FieldAttributes {
    pub fn parse(attrs: &[Attribute], name: &Ident) -> Result<Self, Error> {
        let mut attributes = Self::default();
        attributes.prompt = name.to_string();
        attributes.fill_attributes(attrs)?;

        Ok(attributes)
    }

    pub fn generate_prompt(&mut self, field: &Field, attrs: &TypeAttributes) -> TokenStream {
        if attrs.prefix.is_some() {
            self.add_prefix(&attrs.prefix.clone().unwrap());
        }

        let (is_option, ty) = parse::options::parse_option(field);

        let validation = if let Some(expr) = &self.validation {
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

        let prompt = &self.prompt;
        let ident = &field.ident;

        let field_prompt = if is_option {
            quote! {
                #ident: {
                    Some(
                        dialoguer::Input::<#ty>::new()
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
        };

        TokenStream::from(field_prompt)
    }
        
    fn fill_attributes(&mut self, attrs: &[Attribute]) -> Result<(), Error> {
        for attrs in parse_field_attributes(attrs) {
            let attrs = attrs?;
            for attr in attrs {
                self.insert_attribute(attr)?;
            }
        }

        Ok(())
    }

    fn insert_attribute(&mut self, attr: FieldAttr) -> Result<(), Error> {
        match attr.kind {
            FieldAttrKind::Prompt(prompt) => {
                self.prompt = prompt.value();
            }
            FieldAttrKind::Validation(expr) => {
                self.validation = Some(expr);
            }
        }

        Ok(())
    }

    fn add_prefix(&mut self, prefix: &String) {
        self.prompt = format!("{}{}", prefix, self.prompt);
    }
}
