use super::DataType;

pub enum Declaration {
    Type {
        name: String,
        underlying_type: DataType,
        rules: Vec<Rule>,
    },
    Entity {
        name: String,
        attributes: Vec<Attribute>,
        rules: Vec<Rule>,
    },
}

pub struct Attribute {
    pub name: String,
    pub base_type: DataType,
    pub optional: bool,
}

pub struct Rule;
