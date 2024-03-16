use feo_types::{
    span::{Span, Spanned},
    utils::{DblPipe, Pipe},
};

use crate::{attribute::OuterAttr, pattern::Pattern, ty::Type};

use super::{BlockExpr, Expression};

#[derive(Debug, Clone)]
pub enum ClosureParamsOpt {
    None(DblPipe),
    Some((Pipe, ClosureParams, Pipe)),
}

impl Spanned for ClosureParamsOpt {
    fn span(&self) -> Span {
        match self {
            ClosureParamsOpt::None(n) => n.span(),
            ClosureParamsOpt::Some(ms) => {
                let s1 = ms.0.span();
                let s2 = ms.2.span();

                Span::join(s1, s2)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClosureWithBlock {
    params: ClosureParamsOpt,
    return_type_opt: Option<Box<Type>>,
    block: BlockExpr,
}

impl Spanned for ClosureWithBlock {
    fn span(&self) -> Span {
        let s1 = self.params.span();
        let s2 = self.block.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct ClosureWithoutBlock {
    pub params: ClosureParamsOpt,
    pub body_operand: Box<Expression>,
}

impl Spanned for ClosureWithoutBlock {
    fn span(&self) -> Span {
        let s1 = self.params.span();
        let s2 = self.body_operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct ClosureParams {
    pub first_param: ClosureParam,
    pub subsequent_params_opt: Option<Vec<ClosureParam>>,
}

impl Spanned for ClosureParams {
    fn span(&self) -> Span {
        let s1 = self.first_param.span();
        let s2 = match &self.subsequent_params_opt {
            Some(sp) => match sp.last() {
                Some(cp) => cp.span(),
                None => self.first_param.span(),
            },
            None => self.first_param.span(),
        };

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct ClosureParam {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub pattern: Box<Pattern>,
    pub type_annotation_opt: Option<Box<Type>>,
}

impl Spanned for ClosureParam {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => self.pattern.span(),
            },
            None => self.pattern.span(),
        };

        let s2 = if let Some(ta) = &self.type_annotation_opt {
            ta.span()
        } else {
            self.pattern.span()
        };

        Span::join(s1, s2)
    }
}
