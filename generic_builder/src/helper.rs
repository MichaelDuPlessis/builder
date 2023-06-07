use crate::attribute::BuilderAttribute;
use syn;

pub fn create_builder_name(ast: &syn::DeriveInput) -> syn::Ident {
    // getting name of struct
    let struct_name = &ast.ident;
    // creating builder struct
    // getting name of the builder struct and preventing name conflicts
    syn::Ident::new(&format!("{struct_name}Builder"), struct_name.span())
}

pub type NameType<'a> = Vec<(Option<&'a syn::Ident>, &'a syn::Type, Vec<BuilderAttribute>)>;

pub fn name_type<'a>(
    punc: &'a syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> NameType<'a> {
    punc.iter()
        .map(|field| {
            (
                field.ident.as_ref(),
                &field.ty,
                field
                    .attrs
                    .iter()
                    .map(|attr| BuilderAttribute::new(attr))
                    .collect(),
            )
        })
        .collect()
}

pub fn is(ty: &syn::Type, wrapper: impl Into<&'static str>) -> Option<Vec<&syn::Type>> {
    if let Some(inner) = inner_types(ty) {
        if inner.0 == wrapper.into() {
            return Some(inner.1);
        }
    }

    None
}

pub fn inner_types(ty: &syn::Type) -> Option<(&syn::Ident, Vec<&syn::Type>)> {
    let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = ty else {
        return None;
    };

    let Some(segment) = segments.first() else {
        return None;
    };

    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {ref args, ..}) = segment.arguments else {
        return None;
    };

    if args.is_empty() {
        return None;
    }

    let tys = args
        .iter()
        .map(|generic| {
            let syn::GenericArgument::Type(ty) = generic else {
            panic!("Not a generic argument")
        };

            ty
        })
        .collect();

    Some((&segment.ident, tys))
}
