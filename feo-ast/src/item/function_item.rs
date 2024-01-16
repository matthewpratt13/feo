use crate::{
    expression::{Attribute, ExprWithBlock},
    identifier::Identifier,
    keyword::KeywordKind,
    pattern::Pattern,
    ty::Type,
    type_utils::{Colon, Comma, Parenthesis, Semicolon, ThinArrow},
};

use super::VisibilityKind;

pub enum FuncQualifier {
    Const(KeywordKind),
    Unsafe(KeywordKind),
    Extern(KeywordKind),
}

pub enum FuncOrMethodParam {
    Func(FuncParam),
    Method(MethodParam),
}

pub struct FunctionWithBody<T> {
    attributes: Vec<Attribute>,
    visibility_opt: Option<VisibilityKind>,
    func_qualifiers_opt: Option<Vec<FuncQualifier>>,
    kw_func: KeywordKind,
    name: Identifier,
    open_parenthesis: Parenthesis,
    func_params_opt: Option<FuncParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<(ThinArrow, Box<dyn Type>)>,
    func_body: Box<dyn ExprWithBlock<T>>,
}

pub struct FunctionSignatureOnly {
    attributes: Vec<Attribute>,
    visibility_opt: Option<VisibilityKind>,
    func_qualifiers_opt: Option<FuncQualifier>,
    kw_func: KeywordKind,
    name: Identifier,
    open_parenthesis: Parenthesis,
    func_params_opt: Option<FuncParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<(ThinArrow, Box<dyn Type>)>,
    semicolon: Semicolon,
}

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
    kw_ref_opt: Option<KeywordKind>,
    kw_mut_opt: Option<KeywordKind>,
    kw_self: KeywordKind,
    self_type_opt: Option<(Colon, Box<dyn Type>)>,
}
