#![allow(dead_code)]

use feo_types::{span::Spanned, Identifier, Keyword, Punctuation};

mod array_expr;
mod attribute;
mod block_expr;
mod call_expr;
mod closure_expr;
mod conditional_expr;
mod field_access_expr;
mod iteration_expr;
mod operator_expr;
mod parenthesized_expr;
mod range_expr;
mod return_expr;
mod struct_expr;
mod tuple_expr;

use crate::statement::Statement;

pub use self::{
    attribute::{InnerAttr, OuterAttr},
    block_expr::BlockExpr,
    struct_expr::StructExpr,
};

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

pub trait Assignable
where
    Self: Spanned,
{
}

impl Assignable for Identifier {}

pub trait BooleanOperand
where
    Self: Expression + 'static,
{
}

impl BooleanOperand for Keyword {}

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

impl Constant for Punctuation {}

pub trait Expression
where
    Self: Spanned,
{
}

impl Expression for Keyword {}

impl Expression for Punctuation {}

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

impl<E> ExprWithoutBlock<E> for Keyword {}

impl<E> ExprWithoutBlock<E> for Punctuation {}

pub trait IterableExpr
where
    Self: Expression + 'static,
{
}

impl IterableExpr for Keyword {}
