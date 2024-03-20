use crate::ty::{TraitBound, Type};

#[derive(Debug, Clone)]
pub struct TypeBound {
    pub ty: Type,
    pub type_param_bounds: Vec<TraitBound>,
}
