use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
    ty::Type,
};

use super::Visibility;

pub enum StructItem {
    Struct(Struct),
    TupleStruct(TupleStruct),
}

pub struct Struct {
    kw_struct: KeywordKind,
    name: Identifier,
    open_brace: (DelimKind, DelimOrientation),
    struct_item_fields_opt: Option<StructItemFields>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct StructItemFields {
    first_field: StructItemField,
    subsequent_fields: Vec<(PuncKind, StructItemField)>,
    trailing_comma_opt: Option<PuncKind>,
}

pub struct StructItemField {
    attributes: Vec<Attribute>,
    visibility_opt: Option<Visibility>,
    field_name: Identifier,
    colon: PuncKind,
    field_type: Type,
}

pub struct TupleStruct {
    kw_struct: KeywordKind,
    name: Identifier,
    open_parenthesis: (DelimKind, DelimOrientation),
    tuple_item_fields_opt: Option<TupleItemFields>,
    close_parenthesis: (DelimKind, DelimOrientation),
    semicolon: PuncKind,
}

pub struct TupleItemFields {
    first_field: TupleItemField,
    subsequent_fields: Vec<(PuncKind, TupleItemField)>,
    trailing_comma_opt: Option<PuncKind>,
}

pub struct TupleItemField {
    attributes: Vec<Attribute>,
    visibility_opt: Option<Visibility>,
    field_type: Type,
}
