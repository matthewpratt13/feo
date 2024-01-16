use feo_types::span::{Span, Spanned};

use crate::{
    pattern::Pattern,
    ty::Type,
    type_utils::{Colon, Comma, DblPipe, Pipe, ThinArrow},
};

use super::{BlockExpr, ClosureExpr, Expression, OuterAttr};

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

impl<C> ClosureExpr<C> for ClosureWithoutReturnType where C: Expression {}

impl Type for ClosureWithoutReturnType {}

impl Spanned for ClosureWithoutReturnType {
    fn span(&self) -> Span {
        let start_pos = self.params.span().start();
        let end_pos = todo!();
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

impl<T, U, C> ClosureExpr<C> for ClosureWithReturnType<T, U> where C: Expression {}

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

pub struct ClosureParam {
    attributes: Vec<OuterAttr>,
    pattern: Box<dyn Pattern>,
    type_annotation_opt: Option<(Colon, Box<dyn Type>)>,
}
