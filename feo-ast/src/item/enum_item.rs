use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Brace, Comma, Parenthesis},
};

use super::{
    struct_item::{StructFields, TupleFields},
    Item, VisibilityKind, WhereClause,
};

pub enum EnumVariantType {
    Struct(EnumVariantStruct),
    Tuple(EnumVariantTuple),
}

pub struct EnumItem {
    visibility_opt: Option<VisibilityKind>,
    kw_enum: KeywordKind,
    name: Identifier,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    enum_variants_opt: Option<EnumVariants>,
    close_brace: Brace,
}

impl Item for EnumItem {}

impl Type for EnumItem {}

pub struct EnumVariants {
    first_variant: EnumVariant,
    subsequent_variants: Vec<(Comma, EnumVariant)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct EnumVariant {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    name: Identifier,
    variant_type_opt: Option<EnumVariantType>,
}

pub struct EnumVariantStruct {
    open_brace: Brace,
    struct_fields_opt: Option<StructFields>,
    close_brace: Brace,
}

pub struct EnumVariantTuple {
    open_parenthesis: Parenthesis,
    tuple_struct_fields_opt: Option<TupleFields>,
    close_parenthesis: Parenthesis,
}
