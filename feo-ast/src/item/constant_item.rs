use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Equals, KwConst, KwMut, KwStatic, Semicolon},
    Identifier,
};

use crate::{
    expression::{
        Assignable, BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr,
        OuterAttr,
    },
    pattern::Pattern,
    statement::Statement,
    ty::Type,
};

use super::{Item, VisibilityKind};

pub struct ConstantItem<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_const: KwConst,
    item_name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Expression<A, B, C, E, I, S, U>)>, // `None` is only allowed in a `TraitDef`
    semicolon: Semicolon,
}

impl<A, B, C, E, I, S, U> Item for ConstantItem<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> Statement for ConstantItem<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> Constant for ConstantItem<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    C: Castable + 'static,
    B: BooleanOperand + Spanned,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Pattern for ConstantItem<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> Spanned for ConstantItem<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_const.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_const.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct StaticItem<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_static: KwStatic,
    kw_mut_opt: Option<KwMut>,
    item_name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Expression<A, B, C, E, I, S, U>)>,
    semicolon: Semicolon,
}

impl<A, B, C, E, I, S, U> Item for StaticItem<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> Statement for StaticItem<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> Constant for StaticItem<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    C: Castable + 'static,
    B: BooleanOperand + Spanned,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for StaticItem<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_static.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_static.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

unsafe impl<A, B, C, E, I, S, U> Sync for StaticItem<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}
