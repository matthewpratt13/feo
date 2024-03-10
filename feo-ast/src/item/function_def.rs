use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, KwFunc, KwSelf, Parenthesis, Semicolon},
    Identifier,
};

use crate::{
    attribute::OuterAttr,
    expression::{ExprWithBlock, RefOperator},
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
    MethodParam(MethodParam),
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
    attributes: Option<Vec<OuterAttr>>,
    visibility_opt: Option<VisibilityKind>,
    kw_func: KwFunc,
    function_name: Identifier,
    open_parenthesis: Parenthesis,
    function_params_opt: Option<FunctionParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<Box<Type>>,
}

impl Spanned for FunctionSig {
    fn span(&self) -> Span {
        let s1 = match &self.attributes {
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
pub struct FunctionParams {
    first_param: FuncOrMethodParam,
    subsequent_params: Option<Vec<FunctionParam>>,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Debug, Clone)]
pub struct FunctionParam {
    param_pattern: Box<Pattern>,
    param_type: Box<Type>,
}

#[derive(Debug, Clone)]
pub struct MethodParam {
    self_param: SelfParam,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Debug, Clone)]
pub struct SelfParam {
    ref_operator: RefOperator,
    kw_self: KwSelf,
    type_annotation_opt: Option<Box<Type>>,
}
