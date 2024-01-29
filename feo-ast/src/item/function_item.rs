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

pub trait FunctionItem
where
    Self: Item + Type,
{
}

pub enum FuncOrMethodParam {
    FuncParam(FuncParam),
    MethodParam(MethodParam),
}

pub struct FunctionDef<T> {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_func: Keyword,
    func_name: Identifier,
    open_parenthesis: Parenthesis,
    func_params_opt: Option<FuncParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<(ThinArrow, Box<dyn Type>)>,
    func_body: Box<dyn ExprWithBlock<T>>,
}

impl<T> FunctionItem for FunctionDef<T> {}

impl<T> Item for FunctionDef<T> {}

impl<T> Statement for FunctionDef<T> {}

impl<T> Type for FunctionDef<T> {}

impl<T> Spanned for FunctionDef<T> {
    fn span(&self) -> Span {
        let s1 = match self.attributes.first() {
            Some(a) => a.span(),
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_func.span(),
            },
        };

        let s2 = self.func_body.span();

        Span::join(s1, s2)

        // let start_pos = match self.attributes.first() {
        //     Some(a) => a.span().start(),
        //     None => match &self.visibility_opt {
        //         Some(v) => v.span().start(),
        //         None => self.kw_func.span().start(),
        //     },
        // };

        // let end_pos = self.func_body.span().end();
        // let source = self.kw_func.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
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
    semicolon: Semicolon,
}

impl FunctionItem for FunctionSig {}

impl Item for FunctionSig {}

impl Statement for FunctionSig {}

impl Type for FunctionSig {}

impl Spanned for FunctionSig {
    fn span(&self) -> Span {
        let s1 = match self.attributes.first() {
            Some(a) => a.span(),
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_func.span(),
            },
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)

        // let start_pos = match self.attributes.first() {
        //     Some(a) => a.span().start(),
        //     None => match &self.visibility_opt {
        //         Some(v) => v.span().start(),
        //         None => self.kw_func.span().start(),
        //     },
        // };

        // let end_pos = self.semicolon.span().end();
        // let source = self.kw_func.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
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
