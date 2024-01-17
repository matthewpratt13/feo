#![allow(dead_code)]

use feo_types::span::Spanned;

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
    Self: Sized + ExprWithBlock<E>,
{
}

pub trait IterationExpr<E>
where
    Self: Sized + ExprWithBlock<E>,
{
}

pub trait LiteralExpr<E>
where
    Self: Sized + ExprWithoutBlock<E>,
{
}

pub trait OperatorExpr<E>
where
    Self: Sized + ExprWithoutBlock<E>,
{
}

pub trait RangeExpr<E>
where
    Self: Sized + ExprWithoutBlock<E>,
{
}

pub trait StructExpr<E>
where
    Self: Sized + ExprWithoutBlock<E>,
{
}

mod attribute {
    use feo_types::span::{Span, Spanned};

    use crate::{
        path::SimplePath,
        statement::Statement,
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

    impl<E> ExprWithoutBlock<E> for InnerAttr {}

    impl Statement for InnerAttr {}

    impl Spanned for InnerAttr {
        fn span(&self) -> Span {
            let start_pos = self.hash_bang.span().start();
            let end_pos = self.close_bracket.span().end();
            let source = self.attribute_path.span().source();

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

    impl<E> ExprWithBlock<E> for OuterAttr {}

    impl<E> ExprWithoutBlock<E> for OuterAttr {}

    impl Statement for OuterAttr {}

    impl Spanned for OuterAttr {
        fn span(&self) -> Span {
            let start_pos = self.hash.span().start();
            let end_pos = self.close_bracket.span().end();
            let source = self.attribute_path.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

mod field_access_expr {
    use feo_types::span::{Span, Spanned};

    use crate::{identifier::Identifier, statement::Statement, type_utils::Dot};

    use super::{ExprWithoutBlock, Expression};

    pub struct FieldAccessExpr {
        object: Box<dyn Expression>,
        dot: Dot,
        field_name: Identifier,
    }

    impl Expression for FieldAccessExpr {}

    impl<E> ExprWithoutBlock<E> for FieldAccessExpr {}

    impl Statement for FieldAccessExpr {}

    impl Spanned for FieldAccessExpr {
        fn span(&self) -> Span {
            let start_pos = self.object.span().start();
            let end_pos = self.field_name.span().end();
            let source = self.field_name.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

mod grouped_expr {
    use feo_types::span::{Span, Spanned};

    use crate::{statement::Statement, type_utils::Parenthesis};

    use super::{ExprWithoutBlock, Expression};

    pub struct GroupedExpr {
        open_parenthesis: Parenthesis,
        expression: Box<dyn Expression>,
        close_parenthesis: Parenthesis,
    }

    impl Expression for GroupedExpr {}

    impl<E> ExprWithoutBlock<E> for GroupedExpr {}

    impl Statement for GroupedExpr {}

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

    use crate::{keyword::Keyword, statement::Statement};

    use super::{ExprWithoutBlock, Expression};

    pub struct ReturnExpr {
        kw_return: Keyword,
        expression_opt: Option<Box<dyn Expression>>,
    }

    impl Expression for ReturnExpr {}

    impl<E> ExprWithoutBlock<E> for ReturnExpr {}

    impl Statement for ReturnExpr {}

    impl Spanned for ReturnExpr {
        fn span(&self) -> Span {
            let start_pos = self.kw_return.span().start();
            let end_pos = if let Some(e) = &self.expression_opt {
                e.span().end()
            } else {
                self.kw_return.span().end()
            };

            let source = self.kw_return.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}
