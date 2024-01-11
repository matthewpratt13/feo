use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::ExprWithBlock,
    identifier::Identifier,
    keyword::KeywordKind,
    pattern::Pattern,
    punctuation::PuncKind,
    ty::Type,
};

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
    open_parenthesis: (DelimKind, DelimOrientation),
    func_params_opt: Option<FuncParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
    return_type_opt: Option<(PuncKind, Type)>,
    func_body: ExprWithBlock,
}

pub struct FunctionSignature {
    func_qualifiers_opt: Option<FuncQualifier>,
    kw_func: KeywordKind,
    name: Identifier,
    open_parenthesis: (DelimKind, DelimOrientation),
    func_params_opt: Option<FuncParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
    return_type_opt: Option<(PuncKind, Type)>,
    semicolon: PuncKind,
}

pub struct FuncParams {
    first_param: FuncOrMethodParam,
    subsequent_params: Vec<(PuncKind, FuncParam)>,
    trailing_comma_opt: Option<PuncKind>,
}

pub struct FuncParam {
    pattern: Pattern,
    colon: PuncKind,
    param_type: Type,
}

pub struct MethodParam {
    self_param: SelfParam,
    trailing_comma_opt: Option<PuncKind>,
}

pub struct SelfParam {
    kw_ref_opt: Option<KeywordKind>,
    kw_mut_opt: Option<KeywordKind>,
    kw_self: KeywordKind,
    self_type_opt: Option<(PuncKind, Type)>,
}
