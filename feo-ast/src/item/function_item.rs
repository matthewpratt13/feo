use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::ExprWithBlock,
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
    ty::Type,
};

pub struct FunctionItem {
    func_qualifiers: FuncQualifiers,
    kw_func: KeywordKind,
    name: Identifier,
    open_parenthesis: (DelimKind, DelimOrientation),
    func_params_opt: Option<FuncParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
    return_type_opt: Option<(PuncKind, Type)>,
    func_body: ExprWithBlock,
}

pub struct FunctionSignature {
    func_qualifiers: FuncQualifiers,
    kw_func: KeywordKind,
    name: Identifier,
    open_parenthesis: (DelimKind, DelimOrientation),
    func_params_opt: Option<FuncParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
    return_type_opt: Option<(PuncKind, Type)>,
    semicolon: PuncKind,
}

pub struct FuncQualifiers {}

pub struct FuncParams {}

pub struct FuncParam {}

pub struct SelfParam {}
