use feo_types::{
    span::{Span, Spanned},
    utils::{DblDot, DotDotEquals},
};

use crate::statement::Statement;

use super::{
    Assignable, BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr,
};

pub trait RangeExpr
where
    Self: ExprWithoutBlock + BooleanOperand + IterableExpr + Constant,
{
}

pub enum RangeExprKind<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    RangeFullExpr(RangeFullExpr),
    RangeFromToExpr(RangeFromToExpr<A, B, C, E, I, S, U>),
    RangeFromExpr(RangeFromExpr<A, B, C, E, I, S, U>),
    RangeToExpr(RangeToExpr<A, B, C, E, I, S, U>),
    RangeInclusiveExpr(RangeInclusiveExpr<A, B, C, E, I, S, U>),
    RangeToInclusiveExpr(RangeToInclusiveExpr<A, B, C, E, I, S, U>),
}

impl<A, B, C, E, I, S, U> Spanned for RangeExprKind<A, B, C, E, I, S, U>
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
        match self {
            RangeExprKind::RangeFullExpr(rfe) => rfe.span(),
            RangeExprKind::RangeFromToExpr(rft) => rft.span(),
            RangeExprKind::RangeFromExpr(rf) => rf.span(),
            RangeExprKind::RangeToExpr(rt) => rt.span(),
            RangeExprKind::RangeInclusiveExpr(ri) => ri.span(),
            RangeExprKind::RangeToInclusiveExpr(rti) => rti.span(),
        }
    }
}

pub type RangeFullExpr = DblDot;

impl RangeExpr for RangeFullExpr {}

impl BooleanOperand for RangeFullExpr {}

impl IterableExpr for RangeFullExpr {}

pub struct RangeFromToExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    from_operand: Box<Expression<A, B, C, E, I, S, U>>,
    dbl_dot: DblDot,
    to_operand_excl: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> RangeExpr for RangeFromToExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for RangeFromToExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for RangeFromToExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for RangeFromToExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for RangeFromToExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for RangeFromToExpr<A, B, C, E, I, S, U>
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
        let start_pos = self.from_operand.span().start();
        let end_pos = self.to_operand_excl.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeFromExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    from_operand: Box<Expression<A, B, C, E, I, S, U>>,
    dbl_dot: DblDot,
}

impl<A, B, C, E, I, S, U> RangeExpr for RangeFromExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for RangeFromExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for RangeFromExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for RangeFromExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for RangeFromExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for RangeFromExpr<A, B, C, E, I, S, U>
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
        let start_pos = self.from_operand.span().start();
        let end_pos = self.dbl_dot.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeToExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    dbl_dot: DblDot,
    to_operand: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> RangeExpr for RangeToExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for RangeToExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for RangeToExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for RangeToExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for RangeToExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for RangeToExpr<A, B, C, E, I, S, U>
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
        let start_pos = self.dbl_dot.span().start();
        let end_pos = self.to_operand.span().end();
        let source = self.dbl_dot.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeInclusiveExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    from_operand: Box<Expression<A, B, C, E, I, S, U>>,
    dot_dot_equals: DotDotEquals,
    to_operand_incl: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> RangeExpr for RangeInclusiveExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for RangeInclusiveExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for RangeInclusiveExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for RangeInclusiveExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for RangeInclusiveExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for RangeInclusiveExpr<A, B, C, E, I, S, U>
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
        let start_pos = self.from_operand.span().start();
        let end_pos = self.to_operand_incl.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeToInclusiveExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    dot_dot_equals: DotDotEquals,
    to_operand_incl: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> RangeExpr for RangeToInclusiveExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for RangeToInclusiveExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for RangeToInclusiveExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for RangeToInclusiveExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for RangeToInclusiveExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for RangeToInclusiveExpr<A, B, C, E, I, S, U>
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
        let start_pos = self.dot_dot_equals.span().start();
        let end_pos = self.to_operand_incl.span().end();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
