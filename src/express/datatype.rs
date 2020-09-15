use super::SimpleExpression;

#[derive(Debug, Clone)]
pub enum DataType {
    Number,
    Integer,
    Real {
        precision: Option<u8>,
    },
    Boolean,
    Logical,
    String {
        width: Option<usize>,
        fixed: bool,
    },
    Binary {
        width: Option<usize>,
        fixed: bool,
    },
    TypeRef {
        name: String,
    },
    Array {
        bound: BoundSpec,
        optional: bool,
        unique: bool,
        base_type: Box<DataType>,
    },
    List {
        bound: Option<BoundSpec>,
        unique: bool,
        base_type: Box<DataType>,
    },
    Bag {
        bound: Option<BoundSpec>,
        base_type: Box<DataType>,
    },
    Set {
        bound: Option<BoundSpec>,
        base_type: Box<DataType>,
    },
    Enum {
        values: Vec<String>,
    },
    Select {
        types: Vec<String>,
    },
    Generic {
        type_label: Option<String>,
    },
    Aggregate {
        type_label: Option<String>,
        base_type: Box<DataType>,
    },
}

#[derive(Debug, Clone)]
pub struct BoundSpec {
    pub start: SimpleExpression,
    pub end: Option<SimpleExpression>,
}
