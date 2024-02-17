use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Comma, DblPipe, Pipe, ThinArrow},
};

use crate::{attribute::OuterAttr, pattern::Pattern, ty::Type};

use super::{BlockExpr, Returnable};

#[derive(Clone)]
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

#[derive(Clone)]
pub enum ClosureExprKind {
    ClosureWithBlock(ClosureWithBlock),
    ClosureWithoutBlock(ClosureWithoutBlock),
}

#[derive(Clone)]
pub struct ClosureWithBlock {
    params: ClosureParamsOpt,
    return_type_opt: Option<(ThinArrow, Box<Type>)>,
    block: BlockExpr,
}

impl Spanned for ClosureWithBlock {
    fn span(&self) -> Span {
        let s1 = self.params.span();
        let s2 = self.block.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct ClosureWithoutBlock {
    params: ClosureParamsOpt,
    body_operand: Box<Returnable>,
}

impl Spanned for ClosureWithoutBlock {
    fn span(&self) -> Span {
        let s1 = self.params.span();
        let s2 = self.body_operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct ClosureParams {
    first_param: ClosureParam,
    subsequent_params: Vec<(Comma, ClosureParam)>,
    trailing_comma_opt: Option<Comma>,
}

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
    }
}

#[derive(Clone)]
pub struct ClosureParam {
    attributes: Vec<OuterAttr>,
    pattern: Box<Pattern>,
    type_annotation_opt: Option<(Colon, Box<Type>)>,
}

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
    }
}
