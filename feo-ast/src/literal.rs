use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    U256,
};

use crate::{
    expression::{BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr},
    pattern::{Pattern, PatternWithoutRange, RangePattBound},
};

#[derive(Debug, Clone)]
pub struct Literal<T> {
    pub inner_value: Primitive<T>,
    span: Span,
}

impl<T> Literal<T>
where
    T: Clone,
{
    pub fn new(raw_value: Primitive<T>, span: Span) -> Literal<T> {
        Literal {
            inner_value: raw_value,
            span,
        }
    }

    pub fn into_inner(&self) -> T {
        self.inner_value.raw_value()
    }
}

impl<T> Spanned for Literal<T> {
    fn span(&self) -> Span {
        self.span.clone()
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

impl<L, E> LiteralExpr<E> for Literal<L> where L: Clone + 'static {}

impl<L, E> ExprWithoutBlock<E> for Literal<L> where L: Clone {}

impl<L> Expression for Literal<L> where L: Clone {}

impl<L> BooleanOperand for Literal<L> where L: Clone + 'static {}

impl Castable for Literal<char> {}

impl Castable for Literal<u8> {}

impl Castable for Literal<u16> {}

impl Castable for Literal<u32> {}

impl Castable for Literal<u64> {}

impl Castable for Literal<U256> {}

impl Castable for Literal<i32> {}

impl Castable for Literal<i64> {}

impl Castable for Literal<f32> {}

impl Castable for Literal<f64> {}

impl Castable for Literal<bool> {}

impl<L> Constant for Literal<L> where L: Clone + 'static {}

impl<L> IterableExpr for Literal<L> where L: Clone + 'static {}

impl<L> LiteralPatt for Literal<L> where L: Clone + 'static {}

impl<L> Pattern for Literal<L> where L: Clone {}

impl<L> PatternWithoutRange for Literal<L> where L: Clone {}

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
