use crate::{
    identifier::Identifier,
    path::SimplePath,
    type_utils::{Brace, Colon, Comma, Parenthesis},
};

use super::{Attribute, Expression};

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
    data: (Identifier, Colon, Box<dyn Expression>),
}

pub struct TupleStruct {
    tuple_struct_path: SimplePath,
    open_parenthesis: Parenthesis,
    params_opt: Option<(
        Box<dyn Expression>,
        Vec<(Comma, Box<dyn Expression>)>,
        Comma,
    )>,
    close_parenthesis: Parenthesis,
}

pub struct TupleStructExprFields {
    first_field: TupleStructExprField,
    subsequent_fields: Vec<(Comma, TupleStructExprField)>,
}

pub struct TupleStructExprField {
    attributes: Vec<Attribute>,
    data: Box<dyn Expression>,
}

pub struct UnitStruct(SimplePath);
