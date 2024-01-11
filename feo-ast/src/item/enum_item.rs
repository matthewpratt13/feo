use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
};

use super::Comma;
use super::{
    struct_item::{StructItemFields, TupleItemFields},
    visibility::Visibility,
};

pub enum EnumVariantType {
    Struct(EnumVariantStruct),
    Tuple(EnumVariantTuple),
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
    subsequent_variants: Vec<(Comma, EnumVariant)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct EnumVariant {
    attributes: Vec<Attribute>,
    visibility_opt: Option<Visibility>,
    name: Identifier,
    variant_type_opt: Option<EnumVariantType>,
}

pub struct EnumVariantStruct {
    open_brace: (DelimKind, DelimOrientation),
    struct_item_fields_opt: Option<StructItemFields>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct EnumVariantTuple {
    open_parenthesis: (DelimKind, DelimOrientation),
    tuple_item_fields_opt: Option<TupleItemFields>,
    close_parenthesis: (DelimKind, DelimOrientation),
}
