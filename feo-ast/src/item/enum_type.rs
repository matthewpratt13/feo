use feo_types::span::{Span, Spanned};

use crate::{
    expression::{Constant, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    program::{ContractItem, LibraryItem},
    statement::Statement,
    ty::Type,
    type_utils::{Brace, Comma, Parenthesis},
};

use super::{Item, StructFields, TupleFields, VisibilityKind};

pub enum EnumVariantType {
    Struct(EnumVariantStruct),
    Tuple(EnumVariantTuple),
}

pub struct EnumType {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_enum: Keyword,
    enum_name: Identifier,
    open_brace: Brace,
    enum_variants_opt: Option<EnumVariants>,
    close_brace: Brace,
}

impl Item for EnumType {}

impl Statement for EnumType {}

impl LibraryItem for EnumType {}

impl Type for EnumType {}

impl ContractItem for EnumType {}

impl Spanned for EnumType {
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

pub struct EnumVariants {
    first_variant: EnumVariant,
    subsequent_variants: Vec<(Comma, EnumVariant)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct EnumVariant {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    variant_name: Identifier,
    variant_type_opt: Option<EnumVariantType>,
}

pub struct EnumVariantStruct {
    open_brace: Brace,
    fields_opt: Option<StructFields>,
    close_brace: Brace,
}

impl Constant for EnumVariantStruct {}

impl Spanned for EnumVariantStruct {
    fn span(&self) -> Span {
        let start_pos = self.open_brace.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.open_brace.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct EnumVariantTuple {
    open_parenthesis: Parenthesis,
    fields_opt: Option<TupleFields>,
    close_parenthesis: Parenthesis,
}

impl Constant for EnumVariantTuple {}

impl Spanned for EnumVariantTuple {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
