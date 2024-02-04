use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, Dot, Parenthesis},
};

use crate::{path::PathExprSegment, pattern::Pattern, statement::Statement};

use super::{Assignable, BooleanOperand, Castable, ExprWithoutBlock, Expression, IterableExpr};

pub struct FunctionCallExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    function_operand: Box<Expression<A, B, C, E, I, S, U>>,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams<A, B, C, E, I, S, U>>,
    close_parenthesis: Parenthesis,
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for FunctionCallExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for FunctionCallExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> IterableExpr for FunctionCallExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Spanned for FunctionCallExpr<A, B, C, E, I, S, U>
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
        let s1 = self.function_operand.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)

        // let start_pos = self.function_operand.span().start();
        // let end_pos = self.close_parenthesis.span().end();
        // let source = self.function_operand.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct MethodCallExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    receiver: Box<Expression<A, B, C, E, I, S, U>>,
    dot: Dot,
    method_path: PathExprSegment,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams<A, B, C, E, I, S, U>>,
    close_parenthesis: Parenthesis,
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for MethodCallExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for MethodCallExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> IterableExpr for MethodCallExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Spanned for MethodCallExpr<A, B, C, E, I, S, U>
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
        let s1 = self.receiver.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)

        // let start_pos = self.receiver.span().start();
        // let end_pos = self.close_parenthesis.span().end();
        // let source = self.receiver.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct CallParams<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    first_param: Box<Expression<A, B, C, E, I, S, U>>,
    subsequent_params: Vec<(Comma, Expression<A, B, C, E, I, S, U>)>,
    trailing_comma_opt: Option<Comma>,
}

impl<A, B, C, E, I, S, U> Pattern for CallParams<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Spanned for CallParams<A, B, C, E, I, S, U>
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
        let s1 = self.first_param.span();

        let s2 = match self.subsequent_params.last() {
            Some(sp) => match &self.trailing_comma_opt {
                Some(tc) => tc.span(),
                None => sp.1.span(),
            },
            None => self.first_param.span(),
        };

        Span::join(s1, s2)

        // let start_pos = self.first_param.span().start();
        // let end_pos = match self.subsequent_params.last() {
        //     Some(sp) => match &self.trailing_comma_opt {
        //         Some(tc) => tc.span().end(),
        //         None => sp.1.span().end(),
        //     },
        //     None => self.first_param.span().end(),
        // };

        // let source = self.first_param.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}
