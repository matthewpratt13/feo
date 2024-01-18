use feo_types::span::{Span, Spanned};

use crate::{
    statement::{Statement, StatementWithExpr},
    type_utils::Brace,
};

use super::{ExprWithBlock, ExprWithoutBlock, Expression};

pub enum StatementsKind<T, U> {
    ExprWithoutBlock(Box<dyn ExprWithoutBlock<T>>),
    Statement(Box<dyn Statement>),
    StatementWithExpr(StatementWithExpr<U>),
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
    open_brace: Brace,
    statements: StatementsKind<T, U>,
    close_brace: Brace,
}

impl<T, U> Expression for BlockExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for BlockExpr<T, U> {}

impl<T, U> Spanned for BlockExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.open_brace.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.statements.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
