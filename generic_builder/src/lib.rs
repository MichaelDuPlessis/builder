mod attribute;
mod helper;

use helper::NameType;
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::parse_macro_input;

use crate::attribute::BuilderAttribute;

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

fn create_fields(name_type: &NameType) -> proc_macro2::TokenStream {
    let fields = name_type.iter().map(|(name, ty, _)| {
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

fn create_funcs(name_type: &NameType) -> proc_macro2::TokenStream {
    let funcs = name_type.iter().map(|(name, ty, attrs)| {
        let ty = if let Some(ty) = helper::is(ty, "Option") {
            ty
        } else {
            ty
        };

        let mut func = quote! {
            pub fn #name(&mut self, #name: impl std::convert::Into<#ty>) -> &mut Self {
                self.#name = Some(#name.into());
                self
            }
        };
        let mut extra_funcs = Vec::new();

        if !attrs.is_empty() {
            for attr in attrs {
                match attr {
                    BuilderAttribute::Single(single) => {
                        let new_name = &single.name;
                        let method = &single.method;
                        let (wrapper, inner_ty) = helper::inner_type(ty).expect("Invalid attribute");

                        if &single.name == name.unwrap() {
                            func = quote! {
                                pub fn #new_name(&mut self, #new_name: impl std::convert::Into<#inner_ty>) -> &mut Self {
                                    if self.#name.is_none() {
                                        self.#name = std::option::Option::Some(#wrapper::new());
                                    }

                                    self.#name.as_mut().unwrap().#method(#new_name.into());
                                    self
                                }
                            };
                        } else {
                            extra_funcs.push(quote! {
                                pub fn #new_name(&mut self, #new_name: impl std::convert::Into<#inner_ty>) -> &mut Self {
                                    if self.#name.is_none() {
                                        self.#name = std::option::Option::Some(#wrapper::new());
                                    }

                                    self.#name.as_mut().unwrap().#method(#new_name.into());
                                    self
                                }
                            });
                        }
                    }
                }
            }
        }

        quote! {
            #(#extra_funcs)*
            #func
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
    let name_type = helper::name_type(named);

    // getting builder structs fields
    let fields = create_fields(&name_type);
    // getting builder structs methods
    let funcs = create_funcs(&name_type);
    // creating build methods
    let build_methods = name_type.iter().map(|(name, ty, _)| {
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
    let build_methods_consume = name_type.iter().map(|(name, ty, _)| {
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
    let builder_construct = name_type.iter().map(|(name, _, _)| {
        quote! {
            #name: None
        }
    });

    quote! {
        pub struct #builder_name {
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

        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_construct),*
                }
            }
        }
    }
}
