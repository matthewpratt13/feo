use feo_types::{
    span::{Span, Spanned},
    type_utils::{DblPipe, Pipe},
};

use crate::{attribute::OuterAttr, pattern::Pattern, ty::Type};

use super::{BlockExpr, Expression, TermCollection};

#[derive(Debug, Clone)]
pub enum ClosureParamsOpt {
    None(DblPipe),
    Some((Pipe, TermCollection<ClosureParam>, Pipe)),
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
pub struct ClosureWithBlock {
    pub params: ClosureParamsOpt,
    pub return_type_opt: Option<Box<Type>>,
    pub block: BlockExpr,
}

impl Spanned for ClosureWithBlock {
    fn span(&self) -> Span {
        let s1 = self.params.span();
        let s2 = self.block.span();

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
