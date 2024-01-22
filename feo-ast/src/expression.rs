#![allow(dead_code)]

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

use feo_types::span::Spanned;

use crate::statement::Statement;

pub use self::attribute::{InnerAttr, OuterAttr};
pub use self::block_expr::BlockExpr;
pub use self::range_expr::RangeExpr;
pub use self::struct_expr::StructExpr;

pub trait Constant
where
    Self: Sized + 'static + Spanned,
{
}

pub trait Expression
where
    Self: Spanned,
{
}

pub trait ExprWithBlock<E>
where
    Self: Expression,
{
}

pub trait ExprWithoutBlock<E>
where
    Self: Expression + Statement,
{
}
