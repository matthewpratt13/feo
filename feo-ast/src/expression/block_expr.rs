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

pub struct BlockExpr<T, U> {
    open_brace: Brace,
    statements: StatementsKind<T, U>,
    close_brace: Brace,
}

impl<T, U> Expression for BlockExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for BlockExpr<T, U> where E: Expression {}

impl<T, U> Spanned for BlockExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.open_brace.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.open_brace.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
