use crate::{expression::Attribute, identifier::Identifier, keyword::KeywordKind, ty::Type};

use super::{Brace, Colon, Comma, Parenthesis, Semicolon, Visibility};

pub enum StructItem {
    Struct(Struct),
    TupleStruct(TupleStruct),
}

pub struct Struct {
    kw_struct: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    struct_item_fields_opt: Option<StructItemFields>,
    close_brace: Brace,
}

pub struct StructItemFields {
    first_field: StructItemField,
    subsequent_fields: Vec<(Comma, StructItemField)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct StructItemField {
    attributes: Vec<Attribute>,
    visibility_opt: Option<Visibility>,
    field_name: Identifier,
    colon: Colon,
    field_type: Type,
}

pub struct TupleStruct {
    kw_struct: KeywordKind,
    name: Identifier,
    open_parenthesis: Parenthesis,
    tuple_item_fields_opt: Option<TupleItemFields>,
    close_parenthesis: Parenthesis,
    semicolon: Semicolon,
}

pub struct TupleItemFields {
    first_field: TupleItemField,
    subsequent_fields: Vec<(Comma, TupleItemField)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct TupleItemField {
    attributes: Vec<Attribute>,
    visibility_opt: Option<Visibility>,
    field_type: Type,
}
