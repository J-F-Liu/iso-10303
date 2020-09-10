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
