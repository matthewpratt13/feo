use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Comma, DblPipe, Pipe, ThinArrow},
};

use crate::{pattern::Pattern, statement::Statement, ty::Type};

use super::{
    Assignable, BlockExpr, BooleanOperand, Castable, ExprWithBlock, ExprWithoutBlock, Expression,
    IterableExpr, OuterAttr,
};

pub trait ClosureExpr
where
    Self: Sized + BooleanOperand + IterableExpr + Type,
{
}

pub enum ClosureParamsOpt {
    None(DblPipe),
    MaybeSome((Pipe, Option<ClosureParams>, Pipe)),
}

impl Spanned for ClosureParamsOpt {
    fn span(&self) -> Span {
        match self {
            ClosureParamsOpt::None(n) => n.span(),
            ClosureParamsOpt::MaybeSome(ms) => {
                let start_pos = ms.0.span().start();
                let end_pos = ms.2.span().end();
                let source = ms.0.span().source();

                let span = Span::new(source.as_str(), start_pos, end_pos);

                span
            }
        }
    }
}

pub struct ClosureWithBlock<E: ExprWithoutBlock, S: Statement> {
    params: ClosureParamsOpt,
    return_type_opt: Option<(ThinArrow, Box<dyn Type>)>,
    block: BlockExpr<E, S>,
}

impl<E, S> ClosureExpr for ClosureWithBlock<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> ExprWithBlock for ClosureWithBlock<E, S>
where
    E: ExprWithoutBlock,
    S: Statement,
{
}

impl<E, S> BooleanOperand for ClosureWithBlock<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> IterableExpr for ClosureWithBlock<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> Type for ClosureWithBlock<E, S>
where
    E: ExprWithoutBlock,
    S: Statement,
{
}

impl<E, S> Spanned for ClosureWithBlock<E, S>
where
    E: ExprWithoutBlock,
    S: Statement,
{
    fn span(&self) -> Span {
        let s1 = self.params.span();
        let s2 = self.block.span();

        Span::join(s1, s2)

        // let start_pos = self.params.span().start();
        // let end_pos = self.block.span().end();
        // let source = self.params.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct ClosureWithoutBlock<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    params: ClosureParamsOpt,
    body_operand: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> ClosureExpr for ClosureWithoutBlock<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> ExprWithoutBlock for ClosureWithoutBlock<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> BooleanOperand for ClosureWithoutBlock<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> IterableExpr for ClosureWithoutBlock<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Type for ClosureWithoutBlock<A, B, C, E, I, S, U>
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

impl<A, B, C, E, I, S, U> Spanned for ClosureWithoutBlock<A, B, C, E, I, S, U>
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
        let s1 = self.params.span();
        let s2 = self.body_operand.span();

        Span::join(s1, s2)

        // let start_pos = self.params.span().start();
        // let end_pos = self.body_operand.span().end();
        // let source = self.params.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct ClosureParams {
    first_param: ClosureParam,
    subsequent_params: Vec<(Comma, ClosureParam)>,
    trailing_comma_opt: Option<Comma>,
}

impl Pattern for ClosureParams {}

impl Spanned for ClosureParams {
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

pub struct ClosureParam {
    attributes: Vec<OuterAttr>,
    pattern: Box<dyn Pattern>,
    type_annotation_opt: Option<(Colon, Box<dyn Type>)>,
}

impl Pattern for ClosureParam {}

impl Spanned for ClosureParam {
    fn span(&self) -> Span {
        let s1 = if let Some(a) = self.attributes.first() {
            a.span()
        } else {
            self.pattern.span()
        };

        let s2 = if let Some(ta) = &self.type_annotation_opt {
            ta.1.span()
        } else {
            self.pattern.span()
        };

        Span::join(s1, s2)

        // let start_pos = if let Some(a) = self.attributes.first() {
        //     a.span().start()
        // } else {
        //     self.pattern.span().start()
        // };

        // let end_pos = if let Some(ta) = &self.type_annotation_opt {
        //     ta.1.span().end()
        // } else {
        //     self.pattern.span().end()
        // };

        // let source = self.pattern.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}
