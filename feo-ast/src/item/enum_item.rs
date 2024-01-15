use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    type_utils::{Brace, Comma, Parenthesis},
};

use super::{
    struct_item::{StructItemFields, TupleItemFields},
    VisibilityKind,
};

pub enum EnumVariantType {
    Struct(EnumVariantStruct),
    Tuple(EnumVariantTuple),
}

pub struct EnumItem {
    kw_enum: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    enum_variants_opt: Option<EnumVariants>,
    close_brace: Brace,
}

pub struct EnumVariants {
    first_variant: EnumVariant,
    subsequent_variants: Vec<(Comma, EnumVariant)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct EnumVariant {
    attributes: Vec<Attribute>,
    visibility_opt: Option<VisibilityKind>,
    name: Identifier,
    variant_type_opt: Option<EnumVariantType>,
}

pub struct EnumVariantStruct {
    open_brace: Brace,
    struct_item_fields_opt: Option<StructItemFields>,
    close_brace: Brace,
}

pub struct EnumVariantTuple {
    open_parenthesis: Parenthesis,
    tuple_item_fields_opt: Option<TupleItemFields>,
    close_parenthesis: Parenthesis,
}
