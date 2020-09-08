use std::ops::Range;

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
        bound: Range<usize>,
        optional: bool,
        unique: bool,
        base_type: Box<DataType>,
    },
    List {
        bound: Option<Range<usize>>,
        unique: bool,
        base_type: Box<DataType>,
    },
    Bag {
        bound: Option<Range<usize>>,
        base_type: Box<DataType>,
    },
    Set {
        bound: Option<Range<usize>>,
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
