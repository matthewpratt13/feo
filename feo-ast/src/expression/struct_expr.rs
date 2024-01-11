use crate::{
    delimiter::{DelimKind, DelimOrientation},
    identifier::Identifier,
    item::{Colon, Comma},
    path::SimplePath,
};

use super::{Attribute, Expression};

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
    subsequent_fields: Vec<(Comma, StructExprField)>,
}

pub struct StructExprField {
    attributes: Vec<Attribute>,
    data: (Identifier, Colon, Box<Expression>),
}

pub struct TupleStruct {
    path: SimplePath,
    open_parenthesis: (DelimKind, DelimOrientation),
    params_opt: Option<(Box<Expression>, Vec<(Comma, Expression)>, Comma)>,
    close_parenthesis: (DelimKind, DelimOrientation),
}

pub struct TupleStructExprFields {
    first_field: TupleStructExprField,
    subsequent_fields: Vec<(Comma, TupleStructExprField)>,
}

pub struct TupleStructExprField {
    attributes: Vec<Attribute>,
    data: Box<Expression>,
}

pub struct UnitStruct(SimplePath);
