use crate::{
    expression::ExprWithBlock, identifier::Identifier, keyword::KeywordKind, pattern::Pattern,
    ty::Type,
};

use super::{Colon, Comma, Parenthesis, Semicolon, ThinArrow};

pub enum FuncQualifier {
    Const(KeywordKind),
    Unsafe(KeywordKind),
    Extern(KeywordKind),
}

pub enum FuncOrMethodParam {
    Func(FuncParam),
    Method(MethodParam),
}

pub struct FunctionItem {
    func_qualifiers_opt: Option<Vec<FuncQualifier>>,
    kw_func: KeywordKind,
    name: Identifier,
    open_parenthesis: Parenthesis,
    func_params_opt: Option<FuncParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<(ThinArrow, Box<Type>)>,
    func_body: ExprWithBlock,
}

pub struct FunctionSignature {
    func_qualifiers_opt: Option<FuncQualifier>,
    kw_func: KeywordKind,
    name: Identifier,
    open_parenthesis: Parenthesis,
    func_params_opt: Option<FuncParams>,
    close_parenthesis: Parenthesis,
    return_type_opt: Option<(ThinArrow, Type)>,
    semicolon: Semicolon,
}

pub struct FuncParams {
    first_param: FuncOrMethodParam,
    subsequent_params: Vec<(Comma, FuncParam)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct FuncParam {
    pattern: Box<Pattern>,
    colon: Colon,
    param_type: Box<Type>,
}

pub struct MethodParam {
    self_param: SelfParam,
    trailing_comma_opt: Option<Comma>,
}

pub struct SelfParam {
    kw_ref_opt: Option<KeywordKind>,
    kw_mut_opt: Option<KeywordKind>,
    kw_self: KeywordKind,
    self_type_opt: Option<(Colon, Box<Type>)>,
}
