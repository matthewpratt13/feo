use crate::expression::{CastableExpr, Constant, ExprWithoutBlock, Expression};
use crate::pattern::{Pattern, RangePattBound};
use crate::statement::Statement;
use crate::ty::Type;
use crate::U256;
use crate::{
    primitive::{Primitive, PrimitiveType},
    span::{Span, Spanned},
};

#[derive(Debug, Clone)]
pub struct Literal<L: 'static + Clone + Primitive> {
    raw_value: L,
    span: Span,
}

impl<L> PrimitiveType<L> for Literal<L>
where
    L: 'static + Clone + Primitive,
{
    fn new(raw_value: L, span: Span) -> Self {
        Self { raw_value, span }
    }

    fn raw_value(&self) -> &L {
        &self.raw_value
    }
}

impl<L> Spanned for Literal<L>
where
    L: 'static + Clone + Primitive,
{
    fn span(&self) -> Span {
        self.clone().span
    }
}

pub trait LiteralExpr<E>
where
    Self: Constant + ExprWithoutBlock<E>,
{
}

pub trait LiteralPatt
where
    Self: Sized + 'static + Pattern,
{
}

impl<L, E> LiteralExpr<E> for Literal<L> where L: 'static + Clone + Primitive {}

impl<L, E> ExprWithoutBlock<E> for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Expression for Literal<L> where L: 'static + Clone + Primitive {}

impl CastableExpr for Literal<char> {}

impl CastableExpr for Literal<u8> {}

impl CastableExpr for Literal<u16> {}

impl CastableExpr for Literal<u32> {}

impl CastableExpr for Literal<u64> {}

impl CastableExpr for Literal<U256> {}

impl CastableExpr for Literal<i32> {}

impl CastableExpr for Literal<i64> {}

impl CastableExpr for Literal<f32> {}

impl CastableExpr for Literal<f64> {}

impl CastableExpr for Literal<bool> {}

impl<L> Statement for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Constant for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> LiteralPatt for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Pattern for Literal<L> where L: 'static + Clone + Primitive {}

impl RangePattBound for Literal<char> {}

impl RangePattBound for Literal<i32> {}

impl RangePattBound for Literal<i64> {}

impl RangePattBound for Literal<u8> {}

impl RangePattBound for Literal<u16> {}

impl RangePattBound for Literal<u32> {}

impl RangePattBound for Literal<u64> {}

impl RangePattBound for Literal<U256> {}

impl RangePattBound for Literal<f32> {}

impl RangePattBound for Literal<f64> {}

impl<L> Type for Literal<L> where L: 'static + Clone + Primitive {}
