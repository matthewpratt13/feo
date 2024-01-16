use crate::{
    expression::Attribute,
    identifier::Identifier,
    path::SimplePath,
    type_utils::{Brace, Colon, Comma, Parenthesis},
};

use super::Pattern;

pub struct StructPatt {
    struct_path: SimplePath,
    open_brace: Brace,
    struct_patt_fields_opt: Option<StructPattFields>,
    trailing_comma_opt: Option<Comma>,
    close_brace: Brace,
}

impl Pattern for StructPatt {}

pub struct StructPattFields {
    first_field: StructPattField,
    subsequent_fields: Vec<(Comma, StructPattField)>,
}

pub struct StructPattField {
    attributes: Vec<Attribute>,
    struct_pattern_kind: Identifier,
    colon: Colon,
    data: Box<dyn Pattern>,
}

pub struct TupleStructPatt {
    tuple_struct_path: SimplePath,
    open_parenthesis: Parenthesis,
    tuple_struct_fields_opt: Option<TupleStructItems>,
    close_parenthesis: Parenthesis,
}

impl Pattern for TupleStructPatt {}

pub struct TupleStructItems {
    first_item: Box<dyn Pattern>,
    subsequent_fields: Vec<(Comma, Box<dyn Pattern>)>,
    trailing_comma_opt: Option<Comma>,
}
