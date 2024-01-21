use feo_types::span::{Span, Spanned};

use crate::{
    statement::{Statement, StatementsWithExpr},
    type_utils::Brace,
};

use super::{Constant, ExprWithBlock, ExprWithoutBlock, Expression, InnerAttr};

pub enum StatementsKind<T, U> {
    Statement(Box<dyn Statement>),
    ExprWithoutBlock(Box<dyn ExprWithoutBlock<T>>),
    StatementWithExpr(StatementsWithExpr<U>),
}

impl<T, U> Spanned for StatementsKind<T, U> {
    fn span(&self) -> Span {
        match self {
            StatementsKind::ExprWithoutBlock(e) => e.span(),
            StatementsKind::Statement(s) => s.span(),
            StatementsKind::StatementWithExpr(swe) => swe.span(),
        }
    }
}

pub struct BlockExpr<T, U> {
    attributes: Vec<InnerAttr>,
    open_brace: Brace,
    statements: Vec<StatementsKind<T, U>>,
    close_brace: Brace,
}

impl<T, U> Expression for BlockExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for BlockExpr<T, U> {}

impl<T, U> Constant for BlockExpr<T, U>
where
    T: 'static,
    U: 'static,
{
}

impl<T, U> Spanned for BlockExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = if let Some(a) = self.attributes.first() {
            a.span().start()
        } else {
            self.open_brace.span().start()
        };

        let end_pos = self.close_brace.span().end();
        let source = self.open_brace.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
