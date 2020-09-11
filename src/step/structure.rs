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

impl From<Parameter> for i64 {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(parameter) => parameter.into(),
            _ => panic!("can not convert"),
        }
    }
}

impl From<Parameter> for f64 {
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

impl<T: From<Parameter>> From<Parameter> for Vec<T> {
    fn from(parameter: Parameter) -> Self {
        match parameter {
            Parameter::UnTypedParameter(untyped_parameter) => match untyped_parameter {
                UnTypedParameter::List(list) => list.into_iter().map(|item| item.into()).collect(),
                _ => panic!("parameter is not an list"),
            },
            _ => panic!("parameter is not an list"),
        }
    }
}

impl From<UnTypedParameter> for i64 {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::Integer(number) => number,
            _ => panic!("can not convert"),
        }
    }
}

impl From<UnTypedParameter> for f64 {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::Real(number) => number,
            _ => panic!("can not convert"),
        }
    }
}

impl From<UnTypedParameter> for String {
    fn from(parameter: UnTypedParameter) -> Self {
        match parameter {
            UnTypedParameter::String(string) => string,
            UnTypedParameter::Null => String::default(),
            _ => panic!("can not convert"),
        }
    }
}