use feo_types::span::{Span, Spanned};

use crate::{
    pattern::Pattern,
    statement::Statement,
    ty::Type,
    type_utils::{Colon, Comma, DblPipe, Pipe, ThinArrow},
};

use super::{BlockExpr, ExprWithoutBlock, Expression, OuterAttr};

pub trait ClosureExpr
where
    Self: Sized + Expression + Type + Spanned,
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

pub struct ClosureWithoutReturnType {
    params: ClosureParamsOpt,
    expression: Box<dyn Expression>,
}

impl Expression for ClosureWithoutReturnType {}

impl ClosureExpr for ClosureWithoutReturnType {}

impl<E> ExprWithoutBlock<E> for ClosureWithoutReturnType {}

impl Statement for ClosureWithoutReturnType {}

impl Type for ClosureWithoutReturnType {}

impl Spanned for ClosureWithoutReturnType {
    fn span(&self) -> Span {
        let start_pos = self.params.span().start();
        let end_pos = self.expression.span().end();
        let source = self.params.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ClosureWithReturnType<T, U> {
    params: ClosureParamsOpt,
    thin_arrow: ThinArrow,
    type_bounds: Box<dyn Type>, // cannot be trait object
    block: BlockExpr<T, U>,
}

impl<T, U> Expression for ClosureWithReturnType<T, U> {}

impl<T, U> ClosureExpr for ClosureWithReturnType<T, U> {}

impl<T, U, E> ExprWithoutBlock<E> for ClosureWithReturnType<T, U> {}

impl<T, U> Statement for ClosureWithReturnType<T, U> {}

impl<T, U> Type for ClosureWithReturnType<T, U> {}

impl<T, U> Spanned for ClosureWithReturnType<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.params.span().start();
        let end_pos = self.block.span().end();
        let source = self.params.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
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
        let start_pos = self.first_param.span().start();
        let end_pos = match self.subsequent_params.last() {
            Some(sp) => match &self.trailing_comma_opt {
                Some(tc) => tc.span().end(),
                None => sp.1.span().end(),
            },
            None => self.first_param.span().end(),
        };

        let source = self.first_param.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
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
        let start_pos = if let Some(a) = self.attributes.first() {
            a.span().start()
        } else {
            self.pattern.span().start()
        };

        let end_pos = if let Some(ta) = &self.type_annotation_opt {
            ta.1.span().end()
        } else {
            self.pattern.span().end()
        };

        let source = self.pattern.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
