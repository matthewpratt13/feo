use feo_types::span::{Span, Spanned};

use crate::{identifier::Identifier, statement::Statement, type_utils::Dot};

use super::{Constant, ExprWithoutBlock, Expression};

pub struct FieldAccessExpr {
    object: Box<dyn Expression>,
    dot: Dot,
    field_name: Identifier,
}

impl Expression for FieldAccessExpr {}

impl<E> ExprWithoutBlock<E> for FieldAccessExpr {}

impl Statement for FieldAccessExpr {}

impl Constant for FieldAccessExpr {}

impl Spanned for FieldAccessExpr {
    fn span(&self) -> Span {
        let start_pos = self.object.span().start();
        let end_pos = self.field_name.span().end();
        let source = self.object.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
