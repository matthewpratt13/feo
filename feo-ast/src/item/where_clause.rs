use feo_types::type_utils::KwWhere;

use crate::{
    expression::TermCollection,
    ty::{TraitBound, Type},
};

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub kw_where: KwWhere,
    pub type_bounds: TermCollection<TypeBound>,
}

#[derive(Debug, Clone)]
pub struct TypeBound {
    pub ty: Type,
    pub type_param_bounds: Vec<TraitBound>,
}