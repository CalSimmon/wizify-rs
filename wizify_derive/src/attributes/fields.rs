use syn::{Attribute, Expr};

use crate::{
    error::Error,
    parse::fields::{parse_field_attributes, FieldAttr, FieldAttrKind}
};

#[derive(Default, Debug)]
pub struct FieldAttributes {
    pub prompt: Option<String>,
    pub validation: Option<Expr>,
}

impl FieldAttributes {
    pub fn parse(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut attributes = Self::default();
        attributes.fill_attributes(attrs)?;

        Ok(attributes)
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
                self.prompt = Some(prompt.value());
            }
            FieldAttrKind::Validation(expr) => {
                self.validation = Some(expr);
            }
        }

        Ok(())
    }
}
