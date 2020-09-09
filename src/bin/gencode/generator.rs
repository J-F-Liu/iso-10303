use heck::*;
use iso_10303::express::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;

pub struct Generator {
    schema: Schema,
    types: HashSet<String>,
    entities: HashSet<String>,
}

fn collect_types(schema: &Schema) -> (HashSet<String>, HashSet<String>) {
    let mut types = HashSet::<String>::new();
    let mut entities = HashSet::<String>::new();

    for declaration in &schema.declarations {
        match declaration {
            Declaration::Type { name, .. } => {
                types.insert(name.to_string());
            }
            Declaration::Entity { name, .. } => {
                entities.insert(name.to_string());
            }
            _ => {}
        }
    }
    (types, entities)
}

impl Generator {
    pub fn new(schema: Schema) -> Generator {
        let (types, entities) = collect_types(&schema);
        Generator {
            schema,
            types,
            entities,
        }
    }

    pub fn gencode(&self) -> String {
        let declarations = self
            .schema
            .declarations
            .iter()
            .map(|declaration| match declaration {
                Declaration::Type {
                    name, underlying_type, ..
                } => self.gen_type_def(name, underlying_type),
                Declaration::Entity {
                    name,
                    is_abstract,
                    attributes,
                    ..
                } => self.gen_entity_def(name, *is_abstract, attributes),
                _ => quote! {},
            })
            .collect::<Vec<_>>();

        let code = quote! {
            pub struct Unimplemented {}
            #( #declarations )*
        };
        code.to_string()
    }

    fn type_name(&self, data_type: &DataType) -> TokenStream {
        match data_type {
            DataType::Number => quote! {f64},
            DataType::Integer => quote! {i64},
            DataType::Real { .. } => quote! {f64},
            DataType::Boolean => quote! {bool},
            DataType::Logical => quote! {Option<bool>},
            DataType::String { .. } => quote! {String},
            DataType::TypeRef { name } => {
                if self.entities.contains(name) {
                    let type_name = format_ident!("I{}", name.to_camel_case());
                    quote! {Box<dyn #type_name>}
                } else {
                    let type_name = format_ident!("{}", name.to_camel_case());
                    quote! {#type_name}
                }
            }
            DataType::Set { base_type, .. } => {
                let item_type = self.type_name(base_type);
                quote!(std::collections::HashSet<#item_type>)
            }
            _ => quote! {Unimplemented},
        }
    }

    fn gen_type_def(&self, name: &String, underlying_type: &DataType) -> TokenStream {
        let ident = format_ident!("{}", name.to_camel_case());
        match underlying_type {
            DataType::Enum { values } => {
                let names = values
                    .iter()
                    .map(|value| format_ident!("{}", value.to_camel_case()))
                    .collect::<Vec<_>>();
                quote! {
                    pub enum #ident {
                        #( #names, )*
                    }
                }
            }
            DataType::Array { bound, base_type, .. } => {
                let data_type = self.type_name(base_type);
                let size = bound.end - bound.start + 1;
                quote! {
                    type #ident = [#data_type; #size];
                }
            }
            _ => {
                let data_type = self.type_name(&underlying_type);
                quote! {
                    type #ident = #data_type;
                }
            }
        }
    }

    fn gen_entity_def(&self, name: &String, is_abstract: bool, attributes: &Vec<Attribute>) -> TokenStream {
        let trait_ident = format_ident!("I{}", name.to_camel_case());
        let fields = attributes
            .iter()
            .map(|attr| {
                let ident = format_ident!("{}", attr.name.to_snake_case());
                let data_type = self.type_name(&attr.data_type);
                quote! {
                    fn #ident(&self) -> #data_type;
                }
            })
            .collect::<Vec<_>>();
        let trait_code = quote! {
            pub trait #trait_ident {
                #( #fields )*
            }
        };
        if is_abstract {
            trait_code
        } else {
            let ident = format_ident!("{}", name.to_camel_case());
            let attrs = attributes
                .iter()
                .map(|attr| {
                    let ident = format_ident!("{}", attr.name.to_snake_case());
                    let data_type = self.type_name(&attr.data_type);
                    quote! {
                        #ident: #data_type,
                    }
                })
                .collect::<Vec<_>>();
            let struct_code = quote! {
                pub struct #ident {
                    #( #attrs )*
                }
            };
            let impl_code = quote! {
                impl #trait_ident for #ident {
                }
            };
            quote! {
                #trait_code
                #struct_code
            }
        }
    }
}
