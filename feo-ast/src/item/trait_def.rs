use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Colon, KwTrait},
    Identifier,
};

use crate::{
    expression::{
        Assignable, BooleanOperand, Castable, ExprWithBlock, ExprWithoutBlock, InnerAttr,
        IterableExpr, OuterAttr,
    },
    statement::Statement,
};

use super::{
    ConstantItem, FunctionDef, FunctionSig, Item, TypeAliasDef, TypeParamBounds, VisibilityKind,
    WhereClause,
};

pub enum TraitDefItem<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    F: ExprWithBlock + Spanned,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    Constant(ConstantItem<A, B, C, E, I, S, U>),
    FuncDef(FunctionDef<F>),
    FuncSig(FunctionSig),
    TypeAlias(TypeAliasDef),
}

pub struct TraitDef<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    F: ExprWithBlock + Spanned,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    outer_attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_trait: KwTrait,
    trait_name: Identifier,
    type_param_bounds_opt: Option<(Colon, Option<TypeParamBounds>)>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<TraitDefItem<A, B, C, E, F, I, S, U>>,
    close_brace: Brace,
}

impl<A, B, C, E, F, I, S, U> Item for TraitDef<A, B, C, E, F, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    F: ExprWithBlock + Spanned,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, F, I, S, U> Statement for TraitDef<A, B, C, E, F, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    F: ExprWithBlock + Spanned,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, F, I, S, U> Spanned for TraitDef<A, B, C, E, F, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    F: ExprWithBlock + Spanned,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
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
