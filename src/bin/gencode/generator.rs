use heck::*;
use iso_10303::express::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

pub struct Generator {
    schema: Schema,
    type_infos: HashMap<String, TypeInfo>,
    entity_infos: HashMap<String, EntityInfo>,
    hashable_types: HashSet<String>,
}

struct EntityInfo {
    name: String,
    attributes: Vec<Attribute>,
    supertypes: Vec<String>,
}

struct TypeInfo {
    is_entity: bool,
    underlying_type: DataType,
}

impl EntityInfo {
    pub fn type_name(&self) -> TokenStream {
        let type_name = format_ident!("{}", self.name);
        quote! {#type_name}
    }
    pub fn trait_name(&self) -> TokenStream {
        let trait_name = format_ident!("I{}", self.name);
        quote! {#trait_name}
    }
}

fn collect_types(schema: &Schema) -> (HashMap<String, TypeInfo>, HashMap<String, EntityInfo>) {
    let mut type_infos = HashMap::<String, TypeInfo>::new();
    let mut entity_infos = HashMap::<String, EntityInfo>::new();

    for declaration in &schema.declarations {
        match declaration {
            Declaration::Entity(entity) => {
                entity_infos.insert(
                    entity.name.to_string(),
                    EntityInfo {
                        name: entity.name.to_camel_case(),
                        attributes: entity
                            .attributes
                            .iter()
                            .map(|attr| Attribute {
                                name: attr.name.to_snake_case(),
                                data_type: attr.data_type.clone(),
                                optional: attr.optional,
                                supertype: attr.supertype.clone(),
                            })
                            .collect(),
                        supertypes: entity.supertypes.clone(),
                    },
                );
            }
            _ => {}
        }
    }

    for declaration in &schema.declarations {
        match declaration {
            Declaration::TypeDef(type_def) => {
                let is_entity = match &type_def.underlying_type {
                    DataType::Select { types } => types.iter().all(|type_name| {
                        entity_infos.contains_key(type_name)
                            || type_infos.get(type_name).map(|info| info.is_entity) == Some(true)
                    }),
                    _ => false,
                };
                type_infos.insert(
                    type_def.name.to_string(),
                    TypeInfo {
                        is_entity,
                        underlying_type: type_def.underlying_type.clone(),
                    },
                );
            }
            _ => {}
        }
    }
    (type_infos, entity_infos)
}

fn collect_hashable_types(
    type_infos: &HashMap<String, TypeInfo>,
    entity_infos: &HashMap<String, EntityInfo>,
) -> HashSet<String> {
    let mut hashable_types = HashSet::new();
    for entity_info in entity_infos.values() {
        for attribute in &entity_info.attributes {
            match &attribute.data_type {
                DataType::Set { base_type, .. } => match &**base_type {
                    DataType::TypeRef { name } => {
                        if type_infos.contains_key(name) {
                            hashable_types.insert(name.clone());
                            collect_inner_hashable_types(&mut hashable_types, type_infos, name);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    hashable_types
}

fn collect_inner_hashable_types(
    hashable_types: &mut HashSet<String>,
    type_infos: &HashMap<String, TypeInfo>,
    name: &str,
) {
    if let Some(type_info) = type_infos.get(name) {
        match &type_info.underlying_type {
            DataType::Select { types } => {
                for type_name in types {
                    if type_infos.contains_key(name) {
                        hashable_types.insert(type_name.clone());
                        collect_inner_hashable_types(hashable_types, type_infos, type_name);
                    }
                }
            }
            _ => {}
        }
    }
}

fn dedup<T, F>(items: &mut Vec<T>, is_equal: F)
where
    F: Fn(&T, &T) -> bool,
{
    let mut current = 0;
    while current < items.len() {
        let mut index = current + 1;
        while index < items.len() {
            if is_equal(&items[index], &items[current]) {
                items.swap_remove(index);
            } else {
                index += 1;
            }
        }
        current += 1;
    }
}

fn is_copy_type(data_type: &DataType) -> bool {
    match data_type {
        DataType::Number => true,
        DataType::Integer => true,
        DataType::Real { .. } => true,
        DataType::Boolean => true,
        DataType::Logical => true,
        _ => false,
    }
}

impl Generator {
    pub fn new(schema: Schema) -> Generator {
        let (type_infos, entity_infos) = collect_types(&schema);
        let hashable_types = collect_hashable_types(&type_infos, &entity_infos);
        Generator {
            schema,
            type_infos,
            entity_infos,
            hashable_types,
        }
    }

    fn get_entity_supertypes(&self, name: &str) -> Vec<&EntityInfo> {
        let entity_info = &self.entity_infos[name];
        let mut supertypes = Vec::new();
        for parent in &entity_info.supertypes {
            supertypes.extend(self.get_entity_supertypes(parent));
        }
        supertypes.push(entity_info);
        dedup(&mut supertypes, |a, b| a.name == b.name);
        supertypes
    }

    fn get_entity_attributes(&self, name: &str) -> Vec<Attribute> {
        let mut attributes: Vec<Attribute> = Vec::new();
        if let Some(entity_info) = self.entity_infos.get(name) {
            for parent in &entity_info.supertypes {
                let mut parent_attributes = self
                    .get_entity_attributes(parent)
                    .into_iter()
                    .filter(|attribute| {
                        !entity_info
                            .attributes
                            .iter()
                            .chain(attributes.iter())
                            .any(|attr| attribute.name == attr.name)
                    })
                    .collect::<Vec<_>>();
                attributes.append(&mut parent_attributes);
            }
            attributes.extend(entity_info.attributes.clone());
        }
        attributes
    }

    pub fn gencode(&self) -> String {
        let declarations = self.schema.declarations.iter().map(|declaration| match declaration {
            Declaration::TypeDef(type_def) => self.gen_type_def(type_def),
            Declaration::Entity(entity) => self.gen_entity_def(entity, &self.entity_infos[&entity.name]),
            _ => quote! {},
        });
        let reader = self.gen_reader();

        let code = quote! {
            //! This file is generated. Do not edit.
            #![allow(dead_code)]
            use iso_10303::step::*;
            use std::collections::HashSet;
            #[derive(Default, Debug)]
            pub struct Unimplemented {}
            #( #declarations )*
            #reader
        };
        code.to_string()
    }

    fn type_name(&self, data_type: &DataType, optional: bool) -> TokenStream {
        let type_token = match data_type {
            DataType::Number => quote! {Real},
            DataType::Integer => quote! {i64},
            DataType::Real { .. } => quote! {Real},
            DataType::Boolean => quote! {bool},
            DataType::Logical => quote! {Option<bool>},
            DataType::String { .. } => quote! {String},
            DataType::TypeRef { name } => {
                if self.entity_infos.contains_key(name)
                    || self.type_infos.get(name).map(|info| info.is_entity) == Some(true)
                {
                    // let trait_name = entity_info.trait_name();
                    // quote! {&'a dyn #trait_name}
                    quote! {EntityRef}
                } else {
                    let type_name = format_ident!("{}", name.to_camel_case());
                    quote! {#type_name}
                }
            }
            DataType::Set { base_type, .. } => {
                let item_type = self.type_name(base_type, false);
                quote!(HashSet<#item_type>)
            }
            DataType::Bag { base_type, .. } => {
                let item_type = self.type_name(base_type, false);
                quote!(Vec<#item_type>)
            }
            DataType::Array { base_type, .. } => {
                let item_type = self.type_name(base_type, false);
                quote!(Vec<#item_type>)
            }
            DataType::List { base_type, .. } => {
                let item_type = self.type_name(base_type, false);
                quote!(Vec<#item_type>)
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
                    #[derive(Eq, PartialEq, Hash, Debug)]
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
            DataType::Bag { base_type, .. } => {
                let data_type = self.type_name(base_type, false);
                quote! {
                    type #ident = Vec<#data_type>;
                }
            }
            DataType::Set { base_type, .. } => {
                let data_type = self.type_name(base_type, false);
                quote! {
                    type #ident = HashSet<#data_type>;
                }
            }
            DataType::Array { base_type, .. } => {
                let data_type = self.type_name(base_type, false);
                quote! {
                    type #ident = Vec<#data_type>;
                }
            }
            DataType::List { base_type, .. } => {
                let data_type = self.type_name(base_type, false);
                quote! {
                    type #ident = Vec<#data_type>;
                }
            }
            DataType::Select { types } => self.gen_select_def(&type_def.name, types),
            _ => {
                let data_type = self.type_name(&type_def.underlying_type, false);
                quote! {
                    type #ident = #data_type;
                }
            }
        }
    }

    fn gen_entity_def(&self, entity: &Entity, entity_info: &EntityInfo) -> TokenStream {
        let trait_name = entity_info.trait_name();
        let supertypes = entity_info
            .supertypes
            .iter()
            .map(|name| self.entity_infos[name].trait_name())
            .collect::<Vec<_>>();
        let fields = entity_info.attributes.iter().map(|attr| {
            let ident = format_ident!("{}", attr.name);
            let data_type = self.type_name(&attr.data_type, attr.optional);
            if is_copy_type(&attr.data_type) {
                quote! {
                    fn #ident(&self) -> #data_type;
                }
            } else {
                quote! {
                    fn #ident(&self) -> &#data_type;
                }
            }
        });
        let trait_code = quote! {
            pub trait #trait_name: #( #supertypes )+*  {
                #( #fields )*
            }
        };
        if entity.is_abstract {
            trait_code
        } else {
            let type_name = entity_info.type_name();
            let attributes = self.get_entity_attributes(&entity.name);
            let fields = attributes
                .iter()
                .map(|attr| {
                    let ident = format_ident!("{}", attr.name);
                    let data_type = self.type_name(&attr.data_type, attr.optional);
                    quote! {
                        #ident: #data_type,
                    }
                })
                .collect::<Vec<_>>();
            let struct_code = quote! {
                #[derive(Default, Debug)]
                pub struct #type_name {
                    #( #fields )*
                }
            };
            let impls = self.get_entity_supertypes(&entity.name).into_iter().map(|supertype| {
                let trait_name = supertype.trait_name();
                let fields = supertype.attributes.iter().map(|attr| {
                    let field = format_ident!("{}", attr.name);
                    let data_type = self.type_name(&attr.data_type, attr.optional);
                    if is_copy_type(&attr.data_type) {
                        if attr.data_type.is_number()
                            && entity.get_attribute(&attr.name).map(|attr| attr.data_type.is_integer()) == Some(true)
                        {
                            quote! {
                                fn #field(&self) -> #data_type {
                                    Real(self.#field as f64)
                                }
                            }
                        } else {
                            quote! {
                                fn #field(&self) -> #data_type {
                                    self.#field
                                }
                            }
                        }
                    } else {
                        quote! {
                            fn #field(&self) -> &#data_type {
                                &self.#field
                            }
                        }
                    }
                });
                quote! {
                    impl #trait_name for #type_name {
                        #( #fields )*
                    }
                }
            });
            let set_fields = attributes.iter().enumerate().map(|(index, attr)| {
                let field = format_ident!("{}", attr.name);
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
                impl #type_name {
                    pub fn form_parameters(parameters: Vec<Parameter>) -> Self {
                        let mut entity = #type_name::default();

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

    fn gen_select_def(&self, type_name: &str, types: &Vec<String>) -> TokenStream {
        let ident = format_ident!("{}", type_name.to_camel_case());
        if types.len() == 1 {
            let variant = format_ident!("{}", types[0].to_camel_case());
            return quote! {
                type #ident = #variant;
            };
        }
        let has_entity = types.iter().any(|name| self.entity_infos.contains_key(name));
        let variants = if has_entity {
            vec![quote! { EntityRef(EntityRef) }]
                .into_iter()
                .chain(types.iter().map(|name| {
                    if let Some(entity_info) = self.entity_infos.get(name) {
                        let type_name = entity_info.type_name();
                        quote! {
                            #type_name(EntityRef)
                        }
                    } else {
                        let type_name = format_ident!("{}", name.to_camel_case());
                        quote! {
                            #type_name(#type_name)
                        }
                    }
                }))
                .collect::<Vec<_>>()
        } else {
            types
                .iter()
                .map(|name| {
                    let type_name = format_ident!("{}", name.to_camel_case());
                    quote! {
                        #type_name(#type_name)
                    }
                })
                .collect::<Vec<_>>()
        };
        let default_value = types
            .iter()
            .next()
            .map(|name| {
                if let Some(entity_info) = self.entity_infos.get(name) {
                    let type_name = entity_info.type_name();
                    quote! {
                        #type_name(EntityRef::default())
                    }
                } else {
                    let type_name = format_ident!("{}", name.to_camel_case());
                    quote! {
                        #type_name(#type_name::default())
                    }
                }
            })
            .unwrap();
        let derive = if self.hashable_types.contains(type_name) {
            quote! { #[derive(Eq, PartialEq, Hash, Debug)] }
        } else {
            quote! { #[derive(Debug)] }
        };
        let convert = if has_entity {
            quote! {
                UnTypedParameter::EntityRef(id) => #ident::EntityRef(EntityRef(id)),
                _ => panic!("parameter is not an instance"),
            }
        } else {
            let variant = format_ident!("{}", types[0].to_camel_case());
            quote! {
                UnTypedParameter::EntityRef(id) => #ident::#variant(#variant::default()),
                _ => panic!("parameter is not an instance"),
            }
        };
        quote! {
            #derive
            pub enum #ident {
                #( #variants, )*
            }
            impl Default for #ident {
                fn default() -> Self {
                    #ident::#default_value
                }
            }
            impl From<Parameter> for #ident {
                fn from(parameter: Parameter) -> Self {
                    match parameter {
                        Parameter::UnTypedParameter(untyped_parameter) => match untyped_parameter {
                            #convert
                        },
                        _ => panic!("parameter is not an instance"),
                    }
                }
            }
        }
    }

    fn gen_reader(&self) -> TokenStream {
        let reader_name = format_ident!("{}Reader", self.schema.name.to_camel_case());
        let read_entities = self
            .schema
            .declarations
            .iter()
            .filter_map(|declaration| match declaration {
                Declaration::Entity(entity) => {
                    if !entity.is_abstract {
                        Some(&entity.name)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .map(|name| {
                let constructor = name.to_uppercase();
                let type_name = self.entity_infos[name].type_name();
                quote! {
                    #constructor => {
                        let entity = #type_name::form_parameters(typed_parameter.parameters);
                        self.add_entity(id, entity);
                    }
                }
            });

        quote! {
            use std::any::{Any, TypeId};
            use std::collections::{BTreeMap, HashMap};

            pub struct #reader_name {
                pub entities: BTreeMap<i64, Box<dyn Any>>,
                pub type_ids: HashMap<TypeId, Vec<i64>>,
            }
            impl #reader_name {
                pub fn new() -> Self {
                    #reader_name {
                        entities: BTreeMap::new(),
                        type_ids: HashMap::new(),
                    }
                }
                pub fn add_entity<T: Any>(&mut self, id: i64, entity: T) {
                    let type_id = entity.type_id();
                    self.entities.insert(id, Box::new(entity));
                    self.type_ids.entry(type_id).or_insert(vec![]).push(id);
                }
                pub fn get_entity<T: Any>(&self, id: i64) -> Option<&T> {
                    self.entities[&id].downcast_ref::<T>()
                }
                pub fn get_entities<T: Any>(&self) -> impl Iterator<Item = &T> {
                    let type_id = TypeId::of::<T>();
                    self.type_ids[&type_id]
                        .iter()
                        .map(move |id| self.entities[id].downcast_ref::<T>().unwrap())
                }
            }

            impl StepReader for #reader_name {
                fn read_simple_entity(&mut self, id: i64, typed_parameter: TypedParameter) {
                    match typed_parameter.type_name.as_str() {
                        #( #read_entities )*
                        _ => println!("{} is not implemented", typed_parameter.type_name),
                    }
                }
            }
        }
    }
}
