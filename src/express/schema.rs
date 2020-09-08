use super::{Constant, Declaration};

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub constants: Vec<Constant>,
    pub declarations: Vec<Declaration>,
}
