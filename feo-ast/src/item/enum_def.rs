use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Comma, KwEnum, Parenthesis},
    Identifier,
};

use crate::attribute::OuterAttr;

use super::{StructDefFields, TupleStructDefElements, VisibilityKind};

#[derive(Debug, Clone)]
pub enum EnumVariantType {
    Struct(EnumVariantStruct),
    Tuple(EnumVariantTuple),
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_enum: KwEnum,
    enum_name: Identifier,
    open_brace: Brace,
    enum_variants_opt: Option<EnumVariants>,
    close_brace: Brace,
}

impl Spanned for EnumDef {
    fn span(&self) -> Span {
        let s1 = match self.attributes.first() {
            Some(a) => a.span(),
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_enum.span(),
            },
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]

pub struct EnumVariants {
    first_variant: EnumVariant,
    subsequent_variants: Vec<(Comma, EnumVariant)>,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    variant_name: Identifier,
    variant_type_opt: Option<EnumVariantType>,
}

#[derive(Debug, Clone)]
pub struct EnumVariantStruct {
    open_brace: Brace,
    fields_opt: Option<StructDefFields>,
    close_brace: Brace,
}

impl Spanned for EnumVariantStruct {
    fn span(&self) -> Span {
        let s1 = self.open_brace.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariantTuple {
    open_parenthesis: Parenthesis,
    fields_opt: Option<TupleStructDefElements>,
    close_parenthesis: Parenthesis,
}

impl Spanned for EnumVariantTuple {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
