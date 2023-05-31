mod builder_attribute;
mod helper;

use helper::NameTypeIterator;
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Builder, attributes(single))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", ast);

    let builder = create_builder(&ast);

    quote! {
        #builder
    }
    .into()
}

fn create_fields(name_type: NameTypeIterator) -> proc_macro2::TokenStream {
    let fields = name_type.map(|(name, ty)| {
        let ty = if let Some(ty) = helper::is(ty, "Option") {
            ty
        } else {
            ty
        };

        quote! {
            #name: std::option::Option<#ty>
        }
    });

    quote! {
        #(#fields),*
    }
}

fn create_funcs(name_type: NameTypeIterator) -> proc_macro2::TokenStream {
    let funcs = name_type.map(|(name, ty)| {
        let ty = if let Some(ty) = helper::is(ty, "Option") {
            ty
        } else {
            ty
        };

        quote! {
            pub fn #name(&mut self, #name: impl Into<#ty>) -> &mut Self {
                self.#name = Some(#name.into());
                self
            }
        }
    });

    quote! {
        #(#funcs)*
    }
}

fn create_builder(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data else {
        unimplemented!()
    };

    // getting names of structs
    let struct_name = &ast.ident;
    let builder_name = helper::create_builder_name(ast);

    // getting builder structs fields
    let fields = create_fields(helper::name_type_iter(named));
    // getting builder structs methods
    let funcs = create_funcs(helper::name_type_iter(named));
    // creating build methods
    let build_methods = helper::name_type_iter(named).map(|(name, ty)| {
        if helper::is(ty, "Option").is_some() {
            quote! {
                #name: self.#name.clone()
            }
        } else {
            quote! {
                #name: self.#name.clone().ok_or(concat!(stringify!(#name), " was not set"))?
            }
        }
    });
    // for when we want to consume the builder
    let build_methods_consume = helper::name_type_iter(named).map(|(name, ty)| {
        if helper::is(ty, "Option").is_some() {
            quote! {
                #name: self.#name
            }
        } else {
            quote! {
                #name: self.#name.ok_or(concat!(stringify!(#name), " was not set"))?
            }
        }
    });

    quote! {
        struct #builder_name {
            #fields
        }

        impl #builder_name {
            #funcs

            pub fn build(&self) -> std::result::Result<#struct_name, std::boxed::Box<dyn std::error::Error>> {
                Ok(#struct_name {
                    #(#build_methods),*
                })
            }

            pub fn build_consume(self) -> std::result::Result<#struct_name, std::boxed::Box<dyn std::error::Error>> {
                Ok(#struct_name {
                    #(#build_methods_consume),*
                })
            }
        }
    }
}
