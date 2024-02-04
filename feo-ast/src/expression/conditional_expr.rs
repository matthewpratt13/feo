use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Comma, FatArrow, KwElse, KwIf, KwMatch},
};

use crate::{pattern::Pattern, statement::Statement};

use super::{
    Assignable, BlockExpr, BooleanOperand, Castable, Constant, ExprWithBlock, ExprWithoutBlock,
    Expression, InnerAttr, IterableExpr, OuterAttr,
};

pub trait ConditionalExpr
where
    Self: ExprWithBlock + BooleanOperand + IterableExpr + Constant,
{
}

pub struct IfExpr<B: BooleanOperand + Spanned, E: ExprWithoutBlock, S: Statement> {
    kw_if: KwIf,
    condition_operand: B,
    if_block: BlockExpr<E, S>,
    else_if_blocks_opt: Option<Vec<(KwElse, Box<IfExpr<B, E, S>>)>>,
    else_block_opt: Option<(KwElse, BlockExpr<E, S>)>,
}

impl<B, E, S> ConditionalExpr for IfExpr<B, E, S>
where
    B: BooleanOperand + Spanned,
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<B, E, S> ExprWithBlock for IfExpr<B, E, S>
where
    B: BooleanOperand + Spanned,
    E: ExprWithoutBlock,
    S: Statement,
{
}

impl<B, E, S> BooleanOperand for IfExpr<B, E, S>
where
    B: BooleanOperand + Spanned,
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<B, E, S> IterableExpr for IfExpr<B, E, S>
where
    B: BooleanOperand + Spanned,
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<B, E, S> Constant for IfExpr<B, E, S>
where
    B: BooleanOperand + Spanned,
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<B, E, S> Spanned for IfExpr<B, E, S>
where
    B: BooleanOperand + Spanned,
    E: ExprWithoutBlock,
    S: Statement,
{
    fn span(&self) -> Span {
        let s1 = self.kw_if.span();
        let s2 = match &self.else_block_opt {
            Some(e) => e.1.span(),
            None => match &self.else_if_blocks_opt {
                Some(ei) => match ei.last() {
                    Some(b) => b.1.span(),
                    None => self.if_block.span(),
                },
                None => self.if_block.span(),
            },
        };

        Span::join(s1, s2)

        // let start_pos = self.kw_if.span().start();
        // let temp_end = self.block.span().end();

        // let end_pos = match &self.else_if_block_opt {
        //     Some(s) => match &self.else_block_opt {
        //         Some(t) => t.1.span().end(),
        //         None => s.1.span().end(),
        //     },

        //     None => temp_end,
        // };

        // let source = self.kw_if.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct MatchExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    kw_match: KwMatch,
    scrutinee: A,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    match_arms_opt: Option<MatchArms<A, B, C, E, I, S, U>>,
    close_brace: Brace,
}

impl<A, B, C, E, I, S, U> ConditionalExpr for MatchExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> ExprWithBlock for MatchExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> BooleanOperand for MatchExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> IterableExpr for MatchExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Constant for MatchExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Pattern for MatchExpr<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Spanned for MatchExpr<A, B, C, E, I, S, U>
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
        let s1 = self.kw_match.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)

        // let start_pos = self.kw_match.span().start();
        // let end_pos = self.close_brace.span().end();
        // let source = self.kw_match.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct MatchArms<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    arms: Vec<(
        MatchArm<B>,
        FatArrow,
        Box<Expression<A, B, C, E, I, S, U>>,
        Option<Comma>,
    )>,
    final_arm: (
        MatchArm<B>,
        FatArrow,
        Box<Expression<A, B, C, E, I, S, U>>,
        Option<Comma>,
    ),
}

pub struct MatchArm<B: BooleanOperand + Spanned> {
    attributes: Vec<OuterAttr>,
    pattern: Box<dyn Pattern>,
    match_arm_guard_opt: Option<MatchArmGuard<B>>,
}

pub struct MatchArmGuard<B: BooleanOperand + Spanned> {
    kw_if: KwIf,
    operand: B,
}
