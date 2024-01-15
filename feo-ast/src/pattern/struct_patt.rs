use crate::{
    expression::Attribute,
    identifier::Identifier,
    type_utils::{Brace, Colon, Comma, Parenthesis},
    path::SimplePath,
};

use super::PatternKind;

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
    data: Box<PatternKind>,
}

pub struct TupleStructPatt {
    tuple_struct_path: SimplePath,
    open_parenthesis: Parenthesis,
    tuple_struct_fields_opt: Option<TupleStructItems>,
    close_parenthesis: Parenthesis,
}

pub struct TupleStructItems {
    first_item: Box<PatternKind>,
    subsequent_fields: Vec<(Comma, PatternKind)>,
    trailing_comma_opt: Option<Comma>,
}
