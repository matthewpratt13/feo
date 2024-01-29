use crate::{
    expression::{ExprWithBlock, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    pattern::Pattern,
    span::{Span, Spanned},
    statement::Statement,
    ty::Type,
    type_utils::{Colon, Comma, Parenthesis, Semicolon, ThinArrow},
};

use super::{Item, VisibilityKind};

pub enum FunctionItem<T> {
    FuncSig((FunctionSig, Semicolon)),
    FuncDef(FunctionDef<T>),
}

impl<T> Item for FunctionItem<T> {}

impl<T> Statement for FunctionItem<T> {}

impl<T> Type for FunctionItem<T> {}

impl<T> Spanned for FunctionItem<T> {
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

pub enum FuncOrMethodParam {
    FuncParam(FuncParam),
    MethodParam(MethodParam),
}

pub struct FunctionDef<T> {
    func_sig: FunctionSig,
    func_body: Box<dyn ExprWithBlock<T>>,
}

impl<T> Spanned for FunctionDef<T> {
    fn span(&self) -> Span {
        let s1 = match self.func_sig.attributes.first() {
            Some(a) => a.span(),
            None => match &self.func_sig.visibility_opt {
                Some(v) => v.span(),
                None => self.func_sig.kw_func.span(),
            },
        };

        let s2 = self.func_body.span();

        Span::join(s1, s2)
    }
}

pub struct FunctionSig {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_func: Keyword,
    func_name: Identifier,
    open_parenthesis: Parenthesis,
    func_params_opt: Option<FuncParams>,
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

pub struct FuncParams {
    first_param: FuncOrMethodParam,
    subsequent_params: Vec<(Comma, FuncParam)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct FuncParam {
    param_pattern: Box<dyn Pattern>,
    colon: Colon,
    param_type: Box<dyn Type>,
}

pub struct MethodParam {
    self_param: SelfParam,
    trailing_comma_opt: Option<Comma>,
}

pub struct SelfParam {
    kw_ref_opt: Option<Keyword>,
    kw_mut_opt: Option<Keyword>,
    kw_self: Keyword,
    type_annotation_opt: Option<(Colon, Box<dyn Type>)>,
}
