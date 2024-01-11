use crate::{
    delimiter::{DelimKind, DelimOrientation},
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
};

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

pub struct StructItemFields {}

pub struct TupleStruct {}

pub struct TupleStructItemFields {}

pub struct TupleStructItemField {}
