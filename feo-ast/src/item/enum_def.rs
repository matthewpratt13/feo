use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, KwEnum, Parenthesis},
    Identifier,
};

use crate::{attribute::OuterAttr, expression::TermCollection};

use super::{StructDefField, TupleStructDefField, VisibilityKind};

#[derive(Debug, Clone)]
pub enum EnumVariantType {
    Struct(EnumVariantStruct),
    Tuple(EnumVariantTuple),
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_enum: KwEnum,
    pub enum_name: Identifier,
    pub open_brace: Brace,
    pub enum_variants_opt: Option<TermCollection<EnumVariant>>,
    pub close_brace: Brace,
}

impl Spanned for EnumDef {
    fn span(&self) -> Span {
        let s1 = if let Some(a) = &self.attributes_opt {
            match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_enum.span(),
                },
            }
        } else {
            if let Some(v) = &self.visibility_opt {
                v.span()
            } else {
                self.kw_enum.span()
            }
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub variant_name: Identifier,
    pub variant_type_opt: Option<EnumVariantType>
}

#[derive(Debug, Clone)]
pub struct EnumVariantStruct {
    pub open_brace: Brace,
    pub fields_opt: Option<TermCollection<StructDefField>>,
    pub close_brace: Brace,
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
    pub open_parenthesis: Parenthesis,
    pub elements_opt: Option<TermCollection<TupleStructDefField>>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for EnumVariantTuple {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
