use syn;

#[derive(Debug)]
struct BuilderAttribute {
    ident: syn::Ident,
}

impl syn::parse::Parse for BuilderAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
        })
    }
}
