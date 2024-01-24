use crate::{
    identifier::Identifier,
    span::{Span, Spanned},
    statement::Statement,
    type_utils::Dot,
};

use super::{AssignableExpr, Constant, ExprWithoutBlock, Expression};

pub struct FieldAccessExpr {
    operand: Box<dyn AssignableExpr>,
    dot: Dot,
    field_name: Identifier,
}

impl Expression for FieldAccessExpr {}

impl<E> ExprWithoutBlock<E> for FieldAccessExpr {}

impl Statement for FieldAccessExpr {}

impl Constant for FieldAccessExpr {}

impl Spanned for FieldAccessExpr {
    fn span(&self) -> Span {
        let start_pos = self.operand.span().start();
        let end_pos = self.field_name.span().end();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
