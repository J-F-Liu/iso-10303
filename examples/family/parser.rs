#![doc = r" This file is generated. Do not edit."]
use iso_10303::step::*;
pub struct Unimplemented {}
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
type Date = Vec<i64>;
pub trait IFemale<'a>: IPerson<'a> {}
#[derive(Default)]
pub struct Female<'a> {
    first_name: String,
    last_name: String,
    nickname: Option<String>,
    birth_date: Date,
    children: std::collections::HashSet<&'a dyn IPerson<'a>>,
    hair: HairType,
}
impl<'a> IPerson<'a> for Female<'a> {
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
    fn children(&self) -> &std::collections::HashSet<&'a dyn IPerson<'a>> {
        &self.children
    }
    fn hair(&self) -> &HairType {
        &self.hair
    }
}
impl<'a> IFemale<'a> for Female<'a> {}
impl<'a> Female<'a> {
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
pub trait IMale<'a>: IPerson<'a> {
    fn wife(&self) -> &Option<&'a dyn IFemale<'a>>;
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
impl<'a> IPerson<'a> for Male<'a> {
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
    fn children(&self) -> &std::collections::HashSet<&'a dyn IPerson<'a>> {
        &self.children
    }
    fn hair(&self) -> &HairType {
        &self.hair
    }
}
impl<'a> IMale<'a> for Male<'a> {
    fn wife(&self) -> &Option<&'a dyn IFemale<'a>> {
        &self.wife
    }
}
impl<'a> Male<'a> {
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
pub trait IPerson<'a> {
    fn first_name(&self) -> &String;
    fn last_name(&self) -> &String;
    fn nickname(&self) -> &Option<String>;
    fn birth_date(&self) -> &Date;
    fn children(&self) -> &std::collections::HashSet<&'a dyn IPerson<'a>>;
    fn hair(&self) -> &HairType;
}
