use feo_types::type_utils::KwWhere;

use crate::ty::{TraitBound, Type};

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub kw_where: KwWhere,
    pub first_bound: TypeBound,
    pub subsequent_bounds_opt: Option<Vec<TypeBound>>,
}

#[derive(Debug, Clone)]
pub struct TypeBound {
    pub ty: Type,
    pub type_param_bounds_opt: Option<TypeParamBounds>,
}

#[derive(Debug, Clone)]
pub struct TypeParamBounds {
    pub first_bound: TraitBound,
    pub subsequent_bounds_opt: Option<Vec<TraitBound>>,
}
