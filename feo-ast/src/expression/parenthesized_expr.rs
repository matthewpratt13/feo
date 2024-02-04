use feo_types::{
    span::{Span, Spanned},
    utils::Parenthesis,
};

use crate::statement::Statement;

use super::{
    Assignable, BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr,
};

pub struct ParenthesizedExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    open_parenthesis: Parenthesis,
    enclosed_operand: Box<Expression<A, B, C, E, I, S, U>>,
    close_parenthesis: Parenthesis,
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for ParenthesizedExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for ParenthesizedExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> IterableExpr for ParenthesizedExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Constant for ParenthesizedExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Spanned for ParenthesizedExpr<A, B, C, E, I, S, U>
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
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
