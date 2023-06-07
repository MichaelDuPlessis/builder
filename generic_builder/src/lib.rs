mod attribute;
mod helper;

use helper::NameType;
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::parse_macro_input;
use crate::attribute::{BuilderAttribute, AutoAttribute, ManualAttribute};

#[proc_macro_derive(Builder, attributes(auto, manual))]
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
            // since ty is option it has only one generic argument
            ty[0]
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
            // since ty is option it has only one generic argument
            ty[0]
        } else {
            ty
        };

        let mut func = quote! {
            pub fn #name(mut self, #name: impl std::convert::Into<#ty>) -> Self {
                self.#name = Some(#name.into());
                self
            }
        };
        let mut extra_funcs = Vec::new();

        if !attrs.is_empty() {
            for attr in attrs {
                match attr {
                    BuilderAttribute::Auto(auto) => {
                        let AutoAttribute {
                            func_name,
                            method,
                        } = auto;

                        let (_, inner_tys) = helper::inner_types(ty).expect("Invalid attribute");
                        let params = (0..inner_tys.len()).map(|i| {
                           syn::Ident::new(&format!("{}{}", func_name, i), func_name.span())
                        });

                        let params_ty = params.clone().zip(inner_tys).map(|(param, ty)| {
                            quote! {
                                #param: impl std::convert::Into<#ty>
                            }
                        });

                        let single_func = quote! {
                                pub fn #func_name(mut self, #(#params_ty),*) -> Self {
                                    if self.#name.is_none() {
                                        self.#name = std::option::Option::Some(std::default::Default::default());
                                    }

                                    self.#name.as_mut().unwrap().#method(#(#params.into()),*);
                                    self
                                }
                            }; 

                        // can unwrap since all fields have names in named struct
                        if func_name == name.unwrap() {
                            func = single_func;
                        } else {
                            extra_funcs.push(single_func);
                        }
                    }
                    BuilderAttribute::Manual(manual) => {
                        let ManualAttribute {
                            single: AutoAttribute {
                                func_name,
                                method,
                            },
                            types,
                        } = manual;

                        let params = (0..types.len()).map(|i|
                           syn::Ident::new(&format!("{}{}", func_name, i), func_name.span())
                        );

                        let param_types = params.clone().zip(types).map(|(param, ty)| quote! {
                            #param: impl std::convert::Into<#ty>
                        });

                        // println!("{:#?}", param_types);

                        let single_func = quote! {
                                pub fn #func_name(mut self, #(#param_types),*) -> Self {
                                    if self.#name.is_none() {
                                        self.#name = std::option::Option::Some(std::default::Default::default());
                                    }

                                    self.#name.as_mut().unwrap().#method(#(#params.into()),*);
                                    self
                                }
                            }; 

                        // can unwrap since all fields have names in named struct
                        if func_name == name.unwrap() {
                            func = single_func;
                        } else {
                            extra_funcs.push(single_func);
                        }
                    },
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
        panic!("Builder only accepts named structs")
    };

    // getting generics
    let syn::DeriveInput {
        generics: syn::Generics {
            params,
            ..
        },
        ..
    } = ast;
    let generics = params.iter().map(|p| if let syn::GenericParam::Type(ty) = p {
        ty
    } else {
        panic!("Not a valid generic argument.")
    }).collect::<Vec<_>>();
    // since we don't always want the trait bound
    let generics_no_bounds = params.iter().map(|p| if let syn::GenericParam::Type(ty) = p {
        &ty.ident
    } else {
        panic!("Not a valid generic argument.")
    }).collect::<Vec<_>>();
    // since builder can only be reusable if all members implement clone
    let generics_clone_bound = params.iter().map(|p| if let syn::GenericParam::Type(ty) = p {
        let ident = &ty.ident;
        if ty.bounds.is_empty() {
            quote! {
                #ident: std::clone::Clone
            }
        } else {
            quote! {
                #ty + std::clone::Clone
            }
        }
    } else {
        panic!("Not a valid generic argument.")
    }).collect::<Vec<_>>();

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
            #name: std::option::Option::None
        }
    });

    quote! {
        pub struct #builder_name<#(#generics),*> {
            #fields
        }

        impl<#(#generics),*> #builder_name<#(#generics_no_bounds),*> {
            #funcs


            pub fn build_consume(self) -> std::result::Result<#struct_name<#(#generics_no_bounds),*>, std::boxed::Box<dyn std::error::Error>> {
                Ok(#struct_name {
                    #(#build_methods_consume),*
                })
            }
        }

        impl<#(#generics_clone_bound),*> #builder_name<#(#generics_no_bounds),*> {
            pub fn build(&self) -> std::result::Result<#struct_name<#(#generics_no_bounds),*>, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#struct_name {
                    #(#build_methods),*
                })
            }
        }

        impl<#(#generics),*> #struct_name<#(#generics_no_bounds),*> {
            pub fn builder() -> #builder_name<#(#generics_no_bounds),*> {
                #builder_name{
                    #(#builder_construct),*
                }
            }
        }
    }
}
