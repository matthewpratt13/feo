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

use crate::{span::Spanned, statement::Statement};

pub use self::attribute::{InnerAttr, OuterAttr};
pub use self::block_expr::BlockExpr;
pub use self::operator_expr::OperatorExpr;
pub use self::range_expr::RangeExpr;
pub use self::struct_expr::StructExpr;

// expressions always produce / evaluate to a value, and may have (side) effects

// expressions:
// - array, index
// - block
// - function / method call
// - closure
// - if, match
// - field access
// - grouped
// - literal (char, string, int, uint, float, bytes32, bool)
// - loop / while / for..in
// - operators:
//      - arithmetic / logical
//      - assignment / compound assignment
//      - bool
//      - borrow
//      - comparison
//      - deref
//      - negation
//      - result / option unwrap
//      - type cast
// - path
// - range:
//      - from..to
//      - from..
//      - ..to
//      - from..=to
//      - ..=to
// - return
// - struct / tuple struct / unit struct
// - tuple

pub trait AssignableExpr
where
    Self: Expression,
{
}

pub trait Castable
where
    Self: 'static + Spanned,
{
}

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
