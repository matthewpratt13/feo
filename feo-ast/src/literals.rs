use feo_types::{Literal, Primitive, U256};

use crate::expression::{Constant, ExprWithoutBlock, Expression};
use crate::pattern::{Pattern, RangePattBound};
use crate::statement::Statement;
use crate::ty::Type;

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

impl RangePattBound for Literal<f64> {}

impl<L> Type for Literal<L> where L: 'static + Clone + Primitive {}
