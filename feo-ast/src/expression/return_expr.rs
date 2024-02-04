use feo_types::{
    span::{Span, Spanned},
    utils::KwReturn,
};

use crate::statement::Statement;

use super::{Assignable, BooleanOperand, Castable, ExprWithoutBlock, Expression, IterableExpr};

pub struct ReturnExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    kw_return: KwReturn,
    expression_opt: Option<Box<Expression<A, B, C, E, I, S, U>>>,
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for ReturnExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> BooleanOperand for ReturnExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for ReturnExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for ReturnExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
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
