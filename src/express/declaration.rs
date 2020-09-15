use super::{DataType, Expression};

#[derive(Debug)]
pub enum Declaration {
    TypeDef(TypeDef),
    Entity(Entity),
    Function(Function),
    Rule(Rule),
}

#[derive(Debug)]
pub struct TypeDef {
    pub name: String,
    pub underlying_type: DataType,
    pub domain_rules: Vec<DomainRule>,
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub is_abstract: bool,
    pub supertypes: Vec<String>,
    pub attributes: Vec<Attribute>,
    pub derives: Vec<DerivedAttribute>,
    pub domain_rules: Vec<DomainRule>,
    pub unique_rules: Vec<UniqueRule>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: DataType,
    pub parameters: Vec<Parameter>,
    pub statements: Vec<Statement>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub entities: Vec<String>,
    pub statements: Vec<Statement>,
    pub domain_rules: Vec<DomainRule>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub supertype: Option<String>,
    pub data_type: DataType,
    pub optional: bool,
}

#[derive(Debug)]
pub struct DerivedAttribute {
    pub name: String,
    pub supertype: Option<String>,
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
