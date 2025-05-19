use proc_macro2::Ident;
use syn::{Attribute, LitStr, Token, parse::Parse, punctuated::Punctuated};

pub fn parse_type_attributes(
    attributes: &[Attribute],
) -> impl Iterator<Item = syn::Result<impl Iterator<Item = TypeAttr>>> + '_ {
    attributes
        .iter()
        .filter(|attr| attr.path().is_ident("wizard"))
        .map(|attr| attr.parse_args_with(Punctuated::<TypeAttr, Token![,]>::parse_terminated))
        .map(|result| result.map(IntoIterator::into_iter))
}

pub struct TypeAttr {
    pub kind: TypeAttrKind,
}

impl TypeAttr {
    pub fn new(kind: TypeAttrKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Debug)]
pub enum TypeAttrKind {
    Prefix(LitStr),
    BeginMsg(LitStr),
    CloseMsg(LitStr),
}

impl Parse for TypeAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use TypeAttrKind::*;

        let name: Ident = input.parse()?;
        let str_name = name.to_string();

        if input.peek(Token![=]) {
            let assign_token = input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                let lit = input.parse::<LitStr>()?;

                match str_name.as_str() {
                    "prefix" => return Ok(Self::new(Prefix(lit))),
                    "begin_msg" => return Ok(Self::new(BeginMsg(lit))),
                    "closing_msg" => return Ok(Self::new(CloseMsg(lit))),
                    _ => {}
                }
            }

            return Err(syn::Error::new(
                assign_token.span, 
                "Expected `string literal` after `=`"
            ));
        }

        Err(syn::Error::new(
                name.span(), 
                format!("Unexpected attribute: {str_name}")
        ))
    }
}
