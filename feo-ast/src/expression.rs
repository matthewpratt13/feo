#![allow(dead_code)]

use feo_types::span::Spanned;

mod array_expr;
mod attribute;
mod block_expr;
mod call_expr;
mod closure_expr;
mod conditional_expr;
mod field_access_expr;
mod grouped_expr;
mod iteration_expr;
mod operator_expr;
mod range_expr;
mod return_expr;
mod struct_expr;
mod tuple_expr;

use crate::statement::Statement;
use crate::ty::Type;

pub use self::attribute::{InnerAttr, OuterAttr};
pub use self::block_expr::BlockExpr;

pub trait Constant
where
    Self: Sized + 'static,
{
}

pub trait Expression
where
    Self: Spanned,
{
}

pub trait ExprWithBlock<E>
where
    Self: Expression + Spanned,
{
}

pub trait ExprWithoutBlock<E>
where
    Self: Expression + Statement + Spanned,
{
}

pub trait ClosureExpr
where
    Self: Sized + Expression + Type + Spanned,
{
}

pub trait ConditionalExpr<E>
where
    Self: Sized + Constant + ExprWithBlock<E>,
{
}

pub trait IterationExpr<E>
where
    Self: Sized + ExprWithBlock<E>,
{
}

pub trait LiteralExpr<E>
where
    Self: Sized + Constant + ExprWithoutBlock<E>,
{
}

pub trait OperatorExpr<E>
where
    Self: Sized + ExprWithoutBlock<E>,
{
}

pub trait RangeExpr<E>
where
    Self: Sized + Constant + ExprWithoutBlock<E>,
{
}

pub trait StructExpr<E>
where
    Self: Sized + Constant + ExprWithoutBlock<E>,
{
}
