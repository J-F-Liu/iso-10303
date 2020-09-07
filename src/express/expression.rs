pub struct Expression {
    pub operand: SimpleExpression,
    pub operations: Vec<(Operator, SimpleExpression)>,
}

pub struct SimpleExpression {
    pub operand: Term,
    pub operations: Vec<(Operator, Term)>,
}

pub struct Term {
    pub operand: Factor,
    pub operations: Vec<(Operator, Factor)>,
}

pub struct Factor {
    pub operand: SimpleFactor,
    pub operations: Vec<(Operator, SimpleFactor)>,
}

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

pub enum Primary {
    Literal(Literal),
    Grouped(Box<Expression>),
    QualifiedAccess(QualifiedAccess),
}

pub enum Literal {
    Binary(String),
    Integer(i64),
    Real(f64),
    Logical(Option<bool>),
    String(String),
}

pub struct QualifiedAccess {
    pub target: String,
    pub accessors: Vec<Accessor>,
}

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
