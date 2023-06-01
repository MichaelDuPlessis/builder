use syn::{self, parse_macro_input};

#[derive(Debug)]
pub enum BuilderAttribute {
    Single(SingleAttribute),
}

impl BuilderAttribute {
    pub fn new(field: &syn::Attribute) -> Self {
        let syn::Meta::List (syn::MetaList { ref tokens, path: syn::Path {ref segments, ..},..}) = field.meta else {
            panic!("Invalid attribute")
        };
        let Some(syn::PathSegment {ident, ..}) = segments.first() else {
            panic!("Invalid attribute")
        };

        if ident == "single" {
            let ast = syn::parse2(tokens.clone()).unwrap();
            Self::Single(ast)
        } else {
            panic!("Cannot get there")
        }
    }
}

#[derive(Debug)]
pub struct SingleAttribute {
    pub ident: syn::Ident,
}

impl syn::parse::Parse for SingleAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
        })
    }
}
