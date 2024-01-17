use feo_types::span::{Span, Spanned};

use crate::{
    statement::Statement,
    type_utils::{Comma, Dot, Parenthesis},
};

use super::{ExprWithoutBlock, Expression};

pub struct TupleExpr {
    open_parenthesis: Parenthesis,
    elements_opt: Option<TupleElements>,
    close_parenthesis: Parenthesis,
}

impl Expression for TupleExpr {}

impl<E> ExprWithoutBlock<E> for TupleExpr {}

impl Statement for TupleExpr {}

impl Spanned for TupleExpr {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TupleElements {
    elements: Vec<(Box<dyn Expression>, Comma)>,
    trailing_element_opt: Option<Box<dyn Expression>>,
}

pub struct TupleIndexingExpr {
    object: Box<dyn Expression>,
    dot: Dot,
    index: usize,
}

impl Spanned for TupleIndexingExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.dot.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

impl Expression for TupleIndexingExpr {}

impl<E> ExprWithoutBlock<E> for TupleIndexingExpr {}

impl Statement for TupleIndexingExpr {}
