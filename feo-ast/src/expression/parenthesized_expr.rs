use crate::{
    span::{Span, Spanned},
    statement::Statement,
    type_utils::Parenthesis,
};

use super::{BooleanOperand, Constant, ExprWithoutBlock, Expression, IterableExpr};

pub struct ParenthesizedExpr {
    open_parenthesis: Parenthesis,
    enclosed_operand: Box<dyn Expression>,
    close_parenthesis: Parenthesis,
}

impl Expression for ParenthesizedExpr {}

impl<E> ExprWithoutBlock<E> for ParenthesizedExpr {}

impl BooleanOperand for ParenthesizedExpr {}

impl Statement for ParenthesizedExpr {}

impl Constant for ParenthesizedExpr {}

impl IterableExpr for ParenthesizedExpr {}

impl Spanned for ParenthesizedExpr {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
