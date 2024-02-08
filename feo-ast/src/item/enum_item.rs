use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Comma, KwEnum, Parenthesis},
    Identifier,
};

use crate::{expression::OuterAttr, pattern::Pattern, ty::Type};

use super::{Item, StructFields, TupleElements, VisibilityKind};

#[derive(Clone)]
pub enum EnumVariantType {
    Struct(EnumVariantStruct),
    Tuple(EnumVariantTuple),
}

#[derive(Clone)]
pub struct EnumItem {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_enum: KwEnum,
    enum_name: Identifier,
    open_brace: Brace,
    enum_variants_opt: Option<EnumVariants>,
    close_brace: Brace,
}

impl Item for EnumItem {}

impl Type for EnumItem {}

impl Spanned for EnumItem {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_enum.span().start(),
            },
        };
        let end_pos = self.close_brace.span().end();
        let source = self.kw_enum.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]

pub struct EnumVariants {
    first_variant: EnumVariant,
    subsequent_variants: Vec<(Comma, EnumVariant)>,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Clone)]
pub struct EnumVariant {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    variant_name: Identifier,
    variant_type_opt: Option<EnumVariantType>,
}

#[derive(Clone)]
pub struct EnumVariantStruct {
    open_brace: Brace,
    fields_opt: Option<StructFields>,
    close_brace: Brace,
}

// impl Expression for EnumVariantStruct {}

impl Pattern for EnumVariantStruct {}

impl Spanned for EnumVariantStruct {
    fn span(&self) -> Span {
        let start_pos = self.open_brace.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.open_brace.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct EnumVariantTuple {
    open_parenthesis: Parenthesis,
    fields_opt: Option<TupleElements>,
    close_parenthesis: Parenthesis,
}

impl Pattern for EnumVariantTuple {}

impl Spanned for EnumVariantTuple {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
