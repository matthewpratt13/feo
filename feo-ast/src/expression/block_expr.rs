use crate::{
    span::{Span, Spanned},
    statement::{Statement, StatementsWithExpr},
    type_utils::Brace,
};

use super::{BooleanOperand, Constant, ExprWithBlock, ExprWithoutBlock, Expression, IterableExpr};

pub enum StatementKind<T> {
    Statement(Box<dyn Statement>),
    ExprWithoutBlock(Box<dyn ExprWithoutBlock<T>>),
    StatementsWithExpr(StatementsWithExpr<T>),
}

impl<T> Spanned for StatementKind<T> {
    fn span(&self) -> Span {
        match self {
            StatementKind::ExprWithoutBlock(e) => e.span(),
            StatementKind::Statement(s) => s.span(),
            StatementKind::StatementsWithExpr(swe) => swe.span(),
        }
    }
}

pub struct BlockExpr<T> {
    open_brace: Brace,
    statements: Vec<StatementKind<T>>,
    close_brace: Brace,
}

impl<T> Expression for BlockExpr<T> {}

impl<T, E> ExprWithBlock<E> for BlockExpr<T> {}

impl<T> BooleanOperand for BlockExpr<T> where T: 'static {}

impl<T> Constant for BlockExpr<T> where T: 'static {}

impl<T> IterableExpr for BlockExpr<T> where T: 'static {}

impl<T> Spanned for BlockExpr<T> {
    fn span(&self) -> Span {
        let s1 = self.open_brace.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)

        // let start_pos = self.open_brace.span().start();
        // let end_pos = self.close_brace.span().end();
        // let source = self.open_brace.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}
