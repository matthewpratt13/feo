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

use super::{AssociatedItem, Item, VisibilityKind};

pub trait FunctionItem
where
    Self: Item,
{
}

pub enum FuncOrMethodParam {
    Func(FuncParam),
    Method(MethodParam),
}

pub struct FunctionDefWithoutBody {
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

impl FunctionItem for FunctionDefWithoutBody {}

impl Item for FunctionDefWithoutBody {}

impl Statement for FunctionDefWithoutBody {}

impl AssociatedItem for FunctionDefWithoutBody {}

impl Spanned for FunctionDefWithoutBody {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_func.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_func.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct FunctionDefWithBody<T> {
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

impl<T> FunctionItem for FunctionDefWithBody<T> {}

impl<T> Item for FunctionDefWithBody<T> {}

impl<T> Statement for FunctionDefWithBody<T> {}

impl<T> Spanned for FunctionDefWithBody<T> {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_func.span().start(),
            },
        };

        let end_pos = self.func_body.span().end();
        let source = self.kw_func.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
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
