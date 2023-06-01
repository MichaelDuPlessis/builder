use syn;

pub fn create_builder_name(ast: &syn::DeriveInput) -> syn::Ident {
    // getting name of struct
    let struct_name = &ast.ident;
    // creating builder struct
    // getting name of the builder struct and preventing name conflicts
    syn::Ident::new(&format!("{struct_name}Builder"), struct_name.span())
}

pub type NameTypeIterator<'a> = Box<
    dyn Iterator<
            Item = (
                Option<&'a syn::Ident>,
                &'a syn::Type,
                &'a Vec<syn::Attribute>,
            ),
        > + 'a,
>;

pub fn name_type_iter<'a>(
    punc: &'a syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> NameTypeIterator<'a> {
    Box::new(
        punc.iter()
            .map(|field| (field.ident.as_ref(), &field.ty, &field.attrs)),
    )
}

pub fn is(ty: &syn::Type, wrapper: impl Into<&'static str>) -> Option<&syn::Type> {
    let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = ty else {
        return None;
    };

    let Some(segment) = segments.first() else {
        return None;
    };

    let wrapper: &str = wrapper.into();
    if segment.ident != wrapper {
        return None;
    }

    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {ref args, ..}) = segment.arguments else {
        return None;
    };

    let Some(generic_arg) = args.first() else {
        return None;
    };

    if let syn::GenericArgument::Type(ty) = generic_arg {
        Some(ty)
    } else {
        None
    }
}
