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
            Declaration::TypeDef(type_def) => {
                types.insert(type_def.name.to_string());
            }
            Declaration::Entity(entity) => {
                entities.insert(entity.name.to_string());
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

    fn get_entity(&self, name: &str) -> Option<&Entity> {
        for declaration in &self.schema.declarations {
            match declaration {
                Declaration::Entity(entity) => {
                    if entity.name == name {
                        return Some(entity);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn get_entity_supertypes(&self, name: &str) -> Vec<&Entity> {
        let entity = self.get_entity(name).unwrap();
        let mut supertypes = Vec::new();
        for parent in &entity.supertypes {
            supertypes.extend(self.get_entity_supertypes(parent));
        }
        supertypes.push(entity);
        supertypes
    }

    fn get_entity_attributes(&self, name: &str) -> Vec<Attribute> {
        let entity = self.get_entity(name).unwrap();
        let mut attributes = Vec::new();
        for parent in &entity.supertypes {
            attributes.extend(self.get_entity_attributes(parent));
        }
        attributes.extend(entity.attributes.clone());
        attributes
    }

    pub fn gencode(&self) -> String {
        let declarations = self
            .schema
            .declarations
            .iter()
            .map(|declaration| match declaration {
                Declaration::TypeDef(type_def) => self.gen_type_def(type_def),
                Declaration::Entity(entity) => self.gen_entity_def(entity),
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

    fn gen_type_def(&self, type_def: &TypeDef) -> TokenStream {
        let ident = format_ident!("{}", type_def.name.to_camel_case());
        match &type_def.underlying_type {
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
                let data_type = self.type_name(&type_def.underlying_type);
                quote! {
                    type #ident = #data_type;
                }
            }
        }
    }

    fn gen_entity_def(&self, entity: &Entity) -> TokenStream {
        let trait_ident = format_ident!("I{}", entity.name.to_camel_case());
        let supertypes = entity
            .supertypes
            .iter()
            .map(|name| format_ident!("I{}", name.to_camel_case()))
            .collect::<Vec<_>>();
        let fields = entity.attributes.iter().map(|attr| {
            let ident = format_ident!("{}", attr.name.to_snake_case());
            let data_type = self.type_name(&attr.data_type);
            quote! {
                fn #ident(&self) -> &#data_type;
            }
        });
        let trait_code = quote! {
            pub trait #trait_ident: #( #supertypes ),*  {
                #( #fields )*
            }
        };
        if entity.is_abstract {
            trait_code
        } else {
            let ident = format_ident!("{}", entity.name.to_camel_case());
            let attributes = self.get_entity_attributes(&entity.name);
            let attrs = attributes.iter().map(|attr| {
                let ident = format_ident!("{}", attr.name.to_snake_case());
                let data_type = self.type_name(&attr.data_type);
                quote! {
                    #ident: #data_type,
                }
            });
            let struct_code = quote! {
                pub struct #ident {
                    #( #attrs )*
                }
            };
            let impls = self.get_entity_supertypes(&entity.name).into_iter().map(|entity| {
                let trait_ident = format_ident!("I{}", entity.name.to_camel_case());
                let fields = entity.attributes.iter().map(|attr| {
                    let ident = format_ident!("{}", attr.name.to_snake_case());
                    let data_type = self.type_name(&attr.data_type);
                    quote! {
                        fn #ident(&self) -> &#data_type {
                            &self.#ident
                        }
                    }
                });
                quote! {
                    impl #trait_ident for #ident {
                        #( #fields )*
                    }
                }
            });
            quote! {
                #trait_code
                #struct_code
                #( #impls )*
            }
        }
    }
}
