use crate::{
    expression::{InnerAttr, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    span::{Span, Spanned},
    statement::Statement,
    type_utils::{Brace, Colon},
};

use super::{
    ConstantItem, FunctionDefWithBody, FunctionDefWithoutBody, Item, TypeAliasDef, TypeParamBounds,
    VisibilityKind, WhereClause,
};

pub enum TraitDefItem<T> {
    Constant(ConstantItem),
    FunctionDef(FunctionDefWithBody<T>),
    FunctionSig(FunctionDefWithoutBody),
    TypeAlias(TypeAliasDef),
}

pub struct TraitDef<T> {
    outer_attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_trait: Keyword,
    trait_name: Identifier,
    type_param_bounds_opt: Option<(Colon, Option<TypeParamBounds>)>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<TraitDefItem<T>>,
    close_brace: Brace,
}

impl<T> Item for TraitDef<T> {}

impl<T> Statement for TraitDef<T> {}

impl<T> Spanned for TraitDef<T> {
    fn span(&self) -> Span {
        let start_pos = match self.outer_attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_trait.span().start(),
            },
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_trait.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
