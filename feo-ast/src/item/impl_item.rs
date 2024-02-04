use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, KwFor, KwImpl},
};

use crate::{
    expression::{
        Assignable, BooleanOperand, Castable, ExprWithBlock, ExprWithoutBlock, InnerAttr,
        IterableExpr, OuterAttr,
    },
    path::PathType,
    statement::Statement,
    ty::Type,
};

use super::{ConstantItem, FunctionDef, Item, TypeAliasDef, WhereClause};

pub trait ImplItem
where
    Self: Sized + Item,
{
}

pub enum InherentImplItem<
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
}

pub enum TraitImplItem<
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
    TypeAlias(TypeAliasDef),
}

pub struct InherentImpl<
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
    kw_impl: KwImpl,
    nominal_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<InherentImplItem<A, B, C, E, F, I, S, U>>,
    close_brace: Brace,
}

impl<A, B, C, E, F, I, S, U> ImplItem for InherentImpl<A, B, C, E, F, I, S, U>
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

impl<A, B, C, E, F, I, S, U> Item for InherentImpl<A, B, C, E, F, I, S, U>
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

impl<A, B, C, E, F, I, S, U> Statement for InherentImpl<A, B, C, E, F, I, S, U>
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

impl<A, B, C, E, F, I, S, U> Spanned for InherentImpl<A, B, C, E, F, I, S, U>
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
        let start_pos = if let Some(a) = self.outer_attributes.first() {
            a.span().start()
        } else {
            self.kw_impl.span().start()
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_impl.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TraitImpl<
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
    kw_impl: KwImpl,
    implemented_trait_path: PathType,
    kw_for: KwFor,
    implementing_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<TraitImplItem<A, B, C, E, F, I, S, U>>,
    close_brace: Brace,
}

impl<A, B, C, E, F, I, S, U> ImplItem for TraitImpl<A, B, C, E, F, I, S, U>
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

impl<A, B, C, E, F, I, S, U> Item for TraitImpl<A, B, C, E, F, I, S, U>
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

impl<A, B, C, E, F, I, S, U> Statement for TraitImpl<A, B, C, E, F, I, S, U>
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

impl<A, B, C, E, F, I, S, U> Spanned for TraitImpl<A, B, C, E, F, I, S, U>
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
        let start_pos = if let Some(a) = self.outer_attributes.first() {
            a.span().start()
        } else {
            self.kw_impl.span().start()
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_impl.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
