#![allow(dead_code)]

use feo_types::U256;

mod array_expr;
mod block_expr;
mod call_expr;
mod conditional_expr;
mod iteration_expr;
mod operator_expr;
mod range_expr;
mod struct_expr;
mod tuple_expr;

use crate::{
    path::SimplePath,
    statement::Statement,
    type_utils::{Bracket, HashSign},
};

pub use self::block_expr::BlockExpr;

pub trait Expression {}

pub trait ExprWithBlock<E>
where
    E: Expression,
{
}

impl<E> Expression for dyn ExprWithBlock<E> {}

pub trait ExprWithoutBlock<E>
where
    E: Expression,
{
}

impl<E> Expression for dyn ExprWithoutBlock<E> {}

impl<E> Statement for dyn ExprWithoutBlock<E> {}

pub trait ConditionalExpr<C>
where
    C: Expression,
{
}

impl<C> Expression for dyn ConditionalExpr<C> {}

impl<C, E> ExprWithBlock<E> for dyn ConditionalExpr<C> where E: Expression {}

pub trait IterationExpr<I>
where
    I: Expression,
{
}

impl<I> Expression for dyn IterationExpr<I> where I: Expression {}

impl<I, E> ExprWithBlock<E> for dyn IterationExpr<I> where E: Expression {}

pub trait LiteralExpr<L>
where
    L: Expression,
{
}

impl<L> Expression for dyn LiteralExpr<L> where L: Expression {}

impl<L, E> ExprWithoutBlock<E> for dyn LiteralExpr<L> where E: Expression {}

pub trait OperatorExpr<O>
where
    O: Expression,
{
}

impl<O> Expression for dyn OperatorExpr<O> where O: Expression {}

impl<O, E> ExprWithoutBlock<E> for dyn OperatorExpr<O> where E: Expression {}

pub trait RangeExpr<R>
where
    R: Expression,
{
}

impl<R> Expression for dyn RangeExpr<R> where R: Expression {}
impl<R, E> ExprWithoutBlock<E> for dyn RangeExpr<R> where E: Expression {}

pub trait StructExpr<S>
where
    S: Expression,
{
}

impl<S> Expression for dyn StructExpr<S> where S: Expression {}

impl<S, E> ExprWithoutBlock<E> for dyn StructExpr<S> where E: Expression {}

impl Expression for char {}

impl Expression for &'static str {}

impl Expression for i64 {}

impl Expression for u64 {}

impl Expression for U256 {}

impl Expression for &[u8; 32] {}

impl Expression for bool {}

pub struct Attribute {
    hash: HashSign,
    open_bracket: Bracket,
    attribute_path: SimplePath,
    close_bracket: Bracket,
}

impl Expression for Attribute {}

impl<E> ExprWithBlock<E> for Attribute where E: Expression {}

impl<E> ExprWithoutBlock<E> for Attribute where E: Expression {}

mod field_access_expr {
    use crate::{identifier::Identifier, type_utils::Dot};

    use super::{ExprWithoutBlock, Expression};

    pub struct FieldAccessExpr {
        object: Box<dyn Expression>,
        dot: Dot,
        field_name: Identifier,
    }

    impl Expression for FieldAccessExpr {}

    impl<E> ExprWithoutBlock<E> for FieldAccessExpr where E: Expression {}
}

mod grouped_expr {
    use crate::type_utils::Parenthesis;

    use super::{ExprWithoutBlock, Expression};

    pub struct GroupedExpr {
        open_parenthesis: Parenthesis,
        expression: Box<dyn Expression>,
        close_parenthesis: Parenthesis,
    }

    impl Expression for GroupedExpr {}

    impl<E> ExprWithoutBlock<E> for GroupedExpr where E: Expression {}
}

mod return_expr {
    use crate::keyword::KeywordKind;

    use super::{ExprWithoutBlock, Expression};

    pub struct ReturnExpr {
        kw_return: KeywordKind,
        expression_opt: Option<Box<dyn Expression>>,
    }

    impl Expression for ReturnExpr {}

    impl<E> ExprWithoutBlock<E> for ReturnExpr where E: Expression {}
}
