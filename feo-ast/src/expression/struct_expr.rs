use crate::{
    delimiter::{DelimKind, DelimOrientation},
    identifier::Identifier,
    punctuation::PuncKind,
};

use super::{path_expr::SimplePath, Attribute, Expression};

pub enum StructExpr {
    Struct(Struct),
    TupleStruct(TupleStruct),
    UnitStruct(UnitStruct),
}

pub struct Struct {
    path: SimplePath,
    open_brace: (DelimKind, DelimOrientation),
    struct_expr_fields_opt: Option<StructExprFields>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct StructExprFields {
    first_field: StructExprField,
    subsequent_fields: Vec<(PuncKind, StructExprField)>,
}

pub struct StructExprField {
    attribute: Vec<Attribute>,
    data: (Identifier, PuncKind, Box<Expression>),
}

pub struct TupleStruct {
    path: SimplePath,
    open_parenthesis: (DelimKind, DelimOrientation),
    params_opt: Option<(Box<Expression>, Vec<(PuncKind, Box<Expression>)>, PuncKind)>,
    close_parenthesis: (DelimKind, DelimOrientation),
}

pub struct UnitStruct(SimplePath);
