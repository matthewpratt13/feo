use feo_types::{
    primitive::{Primitive, PrimitiveType},
    span::{Span, Spanned},
    U256,
};

use crate::{
    expression::{BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr},
    pattern::{Pattern, PatternWithoutRange, RangePattBound},
    ty::Type,
};

#[derive(Debug, Clone)]
pub struct Literal<T: Clone + PrimitiveType> {
    pub inner_value: Primitive<T>,
    span: Span,
}

impl<T> Literal<T>
where
    T: Clone + PrimitiveType,
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

impl<T> Spanned for Literal<T>
where
    T: Clone + PrimitiveType,
{
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
    Self: Sized + Pattern + 'static,
{
}

impl<T, E> LiteralExpr<E> for Literal<T> where T: Clone + PrimitiveType + 'static {}

impl<T, E> ExprWithoutBlock<E> for Literal<T> where T: Clone + PrimitiveType {}

impl<T> Expression for Literal<T> where T: Clone + PrimitiveType {}

impl<T> BooleanOperand for Literal<T> where T: Clone + PrimitiveType + 'static {}

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

impl<T> Constant for Literal<T> where T: Clone + PrimitiveType + 'static {}

impl<T> IterableExpr for Literal<T> where T: Clone + PrimitiveType + 'static {}

impl<T> LiteralPatt for Literal<T> where T: Clone + PrimitiveType + 'static {}

impl<T> Pattern for Literal<T> where T: Clone + PrimitiveType {}

impl<T> PatternWithoutRange for Literal<T> where T: Clone + PrimitiveType {}

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

impl<T> Type for Literal<T> where T: Clone + PrimitiveType {}
