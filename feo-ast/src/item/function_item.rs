use feo_types::span::{Span, Spanned};

use crate::{
    expression::{ExprWithBlock, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    pattern::Pattern,
    ty::Type,
    type_utils::{Colon, Comma, Parenthesis, Semicolon, ThinArrow},
};

use super::{AssociatedItem, FunctionItem, Item, VisibilityKind, WhereClause};

pub enum FuncQualifier {
    Const(Keyword),
    Unsafe(Keyword),
    Extern(Keyword),
}

impl Spanned for FuncQualifier {
    fn span(&self) -> Span {
        match self {
            FuncQualifier::Const(c) => c.span(),
            FuncQualifier::Unsafe(u) => u.span(),
            FuncQualifier::Extern(e) => e.span(),
        }
    }
}

pub enum FuncOrMethodParam {
    Func(FuncParam),
    Method(MethodParam),
}

pub struct FunctionSignatureOnly {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    func_qualifiers_opt: Option<FuncQualifier>,
    kw_func: Keyword,
    name: Identifier,
    open_parenthesis: Parenthesis,
    func_params_opt: Option<FuncParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<(ThinArrow, Box<dyn Type>)>,
    semicolon: Semicolon,
}

impl Item for FunctionSignatureOnly {}

impl<A> AssociatedItem<A> for FunctionSignatureOnly where A: Item {}

impl<F> FunctionItem<F> for FunctionSignatureOnly where F: Item {}

impl Type for FunctionSignatureOnly {}

impl Spanned for FunctionSignatureOnly {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => match &self.func_qualifiers_opt {
                    Some(fq) => fq.span().start(),
                    None => self.kw_func.span().start(),
                },
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.name.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct FunctionWithBody<T> {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    func_qualifiers_opt: Option<Vec<FuncQualifier>>,
    kw_func: Keyword,
    name: Identifier,
    open_parenthesis: Parenthesis,
    func_params_opt: Option<FuncParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<(ThinArrow, Box<dyn Type>)>,
    where_clause_opt: Option<WhereClause>,
    func_body: Box<dyn ExprWithBlock<T>>,
}

impl<T> Item for FunctionWithBody<T> {}

impl<T, A> AssociatedItem<A> for FunctionWithBody<T> where A: Item {}

impl<T, F> FunctionItem<F> for FunctionWithBody<T> where F: Item {}

impl<T> Type for FunctionWithBody<T> {}

pub struct FuncParams {
    first_param: FuncOrMethodParam,
    subsequent_params: Vec<(Comma, FuncParam)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct FuncParam {
    pattern: Box<dyn Pattern>,
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
    self_type_opt: Option<(Colon, Box<dyn Type>)>,
}
