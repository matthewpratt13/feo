use feo_types::utils::{Comma, KwWhere};

use crate::ty::{TraitBound, Type};

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub kw_where: KwWhere,
    pub first_bound: TypeBound,
    pub subsequent_bounds_opt: Option<Vec<TypeBound>>,
    pub trailing_type_bound_opt: Option<TypeBound>,
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
    pub trailing_comma_opt: Option<Comma>,
}
