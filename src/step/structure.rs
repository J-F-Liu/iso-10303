use super::Real;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum Parameter {
    TypedParameter(TypedParameter),
    UnTypedParameter(UnTypedParameter),
    OmittedParameter,
}

#[derive(Debug)]
pub struct TypedParameter {
    pub type_name: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub enum UnTypedParameter {
    List(Vec<Parameter>),
    EnumValue(String),
    EntityRef(i64),
    ConstantRef(String),
    Integer(i64),
    Real(f64),
    String(String),
    Binary(()),
    Null,
}

#[derive(Debug)]
pub struct EntityInstance {
    pub id: i64,
    pub value: Vec<TypedParameter>,
}

#[derive(Eq, PartialEq, Hash, Debug, Default)]
pub struct EntityRef(pub i64);

#[derive(Debug)]
pub struct ExchangeFile {
    pub header: Vec<TypedParameter>,
    pub data: Vec<EntityInstance>,
}

impl Parameter {
    pub fn is_null(&self) -> bool {
        match self {
            Parameter::UnTypedParameter(parameter) => match parameter {
                UnTypedParameter::Null => true,
                _ => false,
            },
            Parameter::OmittedParameter => true,
            _ => false,
        }
    }
}

impl From<Parameter> for bool {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(parameter) => parameter.into(),
            _ => panic!("can not convert"),
        }
    }
}

impl From<Parameter> for Option<bool> {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(parameter) => parameter.into(),
            Parameter::OmittedParameter => None,
            _ => panic!("can not convert"),
        }
    }
}

impl From<Parameter> for i64 {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(parameter) => parameter.into(),
            _ => panic!("can not convert"),
        }
    }
}

impl From<Parameter> for Real {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(parameter) => parameter.into(),
            _ => panic!("can not convert"),
        }
    }
}

impl From<Parameter> for String {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(parameter) => parameter.into(),
            _ => panic!("can not convert"),
        }
    }
}

impl From<Parameter> for EntityRef {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(parameter) => parameter.into(),
            Parameter::OmittedParameter => EntityRef(0),
            _ => panic!("can not convert"),
        }
    }
}

impl<T: From<Parameter>> From<Parameter> for Vec<T> {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(untyped_parameter) => match untyped_parameter {
                UnTypedParameter::List(list) => list.into_iter().map(|item| item.into()).collect(),
                _ => panic!("parameter is not an list"),
            },
            Parameter::OmittedParameter => Vec::new(),
            _ => panic!("parameter is not an list"),
        }
    }
}

impl<T: From<Parameter> + Eq + Hash> From<Parameter> for HashSet<T> {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(parameter) => parameter.into(),
            Parameter::OmittedParameter => HashSet::new(),
            _ => panic!("can not convert"),
        }
    }
}

impl From<UnTypedParameter> for bool {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::EnumValue(value) => match value.as_str() {
                "T" => true,
                "F" => false,
                _ => panic!("invalid boolean value {}", value),
            },
            _ => panic!("can not convert to boolean"),
        }
    }
}

impl From<UnTypedParameter> for Option<bool> {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::EnumValue(value) => match value.as_str() {
                "T" => Some(true),
                "F" => Some(false),
                "U" => None,
                _ => panic!("invalid boolean value {}", value),
            },
            _ => panic!("can not convert to boolean"),
        }
    }
}

impl From<UnTypedParameter> for i64 {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::Integer(number) => number,
            _ => panic!("can not convert to integer"),
        }
    }
}

impl From<UnTypedParameter> for Real {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::Real(number) => Real(number),
            _ => panic!("can not convert to real"),
        }
    }
}

impl From<UnTypedParameter> for String {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::String(string) => string,
            UnTypedParameter::Null => String::default(),
            _ => panic!("can not convert to string"),
        }
    }
}

impl From<UnTypedParameter> for EntityRef {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::EntityRef(id) => EntityRef(id),
            _ => panic!("can not convert"),
        }
    }
}

impl<T: From<Parameter> + Eq + Hash> From<UnTypedParameter> for HashSet<T> {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::List(values) => HashSet::from_iter(values.into_iter().map(|value| T::from(value))),
            _ => panic!("can not convert"),
        }
    }
}
