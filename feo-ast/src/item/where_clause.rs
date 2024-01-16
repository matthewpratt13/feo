use crate::{
    keyword::KeywordKind,
    ty::{TraitBound, Type},
    type_utils::{Colon, Comma, Plus},
};

pub struct WhereClause {
    kw_where: KeywordKind,
    type_bounds: Vec<(TypeBound, Comma)>,
    trailing_type_bound_opt: Option<TypeBound>,
}

pub struct TypeBound {
    ty: Box<dyn Type>,
    colon: Colon,
    type_param_bounds_opt: Option<TypeParamBounds>,
}

pub struct TypeParamBounds {
    first_bound: TraitBound,
    subsequent_bounds: Vec<(Plus, TraitBound)>,
    trailing_plus_opt: Option<Plus>,
}
