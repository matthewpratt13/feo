use feo_types::{
    span::{Span, Spanned},
    utils::Brace,
};

use crate::statement::Statement;

use super::{BooleanOperand, Constant, ExprWithBlock, ExprWithoutBlock, IterableExpr};

pub struct BlockExpr<T> {
    open_brace: Brace,
    statements: Vec<Box<dyn Statement>>,
    final_operand_opt: Option<Box<dyn ExprWithoutBlock<T>>>,
    close_brace: Brace,
}

// impl<T> Expression for BlockExpr<T> {}

impl<T, E> ExprWithBlock<E> for BlockExpr<T> {}

impl<T> BooleanOperand for BlockExpr<T> where T: 'static {}

impl<T> IterableExpr for BlockExpr<T> where T: 'static {}

impl<T> Constant for BlockExpr<T> where T: 'static {}

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
