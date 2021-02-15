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
        bound: Option<BoundSpec>,
        optional: bool,
        unique: bool,
        base_type: Box<DataType>,
    },
    /// Ordered elements
    List {
        bound: Option<BoundSpec>,
        unique: bool,
        base_type: Box<DataType>,
    },
    /// Unordered elements
    Bag {
        bound: Option<BoundSpec>,
        base_type: Box<DataType>,
    },
    /// Unordered and unqiue elements
    Set {
        bound: Option<BoundSpec>,
        base_type: Box<DataType>,
    },
    /// Enumeration
    Enum {
        values: Vec<String>,
    },
    /// Union of named types
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

impl DataType {
    pub fn is_number(&self) -> bool {
        match *self {
            DataType::Number => true,
            _ => false,
        }
    }
    pub fn is_real(&self) -> bool {
        match *self {
            DataType::Real { .. } => true,
            _ => false,
        }
    }
    pub fn is_integer(&self) -> bool {
        match *self {
            DataType::Integer => true,
            _ => false,
        }
    }
    pub fn type_ref(&self) -> Option<&String> {
        match self {
            DataType::TypeRef { name } => Some(name),
            _ => None,
        }
    }
}
