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
