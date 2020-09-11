use heck::*;
use iso_10303::express::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

pub struct Generator {
    types: HashMap<String, TypeDef>,
    entities: HashMap<String, Entity>,
    entity_infos: HashMap<String, EntityInfo>,
}

struct EntityInfo {
    type_name: String,
    has_lifetime: bool,
    attributes: Vec<Attribute>,
}

impl EntityInfo {
    pub fn trait_name(&self) -> TokenStream {
        let trait_name = format_ident!("I{}", self.type_name);
        if self.has_lifetime {
            quote! {#trait_name<'a>}
        } else {
            quote! {#trait_name}
        }
    }
}

fn collect_types(schema: Schema) -> (HashMap<String, TypeDef>, HashMap<String, Entity>) {
    let mut types = HashMap::<String, TypeDef>::new();
    let mut entities = HashMap::<String, Entity>::new();

    for declaration in schema.declarations {
        match declaration {
            Declaration::TypeDef(type_def) => {
                types.insert(type_def.name.to_string(), type_def);
            }
            Declaration::Entity(entity) => {
                entities.insert(entity.name.to_string(), entity);
            }
            _ => {}
        }
    }
    (types, entities)
}

fn get_entity_attributes(entities: &HashMap<String, Entity>, name: &str) -> Vec<Attribute> {
    let mut attributes = Vec::new();
    if let Some(entity) = entities.get(name) {
        for parent in &entity.supertypes {
            attributes.extend(get_entity_attributes(entities, parent));
        }
        attributes.extend(entity.attributes.clone());
    }
    attributes
}

fn has_entity_ref(entities: &HashMap<String, Entity>, data_type: &DataType) -> bool {
    match data_type {
        DataType::TypeRef { name } => entities.contains_key(name),
        DataType::Set { base_type, .. } => has_entity_ref(entities, base_type),
        _ => false,
    }
}

fn collect_entity_infos(entities: &HashMap<String, Entity>) -> HashMap<String, EntityInfo> {
    let mut entity_infos = HashMap::new();
    for name in entities.keys() {
        let attributes = get_entity_attributes(entities, name);
        let has_lifetime = attributes.iter().any(|attr| has_entity_ref(entities, &attr.data_type));
        let type_name = name.to_camel_case();
        // let trait_name = format!("I{}{}", type_name)
        entity_infos.insert(
            name.to_string(),
            EntityInfo {
                type_name,
                has_lifetime,
                attributes,
            },
        );
    }
    entity_infos
}

impl Generator {
    pub fn new(schema: Schema) -> Generator {
        let (types, entities) = collect_types(schema);
        let entity_infos = collect_entity_infos(&entities);
        Generator {
            types,
            entities,
            entity_infos,
        }
    }

    fn get_entity_supertypes(&self, name: &str) -> Vec<&Entity> {
        let entity = &self.entities[name];
        let mut supertypes = Vec::new();
        for parent in &entity.supertypes {
            supertypes.extend(self.get_entity_supertypes(parent));
        }
        supertypes.push(entity);
        supertypes
    }

    pub fn gencode(&self) -> String {
        let type_defs = self.types.values().map(|type_def| self.gen_type_def(type_def));
        let entity_defs = self
            .entities
            .values()
            .map(|entity| self.gen_entity_def(entity, &self.entity_infos[&entity.name]));

        let code = quote! {
            //! This file is generated. Do not edit.
            use iso_10303::step::*;
            pub struct Unimplemented {}
            #( #type_defs )*
            #( #entity_defs )*
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
                if let Some(entity_info) = self.entity_infos.get(name) {
                    let trait_name = entity_info.trait_name();
                    quote! {&'a dyn #trait_name}
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

    fn gen_entity_def(&self, entity: &Entity, entity_info: &EntityInfo) -> TokenStream {
        let life_time = if entity_info.has_lifetime {
            quote! { <'a> }
        } else {
            quote! {}
        };

        let trait_ident = format_ident!("I{}", entity_info.type_name);
        let supertypes = entity
            .supertypes
            .iter()
            .map(|name| self.entity_infos[name].trait_name())
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
            let ident = format_ident!("{}", entity_info.type_name);
            let attrs = entity_info
                .attributes
                .iter()
                .map(|attr| {
                    let ident = format_ident!("{}", attr.name.to_snake_case());
                    let data_type = self.type_name(&attr.data_type, attr.optional);
                    quote! {
                        #ident: #data_type,
                    }
                })
                .collect::<Vec<_>>();
            let struct_code = quote! {
                #[derive(Default)]
                pub struct #ident#life_time {
                    #( #attrs )*
                }
            };
            let impls = self.get_entity_supertypes(&entity.name).into_iter().map(|entity| {
                let trait_name = self.entity_infos[&entity.name].trait_name();
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
                    impl#life_time #trait_name for #ident#life_time {
                        #( #fields )*
                    }
                }
            });
            let set_fields = entity_info.attributes.iter().enumerate().map(|(index, attr)| {
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
                impl#life_time #ident#life_time {
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
