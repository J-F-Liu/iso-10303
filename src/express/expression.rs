#[derive(Debug)]
pub struct Expression {
    pub operand: SimpleExpression,
    pub operations: Vec<(Operator, SimpleExpression)>,
}

#[derive(Debug)]
pub struct SimpleExpression {
    pub operand: Term,
    pub operations: Vec<(Operator, Term)>,
}

#[derive(Debug)]
pub struct Term {
    pub operand: Factor,
    pub operations: Vec<(Operator, Factor)>,
}

#[derive(Debug)]
pub struct Factor {
    pub operand: SimpleFactor,
    pub operations: Vec<(Operator, SimpleFactor)>,
}

#[derive(Debug)]
pub enum Operator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    InstanceEqual,
    InstanceNotEqual,
    In,
    Like,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Or,
    Xor,
    And,
    Power,
}

#[derive(Debug)]
pub enum SimpleFactor {
    AggregateInitializer {
        elements: Vec<Box<Expression>>,
    },
    EnityConstructor {
        entity: String,
        parameters: Vec<Box<Expression>>,
    },
    EnumReference,
    Interval {
        low: Box<SimpleExpression>,
        op1: Operator,
        term: Box<SimpleExpression>,
        op2: Operator,
        high: Box<SimpleExpression>,
    },
    QueryExpression {
        variable: String,
        source: Box<SimpleExpression>,
        condition: Box<Expression>,
    },
    Primary(Primary),
}

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Grouped(Box<Expression>),
    QualifiedAccess(QualifiedAccess),
}

#[derive(Debug)]
pub enum Literal {
    Binary(String),
    Integer(i64),
    Real(f64),
    Logical(Option<bool>),
    String(String),
}

#[derive(Debug)]
pub struct QualifiedAccess {
    pub target: String,
    pub accessors: Vec<Accessor>,
}

#[derive(Debug)]
pub enum Accessor {
    FunctionCall {
        parameters: Vec<Box<Expression>>,
    },
    Indexer {
        start: Box<SimpleExpression>,
        end: Option<Box<SimpleExpression>>,
    },
    Attribute {
        name: String,
    },
    Group {
        entity: String,
    },
}
