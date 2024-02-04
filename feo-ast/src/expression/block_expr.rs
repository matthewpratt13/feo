use feo_types::{
    span::{Span, Spanned},
    utils::Brace,
};

use crate::statement::Statement;

use super::{BooleanOperand, Constant, ExprWithBlock, ExprWithoutBlock, IterableExpr};

pub struct BlockExpr<E: ExprWithoutBlock, S: Statement> {
    open_brace: Brace,
    statements: Vec<S>,
    final_operand_opt: Option<E>,
    close_brace: Brace,
}

impl<E, S> ExprWithBlock for BlockExpr<E, S>
where
    E: ExprWithoutBlock,
    S: Statement,
{
}

impl<E, S> BooleanOperand for BlockExpr<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> IterableExpr for BlockExpr<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> Constant for BlockExpr<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> Spanned for BlockExpr<E, S>
where
    E: ExprWithoutBlock,
    S: Statement,
{
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
