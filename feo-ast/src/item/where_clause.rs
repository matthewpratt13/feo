use feo_types::{utils::Comma, Keyword};

use crate::ty::{TraitBound, Type};

#[derive(Debug, Clone)]
pub struct WhereClause {
    kw_where: Keyword,
    type_bounds: Vec<TypeBound>,
    trailing_type_bound_opt: Option<TypeBound>,
}

#[derive(Debug, Clone)]
pub struct TypeBound {
    pub ty: Type,
    pub type_param_bounds_opt: Option<TypeParamBounds>,
}

#[derive(Debug, Clone)]
pub struct TypeParamBounds {
    pub first_bound: TraitBound,
    pub subsequent_bounds: Option<Vec<TraitBound>>,
    pub trailing_comma_opt: Option<Comma>,
}
