#![doc = r" This file is generated. Do not edit."]
#![allow(dead_code)]
use iso_10303::step::*;
use std::collections::HashSet;
#[derive(Default, Debug)]
pub struct Unimplemented {}
type Date = Vec<i64>;
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum HairType {
    Blonde,
    Brown,
    Black,
    Red,
    White,
}
impl Default for HairType {
    fn default() -> Self {
        HairType::Blonde
    }
}
impl From<String> for HairType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BLONDE" => HairType::Blonde,
            "BROWN" => HairType::Brown,
            "BLACK" => HairType::Black,
            "RED" => HairType::Red,
            "WHITE" => HairType::White,
            _ => panic!("{} is not a valid value", value),
        }
    }
}
impl From<Parameter> for HairType {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(untyped_parameter) => match untyped_parameter {
                UnTypedParameter::EnumValue(value) => value.into(),
                _ => panic!("parameter is not an enum value"),
            },
            Parameter::OmittedParameter => HairType::default(),
            _ => panic!("parameter is not an enum value"),
        }
    }
}
pub trait IPerson {
    fn first_name(&self) -> &String;
    fn last_name(&self) -> &String;
    fn nickname(&self) -> &Option<String>;
    fn birth_date(&self) -> &Date;
    fn children(&self) -> &HashSet<EntityRef>;
    fn hair(&self) -> &HairType;
}
pub trait IFemale: IPerson {}
#[derive(Default, Debug)]
pub struct Female {
    first_name: String,
    last_name: String,
    nickname: Option<String>,
    birth_date: Date,
    children: HashSet<EntityRef>,
    hair: HairType,
}
impl IPerson for Female {
    fn first_name(&self) -> &String {
        &self.first_name
    }
    fn last_name(&self) -> &String {
        &self.last_name
    }
    fn nickname(&self) -> &Option<String> {
        &self.nickname
    }
    fn birth_date(&self) -> &Date {
        &self.birth_date
    }
    fn children(&self) -> &HashSet<EntityRef> {
        &self.children
    }
    fn hair(&self) -> &HairType {
        &self.hair
    }
}
impl IFemale for Female {}
impl Female {
    pub fn form_parameters(parameters: Vec<Parameter>) -> Self {
        let mut entity = Female::default();
        for (index, parameter) in parameters.into_iter().enumerate() {
            match index {
                0usize => entity.first_name = parameter.into(),
                1usize => entity.last_name = parameter.into(),
                2usize => {
                    entity.nickname = if parameter.is_null() {
                        None
                    } else {
                        Some(parameter.into())
                    }
                }
                3usize => entity.birth_date = parameter.into(),
                4usize => entity.children = parameter.into(),
                5usize => entity.hair = parameter.into(),
                _ => {}
            }
        }
        entity
    }
}
pub trait IMale: IPerson {
    fn wife(&self) -> &Option<EntityRef>;
}
#[derive(Default, Debug)]
pub struct Male {
    first_name: String,
    last_name: String,
    nickname: Option<String>,
    birth_date: Date,
    children: HashSet<EntityRef>,
    hair: HairType,
    wife: Option<EntityRef>,
}
impl IPerson for Male {
    fn first_name(&self) -> &String {
        &self.first_name
    }
    fn last_name(&self) -> &String {
        &self.last_name
    }
    fn nickname(&self) -> &Option<String> {
        &self.nickname
    }
    fn birth_date(&self) -> &Date {
        &self.birth_date
    }
    fn children(&self) -> &HashSet<EntityRef> {
        &self.children
    }
    fn hair(&self) -> &HairType {
        &self.hair
    }
}
impl IMale for Male {
    fn wife(&self) -> &Option<EntityRef> {
        &self.wife
    }
}
impl Male {
    pub fn form_parameters(parameters: Vec<Parameter>) -> Self {
        let mut entity = Male::default();
        for (index, parameter) in parameters.into_iter().enumerate() {
            match index {
                0usize => entity.first_name = parameter.into(),
                1usize => entity.last_name = parameter.into(),
                2usize => {
                    entity.nickname = if parameter.is_null() {
                        None
                    } else {
                        Some(parameter.into())
                    }
                }
                3usize => entity.birth_date = parameter.into(),
                4usize => entity.children = parameter.into(),
                5usize => entity.hair = parameter.into(),
                6usize => {
                    entity.wife = if parameter.is_null() {
                        None
                    } else {
                        Some(parameter.into())
                    }
                }
                _ => {}
            }
        }
        entity
    }
}
use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap};
pub struct ExampleReader {
    pub entities: BTreeMap<i64, Box<dyn Any>>,
    pub type_ids: HashMap<TypeId, Vec<i64>>,
}
impl ExampleReader {
    pub fn new() -> Self {
        ExampleReader {
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
impl StepReader for ExampleReader {
    fn read_simple_entity(&mut self, id: i64, typed_parameter: TypedParameter) {
        match typed_parameter.type_name.as_str() {
            "FEMALE" => {
                let entity = Female::form_parameters(typed_parameter.parameters);
                self.add_entity(id, entity);
            }
            "MALE" => {
                let entity = Male::form_parameters(typed_parameter.parameters);
                self.add_entity(id, entity);
            }
            _ => println!("{} is not implemented", typed_parameter.type_name),
        }
    }
}
