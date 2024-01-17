use feo_types::span::{Span, Spanned};

use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    keyword::Keyword,
    program::LibraryItem,
    ty::Type,
    type_utils::{Brace, Comma, Parenthesis},
};

use super::{Item, StructFields, TupleFields, VisibilityKind, WhereClause};

pub enum EnumVariantType {
    Struct(EnumVariantStruct),
    Tuple(EnumVariantTuple),
}

pub struct EnumItem {
    visibility_opt: Option<VisibilityKind>,
    kw_enum: Keyword,
    name: Identifier,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    enum_variants_opt: Option<EnumVariants>,
    close_brace: Brace,
}

impl Item for EnumItem {}

impl<L> LibraryItem<L> for EnumItem where L: Item {}

impl Type for EnumItem {}

impl Spanned for EnumItem {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_enum.span().start()
        };

        let end_pos = self.close_brace.span().end();
        let source = self.name.span().source();

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
