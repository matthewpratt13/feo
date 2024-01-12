use crate::{
    expression::Attribute,
    identifier::Identifier,
    item::{Brace, Colon, Comma, Parenthesis},
    keyword::KeywordKind,
    path::SimplePath,
};

use super::Pattern;

pub enum StructPattKind {
    WithoutBody(StructWithoutBody),
    WithBody(StructWithBody),
}

pub struct StructWithoutBody {
    kw_ref_opt: Option<KeywordKind>,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
}

pub struct StructWithBody {
    name: Identifier,
    colon: Colon,
    pattern: Box<Pattern>,
}

pub struct StructPatt {
    path: SimplePath,
    open_brace: Brace,
    struct_patt_fields_opt: Option<StructPattFields>,
    trailing_comma_opt: Option<Comma>,
    close_brace: Brace,
}

pub struct StructPattFields {
    first_field: StructPattField,
    subsequent_fields: Vec<(Comma, StructPattField)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct StructPattField {
    attribute: Attribute,
    struct_pattern_kind: StructPattKind,
}

pub struct TupleStructPatt {
    path: SimplePath,
    open_parenthesis: Parenthesis,
    tuple_struct_fields_opt: Option<TupleStructFields>,
    close_parenthesis: Parenthesis,
}

pub struct TupleStructFields {
    first_field: Box<Pattern>,
    subsequent_fields: Vec<(Comma, Pattern)>,
    trailing_comma_opt: Option<Comma>,
}
