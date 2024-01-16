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

pub struct ClosureWithoutReturnType {
    params: ClosureParamsOpt,
    expression: Box<dyn Expression>,
}

impl Expression for ClosureWithoutReturnType {}

impl<C> ClosureExpr<C> for ClosureWithoutReturnType where C: Expression {}

pub struct ClosureWithReturnType<T, U> {
    params: ClosureParamsOpt,
    thin_arrow: ThinArrow,
    type_bounds: Box<dyn Type>, // cannot be trait object
    block: BlockExpr<T, U>,
}

impl<T, U> Expression for ClosureWithReturnType<T, U> {}

impl<T, U, C> ClosureExpr<C> for ClosureWithReturnType<T, U> where C: Expression {}

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
