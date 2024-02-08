use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Comma, KwFunc, KwMut, KwRef, KwSelf, Parenthesis, Semicolon, ThinArrow},
    Identifier,
};

use crate::{
    expression::{ExprWithBlock, OuterAttr},
    pattern::Pattern,
    ty::Type,
};

use super::{Item, VisibilityKind};

#[derive(Clone)]
pub enum FunctionItem {
    FuncSig((FunctionSig, Semicolon)),
    FuncDef(FunctionDef),
}

impl Item for FunctionItem {}

impl Type for FunctionItem {}

impl Spanned for FunctionItem {
    fn span(&self) -> Span {
        match self {
            FunctionItem::FuncSig(fs) => {
                let s1 = match fs.0.attributes.first() {
                    Some(a) => a.span(),
                    None => match &fs.0.visibility_opt {
                        Some(v) => v.span(),
                        None => fs.0.kw_func.span(),
                    },
                };

                let s2 = fs.1.span();

                Span::join(s1, s2)
            }
            FunctionItem::FuncDef(fd) => fd.span(),
        }
    }
}

#[derive(Clone)]
pub enum FuncOrMethodParam {
    FuncParam(FunctionParam),
    MethodParam(MethodParam),
}

#[derive(Clone)]
pub struct FunctionDef {
    function_sig: FunctionSig,
    function_body: ExprWithBlock,
}

impl Spanned for FunctionDef {
    fn span(&self) -> Span {
        let s1 = match self.function_sig.attributes.first() {
            Some(a) => a.span(),
            None => match &self.function_sig.visibility_opt {
                Some(v) => v.span(),
                None => self.function_sig.kw_func.span(),
            },
        };

        let s2 = self.function_body.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct FunctionSig {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_func: KwFunc,
    function_name: Identifier,
    open_parenthesis: Parenthesis,
    function_params_opt: Option<FunctionParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<(ThinArrow, Box<dyn Type>)>,
}

impl Spanned for FunctionSig {
    fn span(&self) -> Span {
        let s1 = match self.attributes.first() {
            Some(a) => a.span(),
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_func.span(),
            },
        };

        let s2 = match &self.return_type_opt {
            Some(rt) => rt.1.span(),
            None => self.close_parenthesis.span(),
        };

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct FunctionParams {
    first_param: FuncOrMethodParam,
    subsequent_params: Vec<(Comma, FunctionParam)>,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Clone)]
pub struct FunctionParam {
    param_pattern: Box<dyn Pattern>,
    colon: Colon,
    param_type: Box<dyn Type>,
}

#[derive(Clone)]
pub struct MethodParam {
    self_param: SelfParam,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Clone)]
pub struct SelfParam {
    kw_ref_opt: Option<KwRef>,
    kw_mut_opt: Option<KwMut>,
    kw_self: KwSelf,
    type_annotation_opt: Option<(Colon, Box<dyn Type>)>,
}
