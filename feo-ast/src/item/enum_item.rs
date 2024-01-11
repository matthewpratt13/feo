use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
};

use super::visibility::Visibility;

pub enum EnumVariantType {
    Struct(EnumItemStruct),
    Tuple(EnumItemTuple),
}

pub struct EnumItem {
    kw_enum: KeywordKind,
    name: Identifier,
    open_brace: (DelimKind, DelimOrientation),
    enum_variants_opt: Option<EnumVariants>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct EnumVariants {
    first_variant: EnumVariant,
    subsequent_variants: Vec<(PuncKind, EnumVariant)>,
    trailing_comma_opt: Option<PuncKind>,
}

pub struct EnumVariant {
    attributes: Vec<Attribute>,
    visibility_opt: Option<Visibility>,
    name: Identifier,
    variant_type_opt: Option<EnumVariantType>,
}

pub struct EnumItemStruct {}

pub struct EnumItemTuple {}
