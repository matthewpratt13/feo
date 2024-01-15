use crate::{
    identifier::Identifier,
    type_utils::{Brace, Colon, Comma, Parenthesis},
    path::SimplePath,
};

use super::{Attribute, ExpressionKind};

pub enum StructExprKind {
    Struct(Struct),
    TupleStruct(TupleStruct),
    UnitStruct(UnitStruct),
}

pub struct Struct {
    struct_path: SimplePath,
    open_brace: Brace,
    struct_expr_fields_opt: Option<StructExprFields>,
    close_brace: Brace,
}

pub struct StructExprFields {
    first_field: StructExprField,
    subsequent_fields: Vec<(Comma, StructExprField)>,
}

pub struct StructExprField {
    attributes: Vec<Attribute>,
    data: (Identifier, Colon, Box<ExpressionKind>),
}

pub struct TupleStruct {
    tuple_struct_path: SimplePath,
    open_parenthesis: Parenthesis,
    params_opt: Option<(Box<ExpressionKind>, Vec<(Comma, ExpressionKind)>, Comma)>,
    close_parenthesis: Parenthesis,
}

pub struct TupleStructExprFields {
    first_field: TupleStructExprField,
    subsequent_fields: Vec<(Comma, TupleStructExprField)>,
}

pub struct TupleStructExprField {
    attributes: Vec<Attribute>,
    data: Box<ExpressionKind>,
}

pub struct UnitStruct(SimplePath);
