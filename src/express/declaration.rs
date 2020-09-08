use super::{DataType, Expression};

#[derive(Debug)]
pub enum Declaration {
    Type {
        name: String,
        underlying_type: DataType,
        domain_rules: Vec<DomainRule>,
    },
    Entity {
        name: String,
        is_abstract: bool,
        supertypes: Vec<String>,
        attributes: Vec<Attribute>,
        derives: Vec<DerivedAttribute>,
        domain_rules: Vec<DomainRule>,
        unique_rules: Vec<UniqueRule>,
    },
    Function {
        name: String,
        return_type: DataType,
        parameters: Vec<Parameter>,
        statements: Vec<Statement>,
    },
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub data_type: DataType,
    pub optional: bool,
}

#[derive(Debug)]
pub struct DerivedAttribute {
    pub name: String,
    pub data_type: DataType,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct AttributeReference {
    pub name: String,
    pub entity: Option<String>,
}

#[derive(Debug)]
pub struct DomainRule {
    pub label: Option<String>,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct UniqueRule {
    pub label: Option<String>,
    pub attributes: Vec<AttributeReference>,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug)]
pub struct Constant {
    pub name: String,
    pub data_type: DataType,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct Statement {
    pub text: String,
}
