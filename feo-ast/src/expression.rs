#![allow(dead_code)]

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

use feo_types::{span::Spanned, Identifier, Keyword, Punctuation};

pub use self::{
    attribute::{AttributeKind, InnerAttr, OuterAttr},
    block_expr::BlockExpr,
    operator_expr::{ArithmeticOrLogicalOperatorKind, DerefOperator},
    struct_expr::{Struct, StructExpr, StructExprField, StructExprFields},
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

pub enum Expression {
    Struct,
}

pub trait Assignable
where
    Self: Spanned,
{
}

impl Assignable for Identifier {}

pub trait BooleanOperand
where
    Self: 'static,
{
}

impl BooleanOperand for Keyword {}

pub trait Castable
where
    Self: Spanned + 'static,
{
}

pub trait Constant
where
    Self: Sized + Spanned + 'static,
{
}

impl Constant for Punctuation {}

// pub trait Expression
// where
//     Self: Spanned,
// {
// }

// impl Expression for Keyword {}

// impl Expression for Punctuation {}

pub trait ExprWithBlock<E> {}

pub trait ExprWithoutBlock<E> {}

impl<E> ExprWithoutBlock<E> for Keyword {}

impl<E> ExprWithoutBlock<E> for Punctuation {}

pub trait IterableExpr
where
    Self: 'static,
{
}

impl IterableExpr for Keyword {}
