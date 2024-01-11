use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::ExprWithBlock,
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
    ty::Type,
};

pub enum FuncQualifier {
    Const(KeywordKind),
    Unsafe(KeywordKind),
    Extern(KeywordKind),
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

pub struct FuncParams {}

pub struct FuncParam {}

pub struct SelfParam {}
