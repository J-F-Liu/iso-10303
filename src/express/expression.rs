#[derive(Debug, Clone)]
pub struct Expression {
    pub operand: SimpleExpression,
    pub operations: Vec<(Operator, SimpleExpression)>,
}

#[derive(Debug, Clone)]
pub struct SimpleExpression {
    pub operand: Term,
    pub operations: Vec<(Operator, Term)>,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub operand: Factor,
    pub operations: Vec<(Operator, Factor)>,
}

#[derive(Debug, Clone)]
pub struct Factor {
    pub operand: SimpleFactor,
    pub operations: Vec<(Operator, SimpleFactor)>,
}

#[derive(Debug, Clone)]
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
    Not,
    Neg,
    Power,
}

#[derive(Debug, Clone)]
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
    UnaryExpression {
        op: Operator,
        operand: QualifiedAccess,
    },
    QualifiedAccess(QualifiedAccess),
}

#[derive(Debug, Clone)]
pub struct QualifiedAccess {
    pub base: Primary,
    pub accessors: Vec<Accessor>,
}

#[derive(Debug, Clone)]
pub enum Primary {
    Literal(Literal),
    Constant(String),
    Reference(String),
    Grouped(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Binary(String),
    Integer(i64),
    Real(f64),
    Logical(Option<bool>),
    String(String),
}

#[derive(Debug, Clone)]
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
