use syn::Attribute;

use crate::{
    error::Error,
    parse::types::{parse_type_attributes, TypeAttr, TypeAttrKind}
};

#[derive(Default, Debug)]
pub struct TypeAttributes {
    pub prefix: Option<String>,
    pub begin_msg: Option<String>,
    pub closing_msg: Option<String>,
}

impl TypeAttributes {
    pub fn parse(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut attributes = Self::default();
        attributes.fill_attributes(attrs)?;

        Ok(attributes)
    }

    fn fill_attributes(&mut self, attrs: &[Attribute]) -> Result<(), Error> {
        for attrs in parse_type_attributes(attrs) {
            let attrs = attrs?;
            for attr in attrs {
                self.insert_attribute(attr)?;
            }
        }

        Ok(())
    }

    fn insert_attribute(&mut self, attr: TypeAttr) -> Result<(), Error> {
        match attr.kind {
            TypeAttrKind::Prefix(prefix) => {
                self.prefix = Some(prefix.value());
            }
            TypeAttrKind::BeginMsg(msg) => {
                self.begin_msg = Some(msg.value());
            }
            TypeAttrKind::CloseMsg(msg) => {
                self.closing_msg = Some(msg.value());
            }
        }
        Ok(())
    }
}
