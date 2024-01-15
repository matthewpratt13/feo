use crate::{
    identifier::Identifier,
    item::{Brace, Colon, Comma, Parenthesis},
    path::SimplePath,
};

use super::{Attribute, Expression};

pub enum StructExprKind {
    Struct(StructExpr),
    TupleStruct(TupleStructExpr),
    UnitStruct(UnitStructExpr),
}

pub struct StructExpr {
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
    data: (Identifier, Colon, Box<Expression>),
}

pub struct TupleStructExpr {
    tuple_struct_path: SimplePath,
    open_parenthesis: Parenthesis,
    params_opt: Option<(Box<Expression>, Vec<(Comma, Expression)>, Comma)>,
    close_parenthesis: Parenthesis,
}

pub struct TupleStructExprFields {
    first_field: TupleStructExprField,
    subsequent_fields: Vec<(Comma, TupleStructExprField)>,
}

pub struct TupleStructExprField {
    attributes: Vec<Attribute>,
    data: Box<Expression>,
}

pub struct UnitStructExpr(SimplePath);
