use feo_types::{
    span::{Span, Spanned},
    type_utils::{Ampersand, KwFunc, KwMut, KwSelf, Parenthesis, Semicolon},
    Identifier,
};

use crate::{
    attribute::OuterAttr,
    expression::{ExprWithBlock, TermCollection},
    pattern::Pattern,
    ty::Type,
};

use super::VisibilityKind;

#[derive(Debug, Clone)]
pub enum FunctionDefKind {
    FuncSig((FunctionSig, Semicolon)),
    FuncDef(FunctionWithBlock),
}

impl Spanned for FunctionDefKind {
    fn span(&self) -> Span {
        match self {
            FunctionDefKind::FuncSig(fs) => fs.0.span(),
            FunctionDefKind::FuncDef(fd) => fd.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FuncOrMethodParam {
    FuncParam(FunctionParam),
    MethodParam(SelfParam),
}

#[derive(Debug, Clone)]
pub struct FunctionWithBlock {
    function_sig: FunctionSig,
    function_body: ExprWithBlock,
}

impl Spanned for FunctionWithBlock {
    fn span(&self) -> Span {
        let s1 = self.function_sig.span();
        let s2 = self.function_body.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_func: KwFunc,
    pub function_name: Identifier,
    pub open_parenthesis: Parenthesis,
    pub function_params_opt: Option<TermCollection<FunctionParam>>,
    pub close_parenthesis: Parenthesis,
    pub return_type_opt: Option<Box<Type>>,
}

impl Spanned for FunctionSig {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_func.span(),
                },
            },
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_func.span(),
            },
        };

        let s2 = match &self.return_type_opt {
            Some(rt) => rt.span(),
            None => self.close_parenthesis.span(),
        };

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub param_pattern: Box<Pattern>,
    pub param_type: Box<Type>,
}

#[derive(Debug, Clone)]
pub struct SelfParam {
    pub ampersand_opt: Option<Ampersand>,
    pub kw_mut_opt: Option<KwMut>,
    pub kw_self: KwSelf,
    pub type_annotation_opt: Option<Box<Type>>,
}
