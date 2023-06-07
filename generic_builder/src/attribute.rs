use syn::{self, Token};

// the different kind of attributes there re
#[derive(Debug)]
pub enum BuilderAttribute {
    Auto(AutoAttribute),
    Manual(ManualAttribute),
}

impl BuilderAttribute {
    pub fn new(field: &syn::Attribute) -> Self {
        let syn::Meta::List (syn::MetaList { ref tokens, path: syn::Path {ref segments, ..},..}) = field.meta else {
            panic!("Invalid attribute")
        };
        let Some(syn::PathSegment {ident, ..}) = segments.first() else {
            panic!("Invalid attribute")
        };

        if ident == "auto" {
            let ast = syn::parse2(tokens.clone()).unwrap();
            Self::Auto(ast)
        } else if ident == "manual" {
            let ast = syn::parse2(tokens.clone()).unwrap();
            Self::Manual(ast)
        } else {
            panic!("Cannot get there")
        }
    }
}

// used for when the mutliple attribute is used
#[derive(Debug)]
pub struct ManualAttribute {
    pub single: AutoAttribute,
    pub types: syn::punctuated::Punctuated<syn::Type, Token![,]>,
}

impl syn::parse::Parse for ManualAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let single = input.parse()?;
        let types = input.parse_terminated(syn::Type::parse, Token![,])?;

        Ok(Self { single, types })
    }
}

// used for when the single attribute is used
#[derive(Debug)]
pub struct AutoAttribute {
    pub func_name: syn::Ident,
    pub method: syn::Ident,
}

impl syn::parse::Parse for AutoAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let func_name = input.parse::<syn::Ident>()?;
        input.parse::<Token![,]>()?;
        let method = input.parse::<syn::Ident>()?;
        // if there is not a trailing comma
        if input.parse::<Token![,]>().is_err() {
            input.parse::<syn::parse::Nothing>()?;
        }

        Ok(Self { func_name, method })
    }
}
