use syn::{self, Token};

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
    pub func_name: syn::Ident,
    pub method: syn::Ident,
}

impl syn::parse::Parse for SingleAttribute {
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
