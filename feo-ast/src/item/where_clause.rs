use feo_types::{
    utils::{Colon, Comma, Plus},
    Keyword,
};

use crate::ty::{TraitBound, Type};

#[derive(Debug, Clone)]
pub struct WhereClause {
    kw_where: Keyword,
    type_bounds: Vec<(TypeBound, Comma)>,
    trailing_type_bound_opt: Option<TypeBound>,
}

#[derive(Debug, Clone)]
pub struct TypeBound {
    ty: Type,
    colon: Colon,
    type_param_bounds_opt: Option<TypeParamBounds>,
}

#[derive(Debug, Clone)]
pub struct TypeParamBounds {
    first_bound: TraitBound,
    subsequent_bounds: Vec<(Plus, TraitBound)>,
}
