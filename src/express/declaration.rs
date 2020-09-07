use super::{DataType, Expression};

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
}

pub struct Attribute {
    pub name: String,
    pub data_type: DataType,
    pub optional: bool,
}

pub struct DerivedAttribute {
    pub name: String,
    pub data_type: DataType,
    pub expr: Expression,
}

pub struct AttributeReference {
    pub name: String,
    pub entity: Option<String>,
}

pub struct DomainRule {
    pub label: Option<String>,
    pub expr: Expression,
}

pub struct UniqueRule {
    pub label: Option<String>,
    pub attributes: Vec<AttributeReference>,
}

pub struct Constant {
    pub name: String,
    pub data_type: DataType,
    pub expr: Expression,
}
