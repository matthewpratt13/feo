use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Brace, Colon, Comma, Parenthesis, Semicolon},
};

use super::VisibilityKind;

// pub enum StructItemKind {
//     Struct(StructItem),
//     TupleStruct(TupleStructItem),
// }

pub struct StructItem {
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
    visibility_opt: Option<VisibilityKind>,
    field_name: Identifier,
    colon: Colon,
    field_type: Box<dyn Type>,
}

pub struct TupleStructItem {
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
    visibility_opt: Option<VisibilityKind>,
    field_type: Box<dyn Type>,
}
