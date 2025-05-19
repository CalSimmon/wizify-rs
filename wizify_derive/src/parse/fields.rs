use proc_macro2::Ident;
use syn::{Attribute, Expr, LitStr, Token, parse::Parse, punctuated::Punctuated};

pub fn parse_field_attributes(
    attributes: &[Attribute],
) -> impl Iterator<Item = syn::Result<impl Iterator<Item = FieldAttr>>> + '_ {
    attributes
        .iter()
        .filter(|attr| attr.path().is_ident("wizard"))
        .map(|attr| attr.parse_args_with(Punctuated::<FieldAttr, Token![,]>::parse_terminated))
        .map(|result| result.map(IntoIterator::into_iter))
}

pub struct FieldAttr {
    pub kind: FieldAttrKind,
}

impl FieldAttr {
    pub fn new(kind: FieldAttrKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Debug)]
pub enum FieldAttrKind {
    Prompt(LitStr),
    Validation(Expr),
}

impl Parse for FieldAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use FieldAttrKind::*;

        let name: Ident = input.parse()?;
        let str_name = name.to_string();

        if input.peek(Token![=]) {
            let assign_token = input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                let lit = input.parse::<LitStr>()?;
                match str_name.as_str() {
                    "prompt" => return Ok(Self::new(Prompt(lit))),
                    _ => {}
                }
            }

            if Expr::peek(input) {
                let expr = input.parse::<Expr>()?;
                match str_name.as_str() {
                    "validation" => return Ok(Self::new(Validation(expr))),
                    _ => {}
                }
            }

            return Err(syn::Error::new(
                assign_token.span,
                "Expected `string literal` or `expression` after `=`",
            ));
        }

        Err(syn::Error::new(
            name.span(),
            format!("Unexpected attribute: {str_name}"),
        ))
    }
}
