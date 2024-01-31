use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Comma, DblPipe, Pipe, ThinArrow},
};

use crate::{pattern::Pattern, ty::Type};

use super::{
    BlockExpr, BooleanOperand, ExprWithBlock, ExprWithoutBlock, Expression, IterableExpr, OuterAttr,
};

pub trait ClosureExpr
where
    Self: Sized + Expression + BooleanOperand + IterableExpr + Type,
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

pub struct ClosureWithBlock<T> {
    params: ClosureParamsOpt,
    return_type_opt: Option<(ThinArrow, Box<dyn Type>)>,
    block: BlockExpr<T>,
}

impl<T> ClosureExpr for ClosureWithBlock<T> where T: 'static {}

impl<T> Expression for ClosureWithBlock<T> {}

impl<T, E> ExprWithBlock<E> for ClosureWithBlock<T> {}

impl<T> BooleanOperand for ClosureWithBlock<T> where T: 'static {}

impl<T> IterableExpr for ClosureWithBlock<T> where T: 'static {}

impl<T> Type for ClosureWithBlock<T> {}

impl<T> Spanned for ClosureWithBlock<T> {
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

pub struct ClosureWithoutBlock {
    params: ClosureParamsOpt,
    body_operand: Box<dyn Expression>,
}

impl ClosureExpr for ClosureWithoutBlock {}

impl Expression for ClosureWithoutBlock {}

impl<E> ExprWithoutBlock<E> for ClosureWithoutBlock {}

impl BooleanOperand for ClosureWithoutBlock {}

impl IterableExpr for ClosureWithoutBlock {}

impl Type for ClosureWithoutBlock {}

impl Spanned for ClosureWithoutBlock {
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
