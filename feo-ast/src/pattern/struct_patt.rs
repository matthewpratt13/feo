use crate::{
    expression::Attribute,
    identifier::Identifier,
    item::{Brace, Colon, Comma, Parenthesis},
    path::SimplePath,
};

use super::Pattern;

pub struct StructPatt {
    struct_path: SimplePath,
    open_brace: Brace,
    struct_patt_fields_opt: Option<StructPattFields>,
    trailing_comma_opt: Option<Comma>,
    close_brace: Brace,
}

pub struct StructPattFields {
    first_field: StructPattField,
    subsequent_fields: Vec<(Comma, StructPattField)>,
}

pub struct StructPattField {
    attributes: Vec<Attribute>,
    struct_pattern_kind: Identifier,
    colon: Colon,
    data: Box<Pattern>,
}

pub struct TupleStructPatt {
    tuple_struct_path: SimplePath,
    open_parenthesis: Parenthesis,
    tuple_struct_fields_opt: Option<TupleStructItems>,
    close_parenthesis: Parenthesis,
}

pub struct TupleStructItems {
    first_item: Box<Pattern>,
    subsequent_fields: Vec<(Comma, Pattern)>,
    trailing_comma_opt: Option<Comma>,
}
