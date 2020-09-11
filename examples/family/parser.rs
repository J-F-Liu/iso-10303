#![doc = r" This file is generated. Do not edit."]
use iso_10303::step::*;
pub struct Unimplemented {}
type Date = Vec<i64>;
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
pub trait IPerson<'a> {
    fn first_name(&self) -> &String;
    fn last_name(&self) -> &String;
    fn nickname(&self) -> &Option<String>;
    fn birth_date(&self) -> &Date;
    fn children(&self) -> &std::collections::HashSet<&'a dyn IPerson>;
    fn hair(&self) -> &HairType;
}
pub trait IFemale<'a>: IPerson {}
#[derive(Default)]
pub struct Female<'a> {
    first_name: String,
    last_name: String,
    nickname: Option<String>,
    birth_date: Date,
    children: std::collections::HashSet<&'a dyn IPerson>,
    hair: HairType,
}
impl<'a> IPerson for Female<'a> {
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
    fn children(&self) -> &std::collections::HashSet<&'a dyn IPerson> {
        &self.children
    }
    fn hair(&self) -> &HairType {
        &self.hair
    }
}
impl<'a> IFemale for Female<'a> {}
impl Female<'a> {
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
pub trait IMale<'a>: IPerson {
    fn wife(&self) -> &Option<&'a dyn IFemale>;
}
#[derive(Default)]
pub struct Male<'a> {
    first_name: String,
    last_name: String,
    nickname: Option<String>,
    birth_date: Date,
    children: std::collections::HashSet<&'a dyn IPerson<'a>>,
    hair: HairType,
    wife: Option<&'a dyn IFemale<'a>>,
}
impl<'a> IPerson for Male<'a> {
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
    fn children(&self) -> &std::collections::HashSet<&'a dyn IPerson> {
        &self.children
    }
    fn hair(&self) -> &HairType {
        &self.hair
    }
}
impl<'a> IMale for Male<'a> {
    fn wife(&self) -> &Option<&'a dyn IFemale> {
        &self.wife
    }
}
impl Male<'a> {
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
