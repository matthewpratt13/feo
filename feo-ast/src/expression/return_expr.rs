use feo_types::{
    span::{Span, Spanned},
    Keyword,
};

use crate::statement::Statement;

use super::{BooleanOperand, ExprWithoutBlock, Expression, IterableExpr};

pub struct ReturnExpr {
    kw_return: Keyword,
    expression_opt: Option<Box<dyn Expression>>,
}

impl Expression for ReturnExpr {}

impl<E> ExprWithoutBlock<E> for ReturnExpr {}

impl Statement for ReturnExpr {}

impl BooleanOperand for ReturnExpr {}

impl IterableExpr for ReturnExpr {}

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
