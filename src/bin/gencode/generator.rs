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
            //! This file is generated. Do not edit.
            use iso_10303::step::*;
            pub struct Unimplemented {}
            #( #declarations )*
        };
        code.to_string()
    }

    fn type_name(&self, data_type: &DataType, optional: bool) -> TokenStream {
        let type_token = match data_type {
            DataType::Number => quote! {f64},
            DataType::Integer => quote! {i64},
            DataType::Real { .. } => quote! {f64},
            DataType::Boolean => quote! {bool},
            DataType::Logical => quote! {Option<bool>},
            DataType::String { .. } => quote! {String},
            DataType::TypeRef { name } => {
                if self.entities.contains(name) {
                    let type_name = format_ident!("I{}", name.to_camel_case());
                    quote! {&'a dyn #type_name}
                } else {
                    let type_name = format_ident!("{}", name.to_camel_case());
                    quote! {#type_name}
                }
            }
            DataType::Set { base_type, .. } => {
                let item_type = self.type_name(base_type, false);
                quote!(std::collections::HashSet<#item_type>)
            }
            _ => quote! {Unimplemented},
        };

        if optional {
            quote! { Option<#type_token>}
        } else {
            type_token
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
                let default_value = &names[0];
                let convertions = names.iter().map(|value| {
                    let upper = value.to_string().to_ascii_uppercase();
                    quote! {
                        #upper => #ident::#value,
                    }
                });
                quote! {
                    pub enum #ident {
                        #( #names, )*
                    }
                    impl Default for #ident {
                        fn default() -> Self {
                            #ident::#default_value
                        }
                    }
                    impl From<String> for #ident {
                        fn from(value: String) -> Self {
                            match value.as_str() {
                                #( #convertions )*
                                _ => panic!("{} is not a valid value", value),
                            }
                        }
                    }
                    impl From<Parameter> for #ident {
                        fn from(parameter: Parameter) -> Self {
                            match parameter {
                                Parameter::UnTypedParameter(untyped_parameter) => match untyped_parameter {
                                    UnTypedParameter::EnumValue(value) => value.into(),
                                    _ => panic!("parameter is not an enum value"),
                                },
                                Parameter::OmittedParameter => #ident::default(),
                                _ => panic!("parameter is not an enum value"),
                            }
                        }
                    }
                }
            }
            DataType::Array { bound, base_type, .. } => {
                let data_type = self.type_name(base_type, false);
                let _size = bound.end - bound.start + 1;
                quote! {
                    type #ident = Vec<#data_type>;
                }
            }
            _ => {
                let data_type = self.type_name(&type_def.underlying_type, false);
                quote! {
                    type #ident = #data_type;
                }
            }
        }
    }

    fn gen_entity_def(&self, entity: &Entity) -> TokenStream {
        let attributes = self.get_entity_attributes(&entity.name);
        let attrs = attributes
            .iter()
            .map(|attr| {
                let ident = format_ident!("{}", attr.name.to_snake_case());
                let data_type = self.type_name(&attr.data_type, attr.optional);
                quote! {
                    #ident: #data_type,
                }
            })
            .collect::<Vec<_>>();
        let has_lifetime = attrs.iter().any(|attr| attr.to_string().contains("'a"));
        let life_time = if has_lifetime {
            quote! { <'a> }
        } else {
            quote! {}
        };

        let trait_ident = format_ident!("I{}", entity.name.to_camel_case());
        let supertypes = entity
            .supertypes
            .iter()
            .map(|name| format_ident!("I{}", name.to_camel_case()))
            .collect::<Vec<_>>();
        let fields = entity.attributes.iter().map(|attr| {
            let ident = format_ident!("{}", attr.name.to_snake_case());
            let data_type = self.type_name(&attr.data_type, attr.optional);
            quote! {
                fn #ident(&self) -> &#data_type;
            }
        });
        let trait_code = quote! {
            pub trait #trait_ident#life_time: #( #supertypes ),*  {
                #( #fields )*
            }
        };
        if entity.is_abstract {
            trait_code
        } else {
            let ident = format_ident!("{}", entity.name.to_camel_case());
            let struct_code = quote! {
                #[derive(Default)]
                pub struct #ident#life_time {
                    #( #attrs )*
                }
            };
            let impls = self.get_entity_supertypes(&entity.name).into_iter().map(|entity| {
                let trait_ident = format_ident!("I{}", entity.name.to_camel_case());
                let fields = entity.attributes.iter().map(|attr| {
                    let field = format_ident!("{}", attr.name.to_snake_case());
                    let data_type = self.type_name(&attr.data_type, attr.optional);
                    quote! {
                        fn #field(&self) -> &#data_type {
                            &self.#field
                        }
                    }
                });
                quote! {
                    impl#life_time #trait_ident for #ident#life_time {
                        #( #fields )*
                    }
                }
            });
            let set_fields = attributes.iter().enumerate().map(|(index, attr)| {
                let field = format_ident!("{}", attr.name.to_snake_case());
                if attr.optional {
                    quote! {
                        #index => entity.#field = if parameter.is_null() {None} else {Some(parameter.into())},
                    }
                } else {
                    quote! {
                        #index => entity.#field = parameter.into(),
                    }
                }
            });
            quote! {
                #trait_code
                #struct_code
                #( #impls )*
                impl #ident#life_time {
                    pub fn form_parameters(parameters: Vec<Parameter>) -> Self {
                        let mut entity = #ident::default();

                        for (index, parameter) in parameters.into_iter().enumerate() {
                            match index {
                                #( #set_fields )*
                                _ => {}
                            }
                        }

                        entity
                    }
                }
            }
        }
    }
}
