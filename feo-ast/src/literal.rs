use feo_types::{
    primitive::{Primitive, PrimitiveType},
    span::{Span, Spanned},
    U256,
};

use crate::{
    expression::{BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr},
    pattern::{Pattern, PatternWithoutRange, RangePattBound},
};

#[derive(Debug, Clone)]
pub struct Literal<L> {
    raw_value: L,
    span: Span,
}

impl<L> PrimitiveType<L> for Literal<L>
where
    Self: Sized,
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

impl<L, E> LiteralExpr<E> for Literal<L> where L: 'static + Clone + Primitive {}

impl<L, E> ExprWithoutBlock<E> for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Expression for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> BooleanOperand for Literal<L> where L: 'static + Clone + Primitive {}

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

impl<L> Constant for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> IterableExpr for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> LiteralPatt for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Pattern for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> PatternWithoutRange for Literal<L> where L: 'static + Clone + Primitive {}

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
