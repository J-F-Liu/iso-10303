use super::{Constant, Declaration};

pub struct Schema {
    pub name: String,
    pub constants: Vec<Constant>,
    pub declarations: Vec<Declaration>,
}
