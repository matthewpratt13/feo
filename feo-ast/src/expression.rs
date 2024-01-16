#![allow(dead_code)]

use feo_types::U256;

mod array_expr;
mod block_expr;
mod call_expr;
mod closure_expr;
mod conditional_expr;
mod iteration_expr;
mod operator_expr;
mod range_expr;
mod struct_expr;
mod tuple_expr;

use crate::statement::Statement;
use crate::ty::Type;

pub use self::attribute::{InnerAttr, OuterAttr};
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

pub trait ClosureExpr<C>
where
    C: Expression,
{
}

impl<C> Expression for dyn ClosureExpr<C> {}

impl<C> Type for dyn ClosureExpr<C> {}

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
impl<L> LiteralExpr<L> for char where L: Expression {}

impl Expression for &'static str {}
impl<L> LiteralExpr<L> for &'static str where L: Expression {}

impl Expression for i64 {}
impl<L> LiteralExpr<L> for i64 where L: Expression {}

impl Expression for u64 {}
impl<L> LiteralExpr<L> for u64 where L: Expression {}

impl Expression for U256 {}
impl<L> LiteralExpr<L> for U256 where L: Expression {}

impl Expression for f64 {}
impl<L> LiteralExpr<L> for f64 where L: Expression {}

impl Expression for &[u8; 32] {}
impl<L> LiteralExpr<L> for &[u8; 32] where L: Expression {}

impl Expression for bool {}
impl<L> LiteralExpr<L> for bool where L: Expression {}

mod attribute {
    use feo_types::span::{Span, Spanned};

    use crate::{
        path::SimplePath,
        type_utils::{Bracket, HashBang, HashSign},
    };

    use super::{ExprWithBlock, ExprWithoutBlock, Expression};

    pub struct InnerAttr {
        hash_bang: HashBang,
        open_bracket: Bracket,
        attribute_path: SimplePath,
        close_bracket: Bracket,
    }

    impl Expression for InnerAttr {}

    impl<E> ExprWithBlock<E> for InnerAttr where E: Expression {}

    impl<E> ExprWithoutBlock<E> for InnerAttr where E: Expression {}

    impl Spanned for InnerAttr {
        fn span(&self) -> Span {
            let start_pos = self.hash_bang.span().start();
            let end_pos = self.close_bracket.span().end();
            let source = self.hash_bang.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }

    pub struct OuterAttr {
        hash: HashSign,
        open_bracket: Bracket,
        attribute_path: SimplePath,
        close_bracket: Bracket,
    }

    impl Expression for OuterAttr {}

    impl<E> ExprWithBlock<E> for OuterAttr where E: Expression {}

    impl<E> ExprWithoutBlock<E> for OuterAttr where E: Expression {}

    impl Spanned for OuterAttr {
        fn span(&self) -> Span {
            let start_pos = self.hash.span().start();
            let end_pos = self.close_bracket.span().end();
            let source = self.hash.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

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
    use feo_types::span::{Span, Spanned};

    use crate::type_utils::Parenthesis;

    use super::{ExprWithoutBlock, Expression};

    pub struct GroupedExpr {
        open_parenthesis: Parenthesis,
        expression: Box<dyn Expression>,
        close_parenthesis: Parenthesis,
    }

    impl Expression for GroupedExpr {}

    impl<E> ExprWithoutBlock<E> for GroupedExpr where E: Expression {}

    impl Spanned for GroupedExpr {
        fn span(&self) -> Span {
            let start_pos = self.open_parenthesis.span().start();
            let end_pos = self.close_parenthesis.span().end();
            let source = self.open_parenthesis.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

mod return_expr {
    use feo_types::span::{Span, Spanned};

    use crate::keyword::Keyword;

    use super::{ExprWithoutBlock, Expression};

    pub struct ReturnExpr {
        kw_return: Keyword,
        expression_opt: Option<Box<dyn Expression>>,
    }

    impl Expression for ReturnExpr {}

    impl<E> ExprWithoutBlock<E> for ReturnExpr where E: Expression {}

    impl Spanned for ReturnExpr {
        fn span(&self) -> Span {
            let start_pos = self.kw_return.span().start();
            todo!()
        }
    }
}
